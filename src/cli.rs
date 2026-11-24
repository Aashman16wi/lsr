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

#[derive(Parser, Debug, Clone)]
#[command(
    name = "lsr",
    version,
    about = "A fast, modern, feature-rich ls replacement written in Rust",
    long_about = "lsr (ls in Rust) - A modern replacement for ls with colors, icons, git integration, and more.",
    arg_required_else_help = false
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
    #[arg(short = 'O', long, value_enum)]
    pub format: Option<OutputFormat>,

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

    /// Group directories before files
    #[arg(long, default_value_t = false)]
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
    #[arg(long, default_value_t = false)]
    pub git: bool,

    /// Show file icons (default: auto)
    #[arg(long, value_enum)]
    pub icons: Option<IconWhen>,

    /// Color output (default: auto)
    #[arg(long, value_enum)]
    pub color: Option<ColorWhen>,

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
    #[arg(short = 'F', long = "classify")]
    pub classify: bool,

    // === New features ===
    /// Show directory sizes (like du)
    #[arg(long)]
    pub du: bool,

    /// Filter files containing text (simple grep)
    #[arg(long = "grep", value_name = "PATTERN")]
    pub grep_pattern: Option<String>,

    /// Group files by: ext, date, size
    #[arg(long = "group-by", value_name = "MODE", value_parser = ["ext", "date", "size"])]
    pub group_by: Option<String>,

    // === Archive ===
    /// List archive contents (zip, tar, tar.gz, tar.zst)
    #[arg(long)]
    pub archive: bool,

    // === Watch ===
    /// Watch for changes and refresh (like watch -n 1)
    #[arg(short = 'w', long)]
    pub watch: bool,

    /// Watch interval in seconds
    #[arg(long = "watch-interval", value_name = "SECONDS", default_value_t = 2)]
    pub watch_interval: u64,

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

    /// Use a built-in color theme
    #[arg(long, value_name = "THEME")]
    pub theme: Option<String>,

    /// Path to config file
    #[arg(long = "config-file", visible_alias = "config", value_name = "PATH")]
    pub config_file: Option<PathBuf>,

    /// Generate shell completions
    #[arg(long = "generate-completion", visible_alias = "completion", value_enum)]
    pub completion: Option<Shell>,

    /// Generate a default config file
    #[arg(long = "generate-config")]
    pub generate_config: bool,
}

impl Args {
    /// Get the output format after applying the format flags.
    pub fn effective_format(&self) -> OutputFormat {
        if self.tree {
            OutputFormat::Tree
        } else if self.long {
            OutputFormat::Long
        } else {
            self.format.unwrap_or(OutputFormat::Default)
        }
    }

    /// True when the user asked for a format on the command line.
    ///
    /// This is used to decide whether a config file's `format.default`
    /// should apply.
    pub fn explicit_format_requested(&self) -> bool {
        self.long || self.tree || self.format.is_some()
    }
}
