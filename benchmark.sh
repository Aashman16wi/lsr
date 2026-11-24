#!/usr/bin/env bash
# Manual benchmark script for lsr vs eza vs lsd vs ls

set -euo pipefail

GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

run_bench() {
    local cmd="$1"
    local name="$2"
    echo -e "${BLUE}=== $name ===${NC}"
    hyperfine --warmup 3 --runs 10 -i "$cmd"
    echo
}

# Check tools
for tool in lsr eza lsd ls hyperfine; do
    if ! command -v "$tool" >/dev/null 2>&1; then
        echo "Missing: $tool"
        exit 1
    fi
done

TARGET_DIR="${1:-/home/aashmanr}"

echo "Benchmarking on: $TARGET_DIR"
echo "=================================="

run_bench "lsr $TARGET_DIR" "lsr (default)"
run_bench "lsr -la $TARGET_DIR" "lsr -la"
run_bench "lsr -la --du $TARGET_DIR" "lsr -la --du"
run_bench "lsr -T --depth 2 $TARGET_DIR" "lsr -T --depth 2"
run_bench "lsr --group-by ext $TARGET_DIR" "lsr --group-by ext"
run_bench "lsr --grep fn $TARGET_DIR" "lsr --grep fn"

run_bench "eza $TARGET_DIR" "eza (default)"
run_bench "eza -la $TARGET_DIR" "eza -la"
run_bench "eza -laR $TARGET_DIR" "eza -laR"
run_bench "eza -T --depth 2 $TARGET_DIR" "eza -T --depth 2"

run_bench "lsd $TARGET_DIR" "lsd (default)"
run_bench "lsd -la $TARGET_DIR" "lsd -la"
run_bench "lsd -T --depth 2 $TARGET_DIR" "lsd -T --depth 2"

run_bench "ls -la $TARGET_DIR" "ls -la"
run_bench "ls -laR $TARGET_DIR" "ls -laR"