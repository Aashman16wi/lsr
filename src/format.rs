//! Output formatting utilities

use crate::cli::{Args, ColorWhen, IconWhen, TimeField};
use crate::config::{ColorsConfig, LsrConfig};
use crate::fs::{EntryData, FileType, GitStatus};
use crate::icons::{get_icon, git_status_color, git_status_icon};
use chrono::{DateTime, Local};
use owo_colors::{OwoColorize, colors};

/// Format options derived from CLI args and config
#[derive(Debug, Clone)]
pub struct FormatOptions {
    pub show_icons: bool,
    pub use_color: bool,
    pub human_readable: bool,
    pub show_inode: bool,
    pub show_links: bool,
    pub show_owner: bool,
    pub show_group: bool,
    pub show_git: bool,
    pub show_total_size: bool,  // --du flag
    pub full_time: bool,
    pub time_field: TimeField,
    pub time_style: Option<String>,
    pub numeric_ids: bool,
    pub classify: bool,
    pub term_width: usize,
    pub colors: ColorsConfig,
}

impl FormatOptions {
    pub fn from_args_and_config(args: &Args, config: &LsrConfig) -> Self {
        let term_width = terminal_size::terminal_size()
            .map(|(w, _)| w.0 as usize)
            .unwrap_or(80);

        // Use effective config for the first path (or default)
        let effective = config.for_path(&args.paths.first().cloned().unwrap_or_else(|| std::path::PathBuf::from(".")));

        // Determine if we should show icons (CLI > config > auto)
        let show_icons = match args.icons {
            IconWhen::Always => true,
            IconWhen::Never => false,
            IconWhen::Auto => {
                // Check config
                if effective.display.icons != "auto" {
                    effective.display.icons == "always"
                } else {
                    // Auto-detect
                    std::env::var("TERM").map(|t| t != "dumb").unwrap_or(false)
                        && !std::env::var("NO_COLOR").is_ok()
                }
            }
        };

        // Determine color (CLI > config > auto)
        let use_color = match args.color {
            ColorWhen::Always => true,
            ColorWhen::Never => false,
            ColorWhen::Auto => {
                if effective.display.color != "auto" {
                    effective.display.color == "always"
                } else {
                    atty::is(atty::Stream::Stdout) && std::env::var("NO_COLOR").is_err()
                }
            }
        };

        Self {
            show_icons,
            use_color,
            human_readable: if args.bytes { false } else { args.human_readable || effective.display.human_readable },
            show_inode: args.inode,
            show_links: args.links,
            show_owner: args.owner,
            show_group: args.group,
            show_git: args.git || effective.display.show_git,
            show_total_size: args.du,
            full_time: args.full_time,
            time_field: args.time_field,
            time_style: args.time_style.clone().or_else(|| {
                if effective.format.time_style != "locale" {
                    Some(effective.format.time_style.clone())
                } else {
                    None
                }
            }),
            numeric_ids: args.numeric_ids,
            classify: args.classify,
            term_width,
            colors: effective.colors.clone(),
        }
    }
}

/// Format file size in human-readable format
pub fn format_size(size: u64, human: bool) -> String {
    if !human {
        return size.to_string();
    }

    const UNITS: &[&str] = &["B", "K", "M", "G", "T", "P"];
    let mut size_f = size as f64;
    let mut unit_idx = 0;

    while size_f >= 1024.0 && unit_idx < UNITS.len() - 1 {
        size_f /= 1024.0;
        unit_idx += 1;
    }

    if unit_idx == 0 {
        format!("{}B", size)
    } else if size_f >= 100.0 {
        format!("{:.0}{}", size_f, UNITS[unit_idx])
    } else if size_f >= 10.0 {
        format!("{:.1}{}", size_f, UNITS[unit_idx])
    } else {
        format!("{:.2}{}", size_f, UNITS[unit_idx])
    }
}

/// Format permissions as rwxrwxrwx string
pub fn format_permissions(mode: u32, file_type: FileType) -> String {
    let type_char = match file_type {
        FileType::Directory => 'd',
        FileType::Symlink => 'l',
        FileType::BlockDevice => 'b',
        FileType::CharDevice => 'c',
        FileType::Fifo => 'p',
        FileType::Socket => 's',
        _ => '-',
    };

    let mut perms = String::with_capacity(10);
    perms.push(type_char);

    // Owner
    perms.push(if mode & 0o400 != 0 { 'r' } else { '-' });
    perms.push(if mode & 0o200 != 0 { 'w' } else { '-' });
    let exec = mode & 0o100 != 0;
    let suid = mode & 0o4000 != 0;
    perms.push(match (exec, suid) {
        (true, true) => 's',
        (true, false) => 'x',
        (false, true) => 'S',
        (false, false) => '-',
    });

    // Group
    perms.push(if mode & 0o040 != 0 { 'r' } else { '-' });
    perms.push(if mode & 0o020 != 0 { 'w' } else { '-' });
    let exec = mode & 0o010 != 0;
    let sgid = mode & 0o2000 != 0;
    perms.push(match (exec, sgid) {
        (true, true) => 's',
        (true, false) => 'x',
        (false, true) => 'S',
        (false, false) => '-',
    });

    // Other
    perms.push(if mode & 0o004 != 0 { 'r' } else { '-' });
    perms.push(if mode & 0o002 != 0 { 'w' } else { '-' });
    let exec = mode & 0o001 != 0;
    let sticky = mode & 0o1000 != 0;
    perms.push(match (exec, sticky) {
        (true, true) => 't',
        (true, false) => 'x',
        (false, true) => 'T',
        (false, false) => '-',
    });

    perms
}

