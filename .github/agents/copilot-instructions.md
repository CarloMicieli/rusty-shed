# rusty-shed Development Guidelines

Auto-generated from all feature plans. Last updated: 2026-01-30

## Active Technologies
- Rust 1.93.0 (edition 2024), TypeScript 5.9.3 (strict), Svelte 5.55.7 (Runes) + Tauri 2.11.x, specta 2 RC, sqlx 0.8.x, garde, SvelteKit 2.60.x, shadcn-svelte/bits-ui, Paraglide-JS (041-entity-management)
- SQLite (via Rust/sqlx; migrations in `src-tauri/migrations`) (041-entity-management)

- TypeScript 5.9.3 (strict mode), Rust 1.93.0 (edition 2024) + SvelteKit (Svelte 5.48.2), Vite 7.3.1, Tauri 2.9.x, shadcn-svelte, Tailwind CSS 4.1.18 (017-dashboard-redesign)
- SQLite via sqlx (existing tables: `purchase_infos`, `collection_items`, `sellers`) (017-dashboard-redesign)

<<<<<<< HEAD

- TypeScript 5.9.3 (strict mode), Svelte 5.48.2 + SvelteKit (Vite 7.3.1), Tailwind CSS 4.1.18, shadcn-svelte, lucide-svelte, @inlang/paraglide-js 2.7.1 (013-responsive-navigation)
- N/A (frontend-only, no persistence changes) (013-responsive-navigation)

- # Rust 1.93.0 (backend), TypeScript 5.9.3 (frontend) (010-data-import-utility)
- Rust 1.93.0 (backend), TypeScript 5.9.3 (frontend) (008-google-drive-backup)
  > > > > > > > ea3d384 (docs: Implement Google Drive Cloud Backup feature)

## Project Structure

```text
src/
tests/
```

## Commands

cargo test [ONLY COMMANDS FOR ACTIVE TECHNOLOGIES][ONLY COMMANDS FOR ACTIVE TECHNOLOGIES] cargo clippy

## Code Style

Rust 1.93.0 (backend), TypeScript 5.9.3 (frontend): Follow standard conventions

## Recent Changes
- 041-entity-management: Added Rust 1.93.0 (edition 2024), TypeScript 5.9.3 (strict), Svelte 5.55.7 (Runes) + Tauri 2.11.x, specta 2 RC, sqlx 0.8.x, garde, SvelteKit 2.60.x, shadcn-svelte/bits-ui, Paraglide-JS
- 040-quick-add-entities: Added [if applicable, e.g., PostgreSQL, CoreData, files or N/A]

- 017-dashboard-redesign: Added TypeScript 5.9.3 (strict mode), Rust 1.93.0 (edition 2024) + SvelteKit (Svelte 5.48.2), Vite 7.3.1, Tauri 2.9.x, shadcn-svelte, Tailwind CSS 4.1.18


<<<<<<< HEAD


  > > > > > > > ea3d384 (docs: Implement Google Drive Cloud Backup feature)

<!-- MANUAL ADDITIONS START -->
<!-- MANUAL ADDITIONS END -->
