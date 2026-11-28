#!/usr/bin/env bash

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/projects.sh"

pnpm_cmd="$1"
shift

if [[ -z "$pnpm_cmd" ]]; then
    echo "Usage: run-project.sh <pnpm-subcommand> [script] <project> [args...]" >&2
    exit 1
fi

if [[ "$pnpm_cmd" == "run" ]]; then
    pnpm_script="$1"
    shift

    if [[ -z "$pnpm_script" ]]; then
        echo "Usage: run-project.sh run <script> <project> [args...]" >&2
        exit 1
    fi

    pnpm_cmd=("run" "$pnpm_script")
else
    pnpm_cmd=("$pnpm_cmd")
fi

project_name="$1"
shift

if [[ -z "$project_name" ]]; then
    echo "Usage: run-project.sh <pnpm-subcommand> [script] <project> [args...]" >&2
    exit 1
fi

project=""

for candidate in "${PROJECTS[@]}"; do
    if [[ "$candidate" == "$project_name" ]]; then
        project="$candidate"
        break
    fi

    package_json="$ROOT/$candidate/package.json"

    if [[ -f "$package_json" ]]; then
        package_name="$(node -e '
            const fs = require("fs")
            const pkg = JSON.parse(fs.readFileSync(process.argv[1], "utf8"))
            process.stdout.write(pkg.name ?? "")
        ' "$package_json")"

        if [[ "$package_name" == "$project_name" ]]; then
            project="$candidate"
            break
        fi
    fi
done

if [[ -z "$project" ]]; then
    echo "Unknown project: $project_name" >&2
    exit 1
fi

echo
echo "========================================"
echo "[pnpm ${pnpm_cmd[*]}] $project"
echo "========================================"

cd "$ROOT/$project"
if [[ "${pnpm_cmd[0]}" == "run" ]]; then
    pnpm "${pnpm_cmd[@]}" -- "$@"
else
    pnpm "${pnpm_cmd[@]}" "$@"
fi