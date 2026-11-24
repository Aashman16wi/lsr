//! Filesystem operations - collecting and reading directory entries

use anyhow::{Context, Result};
use globset::{GlobBuilder, GlobMatcher};
use ignore::{DirEntry, Walk, WalkBuilder};
use rayon::prelude::*;
use std::collections::HashMap;
use std::fs;
use std::os::unix::fs::{FileTypeExt, MetadataExt};
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use crate::cli::{Args, SortField, TimeField};
use crate::git::GitRepo;

/// Represents a filesystem entry with all metadata
#[derive(Debug, Clone)]
pub struct EntryData {
    pub path: PathBuf,
    pub name: String,
    pub file_type: FileType,
    pub size: u64,
    pub total_size: u64, // For --du: total size of directory tree
    pub blocks: u64,
    pub permissions: u32,
    pub nlink: u64,
    pub uid: u32,
    pub gid: u32,
    pub inode: u64,
    pub modified: SystemTime,
    pub accessed: SystemTime,
    pub created: Option<SystemTime>,
    pub changed: SystemTime,
    pub symlink_target: Option<PathBuf>,
    pub git_status: Option<GitStatus>,
    pub mime_type: Option<String>,
}

/// Git status for a file
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GitStatus {
    Unmodified,
    Modified,
    Added,
    Deleted,
    Renamed,
    Copied,
    Untracked,
    Ignored,
    Conflicted,
}

/// File type classification
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileType {
    Regular,
    Directory,
    Symlink,
    BlockDevice,
    CharDevice,
    Fifo,
    Socket,
    Unknown,
}

impl FileType {
    pub fn from_metadata(metadata: &fs::Metadata) -> Self {
        let ft = metadata.file_type();
        if ft.is_dir() {
            FileType::Directory
        } else if ft.is_file() {
            FileType::Regular
        } else if ft.is_symlink() {
            FileType::Symlink
        } else if ft.is_block_device() {
            FileType::BlockDevice
        } else if ft.is_char_device() {
            FileType::CharDevice
        } else if ft.is_fifo() {
            FileType::Fifo
        } else if ft.is_socket() {
            FileType::Socket
        } else {
            FileType::Unknown
        }
    }

    pub fn from_dir_entry(entry: &DirEntry) -> Self {
        entry
            .file_type()
            .map(|ft| {
                if ft.is_dir() {
                    FileType::Directory
                } else if ft.is_file() {
                    FileType::Regular
                } else if ft.is_symlink() {
                    FileType::Symlink
                } else if ft.is_block_device() {
                    FileType::BlockDevice
                } else if ft.is_char_device() {
                    FileType::CharDevice
                } else if ft.is_fifo() {
                    FileType::Fifo
                } else if ft.is_socket() {
                    FileType::Socket
                } else {
                    FileType::Unknown
                }
            })
            .unwrap_or(FileType::Unknown)
    }

    pub fn classify_char(&self, perms: u32) -> char {
        match self {
            FileType::Directory => '/',
            FileType::Symlink => '@',
            FileType::Regular if perms & 0o111 != 0 => '*',
            FileType::Fifo => '|',
            FileType::Socket => '=',
            _ => ' ',
        }
    }
}

/// Collect all entries for the given paths.
///
/// `max_depth` controls how deep the walker descends:
/// `Some(1)` for a normal listing, `Some(n)` for a depth-limited tree,
/// and `None` for a fully recursive listing.
pub fn collect_entries(
    paths: &[PathBuf],
    args: &Args,
    git_repos: &HashMap<PathBuf, GitRepo>,
    max_depth: Option<usize>,
    include_git: bool,
    group_dirs_first: bool,
) -> Result<Vec<EntryData>> {
    let mut all_entries = Vec::new();

    for path in paths {
        let entries = collect_single_path(path, args, max_depth)?;
        all_entries.extend(entries);
    }

    // Add git status if enabled
    if include_git && !git_repos.is_empty() {
        for entry in &mut all_entries {
            entry.git_status = get_git_status(&entry.path, git_repos);
        }
    }

    // Apply include globs before sorting so the visible set is what gets sorted.
    if !args.glob_patterns.is_empty() {
        let matchers = compile_globs(&args.glob_patterns);
        all_entries.retain(|entry| entry_matches(&entry.name, &entry.path, &matchers));
    }

    // Sort entries
    sort_entries(&mut all_entries, args, group_dirs_first);

    // Filter by content grep if requested
    if let Some(ref pattern) = args.grep_pattern {
        filter_by_grep(&mut all_entries, pattern)?;
    }

    // Calculate directory sizes if --du is enabled
    if args.du {
        calculate_total_sizes(&mut all_entries, args);
    }

    Ok(all_entries)
}

