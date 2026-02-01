# Implementation Plan: Modern Steampunk Theme System

**Branch**: `011-steampunk-theme` | **Date**: 2026-01-30 | **Spec**: [spec.md](spec.md)
**Input**: Feature specification from `/specs/011-steampunk-theme/spec.md`

## Summary

Transform Rusty Shed from the default Cerberus dark theme to a bespoke "Modern Steampunk" design system featuring dual light/dark themes. Theme preferences persist via Tauri backend settings in SQLite, textures are CSS-only, and new `variant-steampunk-*` classes extend Skeleton UI without overriding defaults.

## Technical Context

**Language/Version**: TypeScript 5.9.3 (frontend), Rust 1.93.0 (backend)  
**Primary Dependencies**: Skeleton UI 4.x, Tailwind CSS 4.1.18, SvelteKit (Svelte 5.48.2), Tauri 2.9.x, sqlx  
**Storage**: SQLite via existing settings table (requires migration for theme column)  
**Testing**: Vitest (frontend), cargo test (backend)  
**Target Platform**: Desktop (Tauri), responsive mobile web  
**Project Type**: Tauri desktop app with SvelteKit frontend  
**Performance Goals**: Theme switch <100ms, no Lighthouse regression >5%  
**Constraints**: CSS-only textures, WCAG 2.1 AA contrast, `prefers-reduced-motion` support  
**Scale/Scope**: ~15 files modified/created, 1 migration, 4 new Svelte components

## Constitution Check

_GATE: Must pass before Phase 0 research. Re-check after Phase 1 design._

| Principle                                    | Status  | Notes                                                                                   |
| -------------------------------------------- | ------- | --------------------------------------------------------------------------------------- |
| **Modular, Library-First Design**            | ✅ PASS | Theme system is self-contained in `src/lib/themes/` and `src/lib/components/steampunk/` |
| **Deterministic Interfaces & Observability** | ✅ PASS | Theme preference exposed via existing typed Tauri commands                              |
| **Test-First Emphasis**                      | ✅ PASS | Tests required for themeStore, theme detection, component rendering                     |
| **Code Quality**                             | ✅ PASS | CSS follows Tailwind conventions, TypeScript strict mode                                |
| **Testing Standards**                        | ✅ PASS | Unit tests for store logic, visual regression tests for components                      |
| **User Experience Consistency**              | ✅ PASS | Theme labels in Paraglide, consistent token usage                                       |
| **Performance Requirements**                 | ✅ PASS | Textures disabled on mobile, animations respect reduced-motion                          |
| **Safe Rust Practices**                      | ✅ PASS | Migration uses sqlx, no unsafe code                                                     |
| **Database (Persistence)**                   | ✅ PASS | Uses existing settings table via sqlx migration                                         |
| **State Management / Persistence Strategy**  | N/A     | Theme is user preference, not domain aggregate                                          |
| **API Design & Transport Boundary**          | ✅ PASS | Extends existing `get_settings`/`update_settings` with new field                        |
| **Domain Logic Location**                    | ✅ PASS | No domain logic—purely presentation layer with preference storage                       |

## Project Structure

### Documentation (this feature)

```text
specs/011-steampunk-theme/
├── plan.md              # This file
├── research.md          # Phase 0: Research findings
├── data-model.md        # Phase 1: Theme data model
├── quickstart.md        # Phase 1: Developer quickstart
├── contracts/           # Phase 1: API contracts
│   └── theme-settings.ts
└── tasks.md             # Phase 2: Implementation tasks (created by /speckit.tasks)
```

### Source Code (repository root)

```text
# Frontend (Svelte/TypeScript)
src/
├── lib/
│   ├── themes/                        # NEW: Theme CSS files
│   │   ├── steampunk-light.css        # Light theme tokens
│   │   ├── steampunk-dark.css         # Dark theme tokens
│   │   └── steampunk-base.css         # Shared variables, textures, variants
│   ├── components/
│   │   └── steampunk/                 # NEW: Themed components
│   │       ├── RivetedCard.svelte
│   │       ├── ToggleValve.svelte
│   │       ├── PressureGauge.svelte
│   │       ├── RailDivider.svelte
│   │       └── index.ts
│   ├── stores/
│   │   └── themeStore.svelte.ts       # NEW: Theme state management
│   └── utils/
│       └── steampunk-transitions.ts   # NEW: Custom transitions
├── routes/
│   ├── +layout.svelte                 # MODIFY: Theme initialization
│   ├── layout.css                     # MODIFY: Import steampunk themes
│   └── my-settings/
│       └── +page.svelte               # MODIFY: Theme selector UI
└── app.html                           # MODIFY: Font loading

# Backend (Rust/Tauri)
src-tauri/
├── migrations/
│   └── 0007_add_theme_setting.sql     # NEW: Add theme column
└── src/
    └── settings.rs                    # MODIFY: Add theme field

# Localization
messages/
├── en.json                            # MODIFY: Theme labels
└── it.json                            # MODIFY: Theme labels
```

**Structure Decision**: Follows existing project conventions. Theme CSS lives in `src/lib/themes/`, themed components in `src/lib/components/steampunk/`, state in `src/lib/stores/`. Backend extends existing settings module.
