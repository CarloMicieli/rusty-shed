# Rusty Shed — Model Railway Collection Manager

[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
![GitHub last commit](https://img.shields.io/github/last-commit/CarloMicieli/rusty-shed)
[![CI](https://github.com/CarloMicieli/rusty-shed/actions/workflows/ci.yml/badge.svg)](https://github.com/CarloMicieli/rusty-shed/actions/workflows/ci.yml)

Rusty Shed helps model railway enthusiasts manage their collections, rolling stock, layouts, and metadata with a native desktop UI powered by Tauri (Rust) and a SvelteKit frontend.

## Quick Overview

- Frontend: SvelteKit (Vite)
- Backend: Tauri (Rust) — IPC via `invoke` between frontend and Rust

## Recommended IDE Setup

[VS Code](https://code.visualstudio.com/) + [Svelte](https://marketplace.visualstudio.com/items?itemName=svelte.svelte-vscode) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer).

**`.vscode/mcp.json`**
```json
{
  "servers": {
    "github-mcp": {
      "type": "http",
      "url": "https://api.githubcopilot.com/mcp"
    },
    "rust-mcp-server": {
      "type": "stdio",
      "command": "${HOME}/.cargo/bin/rust-mcp-server",
      "args": [
        "--log-file",
        "log/folder/rust-mcp-server.log"
      ]
    },
    "svelte": {
      "type": "stdio",
      "command": "npx",
      "args": [
        "-y",
        "@sveltejs/mcp"
      ]
    }
  },
  "inputs": []
}
```

## Running on Ubuntu (prerequisites)

You need Node (pnpm recommended), the Rust toolchain (`rustup`, `cargo`) and the Tauri CLI if you prefer to use it. On Ubuntu, this project required a few additional system packages to compile GTK / webview dependencies — install these before building:

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

1. Install JS and Rust deps:

```bash
pnpm install
rustup toolchain install stable
```

2. Start the frontend dev server:

```bash
pnpm dev
```

3. In a separate terminal run Tauri (launches the desktop app using the Vite dev server):

```bash
pnpm tauri dev
```

## Build (production)

```bash
pnpm build
pnpm tauri build
```

## Committing

This repository follows Conventional Commits. Use the provided Commitizen config to compose messages that follow the project's commit rules:

```bash
pnpm install
pnpm commit
```

This will launch the interactive Commitizen prompt which enforces the allowed commit prefixes (eg. `feat`, `fix`, `docs`, `chore`, etc.).

## Rust Commands

You can run common Cargo commands for the Tauri/Rust crate located in `src-tauri` using the `pnpm` scripts added to `package.json`.

Examples:

```bash
pnpm run rust:fmt     # runs `cargo fmt --manifest-path src-tauri/Cargo.toml`
pnpm run rust:build   # runs `cargo build --manifest-path src-tauri/Cargo.toml`
pnpm run rust:run     # runs `cargo run --manifest-path src-tauri/Cargo.toml`
pnpm run rust:test    # runs `cargo test --manifest-path src-tauri/Cargo.toml`
pnpm run rust:clean   # runs `cargo clean --manifest-path src-tauri/Cargo.toml`
pnpm run rust:lint    # runs `cargo clippy --manifest-path src-tauri/Cargo.toml`
```

Pass extra Cargo flags after `--`, for example:

```bash
pnpm run rust:build -- --release
pnpm run rust:run -- --bin <binary-name>
```

These commands let you invoke Cargo for the `src-tauri` crate without changing directories.
