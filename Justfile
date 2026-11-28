# ==================== SETTINGS ====================

set shell := ["bash", "-c"]

workspace := "adarium.code-workspace"

# ================== DEFAULT ====================

default: help

# ================== GIT ====================

push *args:
    if [ -z "{{args}}" ]; then \
        echo "No commit message provided"; \
        exit 1; \
    else \
        git add --all; \
        git commit -m "{{args}}"; \
        git push; \
    fi

rebase:
    @git fetch origin
    @git rebase origin/main
    @git push --force-with-lease

sync-main:
    @git switch main
    @git merge dev
    @git push

done *args:
    @test "$(git branch --show-current)" = "dev" || \
        (echo "Must be on dev"; exit 1)
    @just push "{{args}}"
    @just rebase
    @just sync-main
    @git switch dev
    
diff:
    @echo "Trees:"
    @git diff --quiet main dev \
        && echo "  ✓ identical" \
        || echo "  ✗ differ"
    @echo "Commits:"
    @test "$(git rev-parse main)" = "$(git rev-parse dev)" \
        && echo "  ✓ identical" \
        || echo "  ✗ differ"

git-untrack glob:
    git ls-files -z "{{glob}}" | xargs -0 -r git rm --cached
    
# ================== DEVELOPMENT ====================
audit:
    pnpm audit

outdated:
    pnpm outdated -r

install:
    pnpm install

update:
    pnpm update -r

# Update selected dependencies in one project
update-latest project *packages:
    pnpm --filter {{project}} update -r --latest {{packages}}

# Update all dependencies in one project
update-latest-all project:
    pnpm --filter {{project}} update -r --latest

# Update everything across the workspace
update-everything:
    pnpm update -r --latest

# test-all *args:
#     pnpm run test -- --config=vitest.all.config.ts --silent=passed-only --reporter=dot {{args}}

update-status:
    pnpm install-scripts ls

approve:
    pnpm approve-builds

dev project:
    ./nx run {{project}}:dev

preview project:
    ./nx run {{project}}:preview

build project:
    ./nx run {{project}}:build

build-all:
    ./nx run-many -t build

# List available nx projects.
projects:
    ./nx show projects
# ==================== QUALITY ====================

type project:
    ./nx run {{project}}:type

type-all:
    ./nx run-many -t type

test project:
    ./nx run {{project}}:test

test-all:
    ./nx run-many -t test

# test *args:
#     if [ -z "{{args}}" ]; then \
#         pnpm run test -- --config=vitest.config.ts --silent ; \
#     else \
#         ./scripts/run-project.sh run test {{args}} --silent ; \
#     fi

# test-all *args:
#     pnpm run test -- --config=vitest.all.config.ts --silent=passed-only --reporter=dot {{args}}

coverage project:
    ./nx run {{project}}:coverage

coverage-all:
    ./nx run-many -t coverage

# coverage *args:
#     if [ -z "{{args}}" ]; then \
#         pnpm run test -- --config=vitest.config.ts --silent=passed-only --coverage; \
#     else \
#         ./scripts/run-project.sh run test {{args}} --silent=passed-only --coverage ; \
#     fi

# coverage-all:
#     just test-all --coverage

# coverage-open:
#     xdg-open coverage/index.html

lint project:
    ./nx run {{project}}:lint

stylelint project:
    ./nx run {{project}}:stylelint

eslint project:
    ./nx run {{project}}:eslint

oxlint project:
    ./nx run {{project}}:oxlint


# lint *args:
#     if [ -z "{{args}}" ]; then \
#         pnpm run lint; \
#     else \
#         ./scripts/lint.sh {{args}}; \
#     fi

# stylelint *args:
#     if [ -z "{{args}}" ]; then \
#         pnpm run stylelint; \
#     else \
#         npx stylelint {{args}}; \
#     fi

# eslint *args:
#     npx eslint {{args}}

# oxlint *args:
#     npx oxlint {{args}}

# style: stylelint

# ox: oxlint

# es: eslint

knip *args:
    pnpm run knip -- {{args}}

# chmod +x scripts/index.sh <- an example for enabling a script

# ==================== EXTENSIONS ====================

install-extension project:
    ./nx run {{project}}:install-extension

install-extensions:
    ./nx run-many -t install-extension

# Builds and installs css-variable-completion (includes css-lsp helper).
install-cvc:
    cd tools/extensions/css-variable-completion && pnpm run package
    code --install-extension tools/extensions/css-variable-completion/css-variable-completion-0.0.1.vsix --force

# Builds and installs project-lint.
install-pl:
    cd tools/extensions/project-lint && pnpm run package
    code --install-extension tools/extensions/project-lint/project-lint-0.0.1.vsix --force

# Builds and installs project-lint.
install-ms:
    cd tools/extensions/match-suggestions && pnpm run package
    code --install-extension tools/extensions/match-suggestions/match-suggestions-0.0.1.vsix --force

# Builds compiler and installs the compiler extension.
install-tokens:
    cd tools/cascade-compiler && pnpm run build
    cd tools/extensions/cascade-vscode && pnpm run package
    code --install-extension tools/extensions/cascade-vscode/cascade-vscode-0.0.1.vsix --force

# Builds and installs slice maps.
install-sm:
    cd tools/extensions/slice-maps && pnpm run package
    code --install-extension tools/extensions/slice-maps/slice-maps-0.0.1.vsix --force

install-sm-windows:
    cd tools/extensions/slice-maps && pnpm run package
    bash scripts/install-vsix.sh tools/extensions/slice-maps/slice-maps-0.0.1.vsix

# Builds installs unfolder.
install-unfolder:
    cd tools/extensions/unfolder && pnpm run package
    code --install-extension tools/extensions/unfolder/unfolder-0.0.1.vsix --force

install-unfolder-windows:
    cd unfolder && pnpm run package
    bash scripts/install-vsix.sh tools/extensions/unfolder/unfolder-0.0.1.vsix

# Builds and installs all tools.
# install-extensions: install-cvc install-pl install-ms install-tokens install-sm install-unfolder

# Runs the compiler in watchMode. File emissions disabled.
compiler:
    cd cascade-compiler && pnpm run cli -- watch ../..

all: build-all install-extensions


# ==================== DELETABLES ====================

# Check whether the current TypeScript version is supported by typescript-eslint
check-ts-v:
    pnpm view typescript-eslint@latest peerDependencies
    pnpm view @typescript-eslint/parser@latest peerDependencies

# Lists the dependencies of project versus tools. Takes dependency as args.
check-deps *args:
    scripts/dependencies.sh {{args}}

    
# ==================== HELP ====================

help:
    @echo ""
    @echo "==================== COMMAND SURFACE ===================="
    @echo ""
    @echo "[JUST] orchestration layer"
    @echo "→ just"
    @just --list
    @echo ""
    @echo "[pnpm] package scripts"
    @echo "→ pnpm run"
    @pnpm pkg get scripts
