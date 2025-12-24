# GitHub Copilot Instructions — rusty-shed (Tauri + SvelteKit)

## Project Overview

- **Architecture**: SvelteKit frontend served by Vite, bundled into a `build` output used by a Tauri Rust backend. The Rust side exposes commands via Tauri IPC (`invoke`) and acts as the native host.
- **Frontend framework**: SvelteKit (Vite)
- **Backend framework**: Tauri (Rust). Optional: `axum` can be added if you want an HTTP server in-process.

## Quick Start (prerequisites)

- Install Node (recommended via `nvm`) and `pnpm`.
- Install Rust toolchain and `cargo` (stable) and `tauri` CLI if you use it globally: `cargo install tauri-cli` (optional).

## Dev workflows

- Install dependencies:

```bash
pnpm install
```

- Run frontend development server (Vite):

```bash
pnpm dev
```

- Run Tauri + frontend together (dev):

```bash
pnpm tauri dev
# or if you prefer explicit: pnpm build && cargo tauri dev
```

- Build for production:

```bash
pnpm build
pnpm tauri build
```

Notes: the Vite dev server is configured to use port `1420` and the Tauri `devUrl` is `http://localhost:1420`. Ensure both match in `vite.config.js` and `src-tauri/tauri.conf.json`.

## Where things live

- Frontend entry: `src/routes/+page.svelte` and `src/routes/+layout.ts`
- Tauri config: `src-tauri/tauri.conf.json`
- Rust code: `src-tauri/src/main.rs`, `src-tauri/src/lib.rs`
- Rust manifest: `src-tauri/Cargo.toml`

## IPC (invoke) examples

Frontend (Svelte) example using the official API:

```ts
import { invoke } from '@tauri-apps/api/tauri'

async function greet(name: string) {
  const message = await invoke<string>('greet', { name })
  return message
}
```

Rust side (Tauri) minimal command handler:

```rust
#[tauri::command]
fn greet(name: String) -> String {
  format!("Hello, {}!", name)
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![greet])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
```

Link the names used by `invoke` to the command function names passed to `generate_handler!`.

## Optional: Integrating axum (HTTP server) in Tauri

If you need an HTTP API (e.g., to use existing web code or a separate backend), you can add `axum` to `src-tauri/Cargo.toml` and run it on an internal port. Basic steps:

1. Add dependencies in `src-tauri/Cargo.toml`:

```toml
axum = "0.7"
tokio = { version = "1", features = ["rt-multi-thread", "macros"] }
```

2. Spawn an async server from Tauri startup using `tauri::async_runtime::spawn` (or `tokio::spawn`) and bind to a localhost port, e.g. `127.0.0.1:3000`.

3. If the frontend needs to call that HTTP server in dev, add a Vite proxy to `vite.config.js` or call `http://127.0.0.1:3000` directly from the frontend.

Security note: prefer IPC (`invoke`) for privileged/native actions; if you expose HTTP endpoints, restrict to `localhost` and avoid binding to public interfaces.

## Tooling & recommended checks

- Rust: `cargo fmt`, `cargo clippy`.
- Frontend: `svelte-check`, TypeScript (`pnpm build` covers type checks).
- Recommended CI steps: `pnpm install --frozen-lockfile`, `pnpm build`, `cargo build --release`, `cargo clippy`.

## Formatting & conventions

- Use `rustfmt` config via `rustfmt.toml` when present. Run `cargo fmt` before commits.
- Keep frontend port and `tauri.devUrl` aligned (see `vite.config.js` and `src-tauri/tauri.conf.json`).

## Troubleshooting

- If Tauri can't find the frontend build, verify `tauri.conf.json`'s `build.distDir` matches the SvelteKit output path (`../build` by default in this repo).
- If `invoke` calls fail, confirm the handler name is exported in `tauri::generate_handler!` and that types serialize via `serde` when needed.

## Conventional Commits (commit message guidelines)

This repository uses Conventional Commits. A Commitizen configuration is present in `.cz.toml` and defines the allowed commit prefixes and their intent. Use these prefixes when creating commit messages to keep history readable and enable automated changelogs.

Allowed commit types (from `.cz.toml`):

- `fix`:      Bug fix (correlates with PATCH in SemVer)
- `feat`:     New feature (correlates with MINOR in SemVer)
- `docs`:     Changes to documentation
- `style`:    Changes that do not affect the meaning of the code (whitespace, formatting, etc.)
- `refactor`: Changes that neither fix a bug nor add a feature
- `perf`:     Changes that improve performance
- `test`:     Adding or refactoring tests
- `build`:    Changes that affect the build system or external dependencies
- `ci`:       Changes to CI config files and scripts
- `chore`:    Other changes that don't modify src or test files
- `revert`:   Reverts a previous commit

Commit message format (template from `.cz.toml`):

```
<type>(<scope>): <subject>

<body>

(BREAKING CHANGE: )<footer>
```

Example:

```
feat(ui): add settings panel

Add a settings panel to configure app preferences.

Closes #123
```

If you use Commitizen, run the configured tool (e.g. `pnpm commit` or `cz`) to compose messages that follow this schema.

## Where this file lives

This file is intended to live at the repo root as `copilot-instructions.md` and to guide GitHub Copilot suggestions and completions for contributors working on this Tauri + SvelteKit project.

---
If you'd like, I can now:

- add a small example `axum` server file under `src-tauri/src/` and update `Cargo.toml`, or
- run `svelte-check` and `cargo fmt` locally (you'll need to run these commands on your machine).