fn collect_single_path(
    path: &Path,
    args: &Args,
    max_depth: Option<usize>,
) -> Result<Vec<EntryData>> {
    let mut entries = Vec::new();

    // Check if path exists
    let metadata = if args.dereference {
        fs::metadata(path).with_context(|| format!("Failed to stat {}", path.display()))?
    } else {
        fs::symlink_metadata(path).with_context(|| format!("Failed to lstat {}", path.display()))?
    };

    if metadata.is_dir() && !args.dereference {
        // It's a directory - walk it
        let walker = build_walker(path, args, max_depth);

        if args.no_parallel || args.threads == 1 {
            // Sequential
            for result in walker {
                match result {
                    Ok(entry) => {
                        // The walker yields the root directory itself; skip it.
                        if entry.depth() == 0 {
                            continue;
                        }
                        if let Some(entry_data) = process_entry(&entry, args)? {
                            entries.push(entry_data);
                        }
                    }
                    Err(e) => {
                        if args.verbose {
                            eprintln!("Warning: {}", e);
                        }
                    }
                }
            }
        } else {
            // Parallel using rayon - process in chunks to avoid memory issues with large directories
            // Collect results in batches to prevent capacity overflow
            let mut chunk = Vec::with_capacity(10000);
            for result in walker {
                match result {
                    Ok(entry) => {
                        if entry.depth() == 0 {
                            continue;
                        }
                        chunk.push(entry);
                        if chunk.len() >= 10000 {
                            // Process chunk in parallel
                            let processed: Vec<_> = chunk
                                .into_iter()
                                .collect::<Vec<_>>()
                                .into_par_iter()
                                .filter_map(|entry| process_entry(&entry, args).ok().flatten())
                                .collect();
                            entries.extend(processed);
                            chunk = Vec::with_capacity(10000);
                        }
                    }
                    Err(e) => {
                        if args.verbose {
                            eprintln!("Warning: {}", e);
                        }
                    }
                }
            }
            // Process remaining chunk
            if !chunk.is_empty() {
                let processed: Vec<_> = chunk
                    .into_iter()
                    .collect::<Vec<_>>()
                    .into_par_iter()
                    .filter_map(|entry| process_entry(&entry, args).ok().flatten())
                    .collect();
                entries.extend(processed);
            }
        }
    } else {
        // Single file or symlink to file
        if let Some(entry_data) = process_single_path(path, args)? {
            entries.push(entry_data);
        }
    }

    Ok(entries)
}

fn build_walker(root: &Path, args: &Args, max_depth: Option<usize>) -> Walk {
    let mut builder = WalkBuilder::new(root);
    builder
        .hidden(!args.all && !args.almost_all)
        .parents(false)
        // `ls` does not hide gitignored files, so neither do we. Users can
        // still filter explicitly with --ignore.
        .git_ignore(false)
        .git_global(false)
        .git_exclude(false)
        .ignore(false)
        .require_git(false)
        .max_depth(max_depth)
        .follow_links(args.dereference);

    // Custom ignore patterns are globs, not ignore-file paths.
    if !args.ignore_patterns.is_empty() {
        let matchers = std::sync::Arc::new(compile_globs(&args.ignore_patterns));
        let root = root.to_path_buf();
        builder.filter_entry(move |entry| {
            if entry.depth() == 0 {
                return true;
            }
            let name = entry.file_name().to_string_lossy();
            let relative = entry.path().strip_prefix(&root).unwrap_or(entry.path());
            !matchers
                .iter()
                .any(|matcher| matcher.is_match(name.as_ref()) || matcher.is_match(relative))
        });
    }

    // Thread count
    if args.threads > 0 {
        builder.threads(args.threads);
    }

    builder.build()
}

