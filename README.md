# lsr

A modern `ls` replacement written in Rust. It acts like `ls` for everyday use and adds the things I kept wishing `ls` had: git status, Nerd Font icons, tree view, recursive directory sizes, archive inspection, and JSON/CSV output.

**Status:** 0.1.0, early development. The CLI will probably change.

---

## Why I built this

`ls` is fine. But I got tired of typing `ls -la`, then `git status`, then `tree -L 2`, then `du -sh *` just to understand a directory. `lsr` puts that in one command.

```
lsr -l --git          # long view with git status
lsr -T --depth 2      # tree, two levels
lsr -l --du           # recursive directory sizes
lsr --grep "TODO"     # find files containing text
lsr --archive data.zip
lsr -O json           # machine output
```

---

## Install

### From source (recommended)

```bash
git clone https://github.com/Aashman16wi/lsr
cd lsr
cargo build --release
mkdir -p ~/.local/bin
install -m755 target/release/lsr ~/.local/bin/lsr
```

Requires Rust 1.85+. Add `~/.local/bin` to your PATH.

### Prebuilt binaries

When there's a release, grab it from the [releases page](https://github.com/Aashman16wi/lsr/releases) or:

```bash
curl -fsSL https://raw.githubusercontent.com/Aashman16wi/lsr/main/install.sh | bash
```

Builds exist for Linux x86_64 (glibc + musl), Linux aarch64 (glibc), and macOS (Intel + Apple Silicon). Not on crates.io yet.

---

## The flags you'll actually use

| Flag | What it does |
|------|--------------|
| `lsr` | basic columns (like `ls`) |
| `lsr -a` | show hidden files |
| `lsr -l` | long view |
| `lsr -l --git` | long + git status |
| `lsr -T` | tree view |
| `lsr -T --depth 2` | tree, 2 levels |
| `lsr --du` | recursive dir sizes |
| `lsr --grep "foo"` | files containing "foo" |
| `lsr --group-by ext` | group by extension |
| `lsr -O json` | JSON output |
| `lsr --archive file.zip` | list archive |
| `lsr -w` | watch for changes |

Full flag list: `lsr --help`.

---

## The details

**Hidden files** — hidden by default (`.` prefix). Gitignored files are *not* hidden; that's what `ls` does. Use `--ignore "*.log"` if you want to filter them.

**Icons** — Nerd Font glyphs. You need a patched font. `--icons never` disables them. `--icons always` forces them. Default is auto (only on a TTY).

**Colors** — truecolor themes. `--color never` / `--color always` / `--color auto`. `NO_COLOR` disables auto color.

**Tree depth** — unlimited by default. Set `format.depth` in config or use `--depth N`. Large trees (100+ entries) fall back to regular listing with a warning.

**`--du`** — walks directories recursively. Respects `--ignore` and hidden settings.

**`--grep`** — case-insensitive, scans regular text files up to 10 MB. Binary/large files and directories stay visible (you still see where matches live).

**`--group-by`** — `ext`, `date`, or `size`. Sorts into contiguous groups, no headers printed.

**Archives** — zip, tar, tar.gz, tar.zst. Paths normalized (no `./` prefixes).

**Config** — `lsr --generate-config > ~/.config/lsr/config.toml`. Precedence: defaults → user config → project `.lsr.toml` → env vars → theme. CLI flags always win.

```toml
[display]
icons = "auto"
color = "auto"

[format]
default = "default"
# depth = 3   # omit for unlimited
```

**Env vars** — `LSR_DISPLAY__ICONS=always`, `LSR_FORMAT__DEFAULT=long`, etc. Double underscore between sections.

**Themes** — `default`, `dev`, `minimal`, `tokyo-night`, `catppuccin-mocha`, `gruvbox`, `rose-pine`. `--theme minimal` gives you plain columns, no icons, no color.

**Exit codes** — 0 success, 1 runtime error (bad path, missing config, unknown theme), 2 usage error. Malformed config warns and falls back to defaults.

---

## Platform support

- Linux: x86_64 (glibc + musl), aarch64 (glibc)
- macOS: x86_64 + Apple Silicon
- Windows: not supported (Unix metadata)

Installer picks musl on musl systems. aarch64 musl: build from source.

---

## License

MIT OR Apache-2.0. See `LICENSE-MIT` and `LICENSE-APACHE`.

---

## Contributing

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
```

Bug reports and PRs welcome.

---

*Built by [Aashman16wi](https://github.com/Aashman16wi) because I got tired of chaining `ls`, `git`, `tree`, and `du`.*