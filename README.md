# lsr

[![Crates.io](https://img.shields.io/crates/v/lsr.svg)](https://crates.io/crates/lsr)
[![GitHub Release](https://img.shields.io/github/v/release/Aashman16wi/lsr)](https://github.com/Aashman16wi/lsr/releases)
[![License](https://img.shields.io/crates/l/lsr.svg)](https://github.com/Aashman16wi/lsr/blob/main/LICENSE)
[![Build](https://github.com/Aashman16wi/lsr/workflows/Release/badge.svg)](https://github.com/Aashman16wi/lsr/actions)

> **lsr** — A fast, modern `ls` replacement written in Rust.

---

## Why lsr?

| Feature | lsr | eza | lsd | ls |
|---------|-----|-----|-----|-----|
| **Memory** | **6.5 MiB** | 8–11 MiB | 9–10 MiB | 5 MiB |
| **Speed** | ~5 ms | ~4 ms | ~4 ms | ~3 ms |
| **Directory sizes (`--du`)** | ✅ | ❌ | ❌ | ❌ |
| **Content search (`--grep`)** | ✅ | ❌ | ❌ | ❌ |
| **Group by ext/date/size** | ✅ | ❌ | ❌ | ❌ |
| **Archive listing** | ✅ | ❌ | ❌ | ❌ |
| **Watch mode** | ✅ | ❌ | ❌ | ❌ |
| **Tree with `+N more`** | ✅ | ❌ | ❌ | ❌ |
| **Per-directory config** | ✅ | ❌ | ❌ | ❌ |

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

## Benchmarks

```bash
# Run on any directory
./benchmark.sh /path/to/dir
```

Results on 42-entry directory:

| Command | lsr | eza | lsd | ls |
|---------|-----|-----|-----|-----|
| **Default** | 9.9 ms / 6.5 MiB | 4.3 ms / 8.6 MiB | **4.1 ms** / 9.2 MiB | 3.5 ms / 5.3 MiB |
| **`-la`** | 10.7 ms / 6.5 MiB | 6.3 ms / 11.1 MiB | 6.6 ms / 10 MiB | **3.5 ms** / 5.3 MiB |
| **`--du`** | **7.2 ms** ✅ | N/A | N/A | N/A |
| **Tree** | 15.6 ms / 7 MiB | **3.6 ms** / 8.4 MiB | 4.3 ms / 8.8 MiB | N/A |
| **Recursive** | N/A | 29 s / 236 MiB ❌ | N/A | 9.3 s / 30 MiB |

---

## Why lsr?

- **Lowest memory** — 6.5 MiB vs 8–11 MiB for eza/lsd
- **Unique features** — `--du`, `--grep`, `--group-by`, `--archive`, `-w`
- **Dev-focused** — 100+ Nerd Font icons for code files
- **Configurable** — TOML config + per-directory overrides
- **Themes** — 6 built-in, all developer-friendly
- **No catastrophic recursion** — eza hits 29s on large dirs

---

## License

MIT OR Apache-2.0

---

## Contributing

PRs welcome! Run `cargo fmt --check` and `cargo clippy -- -D warnings` before submitting.

---

## Author

[Aashman16wi](https://github.com/Aashman16wi)