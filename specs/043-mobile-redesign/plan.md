# Implementation Plan: Mobile Redesign

**Branch**: `043-mobile-redesign` | **Date**: 2026-06-28 | **Spec**: `/specs/043-mobile-redesign/spec.md`
**Input**: Feature specification from `/specs/043-mobile-redesign/spec.md`

## Summary

Implement a mobile-only redesign for viewports below 768 px using CSS-first responsive variants and sheet-based interaction patterns, while preserving desktop behavior at 768 px and above. The plan prioritizes zero-regression rollout by shipping foundation primitives first (safe-area utilities, responsive shell rules, sheet registry constraints), then migrating components and routes in milestone slices with strict i18n and touch-target audits.

## Technical Context

**Language/Version**: TypeScript (strict) + Svelte 5 (Runes), Rust 2024 for Tauri backend  
**Primary Dependencies**: SvelteKit, Tailwind CSS v4, shadcn-svelte/bits-ui, Paraglide i18n, Tauri 2 IPC/plugin stack  
**Storage**: Existing SQLite persistence via Rust + sqlx; no new persistence model required for this feature  
**Testing**: Vitest + Testing Library, `pnpm svelte-check`, Rust `cargo test`, `cargo clippy -- -D warnings`  
**Target Platform**: Tauri desktop/mobile webviews (Linux/macOS/Windows + Android/iOS webview runtimes)
**Project Type**: Single repo app (Svelte frontend + Rust backend)  
**Performance Goals**: Maintain responsive sheet interactions targeting 60fps on common devices; avoid startup layout pop-in during shell warm-up; maintain sub-200ms UI-critical command reads per constitution  
**Constraints**: Mobile scope is strictly `< 768px`; no desktop behavior regressions; all strings in Paraglide (en/it); no new dependencies without approval; max sheet depth parent+child only; touch targets 44x44 px except chip-remove 36x36 px  
**Scale/Scope**: App shell + navigation, collection workflow components, detail/edit sheet flow, and supplementary sheets across core routes

## Constitution Check

_GATE: Must pass before Phase 0 research. Re-check after Phase 1 design._

### Pre-Research Gate Review

- Modular, Library-First Design: PASS. Plan isolates mobile behavior into shared state/utilities and feature-local UI modules.
- Deterministic Interfaces & Observability: PASS. Existing Tauri invoke boundaries remain source of truth; behavior contracts documented in `contracts/mobile-redesign.openapi.yaml`.
- Test-First Emphasis: PASS. Plan includes viewport-specific route/component tests and regression checks.
- Code Quality: PASS. Strict TypeScript, Runes-only Svelte, and no ad-hoc transport typing.
- Testing Standards: PASS. Deterministic UI tests with mocked viewport/media capabilities and no external network dependency.
- User Experience Consistency: PASS. Paraglide-only text, design-token-compatible classes, explicit touch-target standards.
- Performance Requirements: PASS. CSS-first responsive branching, GPU-friendly sheet animation guidance, reduced-motion handling.
- Safe Rust Practices: PASS. No new unsafe backend behavior; existing `Result`-based command boundaries retained.
- Architectural Laws (DB/state/transport/domain): PASS. No persistence-law changes; IPC boundaries retained; business logic remains backend-owned.

### Post-Design Gate Review

- PASS: Phase 1 artifacts keep transport contracts explicit and avoid law violations.
- PASS: No constitution waiver required.

## Project Structure

### Documentation (this feature)

```text
specs/043-mobile-redesign/
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── contracts/
│   └── mobile-redesign.openapi.yaml
└── tasks.md
```

### Source Code (repository root)

```text
src/
├── routes/
│   ├── +layout.svelte
│   ├── layout.css
│   ├── collection/
│   ├── settings/
│   └── ...
├── lib/
│   ├── components/
│   │   ├── ui/
│   │   ├── navigation/
│   │   └── ...
│   ├── features/
│   ├── state/
│   ├── viewport.svelte.ts
│   └── services/
├── __tests__/
│   ├── routes/
│   ├── components/
│   └── features/
└── paraglide/

messages/
├── en.json
└── it.json

src-tauri/
└── src/
    ├── media/
    ├── settings/
    └── ...
```

**Structure Decision**: Reuse existing frontend and backend structure; place responsive behavior and sheet-state logic in shared frontend state/util modules while keeping media and settings persistence through existing backend command surfaces.

## Complexity Tracking

No constitution violations identified.
