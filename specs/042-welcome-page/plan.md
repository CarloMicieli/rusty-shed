# Implementation Plan: Welcome Wizard First-Run Onboarding

**Branch**: `042-welcome-page` | **Date**: 2026-05-20 | **Spec**: `/specs/042-welcome-page/spec.md`
**Input**: Feature specification from `/specs/042-welcome-page/spec.md`

## Summary

Implement a guarded app shell that checks `has_completed_onboarding` at startup and routes users into a 3-step Welcome Wizard (Basics, Modeling, Archive/Sync) before rendering the main dashboard shell when onboarding is incomplete. Persist selected settings through existing settings commands, provide import/restore actions in Step 3, and ensure users can always skip import and start fresh.

## Technical Context

**Language/Version**: TypeScript (strict) + Svelte 5 (Runes), Rust 2024 for Tauri backend  
**Primary Dependencies**: SvelteKit, shadcn-svelte/bits-ui, Tailwind CSS v4, Tauri 2 plugins (`dialog`, settings/invoke layer), Paraglide i18n  
**Storage**: Existing user settings persistence via backend commands (`initialize_settings`, `get_settings`, `update_settings`), app database for imported collection data  
**Testing**: Vitest + Testing Library for frontend routes/components, integration checks for onboarding completion flow, Rust tests for import and command boundaries  
**Target Platform**: Desktop app via Tauri 2 (Linux/macOS/Windows)  
**Project Type**: Monorepo-style single app (Svelte frontend + Rust backend)  
**Performance Goals**: Initialization screen renders in <100ms from webview-ready; onboarding gate resolves before dashboard render; step transitions target smooth 60fps interaction  
**Constraints**: No hardcoded user strings (Paraglide only), Svelte 5 Runes only, no new dependencies without approval, keyboard-accessible step navigation, disable navigation during import actions, canonical onboarding key is `has_completed_onboarding`  
**Scale/Scope**: One root layout gate, one onboarding flow component family, settings persistence updates, import entry points and completion flag handling

## Constitution Check

_GATE: Must pass before Phase 0 research. Re-check after Phase 1 design._

### Pre-Research Gate Review

- Modular, Library-First Design: PASS. Plan isolates onboarding shell logic and wizard components from existing dashboard shell.
- Deterministic Interfaces & Observability: PASS. Reuse explicit Tauri command boundaries and typed invoke payloads; keep onboarding completion contract explicit.
- Test-First Emphasis: PASS. Plan includes route-guard tests and step/action tests.
- Code Quality: PASS. Type-safe, strict TS, no ad-hoc transport shapes.
- Testing Standards: PASS. Deterministic unit/integration tests with mocked Tauri boundaries.
- User Experience Consistency: PASS. Paraglide keys required for all text; shadcn patterns + keyboard support.
- Performance Requirements: PASS. Guarded shell prevents expensive dashboard mount before onboarding decision.
- Safe Rust Practices: PASS. Any backend changes continue `Result`-based paths; no `unwrap()`.
- Architectural Laws (DB/state/transport/domain): PASS. Existing Tauri IPC and settings workflow retained; no direct frontend persistence bypass.

### Post-Design Gate Review

- All gates remain PASS after Phase 1 artifacts.
- No constitution violations requiring complexity waiver.

## Project Structure

### Documentation (this feature)

```text
specs/042-welcome-page/
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── contracts/
│   └── onboarding.openapi.yaml
└── tasks.md
```

### Source Code (repository root)

```text
src/
├── routes/
│   └── +layout.svelte                     # Guarded shell and onboarding switch
├── lib/
│   ├── features/
│   │   ├── onboarding/
│   │   │   ├── WelcomeWizard.svelte
│   │   │   ├── steps/
│   │   │   │   ├── BasicsStep.svelte
│   │   │   │   ├── ModelingStep.svelte
│   │   │   │   └── ImportStep.svelte
│   │   │   └── onboarding-state.svelte.ts
│   │   └── settings/
│   │       └── SettingsState.svelte.ts    # Existing persistence integration
│   ├── services/
│   │   ├── settings.ts
│   │   └── import/                         # Archive/drive orchestration wrappers
│   └── paraglide/messages.js
├── __tests__/
│   ├── routes/layout.test.ts
│   └── features/onboarding/
│       ├── WelcomeWizard.test.ts
│       └── onboarding-gate.test.ts
messages/
├── en.json
└── it.json

src-tauri/
└── src/
    └── ...                                 # Existing commands reused or extended for import entry points
```

**Structure Decision**: Use existing frontend-root guarded layout and feature-local onboarding module under `src/lib/features/onboarding`, while keeping backend integration through existing Tauri command boundaries and settings model.

## Complexity Tracking

No constitution violations identified.
