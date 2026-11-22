//! Command-line argument parsing

use clap::{Parser, ValueEnum};
use clap_complete::Shell;
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, ValueEnum, PartialEq, Eq)]
pub enum OutputFormat {
    /// Default table output
    Default,
    /// Long format with details (-l)
    Long,
    /// Tree view (-T)
    Tree,
    /// JSON output
    Json,
    /// CSV output
    Csv,
}

#[derive(Debug, Clone, Copy, ValueEnum, PartialEq, Eq)]
pub enum SortField {
    /// Sort by name (default)
    Name,
    /// Sort by file size
    Size,
    /// Sort by modification time
    Time,
    /// Sort by extension
    Extension,
    /// Sort by inode number
    Inode,
    /// Don't sort (directory order)
    None,
}

#[derive(Debug, Clone, Copy, ValueEnum, PartialEq, Eq)]
pub enum TimeField {
    /// Modification time (default)
    Modified,
    /// Access time
    Accessed,
    /// Creation time
    Created,
    /// Status change time
    Changed,
}

#[derive(Debug, Clone, Copy, ValueEnum, PartialEq, Eq)]
pub enum ColorWhen {
    /// Auto-detect (default)
    Auto,
    /// Always use colors
    Always,
    /// Never use colors
    Never,
}

#[derive(Debug, Clone, Copy, ValueEnum, PartialEq, Eq)]
pub enum IconWhen {
    /// Auto-detect (default)
    Auto,
    /// Always show icons
    Always,
    /// Never show icons
    Never,
}

#[derive(Parser, Debug)]
#[command(
    name = "lsr",
    version,
    about = "A fast, modern, feature-rich ls replacement written in Rust",
    long_about = "lsr (ls in Rust) - A modern replacement for ls with colors, icons, git integration, and more.",
    arg_required_else_help = false,
    disable_version_flag = true
)]
pub struct Args {
    /// Paths to list (default: current directory)
    #[arg(value_name = "PATH", default_value = ".")]
    pub paths: Vec<PathBuf>,

    // === Output format ===
    /// Use long listing format
    #[arg(short = 'l', long, conflicts_with = "format")]
    pub long: bool,

    /// Output format
    #[arg(short = 'O', long, value_enum, default_value_t = OutputFormat::Default)]
    pub format: OutputFormat,

    /// Show as tree
    #[arg(short = 'T', long, conflicts_with = "format")]
    pub tree: bool,

    /// Maximum depth for tree view
    #[arg(long, default_value_t = usize::MAX)]
    pub depth: usize,

    // === Filtering ===
    /// Show hidden files (starting with .)
    #[arg(short = 'a', long)]
    pub all: bool,

    /// Show hidden files but not . and ..
    #[arg(short = 'A', long)]
    pub almost_all: bool,

    /// Ignore entries matching PATTERN (glob)
    #[arg(long = "ignore", value_name = "PATTERN")]
    pub ignore_patterns: Vec<String>,

    /// Only show files matching PATTERN (glob)
    #[arg(long = "glob", value_name = "PATTERN")]
    pub glob_patterns: Vec<String>,

    /// Only show files of type (f=file, d=dir, l=symlink, etc.)
    #[arg(long = "type", value_name = "TYPE")]
    pub file_types: Vec<String>,

    // === Sorting ===
    /// Sort field
    #[arg(short = 's', long = "sort", value_enum, default_value_t = SortField::Name)]
    pub sort: SortField,

    /// Reverse sort order
    #[arg(short = 'r', long)]
    pub reverse: bool,

    /// Group directories first
    #[arg(long, default_value_t = true)]
    pub group_dirs_first: bool,

    /// Time field for sorting/display
    #[arg(long = "time", value_enum, default_value_t = TimeField::Modified)]
    pub time_field: TimeField,

    // === Display options ===
    /// Show file sizes in human-readable format (e.g., 1K, 234M, 2G)
    #[arg(short = 'H', long)]
    pub human_readable: bool,

    /// Show file sizes in bytes
    #[arg(long, conflicts_with = "human_readable")]
    pub bytes: bool,

    /// Show inode numbers
    #[arg(short = 'i', long)]
    pub inode: bool,

    /// Show number of hard links
    #[arg(long)]
    pub links: bool,

    /// Show file owner
    #[arg(short = 'o', long)]
    pub owner: bool,

    /// Show file group
    #[arg(short = 'g', long)]
    pub group: bool,

    /// Show git status
    #[arg(long, default_value_t = true)]
    pub git: bool,

    /// Show file icons
    #[arg(long, value_enum, default_value_t = IconWhen::Auto)]
    pub icons: IconWhen,

    /// Color output
    #[arg(long, value_enum, default_value_t = ColorWhen::Auto)]
    pub color: ColorWhen,

    /// Show full time (nanosecond precision)
    #[arg(long = "full-time")]
    pub full_time: bool,

    /// Use ISO 8601 time format
    #[arg(long = "time-style", value_name = "STYLE")]
    pub time_style: Option<String>,

    /// Don't print owner/group names (numeric IDs)
    #[arg(long)]
    pub numeric_ids: bool,

    /// Follow symlinks
    #[arg(short = 'L', long)]
    pub dereference: bool,

    /// Classify files (append /, @, *, etc.)
    #[arg(short = 'F', long = "classify", conflicts_with = "format")]
    pub classify: bool,

    // === Performance ===
    /// Number of threads (0 = auto)
    #[arg(long, default_value_t = 0)]
    pub threads: usize,

    /// Disable parallel processing
    #[arg(long)]
    pub no_parallel: bool,

    // === Misc ===
    /// Recurse into subdirectories
    #[arg(short = 'R', long)]
    pub recursive: bool,

    /// One entry per line
    #[arg(short = '1', long = "one-line")]
    pub one_line: bool,

    /// Verbose output (debug logging)
    #[arg(short = 'v', long)]
    pub verbose: bool,

    /// Print version and exit
    #[arg(short = 'V', long)]
    pub version: bool,

    /// Generate shell completions
    #[arg(long = "generate-completion", value_enum, hide = true)]
    pub completion: Option<Shell>,
}

impl Args {
    /// Get effective output format
    pub fn effective_format(&self) -> OutputFormat {
        if self.tree {
            OutputFormat::Tree
        } else if self.long {
            OutputFormat::Long
        } else {
            self.format
        }
    }

    /// Check if long format is requested
    pub fn is_long(&self) -> bool {
        matches!(self.effective_format(), OutputFormat::Long | OutputFormat::Tree)
    }
}