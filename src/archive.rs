//! Archive listing support (zip, tar, tar.gz, tar.zst)

use anyhow::{Context, Result};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use crate::cli::Args;
use crate::fs::{EntryData, FileType};
use crate::format::FormatOptions;
use crate::output::print_trees;
use std::time::SystemTime;

/// Check if a path is an archive file we can read
pub fn is_archive(path: &Path) -> bool {
    let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("");
    matches!(ext.to_lowercase().as_str(), "zip" | "tar" | "gz" | "tgz" | "zst" | "tzst")
}

/// List contents of an archive file (CLI entry point)
pub fn list_archive_cli(args: Args) -> Result<()> {
    if args.paths.is_empty() {
        anyhow::bail!("archive: missing archive path");
    }
    let path = &args.paths[0];
    if !path.exists() {
        anyhow::bail!("archive: {}: no such file", path.display());
    }

    let entries = list_archive(path)?;

    let format_opts = FormatOptions::from_args_and_config(&args, &crate::config::LsrConfig::default());

    // Use tree view for archives by default
    print_trees(&entries, &format_opts, 3, &[path.clone()], 1000)?;

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

fn list_zip(path: &Path) -> Result<Vec<EntryData>> {
    let file = File::open(path).context("Failed to open zip file")?;
    let mut archive = zip::ZipArchive::new(file).context("Failed to read zip archive")?;
    let mut entries = Vec::new();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).context("Failed to read zip entry")?;
        let name = file.name().to_string();
        let is_dir = file.is_dir();
        let size = file.size();
        let modified = to_system_time(file.last_modified());

        let file_type = if is_dir {
            FileType::Directory
        } else {
            FileType::Regular
        };

        let entry = EntryData {
            path: std::path::PathBuf::from(&name),
            name,
            file_type,
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
        };
        entries.push(entry);
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
        let name = header.path().context("Invalid path in tar")?.to_string_lossy().to_string();
        let is_dir = header.entry_type().is_dir();
        let size = header.size().unwrap_or(0);
        let modified = SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(header.mtime().unwrap_or(0));

        let file_type = if is_dir {
            FileType::Directory
        } else {
            FileType::Regular
        };

        let entry_data = EntryData {
            path: std::path::PathBuf::from(&name),
            name,
            file_type,
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
        };
        entries.push(entry_data);
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
        let name = header.path().context("Invalid path in tar.gz")?.to_string_lossy().to_string();
        let is_dir = header.entry_type().is_dir();
        let size = header.size().unwrap_or(0);
        let modified = SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(header.mtime().unwrap_or(0));

        let file_type = if is_dir {
            FileType::Directory
        } else {
            FileType::Regular
        };

        let entry_data = EntryData {
            path: std::path::PathBuf::from(&name),
            name,
            file_type,
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
        };
        entries.push(entry_data);
    }

    Ok(entries)
}

fn list_tar_zst(path: &Path) -> Result<Vec<EntryData>> {
    let file = File::open(path).context("Failed to open tar.zst file")?;
    let mut decoder = zstd::stream::Decoder::new(BufReader::new(file)).context("Failed to create zstd decoder")?;
    let mut archive = tar::Archive::new(&mut decoder);
    let mut entries = Vec::new();

    for entry in archive.entries().context("Failed to read tar.zst entries")? {
        let entry = entry.context("Failed to read tar.zst entry")?;
        let header = entry.header();
        let name = header.path().context("Invalid path in tar.zst")?.to_string_lossy().to_string();
        let is_dir = header.entry_type().is_dir();
        let size = header.size().unwrap_or(0);
        let modified = SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(header.mtime().unwrap_or(0));

        let file_type = if is_dir {
            FileType::Directory
        } else {
            FileType::Regular
        };

        let entry_data = EntryData {
            path: std::path::PathBuf::from(&name),
            name,
            file_type,
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
        };
        entries.push(entry_data);
    }

    Ok(entries)
}

fn to_system_time(dt: zip::DateTime) -> SystemTime {
    use chrono::Utc;
    let naive = chrono::NaiveDate::from_ymd_opt(dt.year() as i32, dt.month() as u32, dt.day() as u32)
        .and_then(|d| d.and_hms_opt(dt.hour() as u32, dt.minute() as u32, dt.second() as u32))
        .unwrap_or_else(|| chrono::Utc::now().naive_utc());
    SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(naive.and_utc().timestamp() as u64)
}