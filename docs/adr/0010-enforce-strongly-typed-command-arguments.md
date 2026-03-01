# ADR 10: Enforce Strongly Typed Command Arguments

Status: Accepted

Date: 2026-03-01

Deciders: Project Lead

## Context and Problem Statement

Our Tauri commands currently rely on primitive types (e.g., `String`, `i32`) for arguments like `language`, `theme`, or `user_role`. This "primitive obsession" leads to:

1. **Silent Failures:** Typos in strings on the frontend result in runtime errors that are hard to debug.
2. **Type Drift:** Manual synchronization between Rust Enums and TypeScript Types/Interfaces is error-prone.
3. **Refactoring Friction:** Renaming a variant in the backend doesn't automatically flag errors in the frontend.

We need a way to ensure the frontend can only send values that the backend is physically capable of deserializing.

## Decision Drivers

- **Developer Experience:** Autocomplete for command arguments in the IDE.
- **Safety:** Compile-time or Build-time validation of the bridge.
- **Maintainability:** Single source of truth (Rust).

## Considered Options

1. **Manual String Validation:** Keep `String` and use `match` statements in Rust to return errors.
2. **Manual Type Sync:** Define Enums in Rust and `type` aliases in TypeScript by hand.
3. **Automated Bridge (Specta):** Use Rust macros to generate TypeScript bindings automatically.

## Decision Outcome

Chosen option: **Option 3: Automated Bridge (Specta)**, because it leverages our existing Specta setup to eliminate "magic strings" and provides a type-safe `invoke` wrapper.

### Consequences

- **Good:** The compiler now acts as the documentation. If a `Language` enum changes, the TypeScript build will fail until the frontend is updated.
- **Good:** Reduced boilerplate in command logic (no need to manually parse/validate strings).
- **Bad:** Requires a code-generation step (either via `main.rs` or a test runner) to update `bindings.ts`.
- **Neutral:** Developers must remember to add `#[specta::specta]` and `#[derive(Type)]` to new commands/structs.

---

## Pros and Cons of the Options

### Option 1: Manual String Validation

- **Pros:** Zero dependencies; works with vanilla Tauri.
- **Cons:** High risk of runtime crashes; no IDE support for valid string values.

### Option 2: Manual Type Sync

- **Pros:** No additional macros or build steps.
- **Cons:** High "Drift Risk." Eventually, the TS type and Rust Enum will diverge, leading to production bugs.

### Option 3: Automated Bridge (Specta)

- **Pros:** Total type safety; the frontend `commands.setLanguage(...)` call is strictly typed.
- **Cons:** Slight increase in binary size/compile time due to macro usage.

---

## More Information

All new commands MUST:

1. Use `#[specta::specta]` macro.
2. Use specific Enums/Structs derived with `specta::Type` instead of `String` or `HashMap`.
3. Be registered in the `tauri_specta::ts::builder()`.
