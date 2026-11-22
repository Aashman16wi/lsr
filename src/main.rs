//! lsr - A fast, modern, feature-rich ls replacement written in Rust
//!
//! Usage: lsr [OPTIONS] [PATHS]...

mod cli;
mod fs;
mod output;
mod git;
mod icons;
mod format;

use anyhow::Result;
use cli::{Args, OutputFormat};
use clap::{CommandFactory, Parser};
use clap_complete::Shell;
use fs::{collect_entries, EntryData};
use git::GitRepo;
use output::{print_entries, print_tree};
use format::FormatOptions;
use std::io;

fn main() -> Result<()> {
    let args = Args::parse();

    // Handle shell completion generation
    if let Some(shell) = args.completion {
        let mut cmd = Args::command();
        clap_complete::generate(shell, &mut cmd, "lsr", &mut io::stdout());
        return Ok(());
    }

    // Initialize tracing if verbose
    if args.verbose {
        tracing_subscriber::fmt()
            .with_env_filter(tracing_subscriber::EnvFilter::from_default_env()
                .add_directive("lsr=debug".parse()?))
            .init();
    }

    // Discover git repos for all paths
    let git_repos = GitRepo::discover_all(&args.paths)?;

    // Collect all entries
    let entries = collect_entries(&args.paths, &args, &git_repos)?;

    // Format and output
    let format_opts = FormatOptions::from_args(&args);

    match args.effective_format() {
        OutputFormat::Default => print_entries(&entries, &format_opts, &args)?,
        OutputFormat::Long => print_entries(&entries, &format_opts, &args)?,
        OutputFormat::Tree => print_tree(&entries, &format_opts, &args)?,
        OutputFormat::Json => print_json(&entries, &format_opts)?,
        OutputFormat::Csv => print_csv(&entries, &format_opts)?,
    }

    Ok(())
}

fn print_json(entries: &[EntryData], opts: &FormatOptions) -> Result<()> {
    let output: Vec<_> = entries.iter().map(|e| e.to_json(opts)).collect();
    println!("{}", serde_json::to_string_pretty(&output)?);
    Ok(())
}

fn print_csv(entries: &[EntryData], opts: &FormatOptions) -> Result<()> {
    // CSV header
    println!("name,path,size,modified,permissions,uid,gid,file_type,git_status");
    for entry in entries {
        println!("{}", entry.to_csv(opts));
    }
    Ok(())
}