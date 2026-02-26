# Implementation Plan: Global Search

**Branch**: `030-global-search` | **Date**: 2026-02-26 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/030-global-search/spec.md`

---

## Summary

Implement a global search experience that lets collectors locate any item across their collection and wishlist by typing a partial term in the header search bar and pressing Enter. The backend extends the existing FTS5 search index (`railway_model_search_idx`) with rolling stock fields and manufacturer name, then adds a new `global_search` Tauri command that joins FTS5 results with `collection_items` and `wishlist_items` to attach source context. The frontend gains a new `/search` page that renders the unified result list, and the existing `SearchBar` component is updated to navigate there on Enter.

---

## Technical Context

**Language/Version**: Rust 1.93.0 (edition 2024) · TypeScript 5.9.3 · SvelteKit (Svelte 5)
**Primary Dependencies**: Tauri 2.9.x, sqlx (async + compile-time checked), specta/tauri-specta (TS bindings), Paraglide 2.7.1 (i18n), shadcn-svelte, Tailwind CSS 4
**Storage**: SQLite via sqlx, FTS5 virtual table (already used in migration 0013)
**Testing**: cargo test + sqlx::test (Rust) · Vitest 4 + happy-dom (frontend)
**Target Platform**: Desktop (Linux, macOS, Windows) via Tauri
**Project Type**: Tauri desktop app — Rust backend in `src-tauri/`, SvelteKit frontend in `src/`
**Performance Goals**: Search results within 1 second of debounce completing across 1,000+ items; FTS5 makes sub-100 ms feasible at 5,000 items
**Constraints**: All DB access from Rust only; no direct JS → SQLite; must not block UI thread; Clippy `-D warnings` must pass; all user-facing strings via Paraglide
**Scale/Scope**: Single-user SQLite; realistic collection sizes 100–5,000 items; max 50 results returned

---

## Constitution Check

_GATE: Must pass before Phase 0 research. Re-checked after Phase 1 design._

| Principle                                    | Status  | Notes                                                                                                                                                          |
| -------------------------------------------- | ------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **Modular, Library-First**                   | ✅ Pass | New `search` domain follows the existing 4-layer hexagonal pattern; reuses existing `catalog` FTS5 infrastructure                                              |
| **Deterministic Interfaces / Observability** | ✅ Pass | New Tauri command defined with `#[specta::specta]`; TS types auto-generated; all IPC payloads typed                                                            |
| **Test-First Emphasis**                      | ✅ Pass | Unit tests for use-case logic; `#[sqlx::test]` integration tests for repository search query; Vitest component tests for search page                           |
| **Code Quality**                             | ✅ Pass | Clippy + fmt required; garde validation at transport boundary; no `unwrap()` in production paths                                                               |
| **Testing Standards**                        | ✅ Pass | Business logic isolated in use-case; repository tests use fixture SQL; frontend tests mock Tauri commands                                                      |
| **UX Consistency**                           | ✅ Pass | Paraglide for all strings; shadcn-svelte components; loading state follows existing patterns                                                                   |
| **Performance Requirements**                 | ✅ Pass | FTS5 is sub-100 ms at scale; debounce prevents excessive calls; search runs off UI thread via Tauri                                                            |
| **Safe Rust Practices**                      | ✅ Pass | Result-based error handling throughout; no panics; sqlx compile-time SQL validation                                                                            |
| **Database (Persistence)**                   | ✅ Pass | New migration 0014 extends FTS5 index via sqlx migration; no ad-hoc schema changes                                                                             |
| **State Management / Domain Events**         | ✅ Pass | Search is a read-only query; no aggregate mutation; no event-sourcing required for search index — index updated via DB triggers (existing established pattern) |
| **API Design / Transport Boundary**          | ✅ Pass | New `GlobalSearchArgs` follows `Args` convention; validated with garde; response DTO follows `View` naming; specta type generation included                    |
| **Domain Logic Location**                    | ✅ Pass | FTS5 query and context-join SQL live in Rust infrastructure; frontend only renders and routes                                                                  |

---

## Project Structure

### Documentation (this feature)

```text
specs/030-global-search/
├── plan.md              ← this file
├── research.md          ← Phase 0 output
├── data-model.md        ← Phase 1 output
├── contracts/
│   └── global_search.md ← Phase 1 output
├── quickstart.md        ← Phase 1 output
└── tasks.md             ← Phase 2 output (/speckit.tasks — not created here)
```

### Source Code

```text
src-tauri/
├── migrations/
│   └── 0014_extend_railway_model_search_idx.sql   ← NEW: extend FTS5 + rolling stock triggers
└── src/
    ├── search/                                     ← NEW domain
    │   ├── domain/
    │   │   ├── mod.rs
    │   │   ├── global_search_result.rs             ← domain value object
    │   │   └── repository.rs                       ← GlobalSearchRepository trait
    │   ├── application/
    │   │   ├── mod.rs
    │   │   └── global_search.rs                    ← use case (GlobalSearch)
    │   ├── infrastructure/
    │   │   ├── mod.rs
    │   │   └── sqlite_global_search_repository.rs  ← SQL impl
    │   ├── interface/
    │   │   ├── mod.rs
    │   │   ├── command_handlers.rs                 ← #[tauri::command] global_search
    │   │   └── command_args.rs                     ← GlobalSearchArgs, GlobalSearchResultView
    │   └── mod.rs
    ├── core/
    │   └── infrastructure/
    │       └── unit_of_work.rs                     ← add GlobalSearchUowExt
    ├── state.rs                                    ← register search repo in AppState
    └── lib.rs                                      ← register global_search command + specta

src/
├── routes/
│   └── search/
│       ├── +page.ts                                ← NEW: load fn reads ?q= param
│       └── +page.svelte                            ← NEW: search results page
└── lib/
    ├── features/
    │   └── search/
    │       ├── SearchService.svelte.ts             ← NEW: state + Tauri calls
    │       ├── components/
    │       │   ├── SearchResultCard.svelte         ← NEW: single result row
    │       │   └── SearchEmptyState.svelte         ← NEW: no-results UI
    │       └── index.ts                            ← barrel export
    ├── components/
    │   └── SearchBar.svelte                        ← MODIFY: Enter → /search?q=
    └── bindings.ts                                 ← AUTO-GENERATED: regenerate after command added

messages/
├── en.json                                         ← MODIFY: add search result keys
└── it.json                                         ← MODIFY: add Italian equivalents
```

**Structure Decision**: Tauri desktop app — Rust hexagonal domain in `src-tauri/src/`, SvelteKit feature modules in `src/lib/features/`. New `search` domain mirrors the structure of `catalog`, `collecting`, and `wishlist` domains exactly.

---

## Complexity Tracking

No constitution violations — no justification table required.
