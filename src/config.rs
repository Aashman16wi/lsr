//! Configuration system for lsr

use anyhow::Result;
use config::{Config, Environment, File};
use dirs::config_dir;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Component, Path, PathBuf};

/// Main configuration structure
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LsrConfig {
    /// Display settings
    #[serde(default)]
    pub display: DisplayConfig,

    /// Output format settings
    #[serde(default)]
    pub format: FormatConfig,

    /// Icon settings
    #[serde(default)]
    pub icons: IconsConfig,

    /// Color settings
    #[serde(default)]
    pub colors: ColorsConfig,

    /// Git settings
    #[serde(default)]
    pub git: GitConfig,

    /// Per-directory overrides
    #[serde(default)]
    pub per_directory: HashMap<String, PerDirectoryConfig>,

    /// Custom icon mappings (extension or filename -> icon)
    #[serde(default)]
    pub custom_icons: HashMap<String, String>,

    /// Custom column definitions
    #[serde(default)]
    pub custom_columns: HashMap<String, CustomColumn>,
}

/// Display settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisplayConfig {
    /// Show icons: auto, always, never
    #[serde(default = "default_auto")]
    pub icons: String,

    /// Color output: auto, always, never
    #[serde(default = "default_auto")]
    pub color: String,

    /// Human readable sizes
    #[serde(default = "default_true")]
    pub human_readable: bool,

    /// Show git status
    #[serde(default = "default_false")]
    pub show_git: bool,

    /// Group directories first
    #[serde(default = "default_true")]
    pub group_dirs_first: bool,
}

impl Default for DisplayConfig {
    fn default() -> Self {
        Self {
            icons: "auto".to_string(),
            color: "auto".to_string(),
            human_readable: true,
            show_git: false,
            group_dirs_first: true,
        }
    }
}

/// Output format settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormatConfig {
    /// Default format: default, long, tree, json, csv
    #[serde(default = "default_format")]
    pub default: String,

    /// Time style: iso, locale, relative, full
    #[serde(default = "default_time_style")]
    pub time_style: String,

    /// Tree depth limit
    #[serde(default = "default_depth")]
    pub depth: usize,

    /// Max entries before tree view falls back to regular listing
    #[serde(default = "default_tree_max_entries")]
    pub tree_max_entries: usize,
}

impl Default for FormatConfig {
    fn default() -> Self {
        Self {
            default: "default".to_string(),
            time_style: "locale".to_string(),
            depth: 1,
            tree_max_entries: 100,
        }
    }
}

/// Icon settings
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IconsConfig {
    /// Custom icon theme name
    #[serde(default)]
    pub theme: String,
}

/// Color settings (truecolor hex)
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ColorsConfig {
    /// Directory color
    #[serde(default)]
    pub dir: Option<String>,

    /// Executable file color
    #[serde(default)]
    pub exec: Option<String>,

    /// Symlink color
    #[serde(default)]
    pub symlink: Option<String>,

    /// Regular file color
    #[serde(default)]
    pub file: Option<String>,

    /// Git modified
    #[serde(default)]
    pub git_modified: Option<String>,

    /// Git added
    #[serde(default)]
    pub git_added: Option<String>,

    /// Git deleted
    #[serde(default)]
    pub git_deleted: Option<String>,

    /// Git untracked
    #[serde(default)]
    pub git_untracked: Option<String>,

    /// Git ignored
    #[serde(default)]
    pub git_ignored: Option<String>,

    /// Git renamed
    #[serde(default)]
    pub git_renamed: Option<String>,

    /// Git conflicted
    #[serde(default)]
    pub git_conflicted: Option<String>,
}

/// Git settings
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GitConfig {
    /// Show ignored files
    #[serde(default)]
    pub show_ignored: bool,

    /// Show untracked files
    #[serde(default = "default_true")]
    pub show_untracked: bool,
}

/// Per-directory configuration override
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PerDirectoryConfig {
    /// Format override
    #[serde(default)]
    pub format: Option<String>,

    /// Depth override for tree
    #[serde(default)]
    pub depth: Option<usize>,

    /// Icons override
    #[serde(default)]
    pub icons: Option<String>,

    /// Color override
    #[serde(default)]
    pub color: Option<String>,

    /// Human readable override
    #[serde(default)]
    pub human_readable: Option<bool>,

    /// Show git override
    #[serde(default)]
    pub show_git: Option<bool>,

    /// Sort override
    #[serde(default)]
    pub sort: Option<String>,

    /// Reverse sort override
    #[serde(default)]
    pub reverse: Option<bool>,
}

