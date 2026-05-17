# Implementation Plan: Centralized Entity Management

**Branch**: `041-entity-management` | **Date**: 2026-05-17 | **Spec**: [spec.md](spec.md)
**Input**: Feature specification from `/specs/041-entity-management/spec.md`

## Summary

Implement a new Settings > Library workspace for centralized CRUD of Manufacturers, Sellers, and Buyers with strong protection semantics. The feature extends the existing quick-add shared form into a mode-based shared entity form (`QUICK` and `FULL`), introduces canonical shared Buyer/Seller party behavior over one table, enforces system-seeded protection and usage-based delete locks, and adds merge capabilities that relink references atomically across buyer and seller contexts.

## Technical Context

**Language/Version**: Rust 1.93.0 (edition 2024), TypeScript 5.9.3 (strict), Svelte 5.55.7 (Runes)  
**Primary Dependencies**: Tauri 2.11.x, specta 2 RC, sqlx 0.8.x, garde, SvelteKit 2.60.x, shadcn-svelte/bits-ui, Paraglide-JS  
**Storage**: SQLite (via Rust/sqlx; migrations in `src-tauri/migrations`)  
**Testing**: `pnpm svelte-check`, Vitest (`pnpm test`), `cargo test`, `cargo clippy -- -D warnings`  
**Target Platform**: Tauri desktop app (Linux/macOS/Windows), responsive mobile viewport behavior in UI components  
**Project Type**: Tauri + Svelte monorepo  
**Performance Goals**: duplicate warnings within 500 ms, UI-critical reads <200 ms common-case, responsive tab/search interactions with 500+ rows per tab  
**Constraints**: no hardcoded user-facing strings (Paraglide only), no TypeScript `any`, no Rust `unwrap()` in production paths, distinct Buyer/Seller command surfaces over shared table, protection enforced server-side  
**Scale/Scope**: entity library management for up to low-thousands of parties/manufacturers; CRUD + merge + lock/protection behavior in Settings scope

## Constitution Check

_GATE: Must pass before Phase 0 research. Re-check after Phase 1 design._

### Pre-Phase 0 Gate Check

| Gate | Status | Notes |
|---|---|---|
| Database + migrations via sqlx | PASS | Changes planned through migrations only, SQLite enforced in Rust backend. |
| Domain logic in Rust backend | PASS | Protection, usage revalidation, merge relinking, and conflict handling remain server-side. |
| Typed IPC boundary (specta) | PASS | New commands and DTOs will be exposed via specta-generated TS bindings. |
| Localization consistency | PASS | Settings Library UI strings must be Paraglide-backed in both `messages/en.json` and `messages/it.json`. |
| Testing standards and quality gates | PASS | Plan includes Vitest, svelte-check, cargo test, cargo clippy, and formatting/lint checks. |

### Post-Phase 1 Design Re-check

| Gate | Status | Notes |
|---|---|---|
| Database + migrations via sqlx | PASS | Data model and contracts preserve shared buyer/seller table and migration-driven schema evolution. |
| Domain logic in Rust backend | PASS | Contracts require backend revalidation (`is_system_seeded=false` and total usage=0) and atomic merge transactions. |
| Typed IPC boundary (specta) | PASS | OpenAPI command contract maps to explicit Buyer/Seller entry points and generated frontend types. |
| Localization consistency | PASS | Quickstart includes i18n updates for all new Settings Library interactions. |
| Testing standards and quality gates | PASS | Quickstart defines unit/integration checks for protection, CRUD, and merge semantics. |

## Project Structure

### Documentation (this feature)

```text
specs/041-entity-management/
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── contracts/
│   └── openapi.yaml
└── tasks.md
```

### Source Code (repository root)

```text
src/
├── routes/settings/+page.svelte
├── lib/features/settings/
│   ├── components/
│   └── SettingsState.svelte.ts
├── lib/features/quick-add/QuickAddEntityForm.svelte
├── lib/components/
│   ├── drawer/
│   └── ui/
└── lib/bindings.ts

src-tauri/
├── src/
│   ├── catalog/interface/manufacturers.rs
│   ├── sellers/interface/command_handlers.rs
│   ├── buyers/                      # planned new bounded context façade over shared seller table
│   ├── app_uow.rs
│   └── lib.rs
└── migrations/

messages/
├── en.json
└── it.json
```

**Structure Decision**: Use existing monorepo layering. Frontend work remains in `src/routes/settings` and `src/lib/features/settings`; backend behavior is implemented through Rust application/domain/interface modules and sqlx migrations in `src-tauri`. No additional top-level project roots are introduced.

## Complexity Tracking

No constitution violations identified; complexity is justified by explicit product requirements (shared-table buyer/seller canonical model, protection enforcement, and atomic merge behavior).