/// Compile a list of user globs, ignoring malformed patterns.
fn compile_globs(patterns: &[String]) -> Vec<GlobMatcher> {
    patterns
        .iter()
        .filter_map(|pattern| {
            let pattern = pattern.trim_end_matches('/');
            GlobBuilder::new(pattern)
                .literal_separator(true)
                .build()
                .ok()
                .map(|glob| glob.compile_matcher())
        })
        .collect()
}

/// Match either the file name (`*.rs`) or the path relative to the listing root.
fn entry_matches(name: &str, path: &Path, matchers: &[GlobMatcher]) -> bool {
    let relative = path.strip_prefix(".").unwrap_or(path);
    matchers
        .iter()
        .any(|matcher| matcher.is_match(name) || matcher.is_match(relative))
}

fn process_entry(entry: &DirEntry, args: &Args) -> Result<Option<EntryData>> {
    let path = entry.path();

    // Skip . and .. unless -a
    if !args.all {
        let name = entry.file_name().to_string_lossy();
        if name == "." || name == ".." {
            return Ok(None);
        }
    }

    // Check file type filter
    if !args.file_types.is_empty() {
        let ft = FileType::from_dir_entry(entry);
        let matches = args.file_types.iter().any(|t| matches_type(t, ft));
        if !matches {
            return Ok(None);
        }
    }

    process_single_path(path, args)
}

fn process_single_path(path: &Path, args: &Args) -> Result<Option<EntryData>> {
    let metadata = if args.dereference {
        fs::metadata(path)?
    } else {
        fs::symlink_metadata(path)?
    };

    let file_type = FileType::from_metadata(&metadata);

    // Get symlink target
    let symlink_target = if file_type == FileType::Symlink {
        Some(fs::read_link(path)?)
    } else {
        None
    };

    // Get MIME type
    let mime_type = mime_guess::from_path(path)
        .first_raw()
        .map(|s| s.to_string());

    let entry = EntryData {
        path: path.to_path_buf(),
        name: path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string(),
        file_type,
        size: metadata.len(),
        total_size: metadata.len(), // Default to file size
        blocks: metadata.blocks(),
        permissions: metadata.mode() & 0o7777,
        nlink: metadata.nlink(),
        uid: metadata.uid(),
        gid: metadata.gid(),
        inode: metadata.ino(),
        modified: metadata.modified()?,
        accessed: metadata.accessed()?,
        created: metadata.created().ok(),
        changed: {
            let ctime = metadata.ctime();
            SystemTime::UNIX_EPOCH + std::time::Duration::new(ctime as u64, 0)
        },
        symlink_target,
        git_status: None, // Will be filled later if needed
        mime_type,
    };

    Ok(Some(entry))
}

fn matches_type(filter: &str, ft: FileType) -> bool {
    match filter.to_lowercase().as_str() {
        "f" | "file" => matches!(ft, FileType::Regular),
        "d" | "dir" | "directory" => matches!(ft, FileType::Directory),
        "l" | "link" | "symlink" => matches!(ft, FileType::Symlink),
        "b" | "block" => matches!(ft, FileType::BlockDevice),
        "c" | "char" => matches!(ft, FileType::CharDevice),
        "p" | "fifo" | "pipe" => matches!(ft, FileType::Fifo),
        "s" | "socket" => matches!(ft, FileType::Socket),
        _ => false,
    }
}

fn get_git_status(path: &Path, git_repos: &HashMap<PathBuf, GitRepo>) -> Option<GitStatus> {
    // A file can be inside nested repositories. Always use the most specific
    // (longest) matching repository path.
    let (repo_path, repo) = git_repos
        .iter()
        .filter(|(repo_path, _)| path.starts_with(repo_path))
        .max_by_key(|(repo_path, _)| repo_path.as_os_str().len())?;

    repo.get_status(path.strip_prefix(repo_path).ok()?)
}