/// Custom column definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomColumn {
    /// Column name
    pub name: String,

    /// Source: "shell" (command), "field" (builtin), "wasm" (plugin)
    #[serde(default = "default_source")]
    pub source: String,

    /// Command or field name
    pub value: String,

    /// Width hint
    #[serde(default)]
    pub width: Option<usize>,
}

/// Built-in themes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Theme {
    pub name: String,
    pub display: Option<DisplayConfig>,
    pub format: Option<FormatConfig>,
    pub icons: Option<IconsConfig>,
    pub colors: Option<ColorsConfig>,
    pub git: Option<GitConfig>,
}

impl LsrConfig {
    /// Load configuration from all sources
    pub fn load(cli_theme: Option<&str>, cli_config_file: Option<&Path>) -> Result<Self> {
        let mut builder = Config::builder()
            // Start with defaults
            .set_default("display.icons", "auto")?
            .set_default("display.color", "auto")?
            .set_default("display.human_readable", true)?
            .set_default("display.show_git", false)?
            .set_default("display.group_dirs_first", true)?
            .set_default("format.default", "default")?
            .set_default("format.time_style", "locale")?
            .set_default("format.depth", i64::MAX)?
            .set_default("git.show_ignored", false)?
            .set_default("git.show_untracked", true)?;

        // Load user config file (or custom path)
        let config_path = cli_config_file.map(|p| p.to_path_buf())
            .or_else(Self::user_config_path);
        
        if let Some(path) = config_path {
            if path.exists() {
                builder = builder.add_source(File::from(path).required(false));
            }
        }

        // Load project-local config (.lsr.toml in current dir or parents)
        if let Some(project_config) = Self::find_project_config() {
            builder = builder.add_source(File::from(project_config).required(false));
        }

        // Environment variables (LSR_DISPLAY__ICONS=always, etc.)
        builder = builder.add_source(
            Environment::with_prefix("LSR")
                .separator("__")
                .try_parsing(true)
                .list_separator(","),
        );

        let mut config: LsrConfig = builder.build()?.try_deserialize()?;

        // Apply built-in theme if specified (after loading config, so it overrides file/env)
        if let Some(theme_name) = cli_theme {
            if let Some(theme) = Self::builtin_themes().get(theme_name) {
                config.apply_theme(theme);
            }
        }

        Ok(config)
    }

    /// Get user config directory path
    fn user_config_path() -> Option<PathBuf> {
        config_dir().map(|dir| dir.join("lsr").join("config.toml"))
    }

    /// Find project-local .lsr.toml (current dir or parents)
    fn find_project_config() -> Option<PathBuf> {
        let mut current = std::env::current_dir().ok()?;
        loop {
            let config_path = current.join(".lsr.toml");
            if config_path.exists() {
                return Some(config_path);
            }
            if !current.pop() {
                break;
            }
        }
        None
    }

    /// Get effective config for a specific path (merges per-directory overrides)
    pub fn for_path(&self, path: &Path) -> EffectiveConfig {
        let mut effective = EffectiveConfig::from_base(self);

        // Apply matching overrides, most specific pattern last so it wins.
        let mut matches: Vec<(&String, &PerDirectoryConfig)> = self
            .per_directory
            .iter()
            .filter(|(pattern, _)| Self::match_path(path, pattern))
            .collect();
        matches.sort_by(|(a, _), (b, _)| {
            Self::pattern_specificity(a)
                .cmp(&Self::pattern_specificity(b))
                .then_with(|| a.cmp(b))
        });

        for (_, override_config) in matches {
            effective.apply_override(override_config);
        }

        effective
    }

    /// Number of literal (non-wildcard) characters in a glob pattern.
    fn pattern_specificity(pattern: &str) -> usize {
        pattern.chars().filter(|c| *c != '*' && *c != '?').count()
    }

