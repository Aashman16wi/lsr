//! File icons and git status symbols.

use crate::fs::FileType;
use std::path::Path;

/// Get the icon for a file based on its name, extension, and type.
pub fn get_icon(path: &Path, file_type: FileType, show_icons: bool) -> &'static str {
    if !show_icons {
        return "";
    }

    let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();

    // Exact filenames first. These are more useful than extension matches
    // for files like Cargo.toml or Dockerfile.
    let icon = match name {
        // Git
        ".gitignore" | ".gitmodules" | ".gitattributes" | ".gitignore_global" | ".gitconfig" => "󰊢",
        // Docker / containers
        ".dockerignore" => "󰡨",
        "Dockerfile"
        | "docker-compose.yml"
        | "docker-compose.yaml"
        | "docker-compose.override.yml" => "󰡨",
        // Editors
        ".editorconfig" => "",
        ".vscode" => "󰨞",
        ".idea" => "",
        ".devcontainer" => "󰡨",
        // Environment
        ".env" | ".env.local" | ".env.production" | ".env.development" | ".env.test" => "",
        // Linting / formatting
        ".eslintrc" | ".eslintrc.js" | ".eslintrc.json" | ".eslintrc.yaml" | ".eslintrc.yml" => "󰱺",
        ".prettierrc" | ".prettierrc.js" | ".prettierrc.json" | ".prettierrc.yaml"
        | ".prettierrc.yml" => "",
        ".rustfmt.toml" => "",
        // CI
        ".github" => "",
        ".gitlab" => "",
        ".circleci" => "󰏓",
        ".travis.yml" => "󰯅",
        // Task runners / toolchains
        "justfile" | "Justfile" => "",
        "Taskfile.yml" | "Taskfile.yaml" => "󰎙",
        "mise.toml" | ".mise.toml" | ".tool-versions" => "󰏔",
        "flake.nix" | "flake.lock" => "󱄅",
        "deno.json" | "deno.jsonc" | "import_map.json" => "󰋯",
        "bun.lockb" | "bun.lock" => "󰎙",
        // Language manifests
        "pyproject.toml"
        | "poetry.lock"
        | "Pipfile"
        | "Pipfile.lock"
        | "requirements.txt"
        | "requirements-dev.txt" => "󰌠",
        "go.mod" | "go.sum" | "go.work" => "󰟓",
        "composer.json" | "composer.lock" => "󰌟",
        "Gemfile" | "Gemfile.lock" => "󰴭",
        "pom.xml"
        | "build.gradle"
        | "build.gradle.kts"
        | "settings.gradle"
        | "settings.gradle.kts"
        | "build.xml"
        | "ivy.xml" => "󰬷",
        "bazel" | "BUILD" | "WORKSPACE" | "BUILD.bazel" => "󰆧",
        // Rust / Node
        "Cargo.toml" | "Cargo.lock" => "",
        "package.json" | "package-lock.json" | "pnpm-lock.yaml" | "yarn.lock" => "󰎙",
        "tsconfig.json" | "jsconfig.json" => "",
        // Bundlers
        "webpack.config.js" | "webpack.config.ts" | "vite.config.ts" | "vite.config.js"
        | "rollup.config.js" | "rollup.config.ts" | "turbo.json" => "󰜫",
        // Build systems
        "Makefile" | "makefile" | "CMakeLists.txt" | "meson.build" | "build.zig" => "",
        // Documentation
        "README.md" | "README.txt" | "README.rst" => "",
        "LICENSE" | "LICENSE.md" | "LICENSE.txt" | "COPYING" => "",
        "CHANGELOG.md" | "CHANGELOG.txt" | "CHANGES.md" => "",
        "CONTRIBUTING.md" | "CONTRIBUTING.txt" | "CODE_OF_CONDUCT.md" => "󰍔",
        "SECURITY.md" | "SECURITY.txt" => "󰌾",
        _ => "",
    };

    if !icon.is_empty() {
        return icon;
    }

    match ext.as_str() {
        // Rust
        "rs" => "",
        "toml" => "",

        // JavaScript / TypeScript
        "js" | "jsx" | "mjs" | "cjs" => "󰌞",
        "ts" | "tsx" | "mts" | "cts" => "",
        "json" | "jsonc" => "",

        // Python
        "py" | "pyw" | "pyc" | "pyo" | "pyd" => "󰌠",
        "ipynb" => "󰠮",

        // Go
        "go" | "mod" | "sum" => "󰟓",

        // C / C++
        "c" | "h" => "",
        "cpp" | "cc" | "cxx" | "hpp" | "hh" | "hxx" => "󰙲",
        "cs" => "󰌛",

        // JVM
        "java" => "󰬷",
        "kt" | "kts" => "󰘦",
        "scala" => "󰜷",
        "groovy" => "󰩏",

        // Web
        "html" | "htm" => "󰌝",
        "css" | "scss" | "sass" | "less" => "󰌛",
        "xml" | "xsl" | "xslt" => "󰗀",
        "svg" => "󰜡",

        // Shell
        "sh" | "bash" | "zsh" | "fish" => "",
        "ps1" | "psm1" => "󰨊",
        "bat" | "cmd" => "",

        // Config
        "yaml" | "yml" => "",
        "ini" | "cfg" | "conf" | "properties" => "",
        "env" => "",

        // Databases
        "sql" | "db" | "sqlite" | "sqlite3" => "󰆼",

        // Documentation
        "md" | "markdown" | "mdown" | "mkd" => "󰍔",
        "rst" | "txt" | "log" => "󰈙",
        "tex" | "latex" => "󰊄",
        "pdf" => "󰈦",

        // Images
        "png" | "jpg" | "jpeg" | "gif" | "bmp" | "tiff" | "tif" | "webp" | "avif" | "ico" => "󰈟",
        "psd" | "xcf" => "󰏘",

        // Video
        "mp4" | "mkv" | "webm" | "mov" | "avi" | "wmv" | "flv" | "m4v" => "󰈫",

        // Audio
        "mp3" | "flac" | "wav" | "ogg" | "m4a" | "aac" | "opus" => "󰎆",

        // Archives
        "zip" | "tar" | "gz" | "bz2" | "xz" | "7z" | "rar" | "zst" => "󰗄",

        // Executables
        "exe" | "msi" | "app" | "apk" | "deb" | "rpm" => "󰏓",
        "so" | "dylib" | "dll" => "󰆧",

        // Fonts
        "ttf" | "otf" | "woff" | "woff2" | "eot" => "󰛖",

        // Other languages
        "php" => "󰌟",
        "rb" | "rake" => "󰴭",
        "pl" | "pm" => "󰿠",
        "lua" => "󰢱",
        "r" => "󰟔",
        "jl" => "",
        "nim" => "󰆥",
        "zig" => "󰩳",
        "v" => "󰋯",
        "dart" => "󰟓",
        "swift" => "󰛥",
        "ml" | "mli" => "λ",
        "fs" | "fsx" => "󰧁",
        "clj" | "cljs" | "cljc" => "",
        "hs" | "lhs" => "󰧑",
        "elm" => "",
        "ex" | "exs" => "󰭌",
        "erl" | "hrl" => "󰱂",
        "prolog" => "󰠵",
        "asm" | "s" => "󰘚",

        _ => match file_type {
            FileType::Directory => "󰉋",
            FileType::Symlink => "󰉒",
            FileType::BlockDevice | FileType::CharDevice => "󰋊",
            FileType::Fifo | FileType::Socket => "󰟛",
            FileType::Regular | FileType::Unknown => "󰈙",
        },
    }
}

