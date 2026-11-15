#!/usr/bin/env bash

set -Eeuo pipefail

# ============================================================
# Local CI - mirrors .github/workflows/ci.yml
# Requirements: Node.js 26, pnpm, Rust 1.99.0
# Distro: Debian / Ubuntu
# ============================================================

readonly SCRIPT_NAME="Local CI"
readonly REQUIRED_NODE_MAJOR=26
readonly REQUIRED_RUST_VERSION="1.100.0"
readonly REQUIRED_TAURI_DRIVER_VERSION="2.0.6"

STEP=0

# ------------------------------------------------------------
# Progress reporting
# ------------------------------------------------------------

run_step() {
    local description="$1"
    shift

    STEP=$((STEP + 1))

    printf "\n\033[1;36m[%02d]\033[0m %s\n" "$STEP" "$description"
    printf '%s\n' "------------------------------------------------------------"

    if "$@"; then
        printf "\033[1;32m[PASS]\033[0m %s\n" "$description"
    else
        local status=$?
        printf "\033[1;31m[FAIL]\033[0m %s (exit code: %d)\n" \
            "$description" "$status"
        exit "$status"
    fi
}

die() {
    printf "\n\033[1;31m[ERROR]\033[0m %s\n" "$*" >&2
    exit 1
}

# ------------------------------------------------------------
# Environment checks
# ------------------------------------------------------------

