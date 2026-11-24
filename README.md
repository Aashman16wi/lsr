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
| **Colors** | Basic | Full ANSI 256-color + themes |
| **Icons** | ❌ | ✅ 100+ Nerd Font dev icons |
| **Git status** | ❌ | ✅ Built-in |
| **Tree view** | ❌ | ✅ `-T` / `--tree` |
| **Directory sizes** | ❌ | ✅ `--du` (parallel, fast) |
| **Content search** | ❌ | ✅ `--grep "PATTERN"` |
| **Group by ext/date/size** | ❌ | ✅ `--group-by` |
| **Archive listing** | ❌ | ✅ `--archive file.zip` |
| **Watch mode** | ❌ | ✅ `-w` auto-refresh |
| **Config file** | ❌ | ✅ TOML + per-dir `.lsr.toml` |
| **Themes** | ❌ | 6 built-in |

---

## Install

```bash
# Cargo (recommended)
cargo install lsr

# Quick install (Linux/macOS)
curl -sSL https://raw.githubusercontent.com/Aashman16wi/lsr/main/install.sh | bash

# Download binary
# See [Releases](https://github.com/Aashman16wi/lsr/releases)
```

---

## Quick Start

```bash
lsr              # Dev theme (long, git, icons, colors)
lsr -T           # Tree view
lsr -T --depth 2 # Tree, 2 levels
lsr --du         # Directory sizes
lsr --grep "TODO" # Find files containing "TODO"
lsr --group-by ext # Group by extension
lsr -w            # Watch for changes
```

---

## Features

| Flag | Description |
|------|-------------|
| `-l`, `--long` | Long format |
| `-T`, `--tree` | Tree view |
| `--depth N` | Tree depth (default: 3) |
| `-a`, `--all` | Show hidden files |
| `-g`, `--git` | Show git status |
| `--du` | Directory sizes (recursive) |
| `--grep PATTERN` | Filter files by content |
| `--group-by ext\|date\|size` | Group files |
| `--archive file.zip` | List archive contents |
| `-w`, `--watch` | Watch for changes |
| `-O json\|csv` | JSON/CSV output |
| `--theme NAME` | Theme (dev, tokyo-night, catppuccin, gruvbox, rose-pine, minimal) |

---

## Configuration

```bash
# Generate default config
lsr --generate-config > ~/.config/lsr/config.toml
```

Edit `~/.config/lsr/config.toml`:

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

# Per-directory overrides
[per_directory."~/projects/*"]
format = "tree"
depth = 2
show_git = true

[theme]
name = "tokyo-night"
```

Or create `.lsr.toml` in any project directory for per-project settings.

---

## Themes

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
```

---

## Man Page

```bash
sudo cp lsr.1 /usr/share/man/man1/
mandb
man lsr
```

---

## Why lsr?

- **Drop-in replacement** — `alias ls="lsr"` just works
- **Dev-first** — 100+ icons for code files (Cargo.toml, package.json, Dockerfile, .env, etc.)
- **Low memory** — ~6.5 MiB vs 8+ MiB for alternatives
- **Configurable** — TOML config + per-directory `.lsr.toml`
- **Themes** — 6 built-in, all developer-friendly
- **No bloat** — Single binary, no daemon, no background services

---

## License

MIT OR Apache-2.0

---

## Contributing

PRs welcome! Run `cargo fmt --check` and `cargo clippy -- -D warnings` before submitting.

---

## Author

[Aashman16wi](https://github.com/Aashman16wi)