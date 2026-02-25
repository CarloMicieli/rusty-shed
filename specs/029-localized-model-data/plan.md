# Implementation Plan: Localized Railway Model Data

**Branch**: `029-localized-model-data` | **Date**: 2026-02-25 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/029-localized-model-data/spec.md`

## Summary

Add per-language storage of `description` and `details` for railway models, supporting English (`en`) and Italian (`it`), with automatic English fallback when Italian is absent. The existing `description`/`details` columns are migrated to a new `railway_model_translations` relational table, and a SQLite FTS5 virtual table provides cross-language full-text search. All other text fields (notes, rich-text) remain unaffected. The Tauri IPC layer is extended with translation-specific commands; the frontend receives resolved `LocalizedField` values and a fallback indicator when English is shown in place of Italian.

## Technical Context

**Language/Version**: Rust 1.93.0 (edition 2024); SvelteKit with Svelte 5.48.2
**Primary Dependencies**: `sqlx` (SQLite + FTS5), `tauri 2.9.x`, `specta`/`tauri-specta`, `serde`, `garde` (backend validation), `thiserror`/`anyhow`; `sveltekit-superforms` + `formsnap` + `Zod` (frontend forms); `Paraglide-JS 2.7.1` (UI strings)
**Storage**: SQLite via `sqlx`; new `railway_model_translations` table + `railway_model_search_idx` FTS5 virtual table; existing `railway_models.description`/`details` columns to be migrated and dropped
**Testing**: `cargo test` + `#[sqlx::test(migrations = "./migrations")]` for Rust; `vitest` + `happy-dom` for frontend
**Target Platform**: Desktop (macOS, Windows, Linux) via Tauri 2
**Performance Goals**: Read queries ≤ 200 ms (constitution SLO); full-text search results ≤ 2 s for up to 10 000 models (spec SC-004)
**Constraints**: Offline-capable (SQLite local); only `en` and `it` language codes in scope; English fallback always available
**Scale/Scope**: Up to 10 000 railway models; two supported languages

## Constitution Check

_GATE: Must pass before Phase 0 research. Re-checked after Phase 1 design._

| Law | Status | Evidence |
| ------------------------------------------- | ------ | ---------------------------------------------------------------------------------------------------------------------------------- |
| **Database / Persistence** — SQLite via `sqlx`, migrations, FK enforcement | ✅ PASS | New migration `0013_add_railway_model_translations.sql` added to `/migrations/`; FK from translations table to `railway_models` enforced; schema changes only via migration files |
| **State Management — Domain Event Tracking** | ✅ PASS | New `RailwayModelEvent::TranslationUpserted` event emitted by aggregate; repository drains and persists events atomically inside a transaction via existing Unit-of-Work pattern |
| **API Design & Transport Boundary — Tauri IPC + specta** | ✅ PASS | New `UpsertRailwayModelTranslationArgs` and `GetRailwayModelTranslationsArgs` derive `Debug, Clone, specta::Type, serde::Deserialize`; validated at boundary via `garde`; TS types auto-generated |
| **Domain Logic Location** | ✅ PASS | COALESCE fallback logic lives entirely in the Rust repository layer; frontend receives resolved `LocalizedField { lang, value }` and never re-implements fallback |

**Post-Phase 1 re-check**: All four laws still pass. No violations detected in the Phase 1 design.

## Project Structure

### Documentation (this feature)

```text
specs/029-localized-model-data/
├── plan.md              # This file
├── research.md          # Phase 0 output
├── data-model.md        # Phase 1 output
├── quickstart.md        # Phase 1 output
├── contracts/           # Phase 1 output
│   ├── get_railway_model_by_id.md
│   ├── get_railway_model_translations.md
│   ├── upsert_railway_model_translation.md
│   ├── search_railway_models.md
│   └── update_railway_model_text.md
└── tasks.md             # Phase 2 output (/speckit.tasks)
```

### Source Code (repository root)

```text
src-tauri/migrations/
└── 0013_add_railway_model_translations.sql          [NEW]

src-tauri/src/catalog/
├── domain/railway_model/
│   ├── localized_field.rs                           [NEW]  LocalizedField value type
│   ├── railway_model_translation.rs                 [NEW]  RailwayModelTranslation read struct
│   ├── railway_model.rs                             [MOD]  description/details → LocalizedField
│   ├── railway_model_event.rs                       [MOD]  + TranslationUpserted variant
│   ├── railway_model_view.rs                        [MOD]  + descriptionLang / detailsLang fields
│   └── repository.rs                                [MOD]  + find_translations, updated find_by_id sig
├── application/
│   ├── get_railway_model_by_id.rs                   [MOD]  + lang parameter
│   ├── get_railway_model_translations.rs            [NEW]  returns all translations for a model
│   ├── upsert_railway_model_translation.rs          [NEW]  create/update one language's translation
│   ├── search_railway_models.rs                     [NEW]  FTS5 full-text search
│   └── update_railway_model_text.rs                 [MOD]  + lang parameter
├── infrastructure/railway_model/
│   └── sqlite_railway_model_repository.rs           [MOD]  COALESCE queries, translation upsert, FTS5 search
└── interface/
    ├── command_handlers.rs                          [MOD]  new + updated commands
    └── command_args.rs                              [MOD]  new Args structs

src/lib/
├── features/catalogue/
│   └── components/
│       ├── TranslationsSection.svelte               [NEW]  EN/IT tab input for create/edit forms
│       └── LocalizedFieldInput.svelte               [NEW]  single-language field with label
├── components/
│   ├── RailwayModelCard.svelte                      [MOD]  show LanguageFallbackBadge when lang mismatches
│   └── LanguageFallbackBadge.svelte                 [NEW]  "(EN)" indicator for fallback content
└── bindings.ts                                      [AUTO] regenerated by specta after Rust changes
```

**Structure Decision**: Desktop app (Tauri). Frontend in `src/`, Rust crate in `src-tauri/`. Feature-scoped components under `src/lib/features/catalogue/components/`; shared display components in `src/lib/components/`.

## Complexity Tracking

> No constitution violations — this section is intentionally empty.
