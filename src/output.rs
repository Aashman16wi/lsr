//! Output formatting and printing

use anyhow::Result;
use crate::cli::Args;
use crate::fs::{EntryData, FileType};
use crate::format::{colorize_name, format_entry_long, FormatOptions};
use crate::icons::get_icon;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// Print entries in default/long format
pub fn print_entries(
    entries: &[EntryData],
    opts: &FormatOptions,
    args: &Args,
    long: bool,
) -> Result<()> {
    if entries.is_empty() {
        return Ok(());
    }

    // Check if we're showing multiple directories
    let mut by_parent: HashMap<&Path, Vec<&EntryData>> = HashMap::new();
    for entry in entries {
        let parent = entry.path.parent().unwrap_or(Path::new("."));
        by_parent.entry(parent).or_default().push(entry);
    }

    let multiple_dirs = by_parent.len() > 1;

    // Sort directories by path
    let mut sorted_dirs: Vec<_> = by_parent.into_iter().collect();
    sorted_dirs.sort_by_key(|(p, _)| *p);

    for (i, (dir, dir_entries)) in sorted_dirs.into_iter().enumerate() {
        if multiple_dirs {
            if i > 0 {
                println!();
            }
            println!("{}:", dir.display());
        }

        // Convert Vec<&EntryData> to Vec<EntryData> for printing
        let owned_entries: Vec<EntryData> = dir_entries.into_iter().cloned().collect();

        if long {
            print_long(&owned_entries, opts)?;
        } else {
            print_default(&owned_entries, opts, args.one_line)?;
        }
    }

    Ok(())
}

/// Render one entry and return its styled text plus visible width.
fn render_entry(entry: &EntryData, opts: &FormatOptions) -> (String, usize) {
    use unicode_width::UnicodeWidthStr;

    let icon = if opts.show_icons {
        get_icon(&entry.path, entry.file_type, true)
    } else {
        ""
    };

    let visible = if opts.show_icons && !icon.is_empty() {
        format!("{} {}", icon, entry.name)
    } else {
        entry.name.clone()
    };

    let mut width = UnicodeWidthStr::width(visible.as_str());

    let mut rendered = if opts.use_color {
        colorize_name(
            &visible,
            entry.file_type,
            entry.permissions,
            &opts.colors,
        )
    } else {
        visible
    };

    if opts.classify {
        let marker = entry.file_type.classify_char(entry.permissions);
        if marker != ' ' {
            rendered.push(marker);
            width += 1;
        }
    }

    (rendered, width)
}

/// Print in default (columnar) format
fn print_default(entries: &[EntryData], opts: &FormatOptions, one_line: bool) -> Result<()> {
    if entries.is_empty() {
        return Ok(());
    }

    let rendered: Vec<(String, usize)> = entries
        .iter()
        .map(|entry| render_entry(entry, opts))
        .collect();

    if one_line {
        for (text, _) in &rendered {
            println!("{}", text);
        }
        return Ok(());
    }

    let max_name_width = rendered
        .iter()
        .map(|(_, width)| *width)
        .max()
        .unwrap_or(0);
    let col_width = max_name_width + 2;
    let cols = (opts.term_width / col_width).max(1);
    let rows = (entries.len() + cols - 1) / cols;

    // Print in columns (column-major order)
    for row in 0..rows {
        let mut line = String::new();
        for col in 0..cols {
            let idx = col * rows + row;
            if idx >= rendered.len() {
                break;
            }

            let (text, width) = &rendered[idx];
            line.push_str(text);

            if col < cols - 1 {
                // `width` is the visible width (no ANSI escapes), so this is safe.
                let padding = col_width.saturating_sub(*width);
                line.push_str(&" ".repeat(padding));
            }
        }
        println!("{}", line);
    }

    Ok(())
}

/// Print in long format
fn print_long(entries: &[EntryData], opts: &FormatOptions) -> Result<()> {
    for entry in entries {
        println!("{}", format_entry_long(entry, opts));
    }

    Ok(())
}

/// Print entries as one tree per root path.
/// Falls back to regular listing if top-level entry count exceeds `tree_max_entries`.
pub fn print_trees(
    entries: &[EntryData],
    opts: &FormatOptions,
    depth: usize,
    roots: &[PathBuf],
    tree_max_entries: usize,
) -> Result<()> {
    let roots: Vec<PathBuf> = if roots.is_empty() {
        vec![PathBuf::from(".")]
    } else {
        roots.to_vec()
    };

    for (index, root) in roots.iter().enumerate() {
        if roots.len() > 1 {
            if index > 0 {
                println!();
            }
            println!("{}:", root.display());
        }

        // Count top-level entries for this root
        let top_level: Vec<&EntryData> = entries
            .iter()
            .filter(|e| {
                e.path.strip_prefix(root)
                    .map(|p| p.components().count() == 1)
                    .unwrap_or(false)
            })
            .collect();

        if top_level.len() > tree_max_entries {
            // Too many entries - fall back to regular listing
            eprintln!(
                "lsr: tree view skipped for {} ({} entries, limit {})",
                root.display(),
                top_level.len(),
                tree_max_entries
            );
            let owned: Vec<EntryData> = top_level.into_iter().cloned().collect();
            print_default(&owned, opts, false)?;
            continue;
        }

        let mut tree = TreeNode::new(root);
        for entry in entries {
            if let Ok(relative) = entry.path.strip_prefix(root) {
                if relative.as_os_str().is_empty() {
                    continue;
                }
                tree.insert(relative, entry);
            }
        }

        tree.print(opts, depth, 0, true, String::new(), Some(50))?;
    }

    Ok(())
}