/// RGB color for a git status.
pub fn git_status_color(status: &crate::fs::GitStatus) -> (u8, u8, u8) {
    match status {
        crate::fs::GitStatus::Modified => (255, 170, 0),
        crate::fs::GitStatus::Added => (0, 200, 0),
        crate::fs::GitStatus::Deleted => (255, 50, 50),
        crate::fs::GitStatus::Untracked => (255, 150, 0),
        crate::fs::GitStatus::Ignored => (128, 128, 128),
        crate::fs::GitStatus::Renamed | crate::fs::GitStatus::Copied => (0, 200, 200),
        crate::fs::GitStatus::Conflicted => (255, 0, 255),
        crate::fs::GitStatus::Unmodified => (100, 100, 100),
    }
}

/// Icon for a git status.
pub fn git_status_icon(status: &crate::fs::GitStatus) -> &'static str {
    match status {
        crate::fs::GitStatus::Modified => "󰷥",
        crate::fs::GitStatus::Added => "󰐕",
        crate::fs::GitStatus::Deleted => "󰍵",
        crate::fs::GitStatus::Untracked => "󰞋",
        crate::fs::GitStatus::Ignored => "󰞋",
        crate::fs::GitStatus::Renamed => "󰑕",
        crate::fs::GitStatus::Copied => "󰅍",
        crate::fs::GitStatus::Conflicted => "󰈊",
        crate::fs::GitStatus::Unmodified => "",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn icons_are_optional() {
        assert_eq!(get_icon(Path::new("main.rs"), FileType::Regular, false), "");
    }

    #[test]
    fn known_filenames_and_extensions() {
        assert_eq!(
            get_icon(Path::new("Cargo.toml"), FileType::Regular, true),
            ""
        );
        assert_eq!(get_icon(Path::new("main.rs"), FileType::Regular, true), "");
        assert_eq!(get_icon(Path::new("src"), FileType::Directory, true), "󰉋");
    }
}
