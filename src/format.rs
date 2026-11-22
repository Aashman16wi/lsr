//! Output formatting utilities

use crate::cli::{Args, ColorWhen, IconWhen, TimeField};
use crate::fs::{EntryData, FileType};
use crate::icons::{get_icon, git_status_color, git_status_icon};
use chrono::{DateTime, Local};
use owo_colors::{OwoColorize, colors};
use unicode_width::UnicodeWidthStr;

/// Format options derived from CLI args
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
    pub show_permissions: bool,
    pub full_time: bool,
    pub time_field: TimeField,
    pub time_style: Option<String>,
    pub numeric_ids: bool,
    pub classify: bool,
    pub term_width: usize,
}

impl FormatOptions {
    pub fn from_args(args: &Args) -> Self {
        let term_width = terminal_size::terminal_size()
            .map(|(w, _)| w.0 as usize)
            .unwrap_or(80);

        // Determine if we should show icons
        let show_icons = match args.icons {
            IconWhen::Always => true,
            IconWhen::Never => false,
            IconWhen::Auto => {
                // Check if terminal supports unicode/nerd fonts
                std::env::var("TERM").map(|t| t != "dumb").unwrap_or(false)
                    && !std::env::var("NO_COLOR").is_ok()
            }
        };

        // Determine color
        let use_color = match args.color {
            ColorWhen::Always => true,
            ColorWhen::Never => false,
            ColorWhen::Auto => {
                atty::is(atty::Stream::Stdout) && std::env::var("NO_COLOR").is_err()
            }
        };

        Self {
            show_icons,
            use_color,
            human_readable: args.human_readable,
            show_inode: args.inode,
            show_links: args.links,
            show_owner: args.owner,
            show_group: args.group,
            show_git: args.git,
            show_permissions: true, // Always show in long format
            full_time: args.full_time,
            time_field: args.time_field,
            time_style: args.time_style.clone(),
            numeric_ids: args.numeric_ids,
            classify: args.classify,
            term_width,
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
        return datetime.format("%Y-%m-%d %H:%M:%S%.9f %z").to_string();
    }

    if let Some(ref style) = opts.time_style {
        return datetime.format(style).to_string();
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
    if !atty::is(atty::Stream::Stdout) {
        return perms.to_string();
    }
    
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

/// Colorize filename by type
pub fn colorize_name(name: &str, file_type: FileType, perms: u32) -> String {
    if !atty::is(atty::Stream::Stdout) {
        return name.to_string();
    }
    
    match file_type {
        FileType::Directory => name.fg::<colors::BrightBlue>().bold().to_string(),
        FileType::Symlink => name.fg::<colors::BrightMagenta>().to_string(),
        FileType::BlockDevice | FileType::CharDevice => name.fg::<colors::BrightYellow>().bold().to_string(),
        FileType::Fifo | FileType::Socket => name.fg::<colors::BrightGreen>().to_string(),
        _ => {
            // Check executable
            if perms & 0o111 != 0 {
                name.fg::<colors::BrightGreen>().bold().to_string()
            } else {
                name.fg::<colors::White>().to_string()
            }
        }
    }
}

/// Colorize git status icon using ANSI escape codes
fn colorize_git_icon(icon: &str, status: crate::fs::GitStatus) -> String {
    if !atty::is(atty::Stream::Stdout) {
        return icon.to_string();
    }
    
    let (r, g, b) = git_status_color(status);
    // Use ANSI 256 color approximation
    let color_code = rgb_to_ansi256(r, g, b);
    format!("\x1b[38;5;{}m{}\x1b[0m", color_code, icon)
}

fn rgb_to_ansi256(r: u8, g: u8, b: u8) -> u8 {
    // Convert RGB to nearest ANSI 256 color
    if r == g && g == b {
        // Grayscale
        if r < 8 { return 16; }
        if r > 248 { return 231; }
        return 232 + ((r - 8) / 10) as u8;
    }
    // Color cube: 16 + 36*r + 6*g + b where r,g,b are 0-5
    let ri = (r as f32 / 255.0 * 5.0).round() as u8;
    let gi = (g as f32 / 255.0 * 5.0).round() as u8;
    let bi = (b as f32 / 255.0 * 5.0).round() as u8;
    16 + 36 * ri + 6 * gi + bi
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
    let size_str = format_size(entry.size, opts.human_readable);
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
        colorize_name(&name_display, entry.file_type, entry.permissions)
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
                colorize_git_icon(icon, status)
            } else {
                format!(" {}", icon)
            };
            parts.push(status_str);
        }
    }

    // Classify
    if opts.classify {
        parts.push(entry.file_type.classify_char().to_string());
    }

    parts.join("")
}

/// Calculate column widths for tabular output
pub fn calculate_column_widths(entries: &[EntryData], opts: &FormatOptions) -> ColumnWidths {
    let mut widths = ColumnWidths::default();

    for entry in entries {
        if opts.show_inode {
            widths.inode = widths.inode.max(entry.inode.to_string().len());
        }
        if opts.show_links {
            widths.links = widths.links.max(entry.nlink.to_string().len());
        }
        if opts.show_owner {
            let owner = get_user_name(entry.uid, opts.numeric_ids);
            widths.owner = widths.owner.max(owner.len());
        }
        if opts.show_group {
            let group = get_group_name(entry.gid, opts.numeric_ids);
            widths.group = widths.group.max(group.len());
        }
        
        let size_str = format_size(entry.size, opts.human_readable);
        widths.size = widths.size.max(size_str.len());
        
        let icon = if opts.show_icons {
            get_icon(&entry.path, entry.file_type, true)
        } else {
            ""
        };
        let name_width = UnicodeWidthStr::width(entry.name.as_str());
        if opts.show_icons && !icon.is_empty() {
            widths.name = widths.name.max(name_width + UnicodeWidthStr::width(icon) + 1);
        } else {
            widths.name = widths.name.max(name_width);
        }
    }

    widths
}

#[derive(Debug, Default)]
pub struct ColumnWidths {
    pub inode: usize,
    pub links: usize,
    pub owner: usize,
    pub group: usize,
    pub size: usize,
    pub name: usize,
}