struct TreeNode {
    name: String,
    path: std::path::PathBuf,
    entry: Option<EntryData>,
    children: HashMap<String, TreeNode>,
}

impl TreeNode {
    fn new(path: &Path) -> Self {
        Self {
            name: path.file_name().unwrap_or_default().to_string_lossy().to_string(),
            path: path.to_path_buf(),
            entry: None,
            children: HashMap::new(),
        }
    }

    fn insert(&mut self, path: &Path, entry: &EntryData) {
        let mut current = self;
        let components: Vec<_> = path.components().collect();
        
        for (i, component) in components.iter().enumerate() {
            let name = component.as_os_str().to_string_lossy().to_string();
            
            if i == components.len() - 1 {
                // This is the file itself
                let child = current.children.entry(name.clone()).or_insert_with(|| TreeNode::new(&current.path.join(&name)));
                child.entry = Some(entry.clone());
            } else {
                // Directory
                current = current.children.entry(name.clone()).or_insert_with(|| TreeNode::new(&current.path.join(&name)));
            }
        }
    }

    fn print(&self, opts: &FormatOptions, max_depth: usize, depth: usize, is_last: bool, prefix: String, max_children: Option<usize>) -> Result<()> {
        if depth > max_depth {
            return Ok(());
        }

        // Print this node
        let connector = if depth == 0 {
            ""
        } else if is_last {
            "└── "
        } else {
            "├── "
        };

        let line_prefix = if depth == 0 {
            String::new()
        } else {
            prefix.clone() + connector
        };

        if let Some(entry) = &self.entry {
            let icon = if opts.show_icons {
                get_icon(&entry.path, entry.file_type, true)
            } else {
                ""
            };
            
            let name = if opts.show_icons && !icon.is_empty() {
                format!("{} {}", icon, entry.name)
            } else {
                entry.name.clone()
            };

            let colored = if opts.use_color {
                colorize_name(&name, entry.file_type, entry.permissions, &opts.colors)
            } else {
                name
            };

            println!("{}{}", line_prefix, colored);
        } else if depth > 0 {
            // Directory node
            let name = if opts.show_icons {
                format!("󰉋 {}", self.name)
            } else {
                self.name.clone()
            };
            let colored = if opts.use_color {
                colorize_name(&name, FileType::Directory, 0o755, &opts.colors)
            } else {
                name
            };
            println!("{}{}/", line_prefix, colored);
        }

        // Print children
        let mut children: Vec<_> = self.children.values().collect();
        children.sort_by(|a, b| a.name.cmp(&b.name));
        
        let child_count = children.len();
        let max_show = max_children.unwrap_or(child_count);
        
        for (i, child) in children.into_iter().enumerate() {
            if i >= max_show {
                // Show "+N more" message
                let remaining = child_count - max_show;
                let msg = format!("{}  ... +{} more", prefix, remaining);
                if opts.use_color {
                    println!("{}", colorize_name(&msg, FileType::Directory, 0, &opts.colors));
                } else {
                    println!("{}", msg);
                }
                break;
            }
            
            let child_is_last = i == child_count - 1 || i == max_show - 1;
            let child_prefix = if depth == 0 {
                String::new()
            } else if is_last {
                prefix.clone() + "    "
            } else {
                prefix.clone() + "│   "
            };
            
            child.print(opts, max_depth, depth + 1, child_is_last, child_prefix, max_children)?;
        }

        Ok(())
    }
}

/// Print entries as JSON
pub fn print_json(entries: &[crate::fs::EntryData], opts: &FormatOptions) -> Result<()> {
    let output: Vec<_> = entries.iter().map(|e| e.to_json(opts)).collect();
    println!("{}", serde_json::to_string_pretty(&output)?);
    Ok(())
}

/// Print entries as CSV
pub fn print_csv(entries: &[crate::fs::EntryData], opts: &FormatOptions) -> Result<()> {
    println!("name,path,size,modified,permissions,uid,gid,file_type,git_status");
    for entry in entries {
        println!("{}", entry.to_csv(opts));
    }
    Ok(())
}