#!/usr/bin/env bash
# lsr installer - downloads latest release from GitHub

set -euo pipefail

REPO="Aashman16wi/lsr"
BINARY="lsr"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

log() { echo -e "${GREEN}[INFO]${NC} $*"; }
warn() { echo -e "${YELLOW}[WARN]${NC} $*"; }
error() { echo -e "${RED}[ERROR]${NC} $*" >&2; exit 1; }

# Detect OS and architecture
detect_platform() {
    local os arch
    os=$(uname -s | tr '[:upper:]' '[:lower:]')
    arch=$(uname -m)

    case "$os" in
        linux)
            if [[ "$arch" == "x86_64" ]]; then
                echo "x86_64-unknown-linux-gnu"
            elif [[ "$arch" == "aarch64" ]]; then
                echo "aarch64-unknown-linux-gnu"
            else
                error "Unsupported Linux architecture: $arch"
            fi
            ;;
        darwin)
            if [[ "$arch" == "x86_64" ]]; then
                echo "x86_64-apple-darwin"
            elif [[ "$arch" == "arm64" ]]; then
                echo "aarch64-apple-darwin"
            else
                error "Unsupported macOS architecture: $arch"
            fi
            ;;
        mingw*|msys*|cygwin*)
            echo "x86_64-pc-windows-gnu"
            ;;
        *)
            error "Unsupported OS: $os"
            ;;
    esac
}

# Get latest release version
get_latest_version() {
    curl -s "https://api.github.com/repos/${REPO}/releases/latest" |
        grep '"tag_name"' | sed -E 's/.*"([^"]+)".*/\1/'
}

# Download and install
main() {
    local platform version url tmpdir
    platform=$(detect_platform)
    version=$(get_latest_version)

    if [[ -z "$version" ]]; then
        error "Could not fetch latest version"
    fi

    log "Installing lsr $version for $platform"

    if [[ "$platform" == *"windows"* ]]; then
        url="https://github.com/${REPO}/releases/download/${version}/lsr-${platform}.exe"
        tmpdir=$(mktemp -d)
        curl -sL "$url" -o "${tmpdir}/lsr.exe" || error "Download failed"
        install -Dm755 "${tmpdir}/lsr.exe" "/usr/local/bin/lsr.exe" 2>/dev/null ||
            install -Dm755 "${tmpdir}/lsr.exe" "$HOME/.local/bin/lsr.exe"
    else
        url="https://github.com/${REPO}/releases/download/${version}/lsr-${platform}.tar.gz"
        tmpdir=$(mktemp -d)
        curl -sL "$url" | tar xz -C "$tmpdir" || error "Download failed"
        install -Dm755 "${tmpdir}/lsr" "/usr/local/bin/lsr" 2>/dev/null ||
            install -Dm755 "${tmpdir}/lsr" "$HOME/.local/bin/lsr"
    fi

    rm -rf "$tmpdir"

    if command -v lsr >/dev/null 2>&1; then
        log "Installed successfully! $(lsr --version)"
    else
        warn "lsr installed but not in PATH. Add ~/.local/bin to your PATH."
    fi
}

main "$@"