fn sort_entries(entries: &mut [EntryData], args: &Args, group_dirs_first: bool) {
    let reverse = args.reverse;
    let group_dirs = group_dirs_first;

    entries.sort_by(|a, b| {
        // Group directories first
        if group_dirs {
            let a_is_dir = matches!(a.file_type, FileType::Directory);
            let b_is_dir = matches!(b.file_type, FileType::Directory);
            if a_is_dir != b_is_dir {
                return if a_is_dir {
                    std::cmp::Ordering::Less
                } else {
                    std::cmp::Ordering::Greater
                };
            }
        }

        // Primary sort
        let cmp = match args.sort {
            SortField::Name => a.name.cmp(&b.name),
            SortField::Size => a.size.cmp(&b.size),
            SortField::Time => {
                let a_time = get_time_field(a, args.time_field);
                let b_time = get_time_field(b, args.time_field);
                a_time.cmp(&b_time)
            }
            SortField::Extension => {
                let a_ext = a.path.extension().and_then(|s| s.to_str()).unwrap_or("");
                let b_ext = b.path.extension().and_then(|s| s.to_str()).unwrap_or("");
                a_ext.cmp(b_ext).then_with(|| a.name.cmp(&b.name))
            }
            SortField::Inode => a.inode.cmp(&b.inode),
            SortField::None => std::cmp::Ordering::Equal,
        };

        if reverse { cmp.reverse() } else { cmp }
    });

    // Apply grouping if requested (--group)
    if let Some(ref group_mode) = args.group_by {
        apply_grouping(entries, group_mode);
    }
}

/// Apply grouping to entries (--group ext|date|size)
fn apply_grouping(entries: &mut [EntryData], mode: &str) {
    match mode {
        "ext" => {
            // Group by file extension
            entries.sort_by(|a, b| {
                let a_ext = a.path.extension().and_then(|s| s.to_str()).unwrap_or("");
                let b_ext = b.path.extension().and_then(|s| s.to_str()).unwrap_or("");
                a_ext.cmp(b_ext).then_with(|| a.name.cmp(&b.name))
            });
        }
        "date" => {
            // Group by modification date (day)
            entries.sort_by(|a, b| {
                let a_day = system_time_secs(a.modified) / 86_400;
                let b_day = system_time_secs(b.modified) / 86_400;
                b_day.cmp(&a_day).then_with(|| a.name.cmp(&b.name)) // newest first
            });
        }
        "size" => {
            // Group by size buckets
            entries.sort_by(|a, b| {
                let a_bucket = size_bucket(a.size);
                let b_bucket = size_bucket(b.size);
                b_bucket.cmp(&a_bucket).then_with(|| a.size.cmp(&b.size)) // largest first
            });
        }
        _ => {}
    }
}

fn size_bucket(size: u64) -> u8 {
    if size == 0 {
        return 0;
    }
    if size < 1_024 {
        return 1;
    } // < 1KB
    if size < 10 * 1_024 {
        return 2;
    } // < 10KB
    if size < 100 * 1_024 {
        return 3;
    } // < 100KB
    if size < 1_024 * 1_024 {
        return 4;
    } // < 1MB
    if size < 10 * 1_024 * 1_024 {
        return 5;
    } // < 10MB
    if size < 100 * 1_024 * 1_024 {
        return 6;
    } // < 100MB
    if size < 1_024 * 1_024 * 1_024 {
        return 7;
    } // < 1GB
    8 // >= 1GB
}

fn system_time_secs(time: SystemTime) -> u64 {
    time.duration_since(std::time::UNIX_EPOCH)
        .map(|duration| duration.as_secs())
        .unwrap_or(0)
}

fn get_time_field(entry: &EntryData, field: TimeField) -> SystemTime {
    match field {
        TimeField::Modified => entry.modified,
        TimeField::Accessed => entry.accessed,
        TimeField::Created => entry.created.unwrap_or(entry.modified),
        TimeField::Changed => entry.changed,
    }
}