check_environment() {
    local errors=()

    echo "Checking required tools..."

    # Node.js
    if ! command -v node >/dev/null 2>&1; then
        errors+=("Node.js is not installed")
    else
        local node_version
        node_version=$(node --version)
        local node_major
        node_major=$(echo "$node_version" | sed 's/^v//' | cut -d. -f1)

        if [[ "$node_major" != "$REQUIRED_NODE_MAJOR" ]]; then
            errors+=("Node.js $REQUIRED_NODE_MAJOR required, found $node_version")
        else
            echo "  [OK] Node.js $node_version"
        fi
    fi

    # pnpm
    if ! command -v pnpm >/dev/null 2>&1; then
        errors+=("pnpm is not installed")
    else
        echo "  [OK] pnpm $(pnpm --version)"
    fi

    # Rust
    if ! command -v rustc >/dev/null 2>&1; then
        errors+=("rustc is not installed")
    else
        local rust_version
        rust_version=$(rustc --version | awk '{print $2}')

        if [[ "$rust_version" != "$REQUIRED_RUST_VERSION" ]]; then
            errors+=("Rust $REQUIRED_RUST_VERSION required, found $rust_version")
        else
            echo "  [OK] Rust $rust_version"
        fi
    fi

    if ! command -v cargo >/dev/null 2>&1; then
        errors+=("cargo is not installed")
    else
        echo "  [OK] Cargo $(cargo --version | awk '{print $2}')"
    fi

    # Git
    if ! command -v git >/dev/null 2>&1; then
        errors+=("git is not installed")
    else
        echo "  [OK] Git"
    fi

    # Rust components
    if command -v cargo >/dev/null 2>&1; then
        if ! cargo fmt --version >/dev/null 2>&1; then
            errors+=("Rust component rustfmt is missing")
        else
            echo "  [OK] rustfmt"
        fi

        if ! cargo clippy --version >/dev/null 2>&1; then
            errors+=("Rust component clippy is missing")
        else
            echo "  [OK] clippy"
        fi
    fi

    # Required system packages
    echo
    echo "Checking Ubuntu/Debian packages..."

    local packages=(
        libsoup-3.0-dev
        libjavascriptcoregtk-4.1-dev
        libgtk-3-dev
        libwebkit2gtk-4.1-dev
        webkitgtk-webdriver
        xvfb
        librsvg2-dev
    )

    local missing_packages=()

    for package in "${packages[@]}"; do
        if dpkg-query -W -f='${db:Status-Status}' "$package" \
            2>/dev/null | grep -qx "installed"; then
            echo "  [OK] $package"
        else
            missing_packages+=("$package")
        fi
    done

    if ((${#missing_packages[@]} > 0)); then
        errors+=("Missing system packages: ${missing_packages[*]}")
    fi

    # Executables provided by system packages
    for executable in xvfb-run; do
        if ! command -v "$executable" >/dev/null 2>&1; then
            errors+=("Required executable '$executable' is missing")
        fi
    done

    # Project files
    if [[ ! -f package.json ]]; then
        errors+=("package.json not found")
    fi

    if [[ ! -f pnpm-lock.yaml ]]; then
        errors+=("pnpm-lock.yaml not found")
    fi

    if [[ ! -f src-tauri/Cargo.toml ]]; then
        errors+=("src-tauri/Cargo.toml not found")
    fi

    if ((${#errors[@]} > 0)); then
        echo
        echo "Environment validation failed:"

        for error in "${errors[@]}"; do
            printf "  \033[1;31m- %s\033[0m\n" "$error"
        done

        if ((${#missing_packages[@]} > 0)); then
            echo
            echo "Install missing system packages with:"
            echo "  sudo apt-get update"
            echo "  sudo apt-get install ${missing_packages[*]}"
        fi

        exit 1
    fi

    echo
    echo "Environment validation successful."
}

# ------------------------------------------------------------
# Cargo utilities
# ------------------------------------------------------------

ensure_tauri_driver() {
    local installed_version=""

    if command -v tauri-driver >/dev/null 2>&1; then
        installed_version=$(tauri-driver --version 2>/dev/null || true)
    fi

    if [[ "$installed_version" == *"$REQUIRED_TAURI_DRIVER_VERSION"* ]]; then
        echo "tauri-driver $REQUIRED_TAURI_DRIVER_VERSION already installed."
        return 0
    fi

    echo "Installing tauri-driver $REQUIRED_TAURI_DRIVER_VERSION..."

    if [[ -n "$installed_version" ]]; then
        cargo install tauri-driver \
            --version "$REQUIRED_TAURI_DRIVER_VERSION" \
            --locked \
            --force
    else
        cargo install tauri-driver \
            --version "$REQUIRED_TAURI_DRIVER_VERSION" \
            --locked
    fi
}

# ------------------------------------------------------------
# Specta binding validation
# ------------------------------------------------------------

check_bindings() {
    local binding_file="src/lib/bindings.ts"

    if [[ ! -f "$binding_file" ]]; then
        echo "Generated bindings file not found: $binding_file"
        return 1
    fi

    if [[ -n "$(git status --porcelain -- "$binding_file")" ]]; then
        echo
        echo "TypeScript bindings are out of sync!"
        echo
        echo "Please run:"
        echo "  pnpm specta:generate"
        echo
        echo "Then commit the generated changes."
        echo
        git diff -- "$binding_file"
        return 1
    fi

    echo "TypeScript bindings are up to date."
}

# ------------------------------------------------------------
# Main
# ------------------------------------------------------------

main() {
    echo
    echo "============================================================"
    echo "  $SCRIPT_NAME"
    echo "============================================================"

    # Resolve repository root
    local root_dir

    root_dir=$(git rev-parse --show-toplevel 2>/dev/null) \
        || die "Not inside a Git repository."

    cd "$root_dir"

    echo "Repository: $root_dir"

    # Preflight
    check_environment

    # --------------------------------------------------------
    # Frontend
    # --------------------------------------------------------

    echo
    echo "================ FRONTEND ================="

    run_step "Install frontend dependencies" \
        pnpm install --frozen-lockfile

    run_step "Check formatting (Prettier)" \
        pnpm format:check

    run_step "Lint (ESLint)" \
        pnpm lint

    run_step "Type check (TypeScript)" \
        pnpm check

    run_step "Build frontend" \
        pnpm build

    run_step "Run frontend tests" \
        pnpm test

    # --------------------------------------------------------
    # Backend
    # --------------------------------------------------------

    echo
    echo "================ BACKEND =================="

    run_step "Install / verify tauri-driver" \
        ensure_tauri_driver

    run_step "Generate Specta bindings" \
        cargo run --manifest-path src-tauri/Cargo.toml \
            --bin gen_types

    run_step "Verify generated bindings" \
        check_bindings

    run_step "Check Rust formatting (rustfmt)" \
        cargo fmt --manifest-path src-tauri/Cargo.toml \
            --all --check

    run_step "Run Clippy" \
        cargo clippy --manifest-path src-tauri/Cargo.toml \
            --all-targets --all-features --keep-going

    run_step "Run Rust tests" \
        cargo test --manifest-path src-tauri/Cargo.toml \
            --workspace --no-fail-fast

    # --------------------------------------------------------
    # End-to-end
    # --------------------------------------------------------

    echo
    echo "================ E2E TESTS ================="

    run_step "Run native E2E tests (Xvfb)" \
        xvfb-run pnpm test:e2e

    # --------------------------------------------------------
    # Summary
    # --------------------------------------------------------

    echo
    echo "============================================================"
    printf "\033[1;32m  ALL CHECKS PASSED\033[0m\n"
    echo "  Completed steps: $STEP"
    echo "============================================================"
    echo
}

main "$@"
