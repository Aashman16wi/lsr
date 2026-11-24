//! Git integration for repository status

use anyhow::Result;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::{Arc, Mutex};

use crate::fs::GitStatus;

/// Wrapper around a git repository with a cached status map.
pub struct GitRepo {
    workdir: PathBuf,
    statuses: Arc<Mutex<Option<HashMap<PathBuf, GitStatus>>>>,
}

impl GitRepo {
    /// Discover git repositories for the given paths.
    ///
    /// This also finds repositories directly below a listed directory so that
    /// nested repositories get correct statuses.
    pub fn discover_all(paths: &[PathBuf]) -> Result<HashMap<PathBuf, GitRepo>> {
        let mut repos = HashMap::new();

        for path in paths {
            if let Some(repo) = Self::discover(path)? {
                repos.insert(repo.workdir.clone(), repo);
            }

            if path.is_dir() {
                if let Ok(children) = std::fs::read_dir(path) {
                    for child in children.flatten() {
                        let child_path = child.path();
                        if child_path.is_dir() && child_path.join(".git").exists() {
                            if let Some(repo) = Self::discover(&child_path)? {
                                repos.insert(repo.workdir.clone(), repo);
                            }
                        }
                    }
                }
            }
        }

        Ok(repos)
    }

    /// Find the git repository for a path (walking up parents)
    fn discover(path: &Path) -> Result<Option<GitRepo>> {
        let mut current = if path.is_dir() {
            path.to_path_buf()
        } else {
            path.parent().unwrap_or(path).to_path_buf()
        };

        loop {
            if current.join(".git").exists() {
                return Ok(Some(GitRepo {
                    workdir: current,
                    statuses: Arc::new(Mutex::new(None)),
                }));
            }

            if !current.pop() {
                break;
            }
        }

        Ok(None)
    }

    /// Get git status for a path relative to the repo root.
    pub fn get_status(&self, rel_path: &Path) -> Option<GitStatus> {
        let mut guard = self.statuses.lock().ok()?;

        if guard.is_none() {
            *guard = Some(self.load_statuses().unwrap_or_default());
        }

        let statuses = guard.as_ref()?;

        if let Some(status) = statuses.get(rel_path) {
            return Some(*status);
        }

        // Directories are not listed by `git status`; use the first status
        // found below the directory instead.
        let child_status = statuses
            .iter()
            .find(|(path, _)| path.starts_with(rel_path))
            .map(|(_, status)| *status);

        Some(child_status.unwrap_or(GitStatus::Unmodified))
    }

    /// Run `git status` once and parse the whole repository into a map.
    fn load_statuses(&self) -> Option<HashMap<PathBuf, GitStatus>> {
        let output = Command::new("git")
            .args(["-C", self.workdir.to_str()?, "status", "--porcelain", "-z"])
            .output()
            .ok()?;

        if !output.status.success() {
            return None;
        }

        let mut statuses = HashMap::new();
        let mut records = output
            .stdout
            .split(|byte| *byte == 0)
            .filter(|r| !r.is_empty());

        while let Some(record) = records.next() {
            if record.len() < 4 {
                continue;
            }

            let status = parse_git_status(&String::from_utf8_lossy(&record[..2]));
            let path = String::from_utf8_lossy(&record[3..]);
            statuses.insert(PathBuf::from(path.as_ref()), status);

            // Renames and copies have the original path as the next record.
            let index = record[0];
            let worktree = record[1];
            if matches!(index, b'R' | b'C') || matches!(worktree, b'R' | b'C') {
                records.next();
            }
        }

        Some(statuses)
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

    if let Some(status) = wt_status {
        return status;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_common_statuses() {
        assert_eq!(parse_git_status(" M"), GitStatus::Modified);
        assert_eq!(parse_git_status("??"), GitStatus::Untracked);
        assert_eq!(parse_git_status("A "), GitStatus::Added);
        assert_eq!(parse_git_status("D "), GitStatus::Deleted);
        assert_eq!(parse_git_status("UU"), GitStatus::Conflicted);
        assert_eq!(parse_git_status("  "), GitStatus::Unmodified);
    }
}
