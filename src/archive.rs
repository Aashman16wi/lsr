//! Archive listing support (zip, tar, tar.gz, tar.zst)

use anyhow::{Context, Result};
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::cli::Args;
use crate::format::FormatOptions;
use crate::fs::{EntryData, FileType};
use crate::output::print_trees;

/// Check if a path is an archive file we can read
pub fn is_archive(path: &Path) -> bool {
    let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("");
    matches!(
        ext.to_lowercase().as_str(),
        "zip" | "tar" | "gz" | "tgz" | "zst" | "tzst"
    )
}

/// List contents of an archive file (CLI entry point)
pub fn list_archive_cli(args: Args) -> Result<()> {
    let path = &args.paths[0];
    if !path.exists() {
        anyhow::bail!("archive: {}: no such file", path.display());
    }
    if path.is_dir() {
        anyhow::bail!(
            "archive: {} is a directory; pass an archive file (zip, tar, tar.gz, tar.zst)",
            path.display()
        );
    }
    if !is_archive(path) {
        anyhow::bail!("archive: unsupported file type: {}", path.display());
    }

    let entries = list_archive(path)?;

    let config =
        match crate::config::LsrConfig::load(args.theme.as_deref(), args.config_file.as_deref()) {
            Ok(config) => config,
            Err(error) => {
                eprintln!("lsr: warning: failed to load config: {error}");
                crate::config::LsrConfig::default()
            }
        };
    let effective = config.for_path(path);
    let depth = if args.depth == usize::MAX {
        effective.format.depth
    } else {
        args.depth
    };

    let format_opts = FormatOptions::from_args_and_config(&args, &config);

    // Archive entry paths are relative to the archive root, so use an empty
    // root path rather than the archive filename.
    let root = PathBuf::new();
    print_trees(
        &entries,
        &format_opts,
        depth,
        std::slice::from_ref(&root),
        1000,
    )?;

    Ok(())
}

/// List contents of an archive file
fn list_archive(path: &Path) -> Result<Vec<EntryData>> {
    let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("");
    let ext_lower = ext.to_lowercase();

    match ext_lower.as_str() {
        "zip" => list_zip(path),
        "tar" => list_tar(path),
        "gz" | "tgz" => list_tar_gz(path),
        "zst" | "tzst" => list_tar_zst(path),
        _ => anyhow::bail!("Unsupported archive format: {}", ext),
    }
}

/// Build an entry for a path inside an archive.
///
/// Archive paths are often prefixed with `./` and directories end in `/`.
/// Normalise those so the tree output is clean.
fn archive_entry(name: &str, is_dir: bool, size: u64, modified: SystemTime) -> Option<EntryData> {
    let normalized = name.trim_end_matches('/');
    let normalized = normalized.strip_prefix("./").unwrap_or(normalized);
    if normalized.is_empty() || normalized == "." {
        return None;
    }

    let path = PathBuf::from(normalized);
    let display_name = path
        .file_name()
        .map(|name| name.to_string_lossy().into_owned())
        .unwrap_or_else(|| normalized.to_string());

    Some(EntryData {
        path,
        name: display_name,
        file_type: if is_dir {
            FileType::Directory
        } else {
            FileType::Regular
        },
        size,
        total_size: size,
        blocks: 0,
        permissions: if is_dir { 0o755 } else { 0o644 },
        nlink: 1,
        uid: 0,
        gid: 0,
        inode: 0,
        modified,
        accessed: modified,
        created: Some(modified),
        changed: modified,
        symlink_target: None,
        git_status: None,
        mime_type: None,
    })
}

fn list_zip(path: &Path) -> Result<Vec<EntryData>> {
    let file = File::open(path).context("Failed to open zip file")?;
    let mut archive = zip::ZipArchive::new(file).context("Failed to read zip archive")?;
    let mut entries = Vec::new();

    for i in 0..archive.len() {
        let file = archive.by_index(i).context("Failed to read zip entry")?;
        let name = file.name().to_string();
        let is_dir = file.is_dir();
        let size = file.size();
        let modified = to_system_time(file.last_modified());

        if let Some(entry) = archive_entry(&name, is_dir, size, modified) {
            entries.push(entry);
        }
    }

    Ok(entries)
}

fn list_tar(path: &Path) -> Result<Vec<EntryData>> {
    let file = File::open(path).context("Failed to open tar file")?;
    let mut archive = tar::Archive::new(file);
    let mut entries = Vec::new();

    for entry in archive.entries().context("Failed to read tar entries")? {
        let entry = entry.context("Failed to read tar entry")?;
        let header = entry.header();
        let name = header
            .path()
            .context("Invalid path in tar")?
            .to_string_lossy()
            .to_string();
        let is_dir = header.entry_type().is_dir();
        let size = header.size().unwrap_or(0);
        let modified = UNIX_EPOCH + Duration::from_secs(header.mtime().unwrap_or(0));

        if let Some(entry) = archive_entry(&name, is_dir, size, modified) {
            entries.push(entry);
        }
    }

    Ok(entries)
}

fn list_tar_gz(path: &Path) -> Result<Vec<EntryData>> {
    let file = File::open(path).context("Failed to open tar.gz file")?;
    let decoder = flate2::read::GzDecoder::new(BufReader::new(file));
    let mut archive = tar::Archive::new(decoder);
    let mut entries = Vec::new();

    for entry in archive.entries().context("Failed to read tar.gz entries")? {
        let entry = entry.context("Failed to read tar.gz entry")?;
        let header = entry.header();
        let name = header
            .path()
            .context("Invalid path in tar.gz")?
            .to_string_lossy()
            .to_string();
        let is_dir = header.entry_type().is_dir();
        let size = header.size().unwrap_or(0);
        let modified = UNIX_EPOCH + Duration::from_secs(header.mtime().unwrap_or(0));

        if let Some(entry) = archive_entry(&name, is_dir, size, modified) {
            entries.push(entry);
        }
    }

    Ok(entries)
}

fn list_tar_zst(path: &Path) -> Result<Vec<EntryData>> {
    let file = File::open(path).context("Failed to open tar.zst file")?;
    let mut decoder = zstd::stream::Decoder::new(BufReader::new(file))
        .context("Failed to create zstd decoder")?;
    let mut archive = tar::Archive::new(&mut decoder);
    let mut entries = Vec::new();

    for entry in archive
        .entries()
        .context("Failed to read tar.zst entries")?
    {
        let entry = entry.context("Failed to read tar.zst entry")?;
        let header = entry.header();
        let name = header
            .path()
            .context("Invalid path in tar.zst")?
            .to_string_lossy()
            .to_string();
        let is_dir = header.entry_type().is_dir();
        let size = header.size().unwrap_or(0);
        let modified = UNIX_EPOCH + Duration::from_secs(header.mtime().unwrap_or(0));

        if let Some(entry) = archive_entry(&name, is_dir, size, modified) {
            entries.push(entry);
        }
    }

    Ok(entries)
}

fn to_system_time(dt: zip::DateTime) -> SystemTime {
    let naive =
        chrono::NaiveDate::from_ymd_opt(dt.year() as i32, dt.month() as u32, dt.day() as u32)
            .and_then(|date| {
                date.and_hms_opt(dt.hour() as u32, dt.minute() as u32, dt.second() as u32)
            })
            .unwrap_or_else(|| chrono::Utc::now().naive_utc());

    let timestamp = naive.and_utc().timestamp();
    if timestamp >= 0 {
        UNIX_EPOCH + Duration::from_secs(timestamp as u64)
    } else {
        UNIX_EPOCH - Duration::from_secs((-timestamp) as u64)
    }
}