impl EntryData {
    /// Convert to JSON value
    pub fn to_json(&self, _opts: &crate::format::FormatOptions) -> serde_json::Value {
        use serde_json::json;
        json!({
            "name": self.name,
            "path": self.path.to_string_lossy(),
            "size": self.size,
            "total_size": self.total_size,
            "blocks": self.blocks,
            "permissions": format!("{:o}", self.permissions),
            "nlink": self.nlink,
            "uid": self.uid,
            "gid": self.gid,
            "inode": self.inode,
            "modified": system_time_secs(self.modified),
            "accessed": system_time_secs(self.accessed),
            "created": self.created.map(system_time_secs),
            "changed": system_time_secs(self.changed),
            "file_type": format!("{:?}", self.file_type).to_lowercase(),
            "git_status": self.git_status.map(|s| format!("{:?}", s).to_lowercase()),
            "mime_type": self.mime_type,
            "symlink_target": self.symlink_target.as_ref().map(|p| p.to_string_lossy().to_string()),
        })
    }

    /// Convert to CSV row
    pub fn to_csv(&self, _opts: &crate::format::FormatOptions) -> String {
        let escape = |s: &str| {
            if s.contains(',') || s.contains('"') || s.contains('\n') {
                format!("\"{}\"", s.replace('"', "\"\""))
            } else {
                s.to_string()
            }
        };

        let permissions = format!("{:o}", self.permissions);
        let file_type = format!("{:?}", self.file_type).to_lowercase();
        let git_status = self
            .git_status
            .map(|s| format!("{:?}", s).to_lowercase())
            .unwrap_or_default();
        let modified = self
            .modified
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);

        format!(
            "{},{},{},{},{},{},{},{},{}",
            escape(&self.name),
            escape(&self.path.to_string_lossy()),
            self.size,
            modified,
            permissions,
            self.uid,
            self.gid,
            file_type,
            git_status,
        )
    }
}

/// Calculate total sizes for directories (used by --du).
///
/// In a normal listing only top-level entries are present, so each directory is
/// walked to get its real recursive size. In tree/recursive listings the
/// descendants are already present and are folded bottom-up instead.
fn calculate_total_sizes(entries: &mut [EntryData], args: &Args) {
    let mut total_sizes: HashMap<PathBuf, u64> = entries
        .iter()
        .map(|entry| (entry.path.clone(), entry.size))
        .collect();

    let mut dirs: Vec<PathBuf> = entries
        .iter()
        .filter(|entry| entry.file_type == FileType::Directory)
        .map(|entry| entry.path.clone())
        .collect();
    dirs.sort_by_key(|path| std::cmp::Reverse(path.components().count()));

    for dir_path in dirs {
        let has_visible_child = entries
            .iter()
            .any(|entry| entry.path != dir_path && entry.path.starts_with(&dir_path));

        if !has_visible_child {
            total_sizes.insert(dir_path.clone(), directory_size(&dir_path, args));
        }

        let dir_size = *total_sizes.get(&dir_path).unwrap_or(&0);
        if let Some(parent) = dir_path.parent() {
            *total_sizes.entry(parent.to_path_buf()).or_insert(0) += dir_size;
        }
    }

    for entry in entries.iter_mut() {
        entry.total_size = *total_sizes.get(&entry.path).unwrap_or(&entry.size);
    }
}

/// Recursively sum the size of regular files in a directory.
fn directory_size(path: &Path, args: &Args) -> u64 {
    let mut total = 0u64;

    for result in build_walker(path, args, None) {
        let Ok(entry) = result else { continue };
        if entry.depth() == 0 {
            continue;
        }
        if entry.file_type().map(|ft| ft.is_file()).unwrap_or(false) {
            if let Ok(metadata) = entry.metadata() {
                total += metadata.len();
            }
        }
    }

    total
}

