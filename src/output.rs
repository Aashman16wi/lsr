//! Output formatting and printing

use anyhow::Result;
use crate::cli::Args;
use crate::fs::{EntryData, FileType};
use crate::format::{colorize_name, format_entry_long, FormatOptions};
use crate::icons::get_icon;
use std::collections::HashMap;
use std::path::Path;

/// Print entries in default/long format
pub fn print_entries(entries: &[EntryData], opts: &FormatOptions, args: &Args) -> Result<()> {
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
        
        if args.effective_format() == crate::cli::OutputFormat::Long || args.long {
            print_long(&owned_entries, opts)?;
        } else {
            print_default(&owned_entries, opts)?;
        }
    }

    Ok(())
}

/// Print in default (columnar) format
fn print_default(entries: &[EntryData], opts: &FormatOptions) -> Result<()> {
    use unicode_width::UnicodeWidthStr;

    if entries.is_empty() {
        return Ok(());
    }

    // Calculate name widths
    let max_name_width = entries.iter()
        .map(|e| {
            let icon = if opts.show_icons {
                get_icon(&e.path, e.file_type, true)
            } else {
                ""
            };
            let w = UnicodeWidthStr::width(e.name.as_str());
            if opts.show_icons && !icon.is_empty() {
                w + UnicodeWidthStr::width(icon) + 1
            } else {
                w
            }
        })
        .max()
        .unwrap_or(0);

    let col_width = max_name_width + 2; // padding
    let cols = (opts.term_width / col_width).max(1);
    let rows = (entries.len() + cols - 1) / cols;

    // Print in columns (column-major order)
    for row in 0..rows {
        let mut line = String::new();
        for col in 0..cols {
            let idx = col * rows + row;
            if idx >= entries.len() {
                break;
            }
            let entry = &entries[idx];
            
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
                colorize_name(&name, entry.file_type, entry.permissions)
            } else {
                name
            };

            let classify = if opts.classify {
                entry.file_type.classify_char().to_string()
            } else {
                String::new()
            };

            let entry_str = format!("{}{}", colored, classify);
            let width = UnicodeWidthStr::width(entry_str.as_str());
            line.push_str(&entry_str);
            
            if col < cols - 1 {
                let padding = col_width - width;
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

/// Print entries as a tree
pub fn print_tree(entries: &[EntryData], opts: &FormatOptions, args: &Args) -> Result<()> {
    // Build tree structure
    let mut tree = TreeNode::new(Path::new("."));
    
    for entry in entries {
        tree.insert(&entry.path, entry);
    }

    tree.print(opts, args.depth, 0, true, String::new())
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

    fn print(&self, opts: &FormatOptions, max_depth: usize, depth: usize, is_last: bool, prefix: String) -> Result<()> {
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
                colorize_name(&name, entry.file_type, entry.permissions)
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
                colorize_name(&name, FileType::Directory, 0o755)
            } else {
                name
            };
            println!("{}{}/", line_prefix, colored);
        }

        // Print children
        let mut children: Vec<_> = self.children.values().collect();
        children.sort_by(|a, b| a.name.cmp(&b.name));
        
        let child_count = children.len();
        for (i, child) in children.into_iter().enumerate() {
            let child_is_last = i == child_count - 1;
            let child_prefix = if depth == 0 {
                String::new()
            } else if is_last {
                prefix.clone() + "    "
            } else {
                prefix.clone() + "│   "
            };
            
            child.print(opts, max_depth, depth + 1, child_is_last, child_prefix)?;
        }

        Ok(())
    }
}