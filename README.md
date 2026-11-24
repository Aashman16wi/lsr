# lsr

A modern `ls` replacement written in Rust.

`lsr` behaves like `ls` for everyday listing and adds optional detail: git
status, Nerd Font icons, themes, tree view, recursive directory sizes, archive
inspection, and JSON/CSV output.

> **Status:** 0.1.0, early development. The CLI may change between releases.

## Why lsr?

| Feature | `ls` | `lsr` |
|---|---|---|
| Familiar flags and columnar output | Yes | Yes |
| Hidden files hidden by default | Yes | Yes |
| Color output | Yes (`--color`) | Yes, with truecolor themes |
| Nerd Font file icons | No | Yes |
| Git status | No | Yes (`--git`) |
| Tree view | No | Yes (`-T`) |
| Recursive directory sizes | No | Yes (`--du`) |
| Content filtering | No | Yes (`--grep`) |
| Grouped sorting by extension/date/size | No | Yes (`--group-by`) |
| Archive listing | No | Yes (`--archive`) |
| Watch mode | No | Yes (`-w`) |
| TOML config and per-directory rules | No | Yes |
| JSON/CSV output | No | Yes |

## Install

### Build from source

Requires Rust 1.85 or newer.

```bash
git clone https://github.com/Aashman16wi/lsr
cd lsr
cargo build --release
mkdir -p ~/.local/bin
install -m755 target/release/lsr ~/.local/bin/lsr
```

Make sure `~/.local/bin` is on your `PATH`.

### Prebuilt binaries

