# rusty-shed Development Guidelines

Auto-generated from all feature plans. Last updated: 2026-01-30

## Active Technologies
- TypeScript (strict) + Svelte 5 (Runes), Rust 2024 for Tauri backend + SvelteKit, Tailwind CSS v4, shadcn-svelte/bits-ui, Paraglide i18n, Tauri 2 IPC/plugin stack (043-mobile-redesign)
- Existing SQLite persistence via Rust + sqlx; no new persistence model required for this feature (043-mobile-redesign)

- Rust 1.95.0 (edition 2024)
- TypeScript 5.9.3 (strict)
- Svelte 5 (Runes) / SvelteKit 2.x
- Tauri 2.x
- specta 2 RC
- sqlx 0.8.x
- garde
- shadcn-svelte / bits-ui
- Paraglide-JS
- Tailwind CSS 4.x
- Vite 8.x
- lucide-svelte (used by some features)
- SQLite (via Rust/sqlx; migrations in `src-tauri/migrations`)

## Project Structure

```text
src/
tests/
```

## Commands

```bash
# Rust
cargo test
cargo clippy

# Frontend
pnpm svelte-check
pnpm vitest
```

## Code Style

Rust 1.95.0 (backend), TypeScript 5.9.3 (frontend): Follow standard conventions

## Recent Changes
- 043-mobile-redesign: Added TypeScript (strict) + Svelte 5 (Runes), Rust 2024 for Tauri backend + SvelteKit, Tailwind CSS v4, shadcn-svelte/bits-ui, Paraglide i18n, Tauri 2 IPC/plugin stack

- 041-entity-management: Added Rust 1.95.0 (edition 2024), TypeScript 5.9.3 (strict), Svelte 5.55.7 (Runes) + Tauri 2.11.x, specta 2 RC, sqlx 0.8.x, garde, SvelteKit 2.60.x, shadcn-svelte/bits-ui, Paraglide-JS
- 040-quick-add-entities: Added [if applicable, e.g., PostgreSQL, CoreData, files or N/A]

<!-- MANUAL ADDITIONS START -->
<!-- MANUAL ADDITIONS END -->
