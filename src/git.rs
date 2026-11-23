//! Git integration for repository status

use anyhow::Result;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::{Arc, Mutex};

use crate::fs::GitStatus;

/// Wrapper around a git repository with cached status
pub struct GitRepo {
    workdir: PathBuf,
    status_cache: Arc<Mutex<HashMap<PathBuf, GitStatus>>>,
}

impl GitRepo {
    /// Discover git repositories for the given paths
    pub fn discover_all(paths: &[PathBuf]) -> Result<HashMap<PathBuf, GitRepo>> {
        let mut repos = HashMap::new();

        for path in paths {
            if let Some(repo) = Self::discover(path)? {
                repos.insert(repo.workdir.clone(), repo);
            }
        }

        Ok(repos)
    }

    /// Find the git repository for a path (walking up parents)
    fn discover(path: &Path) -> Result<Option<GitRepo>> {
        let mut current = path.to_path_buf();

        loop {
            let git_dir = current.join(".git");
            if git_dir.exists() {
                let workdir = current;
                return Ok(Some(GitRepo {
                    workdir,
                    status_cache: Arc::new(Mutex::new(HashMap::new())),
                }));
            }

            if !current.pop() {
                break;
            }
        }

        Ok(None)
    }

    /// Get git status for a path relative to repo root (with caching)
    pub fn get_status(&self, rel_path: &Path) -> Option<GitStatus> {
        // Check cache first
        {
            let cache = self.status_cache.lock().ok()?;
            if let Some(&status) = cache.get(rel_path) {
                return Some(status);
            }
        }

        // Compute and cache
        let status = self.compute_status(rel_path)?;
        {
            let mut cache = self.status_cache.lock().ok()?;
            cache.insert(rel_path.to_path_buf(), status);
        }
        Some(status)
    }

    fn compute_status(&self, rel_path: &Path) -> Option<GitStatus> {
        // Use git status --porcelain for reliable, well-documented output
        // Format: XY PATH (where X=index, Y=worktree)
        // https://git-scm.com/docs/git-status#_short_format
        let output = Command::new("git")
            .args(["-C", &self.workdir.to_string_lossy(), "status", "--porcelain", "--ignored"])
            .output()
            .ok()?;

        if !output.status.success() {
            return None;
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        
        // Find matching line
        let rel_str = rel_path.to_string_lossy();
        for line in stdout.lines() {
            if line.len() < 3 {
                continue;
            }
            let status_chars = &line[0..2];
            let path = &line[3..];
            
            if path == rel_str {
                return Some(parse_git_status(status_chars));
            }
        }

        // If not found in output, it's unmodified
        Some(GitStatus::Unmodified)
    }
}

fn parse_git_status(chars: &str) -> GitStatus {
    // chars[0] = index status, chars[1] = worktree status
    let index = chars.chars().next().unwrap_or(' ');
    let worktree = chars.chars().nth(1).unwrap_or(' ');

    // Check for conflicts first
    if index == 'U' || worktree == 'U' {
        return GitStatus::Conflicted;
    }

    // Check worktree status (unstaged changes) - priority
    let wt_status = match worktree {
        'M' => Some(GitStatus::Modified),
        'D' => Some(GitStatus::Deleted),
        '?' => Some(GitStatus::Untracked),
        '!' => Some(GitStatus::Ignored),
        _ => None,
    };

    if let Some(s) = wt_status {
        return s;
    }

    // Check index status (staged changes)
    match index {
        'A' => GitStatus::Added,
        'M' => GitStatus::Modified,
        'D' => GitStatus::Deleted,
        'R' => GitStatus::Renamed,
        'C' => GitStatus::Copied,
        _ => GitStatus::Unmodified,
    }
}