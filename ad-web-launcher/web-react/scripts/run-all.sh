#!/usr/bin/env bash
set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/projects.sh"
source "$SCRIPT_DIR/lib.sh"

pnpm_cmd="$1"; shift

if [[ -z "$pnpm_cmd" ]]; then
    echo "Usage: run-all.sh <pnpm-subcommand> [args]" >&2
    exit 1
fi

failed=()
skipped=()

for project in "${PROJECTS[@]}"; do
    echo
    echo "========================================"
    echo "[pnpm $pnpm_cmd] $project"
    echo "========================================"

    if ! ( cd "$project" && pnpm "$pnpm_cmd" "$@" ); then
        failed+=("$project")
    fi
done

print_summary failed skipped