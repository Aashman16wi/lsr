//! File icons and symbols

use crate::fs::FileType;
use std::path::Path;

/// Get icon for a file based on name, extension, and type
pub fn get_icon(path: &Path, file_type: FileType, show_icons: bool) -> &'static str {
    if !show_icons {
        return "";
    }

    let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
    let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("").to_lowercase();

    // Special filenames first
    let icon = match name {
        // Config files
        ".gitignore" | ".gitmodules" | ".gitattributes" => "󰊢",
        ".dockerignore" => "󰡨",
        ".editorconfig" => "",
        ".env" | ".env.local" | ".env.production" => "",
        ".eslintrc" | ".eslintrc.js" | ".eslintrc.json" => "󰱺",
        ".prettierrc" | ".prettierrc.js" | ".prettierrc.json" => "",
        ".rustfmt.toml" => "",
        "Cargo.toml" | "Cargo.lock" => "",
        "package.json" | "package-lock.json" | "pnpm-lock.yaml" | "yarn.lock" => "󰎙",
        "tsconfig.json" | "jsconfig.json" => "",
        "webpack.config.js" | "vite.config.ts" | "rollup.config.js" => "󰜫",
        "Makefile" | "makefile" | "CMakeLists.txt" => "",
        "Dockerfile" | "docker-compose.yml" | "docker-compose.yaml" => "󰡨",
        "README.md" | "README.txt" | "README.rst" => "",
        "LICENSE" | "LICENSE.md" | "LICENSE.txt" => "",
        "CHANGELOG.md" | "CHANGELOG.txt" => "",
        ".github" => "",
        ".gitlab" => "",
        ".vscode" => "󰨞",
        ".idea" => "",
        _ => {
            // By extension
            match ext.as_str() {
                // Rust
                "rs" => "",
                "toml" => "",
                
                // JavaScript/TypeScript
                "js" | "jsx" | "mjs" | "cjs" => "󰌞",
                "ts" | "tsx" | "mts" | "cts" => "",
                "json" => "",
                "jsonc" => "",
                
                // Python
                "py" | "pyw" | "pyc" | "pyo" | "pyd" => "󰌠",
                "ipynb" => "󰠮",
                "requirements.txt" | "setup.py" | "setup.cfg" | "pyproject.toml" | "pipfile" => "󰌠",
                
                // Go
                "go" | "mod" | "sum" => "󰟓",
                
                // C/C++
                "c" | "h" => "",
                "cpp" | "cc" | "cxx" | "hpp" | "hh" | "hxx" => "󰙲",
                "cs" => "󰌛",
                
                // Java/Kotlin
                "java" => "󰬷",
                "kt" | "kts" => "󰘦",
                "scala" => "󰜷",
                "groovy" => "󰩏",
                
                // Web
                "html" | "htm" => "󰌝",
                "css" | "scss" | "sass" | "less" => "󰌛",
                "xml" | "xsl" | "xslt" => "󰗀",
                "svg" => "󰜡",
                
                // Shell/Scripts
                "sh" | "bash" | "zsh" | "fish" => "",
                "ps1" | "psm1" => "󰨊",
                "bat" | "cmd" => "",
                
                // Data/Config
                "yaml" | "yml" => "",
                "ini" | "cfg" | "conf" => "",
                "env" => "",
                "properties" => "",
                
                // Database
                "sql" => "󰆼",
                "db" | "sqlite" | "sqlite3" => "󰆼",
                
                // Documentation
                "md" | "markdown" | "mdown" | "mkd" => "󰍔",
                "rst" => "󰈙",
                "txt" | "log" => "󰈙",
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
                
                // Other code
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
                
                _ => {
                    // By file type (fallback)
                    match file_type {
                        FileType::Directory => "󰉋",
                        FileType::Symlink => "󰉒",
                        FileType::BlockDevice => "󰋊",
                        FileType::CharDevice => "󰋊",
                        FileType::Fifo => "󰟛",
                        FileType::Socket => "󰟛",
                        FileType::Regular => "󰈙",
                        FileType::Unknown => "󰈙",
                    }
                }
            }
        }
    };

    icon
}

/// Git status color as RGB tuple
pub fn git_status_color(status: crate::fs::GitStatus) -> (u8, u8, u8) {
    use crate::fs::GitStatus;
    match status {
        GitStatus::Unmodified => (0x60, 0x60, 0x60),
        GitStatus::Modified => (0xFA, 0xC2, 0x00),   // Yellow
        GitStatus::Added => (0x5F, 0xD7, 0x5F),       // Green
        GitStatus::Deleted => (0xFF, 0x5F, 0x5F),     // Red
        GitStatus::Renamed => (0x5F, 0xAF, 0xFF),     // Blue
        GitStatus::Copied => (0xAF, 0x5F, 0xFF),      // Magenta
        GitStatus::Untracked => (0xFF, 0xAF, 0x00),   // Orange
        GitStatus::Ignored => (0x60, 0x60, 0x60),     // Gray
        GitStatus::Conflicted => (0xFF, 0x00, 0xFF),  // Bright Magenta
    }
}

/// Get the Nerd Font icon for a git status
pub fn git_status_icon(status: crate::fs::GitStatus) -> &'static str {
    use crate::fs::GitStatus;
    match status {
        GitStatus::Unmodified => " ",
        GitStatus::Modified => "󰷥",
        GitStatus::Added => "󰐖",
        GitStatus::Deleted => "󰍴",
        GitStatus::Renamed => "󰑕",
        GitStatus::Copied => "󰆏",
        GitStatus::Untracked => "󰞋",
        GitStatus::Ignored => "󰘓",
        GitStatus::Conflicted => "󰘯",
    }
}