    /// Match path against a glob pattern.
    ///
    /// Supports `~` at the start, `*` (any characters except `/`),
    /// `**` (any characters including `/`) and `?` (one character except `/`).
    fn match_path(path: &Path, pattern: &str) -> bool {
        let expanded = match pattern.strip_prefix('~') {
            Some(rest) => {
                let home = dirs::home_dir().unwrap_or_default();
                format!("{}{}", home.to_string_lossy(), rest)
            }
            None => pattern.to_string(),
        };

        // Trailing slashes do not change the directory being matched.
        let expanded = expanded.trim_end_matches('/');
        let pattern = if expanded.is_empty() { "/" } else { expanded };

        let path = Self::absolute_lexical(path);
        let path_str = path.to_string_lossy();

        regex::Regex::new(&Self::glob_to_regex(pattern))
            .map(|re| re.is_match(&path_str))
            .unwrap_or(false)
    }

    /// Make a path absolute without touching the filesystem (resolves `.`/`..`).
    fn absolute_lexical(path: &Path) -> PathBuf {
        let absolute = if path.is_absolute() {
            path.to_path_buf()
        } else {
            std::env::current_dir()
                .unwrap_or_default()
                .join(path)
        };

        let mut normalized = PathBuf::new();
        for component in absolute.components() {
            match component {
                Component::CurDir => {}
                Component::ParentDir => {
                    normalized.pop();
                }
                other => normalized.push(other.as_os_str()),
            }
        }
        normalized
    }

    /// Convert a glob pattern into an anchored regular expression.
    fn glob_to_regex(pattern: &str) -> String {
        let mut re = String::from("^");
        let chars: Vec<char> = pattern.chars().collect();
        let mut i = 0;

        while i < chars.len() {
            match chars[i] {
                '*' => {
                    if i + 1 < chars.len() && chars[i + 1] == '*' {
                        re.push_str(".*");
                        i += 1;
                    } else {
                        re.push_str("[^/]*");
                    }
                }
                '?' => re.push_str("[^/]"),
                c => {
                    if "\\^$.|+()[]{}".contains(c) {
                        re.push('\\');
                    }
                    re.push(c);
                }
            }
            i += 1;
        }

        re.push('$');
        re
    }

