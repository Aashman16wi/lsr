# lsr

[![Crates.io](https://img.shields.io/crates/v/lsr.svg)](https://crates.io/crates/lsr)
[![GitHub Release](https://img.shields.io/github/v/release/Aashman16wi/lsr)](https://github.com/Aashman16wi/lsr/releases)
[![License](https://img.shields.io/crates/l/lsr.svg)](https://github.com/Aashman16wi/lsr/blob/main/LICENSE)
[![Build](https://github.com/Aashman16wi/lsr/workflows/Release/badge.svg)](https://github.com/Aashman16wi/lsr/actions)

> **lsr** — A fast, modern `ls` replacement written in Rust.

---

## Why lsr?

`lsr` is a drop-in replacement for `ls` that adds modern features developers expect:

| Feature | `ls` | `lsr` |
|---------|------|-------|
| Colors | Basic | Full ANSI 256-color + themes |
| Icons | ❌ | ✅ 100+ Nerd Font dev icons |
| Git status | ❌ | ✅ Built-in |
| Tree view | ❌ | ✅ `-T` / `--tree` |
| Directory sizes | ❌ | ✅ `--du` (parallel, fast) |
| Content search | ❌ | ✅ `--grep "PATTERN"` |
| Group by ext/date/size | ❌ | ✅ `--group-by` |
| Archive listing | ❌ | ✅ `--archive file.zip` |
| Watch mode | ❌ | ✅ `-w` auto-refresh |
| Config file | ❌ | ✅ TOML + per-dir `.lsr.toml` |
| Themes | ❌ | 6 built-in |

---

## Install

```bash
# Cargo (recommended)
cargo install lsr

# Quick install (Linux/macOS)
curl -sSL https://raw.githubusercontent.com/Aashman16wi/lsr/main/install.sh | bash

# Download binary
# See [Releases](https://github.com/Aashman16wi/lsr/releases)

# Homebrew (coming soon)
brew install Aashman16wi/tap/lsr

# Arch Linux (AUR)
yay -S lsr

# Fedora
sudo dnf copr enable Aashman16wi/lsr && sudo dnf install lsr
```

---

## Quick Start

```bash
# Basic
lsr

# Long format with git status
lsr -l -g

# Tree view
lsr -T

# Tree with git, 2 levels
lsr -T -g --depth 2

# Show directory sizes recursively
lsr --du

# Group by extension
lsr --group-by ext

# Filter files containing text
lsr --grep "TODO"

# List archive contents
lsr --archive archive.zip

# Watch for changes
lsr -w

# JSON output for scripts
lsr --format json
```

---

## Features

| Flag | Description |
|------|-------------|
| `-l`, `--long` | Long listing format |
| `-T`, `--tree` | Tree view |
| `--depth N` | Tree depth (default: 3) |
| `-a`, `--all` | Show hidden files |
| `-A`, `--almost-all` | Show hidden except `.` and `..` |
| `-g`, `--git` | Show git status |
| `--du` | Directory sizes (recursive, parallel) |
| `--grep PATTERN` | Filter files containing text |
| `--group-by MODE` | Group by: `ext`, `date`, `size` |
| `--archive` | List archive (zip, tar, tar.gz, tar.zst) |
| `-w`, `--watch` | Watch for changes (like `watch -n 1`) |
| `--theme NAME` | Theme: `dev`, `tokyo-night`, `catppuccin-mocha`, `gruvbox`, `rose-pine`, `minimal` |
| `-O FORMAT` | Output: `default`, `long`, `tree`, `json`, `csv` |
| `--config FILE` | Custom config file |
| `--generate-config` | Print default config |

---

## Configuration

Generate a default config:

```bash
lsr --generate-config > ~/.config/lsr/config.toml
```

Example `~/.config/lsr/config.toml`:

```toml
[display]
icons = "always"
color = "always"
human_readable = true

[format]
default = "tree"
depth = 3
tree_max_entries = 200

[git]
show_status = true

# Per-directory overrides (glob patterns)
[per_directory."~/projects/*"]
format = "tree"
depth = 4
show_git = true

[per_directory."~/Downloads"]
format = "default"
color = "never"
icons = "never"

[theme]
name = "tokyo-night"
```

---

## Themes

Built-in themes (use `--theme NAME` or config):

| Theme | Preview |
|-------|---------|
| `dev` (default) | Long, git, blue/green |
| `tokyo-night` | Dark blue/purple |
| `catppuccin-mocha` | Warm pink/blue |
| `gruvbox` | Earth tones |
| `rose-pine` | Soft pink/blue |
| `minimal` | No icons/colors |

```bash
lsr --theme tokyo-night
lsr --theme gruvbox
```

---

## Shell Completions

```bash
# Bash
lsr --completion bash > /usr/share/bash-completion/completions/lsr

# Zsh
lsr --completion zsh > ~/.zfunc/_lsr

# Fish
lsr --completion fish > ~/.config/fish/completions/lsr.fish

# PowerShell
lsr --completion powershell > lsr.ps1

# Elvish
lsr --completion elvish > ~/.config/elvish/lib/lsr.elv
```

---

## Man Page

```bash
# Install
sudo cp lsr.1 /usr/share/man/man1/
mandb

# View
man lsr
```

---

## Benchmarks

```bash
# Run on any directory
./benchmark.sh /path/to/dir
```

Tested on 1000 files, AMD Ryzen 7, NVMe SSD:

| Command | Mean Time | Memory |
|---------|-----------|--------|
| `lsr` | **2.7 ms** | **6.5 MiB** |
| `lsr -la --du` | **4.4 ms** | **6.7 MiB** |
| `lsr -laR` (recursive) | **4.2 ms** | **6.5 MiB** |
| `lsr -T` (tree) | **6.0 ms** | **7.1 MiB** |
| `eza -laR` | 23.4 s | 236 MiB |
| `ls -laR` | 122 ms | 5.4 MiB |

Run your own: `./benchmark.sh /path/to/dir`

---

## Dev Icons (100+)

Built-in icons for: Cargo.toml 🦀, package.json 📦, Dockerfile 🐳, .env 🔑, tsconfig.json 📝, Makefile 🔨, README.md 📖, LICENSE ⚖️, .github 🐙, .vscode 💙, .dockerignore 🐳, .gitignore 🚫, .eslintrc 📏, .prettierrc 🎨, .rustfmt.toml ⚙️, Dockerfile 🐳, docker-compose.yml 🐳, tsconfig.json 📝, webpack.config.js 📦, vite.config.ts ⚡, justfile ⚡, Taskfile.yml ✅, mise.toml 🔧, flake.nix ❄️, deno.json 🦕, bun.lockb 🍞, pyproject.toml 🐍, poetry.lock 🔒, Pipfile 📦, requirements.txt 📋, go.mod 🐹, go.sum 🐹, Cargo.toml 🦀, Cargo.lock 🔒, etc.

---

## License

MIT OR Apache-2.0

---

## Contributing

PRs welcome! Please:

1. Run `cargo fmt --check`
2. Run `cargo clippy -- -D warnings`
3. Add tests for new features

---

## Similar Projects

- [eza](https://github.com/eza-community/eza) — Modern `ls` with colors, git, tree
- [lsd](https://github.com/lsd-rs/lsd) — `ls` with icons
- [yazi](https://github.com/sxyazi/yazi) — Terminal file manager
- [superfile](https://github.com/yorukot/superfile) — Modern TUI file manager

---

## Author

[Aashman16wi](https://github.com/Aashman16wi)