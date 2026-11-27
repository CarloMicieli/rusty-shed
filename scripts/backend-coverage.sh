#!/usr/bin/env bash

set -Eeuo pipefail

readonly SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
readonly ROOT_DIR="$(cd -- "$SCRIPT_DIR/.." && pwd)"
readonly REPORT_DIR="$ROOT_DIR/coverage/backend"
readonly HTML_REPORT_DIR="$REPORT_DIR/html"

die() {
    printf '\n\033[1;31m[ERROR]\033[0m %s\n' "$*" >&2
    exit 1
}

if ! command -v rustup >/dev/null 2>&1; then
    die "rustup is required for Rust coverage. Install Rust with rustup: https://rustup.rs/"
fi

if ! command -v cargo >/dev/null 2>&1 || ! cargo llvm-cov --version >/dev/null 2>&1; then
    die "cargo-llvm-cov is required. Install it with: cargo install cargo-llvm-cov --locked"
fi

if ! rustup component list --installed 2>/dev/null | grep -q '^llvm-tools'; then
    die "The Rust llvm-tools-preview component is required. Install it with: rustup component add llvm-tools-preview"
fi

mkdir -p "$REPORT_DIR"
rm -rf -- "$HTML_REPORT_DIR"
rm -f -- "$REPORT_DIR/coverage.json"

cd "$ROOT_DIR/src-tauri"

cargo llvm-cov \
    --workspace \
    --all-features \
    --all-targets \
    --no-fail-fast \
    --no-report

cargo llvm-cov report \
    --html \
    --output-dir "$REPORT_DIR"

cargo llvm-cov report \
    --json \
    --summary-only \
    --output-path "$REPORT_DIR/coverage.json"

node - "$REPORT_DIR/coverage.json" <<'NODE'
const fs = require('node:fs');

const report = JSON.parse(fs.readFileSync(process.argv[2], 'utf8'));
const totals = report.data.reduce(
    (combined, entry) => ({
        functions: combined.functions + (entry.totals?.functions.count ?? 0),
        coveredFunctions:
            combined.coveredFunctions + (entry.totals?.functions.covered ?? 0),
        lines: combined.lines + (entry.totals?.lines.count ?? 0),
        coveredLines: combined.coveredLines + (entry.totals?.lines.covered ?? 0),
        regions: combined.regions + (entry.totals?.regions.count ?? 0),
        coveredRegions: combined.coveredRegions + (entry.totals?.regions.covered ?? 0)
    }),
    {
        functions: 0,
        coveredFunctions: 0,
        lines: 0,
        coveredLines: 0,
        regions: 0,
        coveredRegions: 0
    }
);

const percent = (covered, count) => (count === 0 ? 100 : (covered / count) * 100);

console.log('\nRust coverage summary');
console.log(
    `  Lines     : ${percent(totals.coveredLines, totals.lines).toFixed(2)}% ` +
        `(${totals.coveredLines}/${totals.lines})`
);
console.log(
    `  Functions : ${percent(totals.coveredFunctions, totals.functions).toFixed(2)}% ` +
        `(${totals.coveredFunctions}/${totals.functions})`
);
console.log(
    `  Regions   : ${percent(totals.coveredRegions, totals.regions).toFixed(2)}% ` +
        `(${totals.coveredRegions}/${totals.regions})`
);
NODE

printf '\nRust coverage reports generated in %s\n' "$REPORT_DIR"
