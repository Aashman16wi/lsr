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
        // Git
        ".gitignore" | ".gitmodules" | ".gitattributes" | ".gitignore_global" | ".gitconfig" => "󰊢",
        // Docker
        ".dockerignore" => "󰡨",
        // Editor
        ".editorconfig" => "",
        // Environment
        ".env" | ".env.local" | ".env.production" | ".env.development" | ".env.test" => "",
        // Linting/Format
        ".eslintrc" | ".eslintrc.js" | ".eslintrc.json" | ".eslintrc.yaml" | ".eslintrc.yml" => "󰱺",
        ".prettierrc" | ".prettierrc.js" | ".prettierrc.json" | ".prettierrc.yaml" | ".prettierrc.yml" => "",
        ".rustfmt.toml" => "",
        // CI/CD
        ".github" => "",
        ".gitlab" => "",
        ".circleci" => "󰏓",
        ".travis.yml" => "󰯅",
        // Editors
        ".vscode" => "󰨞",
        ".idea" => "",
        // Containers
        ".devcontainer" => "󰡨",
        // Task runners
        "justfile" | "Justfile" => "",
        "Taskfile.yml" | "Taskfile.yaml" => "󰎙",
        "mise.toml" | ".mise.toml" | ".tool-versions" => "󰏔",
        "flake.nix" | "flake.lock" => "󱄅",
        // Deno
        "deno.json" | "deno.jsonc" | "import_map.json" => "󰋯",
        // Bun
        "bun.lockb" | "bun.lock" => "󰎙",
        // Python
        "pyproject.toml" | "poetry.lock" | "Pipfile" | "Pipfile.lock" | "requirements.txt" | "requirements-dev.txt" => "󰌠",
        // Go
        "go.mod" | "go.sum" | "go.work" => "󰟓",
        // PHP
        "composer.json" | "composer.lock" => "󰌟",
        // Ruby
        "Gemfile" | "Gemfile.lock" => "󰴭",
        // Java/Kotlin
        "pom.xml" | "build.gradle" | "build.gradle.kts" | "settings.gradle" | "settings.gradle.kts" => "󰬷",
        "build.xml" | "ivy.xml" => "󰬷",
        // Bazel
        "bazel" | "BUILD" | "WORKSPACE" | "BUILD.bazel" => "󰆧",
        // Config files
        ".gitignore" | ".gitmodules" | ".gitattributes" | ".gitignore_global" | ".gitconfig" => "󰊢",
        ".dockerignore" => "󰡨",
        ".editorconfig" => "",
        ".env" | ".env.local" | ".env.production" | ".env.development" | ".env.test" => "",
        ".eslintrc" | ".eslintrc.js" | ".eslintrc.json" | ".eslintrc.yaml" | ".eslintrc.yml" => "󰱺",
        ".prettierrc" | ".prettierrc.js" | ".prettierrc.json" | ".prettierrc.yaml" | ".prettierrc.yml" => "",
        ".rustfmt.toml" => "",
        // CI/CD
        ".github" => "",
        ".gitlab" => "",
        ".circleci" => "󰏓",
        ".travis.yml" => "󰯅",
        // Editors
        ".vscode" => "󰨞",
        ".idea" => "",
        // Containers
        ".devcontainer" => "󰡨",
        // Task runners
        "justfile" | "Justfile" => "",
        "Taskfile.yml" | "Taskfile.yaml" => "󰎙",
        "mise.toml" | ".mise.toml" | ".tool-versions" => "󰏔",
        "flake.nix" | "flake.lock" => "󱄅",
        // Deno
        "deno.json" | "deno.jsonc" | "import_map.json" => "󰋯",
        // Bun
        "bun.lockb" | "bun.lock" => "󰎙",
        // Python
        "pyproject.toml" | "poetry.lock" | "Pipfile" | "Pipfile.lock" | "requirements.txt" | "requirements-dev.txt" => "󰌠",
        // Go
        "go.mod" | "go.sum" | "go.work" => "󰟓",
        // PHP
        "composer.json" | "composer.lock" => "󰌟",
        // Ruby
        "Gemfile" | "Gemfile.lock" => "󰴭",
        // Java/Kotlin
        "pom.xml" | "build.gradle" | "build.gradle.kts" | "settings.gradle" | "settings.gradle.kts" => "󰬷",
        "build.xml" | "ivy.xml" => "󰬷",
        // Bazel
        "bazel" | "BUILD" | "WORKSPACE" | "BUILD.bazel" => "󰆧",
        // Config files
        ".gitignore" | ".gitmodules" | ".gitattributes" | ".gitignore_global" | ".gitconfig" => "󰊢",
        ".dockerignore" => "󰡨",
        ".editorconfig" => "",
        ".env" | ".env.local" | ".env.production" | ".env.development" | ".env.test" => "",
        ".eslintrc" | ".eslintrc.js" | ".eslintrc.json" | ".eslintrc.yaml" | ".eslintrc.yml" => "󰱺",
        ".prettierrc" | ".prettierrc.js" | ".prettierrc.json" | ".prettierrc.yaml" | ".prettierrc.yml" => "",
        ".rustfmt.toml" => "",
        // CI/CD
        ".github" => "",
        ".gitlab" => "",
        ".circleci" => "󰏓",
        ".travis.yml" => "󰯅",
        // Editors
        ".vscode" => "󰨞",
        ".idea" => "",
        // Containers
        ".devcontainer" => "󰡨",
        // Task runners
        "justfile" | "Justfile" => "",
        "Taskfile.yml" | "Taskfile.yaml" => "󰎙",
        "mise.toml" | ".mise.toml" | ".tool-versions" => "󰏔",
        "flake.nix" | "flake.lock" => "󱄅",
        // Deno
        "deno.json" | "deno.jsonc" | "import_map.json" => "󰋯",
        // Bun
        "bun.lockb" | "bun.lock" => "󰎙",
        // Python
        "pyproject.toml" | "poetry.lock" | "Pipfile" | "Pipfile.lock" | "requirements.txt" | "requirements-dev.txt" => "󰌠",
        // Go
        "go.mod" | "go.sum" | "go.work" => "󰟓",
        // PHP
        "composer.json" | "composer.lock" => "󰌟",
        // Ruby
        "Gemfile" | "Gemfile.lock" => "󰴭",
        // Java/Kotlin
        "pom.xml" | "build.gradle" | "build.gradle.kts" | "settings.gradle" | "settings.gradle.kts" => "󰬷",
        "build.xml" | "ivy.xml" => "󰬷",
        // Bazel
        "bazel" | "BUILD" | "WORKSPACE" | "BUILD.bazel" => "󰆧",
        // Config files
        ".gitignore" | ".gitmodules" | ".gitattributes" | ".gitignore_global" | ".gitconfig" => "󰊢",
        ".dockerignore" => "󰡨",
        ".editorconfig" => "",
        ".env" | ".env.local" | ".env.production" | ".env.development" | ".env.test" => "",
        ".eslintrc" | ".eslintrc.js" | ".eslintrc.json" | ".eslintrc.yaml" | ".eslintrc.yml" => "󰱺",
        ".prettierrc" | ".prettierrc.js" | ".prettierrc.json" | ".prettierrc.yaml" | ".prettierrc.yml" => "",
        ".rustfmt.toml" => "",
        // CI/CD
        ".github" => "",
        ".gitlab" => "",
        ".circleci" => "󰏓",
        ".travis.yml" => "󰯅",
        // Editors
        ".vscode" => "󰨞",
        ".idea" => "",
        // Containers
        ".devcontainer" => "󰡨",
        // Task runners
        "justfile" | "Justfile" => "",
        "Taskfile.yml" | "Taskfile.yaml" => "󰎙",
        "mise.toml" | ".mise.toml" | ".tool-versions" => "󰏔",
        "flake.nix" | "flake.lock" => "󱄅",
        // Deno
        "deno.json" | "deno.jsonc" | "import_map.json" => "󰋯",
        // Bun
        "bun.lockb" | "bun.lock" => "󰎙",
        // Python
        "pyproject.toml" | "poetry.lock" | "Pipfile" | "Pipfile.lock" | "requirements.txt" | "requirements-dev.txt" => "󰌠",
        // Go
        "go.mod" | "go.sum" | "go.work" => "󰟓",
        // PHP
        "composer.json" | "composer.lock" => "󰌟",
        // Ruby
        "Gemfile" | "Gemfile.lock" => "󰴭",
        // Java/Kotlin
        "pom.xml" | "build.gradle" | "build.gradle.kts" | "settings.gradle" | "settings.gradle.kts" => "󰬷",
        "build.xml" | "ivy.xml" => "󰬷",
        // Bazel
        "bazel" | "BUILD" | "WORKSPACE" | "BUILD.bazel" => "󰆧",
        // Docs
        "README.md" | "README.txt" | "README.rst" => "",
        "LICENSE" | "LICENSE.md" | "LICENSE.txt" | "COPYING" => "",
        "CHANGELOG.md" | "CHANGELOG.txt" | "CHANGES.md" => "",
        "CONTRIBUTING.md" | "CONTRIBUTING.txt" => "󰍔",
        "SECURITY.md" | "SECURITY.txt" => "󰌾",
        "CODE_OF_CONDUCT.md" => "󰍔",
        // Build
        "Makefile" | "makefile" | "CMakeLists.txt" | "meson.build" | "build.zig" => "",
        // Docker
        "Dockerfile" | "docker-compose.yml" | "docker-compose.yaml" | "docker-compose.override.yml" => "󰡨",
        // Rust
        "Cargo.toml" | "Cargo.lock" => "",
        // Node.js
        "package.json" | "package-lock.json" | "pnpm-lock.yaml" | "yarn.lock" | "bun.lockb" | "bun.lock" => "󰎙",
        "tsconfig.json" | "jsconfig.json" => "",
        // Bundlers
        "webpack.config.js" | "webpack.config.ts" | "vite.config.ts" | "vite.config.js" | "rollup.config.js" | "rollup.config.ts" | "turbo.json" => "󰜫",
        // Build
        "Makefile" | "makefile" | "CMakeLists.txt" | "meson.build" | "build.zig" => "",
        // Docker
        "Dockerfile" | "docker-compose.yml" | "docker-compose.yaml" | "docker-compose.override.yml" => "󰡨",
        ".devcontainer" => "󰡨",
        // Docs
        "README.md" | "README.txt" | "README.rst" => "",
        "LICENSE" | "LICENSE.md" | "LICENSE.txt" | "COPYING" => "",
        "CHANGELOG.md" | "CHANGELOG.txt" | "CHANGES.md" => "",
        "CONTRIBUTING.md" | "CONTRIBUTING.txt" => "󰍔",
        "SECURITY.md" | "SECURITY.txt" => "󰌾",
        "CODE_OF_CONDUCT.md" => "󰍔",
        // Git hosting
        ".github" => "",
        ".gitlab" => "",
        // Editors
        ".vscode" => "󰨞",
        ".idea" => "",
        // Containers/Tools
        ".devcontainer" => "󰡨",
        "justfile" | "Justfile" => "",
        "Taskfile.yml" | "Taskfile.yaml" => "󰎙",
        "mise.toml" | ".mise.toml" | ".tool-versions" => "󰏔",
        "flake.nix" | "flake.lock" => "󱄅",
        // Deno
        "deno.json" | "deno.jsonc" | "import_map.json" => "󰋯",
        // Bun
        "bun.lockb" | "bun.lock" => "󰎙",
        // Python
        "pyproject.toml" | "poetry.lock" | "Pipfile" | "Pipfile.lock" | "requirements.txt" | "requirements-dev.txt" => "󰌠",
        // Go
        "go.mod" | "go.sum" | "go.work" => "󰟓",
        // PHP
        "composer.json" | "composer.lock" => "󰌟",
        // Ruby
        "Gemfile" | "Gemfile.lock" => "󰴭",
        // Java/Kotlin
        "pom.xml" | "build.gradle" | "build.gradle.kts" | "settings.gradle" | "settings.gradle.kts" => "󰬷",
        "build.xml" | "ivy.xml" => "󰬷",
        // Bazel
        "bazel" | "BUILD" | "WORKSPACE" | "BUILD.bazel" => "󰆧",
        // Nix
        "flake.nix" | "flake.lock" => "󱄅",
        // Deno
        "deno.json" | "deno.jsonc" | "import_map.json" => "󰋯",
        // Bun
        "bun.lockb" | "bun.lock" => "󰎙",
        // Python
        "pyproject.toml" | "poetry.lock" | "Pipfile" | "Pipfile.lock" | "requirements.txt" | "requirements-dev.txt" => "󰌠",
        // Go
        "go.mod" | "go.sum" | "go.work" => "󰟓",
        // PHP
        "composer.json" | "composer.lock" => "󰌟",
        // Ruby
        "Gemfile" | "Gemfile.lock" => "󰴭",
        // Java/Kotlin
        "pom.xml" | "build.gradle" | "build.gradle.kts" | "settings.gradle" | "settings.gradle.kts" => "󰬷",
        "build.xml" | "ivy.xml" => "󰬷",
        // Bazel
        "bazel" | "BUILD" | "WORKSPACE" | "BUILD.bazel" => "󰆧",
        // Nix
        "flake.nix" | "flake.lock" => "󱄅",
        // Deno
        "deno.json" | "deno.jsonc" | "import_map.json" => "󰋯",
        // Bun
        "bun.lockb" | "bun.lock" => "󰎙",
        // Python
        "pyproject.toml" | "poetry.lock" | "Pipfile" | "Pipfile.lock" | "requirements.txt" | "requirements-dev.txt" => "󰌠",
        // Go
        "go.mod" | "go.sum" | "go.work" => "󰟓",
        // PHP
        "composer.json" | "composer.lock" => "󰌟",
        // Ruby
        "Gemfile" | "Gemfile.lock" => "󰴭",
        // Java/Kotlin
        "pom.xml" | "build.gradle" | "build.gradle.kts" | "settings.gradle" | "settings.gradle.kts" => "󰬷",
        "build.xml" | "ivy.xml" => "󰬷",
        // Bazel
        "bazel" | "BUILD" | "WORKSPACE" | "BUILD.bazel" => "󰆧",
        // Config files
        ".gitignore" | ".gitmodules" | ".gitattributes" | ".gitignore_global" | ".gitconfig" => "󰊢",
        ".dockerignore" => "󰡨",
        ".editorconfig" => "",
        ".env" | ".env.local" | ".env.production" | ".env.development" | ".env.test" => "",
        ".eslintrc" | ".eslintrc.js" | ".eslintrc.json" | ".eslintrc.yaml" | ".eslintrc.yml" => "󰱺",
        ".prettierrc" | ".prettierrc.js" | ".prettierrc.json" | ".prettierrc.yaml" | ".prettierrc.yml" => "",
        ".rustfmt.toml" => "",
        // CI/CD
        ".github" => "",
        ".gitlab" => "",
        ".circleci" => "󰏓",
        ".travis.yml" => "󰯅",
        // Editors
        ".vscode" => "󰨞",
        ".idea" => "",
        // Containers
        ".devcontainer" => "󰡨",
        // Task runners
        "justfile" | "Justfile" => "",
        "Taskfile.yml" | "Taskfile.yaml" => "󰎙",
        "mise.toml" | ".mise.toml" | ".tool-versions" => "󰏔",
        // Nix
        "flake.nix" | "flake.lock" => "󱄅",
        // Deno
        "deno.json" | "deno.jsonc" | "import_map.json" => "󰋯",
        // Bun
        "bun.lockb" | "bun.lock" => "󰎙",
        // Python
        "pyproject.toml" | "poetry.lock" | "Pipfile" | "Pipfile.lock" | "requirements.txt" | "requirements-dev.txt" => "󰌠",
        // Go
        "go.mod" | "go.sum" | "go.work" => "󰟓",
        // PHP
        "composer.json" | "composer.lock" => "󰌟",
        // Ruby
        "Gemfile" | "Gemfile.lock" => "󰴭",
        // Java/Kotlin
        "pom.xml" | "build.gradle" | "build.gradle.kts" | "settings.gradle" | "settings.gradle.kts" => "󰬷",
        "build.xml" | "ivy.xml" => "󰬷",
        // Bazel
        "bazel" | "BUILD" | "WORKSPACE" | "BUILD.bazel" => "󰆧",
        // Nix
        "flake.nix" | "flake.lock" => "󱄅",
        // Deno
        "deno.json" | "deno.jsonc" | "import_map.json" => "󰋯",
        // Bun
        "bun.lockb" | "bun.lock" => "󰎙",
        // Python
        "pyproject.toml" | "poetry.lock" | "Pipfile" | "Pipfile.lock" | "requirements.txt" | "requirements-dev.txt" => "󰌠",
        // Go
        "go.mod" | "go.sum" | "go.work" => "󰟓",
        // PHP
        "composer.json" | "composer.lock" => "󰌟",
        // Ruby
        "Gemfile" | "Gemfile.lock" => "󰴭",
        // Java/Kotlin
        "pom.xml" | "build.gradle" | "build.gradle.kts" | "settings.gradle" | "settings.gradle.kts" => "󰬷",
        "build.xml" | "ivy.xml" => "󰬷",
        // Bazel
        "bazel" | "BUILD" | "WORKSPACE" | "BUILD.bazel" => "󰆧",
        // Config files
        ".gitignore" | ".gitmodules" | ".gitattributes" | ".gitignore_global" | ".gitconfig" => "󰊢",
        ".dockerignore" => "󰡨",
        ".editorconfig" => "",
        ".env" | ".env.local" | ".env.production" | ".env.development" | ".env.test" => "",
        ".eslintrc" | ".eslintrc.js" | ".eslintrc.json" | ".eslintrc.yaml" | ".eslintrc.yml" => "󰱺",
        ".prettierrc" | ".prettierrc.js" | ".prettierrc.json" | ".prettierrc.yaml" | ".prettierrc.yml" => "",
        ".rustfmt.toml" => "",
        // CI/CD
        ".github" => "",
        ".gitlab" => "",
        ".circleci" => "󰏓",
        ".travis.yml" => "󰯅",
        // Editors
        ".vscode" => "󰨞",
        ".idea" => "",
        // Containers
        ".devcontainer" => "󰡨",
        // Task runners
        "justfile" | "Justfile" => "",
        "Taskfile.yml" | "Taskfile.yaml" => "󰎙",
        "mise.toml" | ".mise.toml" | ".tool-versions" => "󰏔",
        "flake.nix" | "flake.lock" => "󱄅",
        // Deno
        "deno.json" | "deno.jsonc" | "import_map.json" => "󰋯",
        // Bun
        "bun.lockb" | "bun.lock" => "󰎙",
        // Python
        "pyproject.toml" | "poetry.lock" | "Pipfile" | "Pipfile.lock" | "requirements.txt" | "requirements-dev.txt" => "󰌠",
        // Go
        "go.mod" | "go.sum" | "go.work" => "󰟓",
        // PHP
        "composer.json" | "composer.lock" => "󰌟",
        // Ruby
        "Gemfile" | "Gemfile.lock" => "󰴭",
        // Java/Kotlin
        "pom.xml" | "build.gradle" | "build.gradle.kts" | "settings.gradle" | "settings.gradle.kts" => "󰬷",
        "build.xml" | "ivy.xml" => "󰬷",
        // Bazel
        "bazel" | "BUILD" | "WORKSPACE" | "BUILD.bazel" => "󰆧",
        // Nix
        "flake.nix" | "flake.lock" => "󱄅",
        // Deno
        "deno.json" | "deno.jsonc" | "import_map.json" => "󰋯",
        // Bun
        "bun.lockb" | "bun.lock" => "󰎙",
        // Python
        "pyproject.toml" | "poetry.lock" | "Pipfile" | "Pipfile.lock" | "requirements.txt" | "requirements-dev.txt" => "󰌠",
        // Go
        "go.mod" | "go.sum" | "go.work" => "󰟓",
        // PHP
        "composer.json" | "composer.lock" => "󰌟",
        // Ruby
        "Gemfile" | "Gemfile.lock" => "󰴭",
        // Java/Kotlin
        "pom.xml" | "build.gradle" | "build.gradle.kts" | "settings.gradle" | "settings.gradle.kts" => "󰬷",
        "build.xml" | "ivy.xml" => "󰬷",
        // Bazel
        "bazel" | "BUILD" | "WORKSPACE" | "BUILD.bazel" => "󰆧",
        // Config files
        ".gitignore" | ".gitmodules" | ".gitattributes" | ".gitignore_global" | ".gitconfig" => "󰊢",
        ".dockerignore" => "󰡨",
        ".editorconfig" => "",
        ".env" | ".env.local" | ".env.production" | ".env.development" | ".env.test" => "",
        ".eslintrc" | ".eslintrc.js" | ".eslintrc.json" | ".eslintrc.yaml" | ".eslintrc.yml" => "󰱺",
        ".prettierrc" | ".prettierrc.js" | ".prettierrc.json" | ".prettierrc.yaml" | ".prettierrc.yml" => "",
        ".rustfmt.toml" => "",
        // CI/CD
        ".github" => "",
        ".gitlab" => "",
        ".circleci" => "󰏓",
        ".travis.yml" => "󰯅",
        // Editors
        ".vscode" => "󰨞",
        ".idea" => "",
        // Containers
        ".devcontainer" => "󰡨",
        // Task runners
        "justfile" | "Justfile" => "",
        "Taskfile.yml" | "Taskfile.yaml" => "󰎙",
        "mise.toml" | ".mise.toml" | ".tool-versions" => "󰏔",
        "flake.nix" | "flake.lock" => "󱄅",
        // Deno
        "deno.json" | "deno.jsonc" | "import_map.json" => "󰋯",
        // Bun
        "bun.lockb" | "bun.lock" => "󰎙",
        // Python
        "pyproject.toml" | "poetry.lock" | "Pipfile" | "Pipfile.lock" | "requirements.txt" | "requirements-dev.txt" => "󰌠",
        // Go
        "go.mod" | "go.sum" | "go.work" => "󰟓",
        // PHP
        "composer.json" | "composer.lock" => "󰌟",
        // Ruby
        "Gemfile" | "Gemfile.lock" => "󰴭",
        // Java/Kotlin
        "pom.xml" | "build.gradle" | "build.gradle.kts" | "settings.gradle" | "settings.gradle.kts" => "󰬷",
        "build.xml" | "ivy.xml" => "󰬷",
        // Bazel
        "bazel" | "BUILD" | "WORKSPACE" | "BUILD.bazel" => "󰆧",
        // Config files
        ".gitignore" | ".gitmodules" | ".gitattributes" | ".gitignore_global" | ".gitconfig" => "󰊢",
        ".dockerignore" => "󰡨",
        ".editorconfig" => "",
        ".env" | ".env.local" | ".env.production" | ".env.development" | ".env.test" => "",
        ".eslintrc" | ".eslintrc.js" | ".eslintrc.json" | ".eslintrc.yaml" | ".eslintrc.yml" => "󰱺",
        ".prettierrc" | ".prettierrc.js" | ".prettierrc.json" | ".prettierrc.yaml" | ".prettierrc.yml" => "",
        ".rustfmt.toml" => "",
        // CI/CD
        ".github" => "",
        ".gitlab" => "",
        ".circleci" => "󰏓",
        ".travis.yml" => "󰯅",
        // Editors
        ".vscode" => "󰨞",
        ".idea" => "",
        // Containers
        ".devcontainer" => "󰡨",
        // Task runners
        "justfile" | "Justfile" => "",
        "Taskfile.yml" | "Taskfile.yaml" => "󰎙",
        "mise.toml" | ".mise.toml" | ".tool-versions" => "󰏔",
        "flake.nix" | "flake.lock" => "󱄅",
        // Deno
        "deno.json" | "deno.jsonc" | "import_map.json" => "󰋯",
        // Bun
        "bun.lockb" | "bun.lock" => "󰎙",
        // Python
        "pyproject.toml" | "poetry.lock" | "Pipfile" | "Pipfile.lock" | "requirements.txt" | "requirements-dev.txt" => "󰌠",
        // Go
        "go.mod" | "go.sum" | "go.work" => "󰟓",
        // PHP
        "composer.json" | "composer.lock" => "󰌟",
        // Ruby
        "Gemfile" | "Gemfile.lock" => "󰴭",
        // Java/Kotlin
        "pom.xml" | "build.gradle" | "build.gradle.kts" | "settings.gradle" | "settings.gradle.kts" => "󰬷",
        "build.xml" | "ivy.xml" => "󰬷",
        // Bazel
        "bazel" | "BUILD" | "WORKSPACE" | "BUILD.bazel" => "󰆧",
        // Config files
        ".gitignore" | ".gitmodules" | ".gitattributes" | ".gitignore_global" | ".gitconfig" => "󰊢",
        ".dockerignore" => "󰡨",
        ".editorconfig" => "",
        ".env" | ".env.local" | ".env.production" | ".env.development" | ".env.test" => "",
        ".eslintrc" | ".eslintrc.js" | ".eslintrc.json" | ".eslintrc.yaml" | ".eslintrc.yml" => "󰱺",
        ".prettierrc" | ".prettierrc.js" | ".prettierrc.json" | ".prettierrc.yaml" | ".prettierrc.yml" => "",
        ".rustfmt.toml" => "",
        // CI/CD
        ".github" => "",
        ".gitlab" => "",
        ".circleci" => "󰏓",
        ".travis.yml" => "󰯅",
        // Editors
        ".vscode" => "󰨞",
        ".idea" => "",
        // Containers
        ".devcontainer" => "󰡨",
        // Task runners
        "justfile" | "Justfile" => "",
        "Taskfile.yml" | "Taskfile.yaml" => "󰎙",
        "mise.toml" | ".mise.toml" | ".tool-versions" => "󰏔",
        "flake.nix" | "flake.lock" => "󱄅",
        // Deno
        "deno.json" | "deno.jsonc" | "import_map.json" => "󰋯",
        // Bun
        "bun.lockb" | "bun.lock" => "󰎙",
        // Python
        "pyproject.toml" | "poetry.lock" | "Pipfile" | "Pipfile.lock" | "requirements.txt" | "requirements-dev.txt" => "󰌠",
        // Go
        "go.mod" | "go.sum" | "go.work" => "󰟓",
        // PHP
        "composer.json" | "composer.lock" => "󰌟",
        // Ruby
        "Gemfile" | "Gemfile.lock" => "󰴭",
        // Java/Kotlin
        "pom.xml" | "build.gradle" | "build.gradle.kts" | "settings.gradle" | "settings.gradle.kts" => "󰬷",
        "build.xml" | "ivy.xml" => "󰬷",
        // Bazel
        "bazel" | "BUILD" | "WORKSPACE" | "BUILD.bazel" => "󰆧",
        // Config files
        ".gitignore" | ".gitmodules" | ".gitattributes" | ".gitignore_global" | ".gitconfig" => "󰊢",
        ".dockerignore" => "󰡨",
        ".editorconfig" => "",
        ".env" | ".env.local" | ".env.production" | ".env.development" | ".env.test" => "",
        ".eslintrc" | ".eslintrc.js" | ".eslintrc.json" | ".eslintrc.yaml" | ".eslintrc.yml" => "󰱺",
        ".prettierrc" | ".prettierrc.js" | ".prettierrc.json" | ".prettierrc.yaml" | ".prettierrc.yml" => "",
        ".rustfmt.toml" => "",
        // CI/CD
        ".github" => "",
        ".gitlab" => "",
        ".circleci" => "󰏓",
        ".travis.yml" => "󰯅",
        // Editors
        ".vscode" => "󰨞",
        ".idea" => "",
        // Containers
        ".devcontainer" => "󰡨",
        // Task runners
        "justfile" | "Justfile" => "",
        "Taskfile.yml" | "Taskfile.yaml" => "󰎙",
        "mise.toml" | ".mise.toml" | ".tool-versions" => "󰏔",
        "flake.nix" | "flake.lock" => "󱄅",
        // Deno
        "deno.json" | "deno.jsonc" | "import_map.json" => "󰋯",
        // Bun
        "bun.lockb" | "bun.lock" => "󰎙",
        // Python
        "pyproject.toml" | "poetry.lock" | "Pipfile" | "Pipfile.lock" | "requirements.txt" | "requirements-dev.txt" => "󰌠",
        // Go
        "go.mod" | "go.sum" | "go.work" => "󰟓",
        // PHP
        "composer.json" | "composer.lock" => "󰌟",
        // Ruby
        "Gemfile" | "Gemfile.lock" => "󰴭",
        // Java/Kotlin
        "pom.xml" | "build.gradle" | "build.gradle.kts" | "settings.gradle" | "settings.gradle.kts" => "󰬷",
        "build.xml" | "ivy.xml" => "󰬷",
        // Bazel
        "bazel" | "BUILD" | "WORKSPACE" | "BUILD.bazel" => "󰆧",
        _ => "", // Not a special filename, check extension
    };

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

/// Get color for git status
pub fn git_status_color(status: &crate::fs::GitStatus) -> (u8, u8, u8) {
    match status {
        crate::fs::GitStatus::Modified => (255, 170, 0),
        crate::fs::GitStatus::Added => (0, 200, 0),
        crate::fs::GitStatus::Deleted => (255, 50, 50),
        crate::fs::GitStatus::Untracked => (255, 150, 0),
        crate::fs::GitStatus::Ignored => (128, 128, 128),
        crate::fs::GitStatus::Renamed => (0, 200, 200),
        crate::fs::GitStatus::Copied => (0, 200, 200),
        crate::fs::GitStatus::Conflicted => (255, 0, 255),
        crate::fs::GitStatus::Unmodified => (100, 100, 100),
    }
}

/// Get icon for git status
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