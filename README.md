# Rusty Shed — Model Railway Collection Manager

[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
![GitHub last commit](https://img.shields.io/github/last-commit/CarloMicieli/rusty-shed)
[![CI](https://github.com/CarloMicieli/rusty-shed/actions/workflows/ci.yml/badge.svg)](https://github.com/CarloMicieli/rusty-shed/actions/workflows/ci.yml)

![Rusty Shed Screenshot](logo.png)

Rusty Shed helps model railway enthusiasts manage their collections, their wish lists, their railway model maintenance with a native desktop UI powered by Tauri (Rust) and a SvelteKit frontend.

## Quick Overview

- Frontend: SvelteKit (Vite)
- Backend: Tauri (Rust) — IPC via `invoke` between frontend and Rust
- JavaScript/TypeScript runtime and task runner: Deno 2.x (`deno task ...`)

## Running on Ubuntu (prerequisites)

You need Deno 2.x, the Rust toolchain (`rustup`, `cargo`) and Linux webview dependencies. On Ubuntu, install these before building:

```bash
sudo apt update
sudo apt install -y \
	libsoup-3.0-dev \
	libjavascriptcoregtk-4.1-dev \
	libgtk-3-dev \
	libwebkit2gtk-4.1-dev \
	librsvg2-dev
```

## Development

1. Install prerequisites:

```bash
deno --version
rustup toolchain install stable
```

If Deno is not installed yet, follow the official installer: https://docs.deno.com/runtime/getting_started/installation/

2. Sync generated project files and run checks once:

```bash
deno task check
```

3. Start the frontend dev server:

```bash
deno task dev
```

4. In a separate terminal run Tauri (launches the desktop app using the Vite dev server):

```bash
deno task tauri dev
```

Useful quality commands:

```bash
deno task lint
deno task test
deno task format:check
```

## Build (production)

```bash
deno task build
deno task tauri build
```

## Committing

This repository follows Conventional Commits. Use the provided Commitizen config to compose messages that follow the project's commit rules:

```bash
deno install
deno run -A npm:commitizen
```

This will launch the interactive Commitizen prompt which enforces the allowed commit prefixes (eg. `feat`, `fix`, `docs`, `chore`, etc.).

## Rust Commands

You can run common Cargo commands for the Tauri/Rust crate located in `src-tauri` using `deno task` entries from `deno.json`.

Examples:

```bash
deno task rust:fmt     # runs `cargo fmt --manifest-path src-tauri/Cargo.toml --all`
deno task rust:build   # runs `cargo build --manifest-path src-tauri/Cargo.toml`
deno task rust:run     # runs `cargo run --manifest-path src-tauri/Cargo.toml`
deno task rust:test    # runs `cargo test --manifest-path src-tauri/Cargo.toml`
deno task rust:clean   # runs `cargo clean --manifest-path src-tauri/Cargo.toml`
deno task rust:clippy  # runs `cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --all-features -- -D warnings`
```

Pass extra Cargo flags after `--`, for example:

```bash
deno task rust:build -- --release
deno task rust:run -- --bin <binary-name>
```

These commands let you invoke Cargo for the `src-tauri` crate without changing directories.
