//! Watch mode - auto-refresh on filesystem changes

use anyhow::{Context, Result};
use notify::{Config, Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::PathBuf;
use std::time::Duration;
use std::sync::mpsc::channel;

use crate::cli::Args;
use crate::config::LsrConfig;
use crate::fs::collect_entries;
use crate::format::FormatOptions;
use crate::output::print_entries;

/// Run watch mode
pub fn run_watch(args: Args) -> Result<()> {
    if args.paths.is_empty() {
        anyhow::bail!("watch: missing path to watch");
    }

    let path = args.paths[0].clone();
    if !path.exists() {
        anyhow::bail!("watch: {}: no such file or directory", path.display());
    }

    let interval = Duration::from_secs(args.watch_interval.max(1));

    // Setup filesystem watcher
    let (tx, rx) = channel();
    let mut watcher = RecommendedWatcher::new(tx, Config::default().with_poll_interval(interval))
        .context("Failed to create file watcher")?;

    watcher.watch(&path, RecursiveMode::Recursive)
        .context("Failed to watch path")?;

    // Load config
    let config = match LsrConfig::load(args.theme.as_deref(), args.config_file.as_deref()) {
        Ok(config) => config,
        Err(error) => {
            eprintln!("lsr: warning: failed to load config: {error}");
            LsrConfig::default()
        }
    };

    // Initial render
    render_once(&args, &config, &path)?;

    println!("\nWatching {} (interval: {}s) — Press Ctrl+C to stop", path.display(), interval.as_secs());

    // Event loop
    loop {
        match rx.recv_timeout(interval) {
            Ok(Ok(event)) => {
                if should_refresh(&event) {
                    // Clear screen and re-render
                    print!("\x1b[2J\x1b[H");
                    render_once(&args, &config, &path)?;
                    println!("\nWatching {} (interval: {}s) — Press Ctrl+C to stop", path.display(), interval.as_secs());
                }
            }
            Ok(Err(e)) => {
                eprintln!("Watch error: {e}");
            }
            Err(_) => {
                // Timeout - periodic refresh
                print!("\x1b[2J\x1b[H");
                render_once(&args, &config, &path)?;
                println!("\nWatching {} (interval: {}s) — Press Ctrl+C to stop", path.display(), interval.as_secs());
            }
        }
    }
}

fn should_refresh(event: &Event) -> bool {
    matches!(
        event.kind,
        EventKind::Create(_) | EventKind::Remove(_) | EventKind::Modify(_) | EventKind::Rename(_, _)
    )
}

fn render_once(args: &Args, config: &LsrConfig, path: &PathBuf) -> Result<()> {
    let primary_path = path.clone();
    let effective = config.for_path(&primary_path);

    let effective_format = if args.explicit_format_requested() {
        args.effective_format()
    } else {
        match effective.format.default.as_str() {
            "long" => crate::cli::OutputFormat::Long,
            "tree" => crate::cli::OutputFormat::Tree,
            "json" => crate::cli::OutputFormat::Json,
            "csv" => crate::cli::OutputFormat::Csv,
            _ => crate::cli::OutputFormat::Default,
        }
    };

    let tree_depth = if args.depth == usize::MAX {
        effective.format.depth
    } else {
        args.depth
    };
    let tree_max_entries = effective.format.tree_max_entries;

    let git_repos = crate::git::GitRepo::discover_all(&[path.clone()])?;

    let walk_depth = match effective_format {
        crate::cli::OutputFormat::Tree => {
            if tree_depth == usize::MAX { None } else { Some(tree_depth) }
        }
        _ if args.recursive => None,
        _ => Some(1),
    };

    let include_git = args.git || effective.display.show_git;
    let group_dirs_first = args.group_dirs_first || effective.display.group_dirs_first;

    let entries = collect_entries(
        &[path.clone()],
        args,
        &git_repos,
        walk_depth,
        include_git,
        group_dirs_first,
    )?;

    let format_opts = FormatOptions::from_args_and_config(args, config);

    match effective_format {
        crate::cli::OutputFormat::Default => crate::output::print_entries(&entries, &format_opts, args, false)?,
        crate::cli::OutputFormat::Long => crate::output::print_entries(&entries, &format_opts, args, true)?,
        crate::cli::OutputFormat::Tree => crate::output::print_trees(&entries, &format_opts, tree_depth, &[path.clone()], tree_max_entries)?,
        crate::cli::OutputFormat::Json => crate::output::print_json(&entries, &format_opts)?,
        crate::cli::OutputFormat::Csv => crate::output::print_csv(&entries, &format_opts)?,
    }

    Ok(())
}

// Add this method to Args in cli.rs
// fn explicit_format_requested(&self) -> bool { ... }