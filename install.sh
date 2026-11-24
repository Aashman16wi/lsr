#!/usr/bin/env bash
# lsr installer - downloads the latest GitHub release.
#
# lsr currently supports Linux and macOS. If no release exists yet, build from
# source instead:
#
#   git clone https://github.com/Aashman16wi/lsr
#   cd lsr
#   cargo build --release
#   mkdir -p ~/.local/bin
#   install -m755 target/release/lsr ~/.local/bin/lsr

set -euo pipefail

REPO="Aashman16wi/lsr"
BINARY="lsr"

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

log() { echo -e "${GREEN}[INFO]${NC} $*"; }
warn() { echo -e "${YELLOW}[WARN]${NC} $*"; }
error() { echo -e "${RED}[ERROR]${NC} $*" >&2; exit 1; }

detect_libc() {
    if command -v ldd >/dev/null 2>&1 && ldd --version 2>&1 | grep -qi musl; then
        echo "musl"
    elif compgen -G "/lib/ld-musl-*" >/dev/null 2>&1; then
        echo "musl"
    else
        echo "gnu"
    fi
}

detect_platform() {
    local os arch libc
    os=$(uname -s | tr '[:upper:]' '[:lower:]')
    arch=$(uname -m)

    case "$os" in
        linux)
            libc=$(detect_libc)
            case "$arch" in
                x86_64)
                    if [[ "$libc" == "musl" ]]; then
                        echo "x86_64-unknown-linux-musl"
                    else
                        echo "x86_64-unknown-linux-gnu"
                    fi
                    ;;
                aarch64|arm64)
                    if [[ "$libc" == "musl" ]]; then
                        error "No prebuilt aarch64 musl binary. Build from source (see the README)."
                    fi
                    echo "aarch64-unknown-linux-gnu"
                    ;;
                *) error "Unsupported Linux architecture: $arch" ;;
            esac
            ;;
        darwin)
            case "$arch" in
                x86_64) echo "x86_64-apple-darwin" ;;
                arm64) echo "aarch64-apple-darwin" ;;
                *) error "Unsupported macOS architecture: $arch" ;;
            esac
            ;;
        *)
            error "Unsupported OS: $os (lsr supports Linux and macOS)"
            ;;
    esac
}

get_latest_version() {
    curl -fsSL "https://api.github.com/repos/${REPO}/releases/latest" |
        sed -n 's/.*"tag_name": *"\([^"]*\)".*/\1/p' |
        head -n 1
}

install_binary() {
    local src="$1"
    local name="$2"

    if install -m755 "$src" "/usr/local/bin/$name" 2>/dev/null; then
        return 0
    fi

    mkdir -p "$HOME/.local/bin"
    install -m755 "$src" "$HOME/.local/bin/$name" 2>/dev/null || {
        cp "$src" "$HOME/.local/bin/$name"
        chmod 755 "$HOME/.local/bin/$name"
    }
}

main() {
    local platform version url tmpdir

    platform=$(detect_platform)
    version=$(get_latest_version)

    if [[ -z "$version" ]]; then
        error "No release found. Build from source instead (see the README)."
    fi

    log "Installing lsr ${version} for ${platform}"

    url="https://github.com/${REPO}/releases/download/${version}/lsr-${platform}.tar.gz"
    tmpdir=$(mktemp -d)
    trap 'rm -rf "$tmpdir"' EXIT

    if ! curl -fsSL "$url" -o "${tmpdir}/lsr.tar.gz"; then
        error "Download failed: $url"
    fi

    tar xzf "${tmpdir}/lsr.tar.gz" -C "$tmpdir" || error "Could not extract archive"
    install_binary "${tmpdir}/${BINARY}" "${BINARY}"

    if [[ -x "/usr/local/bin/${BINARY}" ]]; then
        log "Installed successfully: $(/usr/local/bin/${BINARY} --version)"
    elif [[ -x "$HOME/.local/bin/${BINARY}" ]]; then
        log "Installed successfully: $("$HOME/.local/bin/${BINARY}" --version)"
    fi

    if ! command -v lsr >/dev/null 2>&1; then
        warn "lsr was installed but is not in PATH. Add ~/.local/bin to your PATH."
    fi
}

main "$@"