/// Format time according to options
pub fn format_time(time: std::time::SystemTime, opts: &FormatOptions) -> String {
    let datetime: DateTime<Local> = time.into();

    if opts.full_time {
        return format_full_time(&datetime);
    }

    if let Some(ref style) = opts.time_style {
        match style.as_str() {
            "iso" => return datetime.format("%Y-%m-%d %H:%M").to_string(),
            "full" => return format_full_time(&datetime),
            "relative" => return format_relative_time(&datetime),
            "locale" | "" => {}
            custom => {
                // Allow a literal chrono format string, but never panic on bad input.
                if let Some(formatted) = try_format_time(&datetime, custom) {
                    return formatted;
                }
            }
        }
    }

    // Default: locale-aware short format
    let now = Local::now();
    let six_months_ago = now - chrono::Duration::days(183);

    if datetime > six_months_ago && datetime < now {
        datetime.format("%b %d %H:%M").to_string()
    } else {
        datetime.format("%b %d  %Y").to_string()
    }
}

fn format_full_time(datetime: &DateTime<Local>) -> String {
    datetime.format("%Y-%m-%d %H:%M:%S%.9f %z").to_string()
}

fn format_relative_time(datetime: &DateTime<Local>) -> String {
    let seconds = Local::now().signed_duration_since(*datetime).num_seconds();

    if seconds < 0 {
        return "in the future".to_string();
    }
    if seconds < 60 {
        return "just now".to_string();
    }

    let minutes = seconds / 60;
    if minutes < 60 {
        return format!("{}m ago", minutes);
    }

    let hours = minutes / 60;
    if hours < 24 {
        return format!("{}h ago", hours);
    }

    let days = hours / 24;
    if days < 30 {
        return format!("{}d ago", days);
    }

    let months = days / 30;
    if months < 12 {
        return format!("{}mo ago", months);
    }

    format!("{}y ago", months / 12)
}

fn try_format_time(datetime: &DateTime<Local>, format: &str) -> Option<String> {
    use chrono::format::{Item, StrftimeItems};

    let items: Vec<Item> = StrftimeItems::new(format).collect();
    if items.iter().any(|item| matches!(item, Item::Error)) {
        None
    } else {
        Some(datetime.format(format).to_string())
    }
}

/// Get username from uid (with caching)
fn get_user_name(uid: u32, numeric: bool) -> String {
    if numeric {
        return uid.to_string();
    }
    users::get_user_by_uid(uid)
        .map(|u| u.name().to_string_lossy().to_string())
        .unwrap_or_else(|| uid.to_string())
}

/// Get group name from gid (with caching)
fn get_group_name(gid: u32, numeric: bool) -> String {
    if numeric {
        return gid.to_string();
    }
    users::get_group_by_gid(gid)
        .map(|g| g.name().to_string_lossy().to_string())
        .unwrap_or_else(|| gid.to_string())
}

/// Colorize permissions string and return styled string
pub fn colorize_permissions(perms: &str, file_type: FileType) -> String {
    match file_type {
        FileType::Directory => perms.fg::<colors::BrightBlue>().to_string(),
        FileType::Symlink => perms.fg::<colors::BrightMagenta>().to_string(),
        FileType::BlockDevice | FileType::CharDevice => perms.fg::<colors::BrightYellow>().to_string(),
        FileType::Fifo | FileType::Socket => perms.fg::<colors::BrightGreen>().to_string(),
        _ => {
            // Check if executable
            if perms.chars().nth(3) == Some('x') || perms.chars().nth(6) == Some('x') || perms.chars().nth(9) == Some('x') {
                perms.fg::<colors::BrightGreen>().to_string() // Green for executable
            } else {
                perms.fg::<colors::White>().to_string() // White for regular
            }
        }
    }
}

/// Colorize filename by type, honoring user color overrides.
pub fn colorize_name(
    name: &str,
    file_type: FileType,
    perms: u32,
    colors: &ColorsConfig,
) -> String {
    match file_type {
        FileType::Directory => paint(name, colors.dir.as_deref(), (0x5f, 0xaf, 0xff), true),
        FileType::Symlink => paint(name, colors.symlink.as_deref(), (0xaf, 0x5f, 0xff), false),
        FileType::BlockDevice | FileType::CharDevice => {
            paint(name, None, (0xff, 0xff, 0x5f), true)
        }
        FileType::Fifo | FileType::Socket => paint(name, None, (0x5f, 0xd7, 0x5f), false),
        _ => {
            if perms & 0o111 != 0 {
                paint(name, colors.exec.as_deref(), (0x5f, 0xd7, 0x5f), true)
            } else {
                paint(name, colors.file.as_deref(), (0xd0, 0xd0, 0xd0), false)
            }
        }
    }
}