/// Filter entries by content grep pattern.
/// Only searches regular text files (skips binary, large files).
pub fn filter_by_grep(entries: &mut Vec<EntryData>, pattern: &str) -> Result<()> {
    use std::io::Read;

    const MAX_GREP_SIZE: u64 = 10_000_000;
    let pattern_lower = pattern.to_lowercase();

    entries.retain(|entry| {
        // Only grep regular files, skip directories, symlinks, etc.
        if entry.file_type != FileType::Regular {
            return true;
        }

        // Avoid reading enormous files into memory.
        if entry.size > MAX_GREP_SIZE {
            return true;
        }

        let Ok(file) = std::fs::File::open(&entry.path) else {
            return true; // Can't read, keep it
        };

        let mut content = Vec::with_capacity(entry.size as usize);
        if file.take(MAX_GREP_SIZE).read_to_end(&mut content).is_err() {
            return true;
        }

        // Binary files are not searched.
        if content.contains(&0) {
            return true;
        }

        String::from_utf8_lossy(&content)
            .to_lowercase()
            .contains(&pattern_lower)
    });

    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;

    fn entry_for(path: &Path, size: u64) -> EntryData {
        EntryData {
            path: path.to_path_buf(),
            name: path
                .file_name()
                .map(|name| name.to_string_lossy().into_owned())
                .unwrap_or_default(),
            file_type: FileType::Regular,
            size,
            total_size: size,
            blocks: 0,
            permissions: 0o644,
            nlink: 1,
            uid: 0,
            gid: 0,
            inode: 0,
            modified: SystemTime::UNIX_EPOCH,
            accessed: SystemTime::UNIX_EPOCH,
            created: Some(SystemTime::UNIX_EPOCH),
            changed: SystemTime::UNIX_EPOCH,
            symlink_target: None,
            git_status: None,
            mime_type: None,
        }
    }

    #[test]
    fn size_buckets_are_monotonic() {
        assert_eq!(size_bucket(0), 0);
        assert_eq!(size_bucket(1_023), 1);
        assert_eq!(size_bucket(1_024), 2);
        assert_eq!(size_bucket(10 * 1_024), 3);
        assert_eq!(size_bucket(100 * 1_024), 4);
        assert_eq!(size_bucket(1_024 * 1_024), 5);
        assert_eq!(size_bucket(10 * 1_024 * 1_024), 6);
        assert_eq!(size_bucket(100 * 1_024 * 1_024), 7);
        assert_eq!(size_bucket(1_024 * 1_024 * 1_024), 8);
    }

    #[test]
    fn pre_epoch_times_do_not_panic() {
        let before_epoch = SystemTime::UNIX_EPOCH - std::time::Duration::from_secs(60);
        assert_eq!(system_time_secs(before_epoch), 0);
    }

    #[test]
    fn globs_match_names_and_relative_paths() {
        let matchers = compile_globs(&["*.rs".to_string(), "src/**".to_string()]);

        assert!(entry_matches("main.rs", Path::new("main.rs"), &matchers));
        assert!(entry_matches("lib.rs", Path::new("src/lib.rs"), &matchers));
        assert!(!entry_matches(
            "README.md",
            Path::new("README.md"),
            &matchers
        ));
    }

    #[test]
    fn single_star_does_not_cross_directories() {
        let matchers = compile_globs(&["a/*.log".to_string()]);

        assert!(entry_matches("x.log", Path::new("a/x.log"), &matchers));
        assert!(!entry_matches("y.log", Path::new("a/b/y.log"), &matchers));

        let recursive = compile_globs(&["a/**/*.log".to_string()]);
        assert!(entry_matches("y.log", Path::new("a/b/y.log"), &recursive));
    }

    #[test]
    fn grep_scans_past_the_first_chunk() {
        let dir = std::env::temp_dir().join(format!("lsr-grep-test-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();

        let file = dir.join("long.txt");
        let mut content = "x".repeat(2_000);
        content.push_str("NEEDLE");
        std::fs::write(&file, &content).unwrap();

        let mut entries = vec![entry_for(&file, content.len() as u64)];
        filter_by_grep(&mut entries, "needle").unwrap();

        assert_eq!(entries.len(), 1);
        std::fs::remove_dir_all(&dir).ok();
    }
}