    /// Get available built-in themes
    pub fn builtin_themes() -> HashMap<String, Theme> {
        let mut themes = HashMap::new();

        // Default theme - developer-friendly
        themes.insert("default".to_string(), Theme {
            name: "default".to_string(),
            display: Some(DisplayConfig {
                icons: "always".to_string(),
                color: "always".to_string(),
                human_readable: true,
                show_git: true,
                group_dirs_first: true,
            }),
            format: Some(FormatConfig {
                default: "long".to_string(),
                time_style: "iso".to_string(),
                depth: 3,
                tree_max_entries: 200,
            }),
            icons: Some(IconsConfig { theme: "nerd".to_string() }),
            colors: Some(ColorsConfig {
                dir: Some("#7aa2f7".to_string()),
                exec: Some("#9ece6a".to_string()),
                symlink: Some("#bb9af7".to_string()),
                file: Some("#a9b1d6".to_string()),
                git_modified: Some("#e0af68".to_string()),
                git_added: Some("#9ece6a".to_string()),
                git_deleted: Some("#f7768e".to_string()),
                git_untracked: Some("#ff9e64".to_string()),
                git_ignored: Some("#565f89".to_string()),
                git_renamed: Some("#7aa2f7".to_string()),
                git_conflicted: Some("#f7768e".to_string()),
            }),
            git: Some(GitConfig { show_ignored: true, show_untracked: true }),
        });

        // Minimal theme
        themes.insert("minimal".to_string(), Theme {
            name: "minimal".to_string(),
            display: Some(DisplayConfig {
                icons: "never".to_string(),
                color: "never".to_string(),
                human_readable: false,
                show_git: false,
                group_dirs_first: false,
            }),
            format: Some(FormatConfig {
                default: "default".to_string(),
                time_style: "locale".to_string(),
                depth: 1,
                tree_max_entries: 100,
            }),
            icons: Some(IconsConfig::default()),
            colors: None,
            git: Some(GitConfig { show_ignored: false, show_untracked: false }),
        });

        // Developer theme (tokyo night inspired)
        themes.insert("dev".to_string(), Theme {
            name: "dev".to_string(),
            display: Some(DisplayConfig {
                icons: "always".to_string(),
                color: "always".to_string(),
                human_readable: true,
                show_git: true,
                group_dirs_first: true,
            }),
            format: Some(FormatConfig {
                default: "long".to_string(),
                time_style: "iso".to_string(),
                depth: 3,
                tree_max_entries: 500,
            }),
            icons: Some(IconsConfig { theme: "nerd".to_string() }),
            colors: Some(ColorsConfig {
                dir: Some("#7aa2f7".to_string()),
                exec: Some("#9ece6a".to_string()),
                symlink: Some("#bb9af7".to_string()),
                file: Some("#a9b1d6".to_string()),
                git_modified: Some("#e0af68".to_string()),
                git_added: Some("#9ece6a".to_string()),
                git_deleted: Some("#f7768e".to_string()),
                git_untracked: Some("#ff9e64".to_string()),
                git_ignored: Some("#565f89".to_string()),
                git_renamed: Some("#7aa2f7".to_string()),
                git_conflicted: Some("#f7768e".to_string()),
            }),
            git: Some(GitConfig { show_ignored: true, show_untracked: true }),
        });

        // Tokyo Night
        themes.insert("tokyo-night".to_string(), Theme {
            name: "tokyo-night".to_string(),
            display: Some(DisplayConfig {
                icons: "always".to_string(),
                color: "always".to_string(),
                human_readable: true,
                show_git: true,
                group_dirs_first: true,
            }),
            format: Some(FormatConfig {
                default: "tree".to_string(),
                time_style: "iso".to_string(),
                depth: 3,
                tree_max_entries: 300,
            }),
            icons: Some(IconsConfig { theme: "nerd".to_string() }),
            colors: Some(ColorsConfig {
                dir: Some("#7aa2f7".to_string()),
                exec: Some("#9ece6a".to_string()),
                symlink: Some("#bb9af7".to_string()),
                file: Some("#a9b1d6".to_string()),
                git_modified: Some("#e0af68".to_string()),
                git_added: Some("#9ece6a".to_string()),
                git_deleted: Some("#f7768e".to_string()),
                git_untracked: Some("#ff9e64".to_string()),
                git_ignored: Some("#565f89".to_string()),
                git_renamed: Some("#7aa2f7".to_string()),
                git_conflicted: Some("#f7768e".to_string()),
            }),
            git: Some(GitConfig { show_ignored: true, show_untracked: true }),
        });

        // Catppuccin Mocha
        themes.insert("catppuccin-mocha".to_string(), Theme {
            name: "catppuccin-mocha".to_string(),
            display: Some(DisplayConfig {
                icons: "always".to_string(),
                color: "always".to_string(),
                human_readable: true,
                show_git: true,
                group_dirs_first: true,
            }),
            format: Some(FormatConfig {
                default: "tree".to_string(),
                time_style: "iso".to_string(),
                depth: 3,
                tree_max_entries: 300,
            }),
            icons: Some(IconsConfig { theme: "nerd".to_string() }),
            colors: Some(ColorsConfig {
                dir: Some("#89b4fa".to_string()),
                exec: Some("#a6e3a1".to_string()),
                symlink: Some("#f5c2e7".to_string()),
                file: Some("#cdd6f4".to_string()),
                git_modified: Some("#f9e2af".to_string()),
                git_added: Some("#a6e3a1".to_string()),
                git_deleted: Some("#f38ba8".to_string()),
                git_untracked: Some("#fab387".to_string()),
                git_ignored: Some("#6c7086".to_string()),
                git_renamed: Some("#89b4fa".to_string()),
                git_conflicted: Some("#f38ba8".to_string()),
            }),
            git: Some(GitConfig { show_ignored: true, show_untracked: true }),
        });

        // Gruvbox
        themes.insert("gruvbox".to_string(), Theme {
            name: "gruvbox".to_string(),
            display: Some(DisplayConfig {
                icons: "always".to_string(),
                color: "always".to_string(),
                human_readable: true,
                show_git: true,
                group_dirs_first: true,
            }),
            format: Some(FormatConfig {
                default: "tree".to_string(),
                time_style: "iso".to_string(),
                depth: 3,
                tree_max_entries: 300,
            }),
            icons: Some(IconsConfig { theme: "nerd".to_string() }),
            colors: Some(ColorsConfig {
                dir: Some("#83a598".to_string()),
                exec: Some("#b8bb26".to_string()),
                symlink: Some("#d3869b".to_string()),
                file: Some("#ebdbb2".to_string()),
                git_modified: Some("#fabd2f".to_string()),
                git_added: Some("#b8bb26".to_string()),
                git_deleted: Some("#fb4934".to_string()),
                git_untracked: Some("#fe8019".to_string()),
                git_ignored: Some("#928374".to_string()),
                git_renamed: Some("#83a598".to_string()),
                git_conflicted: Some("#fb4934".to_string()),
            }),
            git: Some(GitConfig { show_ignored: true, show_untracked: true }),
        });

        // Rose Pine
        themes.insert("rose-pine".to_string(), Theme {
            name: "rose-pine".to_string(),
            display: Some(DisplayConfig {
                icons: "always".to_string(),
                color: "always".to_string(),
                human_readable: true,
                show_git: true,
                group_dirs_first: true,
            }),
            format: Some(FormatConfig {
                default: "tree".to_string(),
                time_style: "iso".to_string(),
                depth: 3,
                tree_max_entries: 300,
            }),
            icons: Some(IconsConfig { theme: "nerd".to_string() }),
            colors: Some(ColorsConfig {
                dir: Some("#9ccfd8".to_string()),
                exec: Some("#31748f".to_string()),
                symlink: Some("#c4a7e7".to_string()),
                file: Some("#e0def4".to_string()),
                git_modified: Some("#f6c177".to_string()),
                git_added: Some("#31748f".to_string()),
                git_deleted: Some("#eb6f92".to_string()),
                git_untracked: Some("#f6c177".to_string()),
                git_ignored: Some("#6e6a86".to_string()),
                git_renamed: Some("#9ccfd8".to_string()),
                git_conflicted: Some("#eb6f92".to_string()),
            }),
            git: Some(GitConfig { show_ignored: true, show_untracked: true }),
        });

        themes
    }

