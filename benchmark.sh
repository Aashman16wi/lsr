#!/usr/bin/env bash
# Manual benchmark helper for lsr vs the system ls.
#
# This script is intentionally not advertised in the README. It is a local
# tool for measuring changes on your own machine.

set -euo pipefail

GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m'

TARGET_DIR="${1:-.}"

for tool in lsr ls hyperfine; do
    if ! command -v "$tool" >/dev/null 2>&1; then
        echo "Missing required tool: $tool" >&2
        exit 1
    fi
done

run_bench() {
    local name="$1"
    local cmd="$2"

    echo -e "${BLUE}=== ${name} ===${NC}"
    hyperfine --warmup 3 --runs 10 "$cmd"
    echo
}

echo -e "${GREEN}Benchmarking on:${NC} $TARGET_DIR"
echo "=================================="

run_bench "lsr (default)" "lsr $TARGET_DIR"
run_bench "lsr -la" "lsr -la $TARGET_DIR"
run_bench "lsr -la --git" "lsr -la --git $TARGET_DIR"
run_bench "lsr -la --du" "lsr -la --du $TARGET_DIR"
run_bench "lsr -T --depth 2" "lsr -T --depth 2 $TARGET_DIR"
run_bench "lsr --group-by ext" "lsr --group-by ext $TARGET_DIR"

run_bench "ls (default)" "ls $TARGET_DIR"
run_bench "ls -la" "ls -la $TARGET_DIR"
run_bench "ls -laR" "ls -laR $TARGET_DIR"
