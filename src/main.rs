//! lsr - A fast, modern, feature-rich ls replacement written in Rust
//!
//! Usage: lsr [OPTIONS] [PATHS]...

pub mod archive;
pub mod cli;
pub mod config;
pub mod format;
pub mod fs;
pub mod git;
pub mod icons;
pub mod output;
pub mod watch;

use anyhow::Result;
use clap::{CommandFactory, Parser};
use cli::{Args, OutputFormat};
use config::LsrConfig;
use format::FormatOptions;
use fs::collect_entries;
use git::GitRepo;
use output::{print_csv, print_entries, print_json, print_trees};
use std::io;
use std::path::PathBuf;

fn main() -> Result<()> {
    reset_sigpipe();

    let args = Args::parse();

    // Verbose logging is most useful for the long-running modes too.
    if args.verbose {
        tracing_subscriber::fmt()
            .with_env_filter(
                tracing_subscriber::EnvFilter::from_default_env()
                    .add_directive("lsr=debug".parse()?),
            )
            .init();
    }

    // Handle shell completion generation
    if let Some(shell) = args.completion {
        let mut cmd = Args::command();
        clap_complete::generate(shell, &mut cmd, "lsr", &mut io::stdout());
        return Ok(());
    }

    // An unknown theme would otherwise be silently ignored.
    if let Some(theme) = &args.theme {
        let themes = LsrConfig::builtin_themes();
        if !themes.contains_key(theme) {
            let mut names: Vec<&str> = themes.keys().map(String::as_str).collect();
            names.sort_unstable();
            anyhow::bail!(
                "unknown theme '{}'; available themes: {}",
                theme,
                names.join(", ")
            );
        }
    }

    // Handle generate config
    if args.generate_config {
        generate_default_config();
        return Ok(());
    }

    // Handle watch mode
    if args.watch {
        return watch::run_watch(args);
    }

    // Handle archive listing
    if args.archive {
        return archive::list_archive_cli(args);
    }

    // An explicitly requested config file must exist.
    if let Some(path) = &args.config_file {
        if !path.exists() {
            anyhow::bail!("config file not found: {}", path.display());
        }
    }

    // Load configuration. A broken config should be visible, not silently ignored.
    let config = match LsrConfig::load(args.theme.as_deref(), args.config_file.as_deref()) {
        Ok(config) => config,
        Err(error) => {
            eprintln!("lsr: warning: failed to load config: {error}");
            LsrConfig::default()
        }
    };

    // Per-directory settings are resolved for the primary path.
    let primary_path = args
        .paths
        .first()
        .cloned()
        .unwrap_or_else(|| PathBuf::from("."));
    let effective = config.for_path(&primary_path);

    // Determine effective format (CLI > per-directory config > global config > default)
    let effective_format = if args.explicit_format_requested() {
        args.effective_format()
    } else {
        match effective.format.default.as_str() {
            "long" => OutputFormat::Long,
            "tree" => OutputFormat::Tree,
            "json" => OutputFormat::Json,
            "csv" => OutputFormat::Csv,
            _ => OutputFormat::Default,
        }
    };

    // For tree view, use depth from config if not explicitly set via CLI
    let tree_depth = if args.depth == usize::MAX {
        effective.format.depth
    } else {
        args.depth
    };
    let tree_max_entries = effective.format.tree_max_entries;

    // Discover git repos for all paths
    let git_repos = GitRepo::discover_all(&args.paths)?;

    // Tree views need recursion; normal listings only walk one level.
    let walk_depth = match effective_format {
        OutputFormat::Tree => {
            if tree_depth == usize::MAX {
                None
            } else {
                Some(tree_depth)
            }
        }
        _ if args.recursive => None,
        _ => Some(1),
    };

    // Collect all entries
    let include_git = args.git || effective.display.show_git;
    let group_dirs_first = args.group_dirs_first || effective.display.group_dirs_first;
    let entries = collect_entries(
        &args.paths,
        &args,
        &git_repos,
        walk_depth,
        include_git,
        group_dirs_first,
    )?;

    // Format and output - merge config with CLI args (CLI wins)
    let format_opts = FormatOptions::from_args_and_config(&args, &config);

    match effective_format {
        OutputFormat::Default => print_entries(&entries, &format_opts, &args, false)?,
        OutputFormat::Long => print_entries(&entries, &format_opts, &args, true)?,
        OutputFormat::Tree => print_trees(
            &entries,
            &format_opts,
            tree_depth,
            &args.paths,
            tree_max_entries,
        )?,
        OutputFormat::Json => print_json(&entries, &format_opts)?,
        OutputFormat::Csv => print_csv(&entries, &format_opts)?,
    }

    Ok(())
}

/// Restore the default SIGPIPE behavior so `lsr | head` exits quietly.
#[cfg(unix)]
fn reset_sigpipe() {
    // Safety: resetting a signal disposition is safe to do before any threads
    // are created, which is the case here.
    unsafe {
        libc::signal(libc::SIGPIPE, libc::SIG_DFL);
    }
}

#[cfg(not(unix))]
fn reset_sigpipe() {}

fn generate_default_config() {
    let config = r###"# lsr configuration file
# Place at ~/.config/lsr/config.toml or .lsr.toml in project root

[display]
# icons = "auto"        # auto | always | never
# color = "auto"        # auto | always | never
# human_readable = true
# show_git = false      # opt-in; also enabled by --git
# group_dirs_first = true

[format]
# default = "default"   # default | long | tree | json | csv
# time_style = "locale" # iso | locale | relative | full, or a chrono format
# depth = 3             # tree depth; omit for unlimited
# tree_max_entries = 100  # auto-fallback to regular listing if exceeded

[colors]
# Colors use truecolor hex (e.g., "#5fafff")
# dir = "#5fafff"
# exec = "#5fd75f"
# symlink = "#af5fff"
# file = "#d0d0d0"
# git_modified = "#fac200"
# git_added = "#5fd75f"
# git_deleted = "#ff5f5f"
# git_untracked = "#ffaf00"
# git_ignored = "#606060"
# git_renamed = "#5fafff"
# git_conflicted = "#ff00ff"

# Per-directory overrides (glob patterns).
# `*` matches within one path component; `**` crosses directories.
# [per_directory."~/projects/*"]
# format = "tree"
# depth = 2
# icons = "always"

# [per_directory."~/.config"]
# icons = "never"
"###;
    println!("{}", config);
}