/// Parse a `#rrggbb` or `rrggbb` color string.
fn parse_hex_color(value: &str) -> Option<(u8, u8, u8)> {
    let hex = value.trim();
    let hex = hex.strip_prefix('#').unwrap_or(hex);
    if hex.len() != 6 || !hex.chars().all(|c| c.is_ascii_hexdigit()) {
        return None;
    }

    Some((
        u8::from_str_radix(&hex[0..2], 16).ok()?,
        u8::from_str_radix(&hex[2..4], 16).ok()?,
        u8::from_str_radix(&hex[4..6], 16).ok()?,
    ))
}

fn paint(name: &str, configured: Option<&str>, default: (u8, u8, u8), bold: bool) -> String {
    let (r, g, b) = configured.and_then(parse_hex_color).unwrap_or(default);
    if bold {
        name.truecolor(r, g, b).bold().to_string()
    } else {
        name.truecolor(r, g, b).to_string()
    }
}

/// Colorize git status icon using ANSI escape codes, honoring color overrides.
fn colorize_git_icon(icon: &str, status: GitStatus, colors: &ColorsConfig) -> String {
    let configured = match status {
        GitStatus::Modified => colors.git_modified.as_deref(),
        GitStatus::Added => colors.git_added.as_deref(),
        GitStatus::Deleted => colors.git_deleted.as_deref(),
        GitStatus::Untracked => colors.git_untracked.as_deref(),
        GitStatus::Ignored => colors.git_ignored.as_deref(),
        GitStatus::Renamed | GitStatus::Copied => colors.git_renamed.as_deref(),
        GitStatus::Conflicted => colors.git_conflicted.as_deref(),
        GitStatus::Unmodified => None,
    };

    let (r, g, b) = configured
        .and_then(parse_hex_color)
        .unwrap_or_else(|| git_status_color(status));

    format!("\x1b[38;2;{};{};{}m{}\x1b[0m", r, g, b, icon)
}

/// Format a single entry for long output
pub fn format_entry_long(entry: &EntryData, opts: &FormatOptions) -> String {
    let mut parts = Vec::new();

    // Inode
    if opts.show_inode {
        parts.push(format!("{:>10} ", entry.inode));
    }

    // Permissions
    let perms = format_permissions(entry.permissions, entry.file_type);
    let perms_styled = if opts.use_color {
        colorize_permissions(&perms, entry.file_type)
    } else {
        perms
    };
    parts.push(perms_styled);

    // Links
    if opts.show_links {
        parts.push(format!(" {:>4}", entry.nlink));
    }

    // Owner
    if opts.show_owner {
        let owner = get_user_name(entry.uid, opts.numeric_ids);
        parts.push(format!(" {:<8}", owner));
    }

    // Group
    if opts.show_group {
        let group = get_group_name(entry.gid, opts.numeric_ids);
        parts.push(format!(" {:<8}", group));
    }

    // Size
    let size_to_show = if opts.show_total_size && entry.file_type == FileType::Directory {
        entry.total_size
    } else {
        entry.size
    };
    let size_str = format_size(size_to_show, opts.human_readable);
    parts.push(format!(" {:>8}", size_str));

    // Time
    let time = match opts.time_field {
        crate::cli::TimeField::Modified => entry.modified,
        crate::cli::TimeField::Accessed => entry.accessed,
        crate::cli::TimeField::Created => entry.created.unwrap_or(entry.modified),
        crate::cli::TimeField::Changed => entry.changed,
    };
    parts.push(format!(" {}", format_time(time, opts)));

    // Name with icon
    let icon = if opts.show_icons {
        get_icon(&entry.path, entry.file_type, true)
    } else {
        ""
    };
    
    let name = &entry.name;
    let name_display = if opts.show_icons && !icon.is_empty() {
        format!("{} {}", icon, name)
    } else {
        name.to_string()
    };

    // Colorize name by type
    let name_colored = if opts.use_color {
        colorize_name(&name_display, entry.file_type, entry.permissions, &opts.colors)
    } else {
        name_display
    };

    parts.push(format!(" {}", name_colored));

    // Symlink target
    if entry.file_type == FileType::Symlink {
        if let Some(target) = &entry.symlink_target {
            parts.push(format!(" -> {}", target.display()));
        }
    }

    // Git status
    if opts.show_git {
        if let Some(status) = entry.git_status {
            let icon = git_status_icon(status);
            let status_str = if opts.use_color {
                colorize_git_icon(icon, status, &opts.colors)
            } else {
                format!(" {}", icon)
            };
            parts.push(status_str);
        }
    }

    // Classify
    if opts.classify {
        parts.push(entry.file_type.classify_char(entry.permissions).to_string());
    }

    parts.join("")
}