//! Filesystem operations - collecting and reading directory entries

use anyhow::{Context, Result};
use ignore::{WalkBuilder, Walk, DirEntry};
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
        entry.file_type()
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

    pub fn classify_char(&self) -> char {
        match self {
            FileType::Directory => '/',
            FileType::Symlink => '@',
            FileType::Regular => '*',
            FileType::Fifo => '|',
            FileType::Socket => '=',
            _ => ' ',
        }
    }
}

/// Collect all entries for the given paths
pub fn collect_entries(paths: &[PathBuf], args: &Args, git_repos: &HashMap<PathBuf, GitRepo>) -> Result<Vec<EntryData>> {
    let mut all_entries = Vec::new();

    for path in paths {
        let entries = collect_single_path(path, args)?;
        all_entries.extend(entries);
    }

    // Add git status if enabled
    if args.git && !git_repos.is_empty() {
        for entry in &mut all_entries {
            entry.git_status = get_git_status(&entry.path, git_repos);
        }
    }

    // Sort entries
    sort_entries(&mut all_entries, args);

    Ok(all_entries)
}

fn collect_single_path(path: &Path, args: &Args) -> Result<Vec<EntryData>> {
    let mut entries = Vec::new();

    // Check if path exists
    let metadata = if args.dereference {
        fs::metadata(path).with_context(|| format!("Failed to stat {}", path.display()))?
    } else {
        fs::symlink_metadata(path).with_context(|| format!("Failed to lstat {}", path.display()))?
    };

    if metadata.is_dir() && !args.dereference {
        // It's a directory - walk it
        let walker = build_walker(path, args);
        
        if args.no_parallel || args.threads == 1 {
            // Sequential
            for result in walker {
                match result {
                    Ok(entry) => {
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
                        chunk.push(entry);
                        if chunk.len() >= 10000 {
                            // Process chunk in parallel
                            let processed: Vec<_> = chunk
                                .into_iter()
                                .collect::<Vec<_>>()
                                .into_par_iter()
                                .filter_map(|entry| {
                                    process_entry(&entry, args).ok().flatten()
                                })
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
                    .filter_map(|entry| {
                        process_entry(&entry, args).ok().flatten()
                    })
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

fn build_walker(root: &Path, args: &Args) -> Walk {
    let mut builder = WalkBuilder::new(root);
    builder
        .hidden(!args.all && !args.almost_all)
        .parents(false)
        .git_ignore(true)
        .git_global(true)
        .git_exclude(true)
        .ignore(true)
        .require_git(false)
        .max_depth(if args.recursive { None } else { Some(1) })
        .follow_links(args.dereference);

    // Add custom ignore patterns
    for pattern in &args.ignore_patterns {
        builder.add_ignore(pattern);
    }

    // Thread count
    if args.threads > 0 {
        builder.threads(args.threads);
    }

    builder.build()
}

fn process_entry(
    entry: &DirEntry,
    args: &Args,
) -> Result<Option<EntryData>> {
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

fn process_single_path(
    path: &Path,
    args: &Args,
) -> Result<Option<EntryData>> {
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
    let mime_type = mime_guess::from_path(path).first_raw().map(|s| s.to_string());

    let entry = EntryData {
        path: path.to_path_buf(),
        name: path.file_name().unwrap_or_default().to_string_lossy().to_string(),
        file_type,
        size: metadata.len(),
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
    for (repo_path, repo) in git_repos {
        if path.starts_with(repo_path) {
            return repo.get_status(path.strip_prefix(repo_path).ok()?);
        }
    }
    None
}

fn sort_entries(entries: &mut [EntryData], args: &Args) {
    let reverse = args.reverse;
    let group_dirs = args.group_dirs_first;

    entries.sort_by(|a, b| {
        // Group directories first
        if group_dirs {
            let a_is_dir = matches!(a.file_type, FileType::Directory);
            let b_is_dir = matches!(b.file_type, FileType::Directory);
            if a_is_dir != b_is_dir {
                return if a_is_dir { std::cmp::Ordering::Less } else { std::cmp::Ordering::Greater };
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
            "blocks": self.blocks,
            "permissions": format!("{:o}", self.permissions),
            "nlink": self.nlink,
            "uid": self.uid,
            "gid": self.gid,
            "inode": self.inode,
            "modified": self.modified.duration_since(std::time::UNIX_EPOCH).unwrap().as_secs(),
            "accessed": self.accessed.duration_since(std::time::UNIX_EPOCH).unwrap().as_secs(),
            "created": self.created.map(|t| t.duration_since(std::time::UNIX_EPOCH).unwrap().as_secs()),
            "changed": self.changed.duration_since(std::time::UNIX_EPOCH).unwrap().as_secs(),
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

        format!(
            "{},{},{},{},{},{},{},{},{}",
            escape(&self.name),
            escape(&self.path.to_string_lossy()),
            self.size,
            self.modified.duration_since(std::time::UNIX_EPOCH).unwrap().as_secs(),
            format!("{:o}", self.permissions),
            self.uid,
            self.gid,
            format!("{:?}", self.file_type).to_lowercase(),
            self.git_status.map(|s| format!("{:?}", s).to_lowercase()).unwrap_or_default(),
        )
    }
}