    /// Apply a theme to this config (overrides existing values)
    pub fn apply_theme(&mut self, theme: &Theme) {
        if let Some(display) = &theme.display {
            self.display = display.clone();
        }
        if let Some(format) = &theme.format {
            self.format = format.clone();
        }
        if let Some(icons) = &theme.icons {
            self.icons = icons.clone();
        }
        if let Some(colors) = &theme.colors {
            self.colors = colors.clone();
        }
        if let Some(git) = &theme.git {
            self.git = git.clone();
        }
    }
}

/// Effective configuration after merging base + overrides
#[derive(Debug, Clone)]
#[allow(dead_code)] // icons/colors/git are exposed for upcoming features
pub struct EffectiveConfig {
    pub display: DisplayConfig,
    pub format: FormatConfig,
    pub icons: IconsConfig,
    pub colors: ColorsConfig,
    pub git: GitConfig,
}

impl EffectiveConfig {
    fn from_base(base: &LsrConfig) -> Self {
        Self {
            display: base.display.clone(),
            format: base.format.clone(),
            icons: base.icons.clone(),
            colors: base.colors.clone(),
            git: base.git.clone(),
        }
    }

    fn apply_override(&mut self, override_config: &PerDirectoryConfig) {
        if let Some(v) = override_config.format.clone() {
            self.format.default = v;
        }
        if let Some(v) = override_config.depth {
            self.format.depth = v;
        }
        if let Some(v) = override_config.icons.clone() {
            self.display.icons = v;
        }
        if let Some(v) = override_config.color.clone() {
            self.display.color = v;
        }
        if let Some(v) = override_config.human_readable {
            self.display.human_readable = v;
        }
        if let Some(v) = override_config.show_git {
            self.display.show_git = v;
        }
    }
}

fn default_auto() -> String { "auto".to_string() }
fn default_true() -> bool { true }
fn default_false() -> bool { false }
fn default_format() -> String { "default".to_string() }
fn default_time_style() -> String { "locale".to_string() }
fn default_depth() -> usize { 1 }
fn default_tree_max_entries() -> usize { 100 }
fn default_source() -> String { "field".to_string() }