When a release is available, download the archive for your platform from the
[Releases](https://github.com/Aashman16wi/lsr/releases) page, or run:

```bash
curl -fsSL https://raw.githubusercontent.com/Aashman16wi/lsr/main/install.sh | bash
```

Release builds target Linux x86_64 (glibc and musl), Linux aarch64 (glibc),
and macOS (Intel and Apple Silicon). `lsr` is not published on crates.io yet;
build from source or use a release archive.

## Quick start

```bash
# Basic listing (same shape as ls)
lsr

# Long view with git status
lsr -l --git

# Tree view, two levels deep
lsr -T --depth 2

# Recursive directory sizes
lsr -l --du

# Files containing a string (case-insensitive, regular text files up to 10 MB)
lsr --grep "TODO"

# Sort into groups by extension
lsr --group-by ext

# Inspect an archive
lsr --archive project.zip

# Machine-readable output
lsr -O json
lsr -O csv

# Watch for changes
lsr -w
```

## Usage

### Listing

```bash
lsr              # current directory, columns
lsr src tests    # multiple directories
lsr -a           # include hidden entries
lsr -A           # include hidden entries except . and ..
lsr -1           # one entry per line
lsr -R           # recurse into subdirectories
lsr -F           # append /, @, * classifiers
```

`lsr` does not hide gitignored files by default. That matches `ls`; use
`--ignore` if you want to filter them explicitly.

### Long view

```bash
lsr -l
lsr -l --git
lsr -l -i --links -o -g
lsr -l -H          # human-readable sizes (1.2K, 4.5M)
lsr -l --bytes     # exact byte counts
lsr -l --time accessed
lsr -l --time-style iso
lsr -l --full-time
```

`-H` is used for human-readable sizes because `-h` is reserved for `--help`.

### Tree view

```bash
lsr -T
lsr -T --depth 2
lsr -T --git
```

The default tree depth is unlimited when no theme or config sets
`format.depth`. Large trees are capped by `format.tree_max_entries` (100 by
default); above that limit `lsr` falls back to a regular listing and prints a
warning.

### Directory sizes

```bash
lsr --du          # sizes in the default view
lsr -l --du       # sizes in the long view
lsr --du --depth 2 -T
```

Directory totals are calculated recursively and respect `--ignore` and hidden
file settings.

### Git status

```bash
lsr -l --git
lsr -T --git --depth 2
```

Status is read from `git status --porcelain` once per repository and cached for
the run. `--git` works in the default, long, and tree views.

### Icons and color

Icons are Nerd Font glyphs. They require a patched Nerd Font in your terminal.

```bash
lsr --icons always
lsr --icons never
lsr --color always
lsr --color never
lsr --theme tokyo-night
```

In auto mode, color and icons are enabled only when stdout is a terminal.
`NO_COLOR` disables color in auto mode.

### Filtering

```bash
lsr --ignore "*.log"       # hide matching entries
lsr --ignore "target"      # hide a directory
lsr --glob "*.rs"          # only show matching entries
lsr --type f               # only regular files
lsr --type d               # only directories
lsr --type l               # only symlinks
```

Patterns are globs. `*` matches within one path component; `**` crosses
directories.

### Content filtering

```bash
lsr --grep "TODO"
lsr --grep "fn main" src
```

`--grep` checks regular text files up to 10 MB and is case-insensitive. Binary
files and larger files are left visible because they cannot be searched
safely. Directories are also kept so the result still shows where matches
live.

### Grouped sorting

```bash
lsr --group-by ext
lsr --group-by date
lsr --group-by size
```

Entries are sorted into contiguous groups: by extension, by modification day
(newest first), or by size bucket (largest first). No group headers are
printed.

### Archives

```bash
lsr --archive project.zip
lsr --archive backup.tar
lsr --archive logs.tar.gz
lsr --archive source.tar.zst
```

Archive contents are shown as a tree. Paths are normalised, so `./dir/file`
entries from tar archives display cleanly.

### Watch mode

```bash
lsr -w
lsr -w --watch-interval 1 src
```

The screen is cleared and the listing is redrawn when files change or when the
interval expires. Press `Ctrl+C` to stop.

### Sorting

```bash
lsr -s name
lsr -s size
lsr -s time
lsr -s extension
lsr -s inode
lsr -s none
lsr -r                 # reverse
lsr --group-dirs-first
```

### Output formats

```bash
lsr -O json
lsr -O csv
lsr -O long
lsr -O tree
lsr -O default
```

JSON and CSV are machine-readable; `long`, `tree`, and `default` are display
formats.

### Other options

```bash
lsr -L                 # follow symlinks
lsr --numeric-ids      # numeric uid/gid instead of names
lsr --threads 4        # worker threads
lsr --no-parallel      # disable parallel processing
lsr -v                 # verbose diagnostics
lsr --config-file PATH # use a specific config file (alias: --config)
lsr --generate-config  # print a default config file to stdout
```

## Themes

Built-in themes:

| Theme | Behaviour |
|---|---|
| `default` | Long view, icons, color, git status, depth 3 |
| `dev` | Same as `default`, tuned for development trees |
| `minimal` | Plain columns, no icons, no color, no git |
| `tokyo-night` | Tree view with a blue/purple palette |
| `catppuccin-mocha` | Tree view with a warm palette |
| `gruvbox` | Tree view with earth tones |
| `rose-pine` | Tree view with soft pink/blue tones |

```bash
lsr --theme default
lsr --theme minimal
lsr --theme gruvbox
```

An unknown theme name is an error and lists the available themes.

## Configuration

Generate a starting point:

```bash
lsr --generate-config > ~/.config/lsr/config.toml
```

Example `~/.config/lsr/config.toml`:

```toml
[display]
icons = "auto"          # auto | always | never
color = "auto"          # auto | always | never
human_readable = true
show_git = false        # --git also enables this
group_dirs_first = true

[format]
default = "default"     # default | long | tree | json | csv
time_style = "locale"   # iso | locale | relative | full, or a chrono format
# depth = 3             # tree depth; omit for unlimited
tree_max_entries = 200  # fall back to a regular listing above this count

[colors]
# Truecolor hex values.
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

# Per-directory overrides.
# `*` matches within one path component; `**` crosses directories.
[per_directory."~/projects/*"]
format = "tree"
depth = 2
show_git = true

[per_directory."~/Downloads"]
format = "default"
icons = "never"
color = "never"
```

Configuration sources are merged in this order: built-in defaults, user
config, project `.lsr.toml`, environment variables, then a built-in theme when
`--theme` is used. Explicit CLI format flags, `--icons`, and `--color` take
precedence over config values. Per-directory overrides are applied to matching
paths after the base config.

Environment variables use the `LSR_` prefix and `__` between sections:

```bash
LSR_DISPLAY__ICONS=always lsr
LSR_FORMAT__DEFAULT=long lsr
```

## Dev icons

`lsr` ships with Nerd Font icons for common development files. Exact filename
matches take priority over extension matches.

Examples of exact names:

```text
Cargo.toml, Cargo.lock, Dockerfile, docker-compose.yml, .dockerignore,
.gitignore, .gitattributes, .env, .editorconfig, .eslintrc, .prettierrc,
Makefile, CMakeLists.txt, justfile, Taskfile.yml, mise.toml, flake.nix,
package.json, tsconfig.json, vite.config.ts, webpack.config.js,
pyproject.toml, requirements.txt, Pipfile, poetry.lock,
go.mod, go.sum, composer.json, Gemfile, pom.xml, build.gradle,
README.md, LICENSE, CHANGELOG.md, CONTRIBUTING.md, SECURITY.md
```

Examples of extensions:

```text
Rust: .rs, .toml
JavaScript/TypeScript: .js, .jsx, .ts, .tsx, .mjs, .cjs
Python: .py, .ipynb
Go: .go
C/C++: .c, .h, .cpp, .hpp, .cc
JVM: .java, .kt, .kts, .scala
Shell: .sh, .bash, .zsh, .fish, .ps1
Config: .yaml, .yml, .json, .ini, .cfg, .conf
Docs: .md, .txt, .rst, .pdf
Media: .png, .jpg, .svg, .mp4, .mp3
Archives: .zip, .tar, .gz, .zst
```

Icons are plain Nerd Font glyphs, not emoji. Disable them with
`--icons never` or `icons = "never"` in the config.

## Shell completions

```bash
# Bash
lsr --generate-completion bash > ~/.local/share/bash-completion/completions/lsr

# Zsh
lsr --generate-completion zsh > ~/.zfunc/_lsr

# Fish
lsr --generate-completion fish > ~/.config/fish/completions/lsr.fish
```

`--completion` is accepted as an alias.

## Platform support

- Linux: x86_64 (glibc and musl), aarch64 (glibc)
- macOS: x86_64 and Apple Silicon

The installer selects the x86_64 musl build on musl systems. aarch64 musl
users can build from source.

Windows is not supported yet because file metadata handling is Unix-specific.

## Exit status

- `0`: success
- `1`: runtime error (missing path, missing `--config-file`, unknown theme)
- `2`: command-line usage error

A malformed config file prints a warning and falls back to defaults rather than
aborting.

## License

Licensed under either of:

- [MIT License](LICENSE-MIT)
- [Apache License, Version 2.0](LICENSE-APACHE)

at your option.

## Contributing

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
```

Bug reports and pull requests are welcome.

## Author

[Aashman16wi](https://github.com/Aashman16wi)
