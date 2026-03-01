## Project DNA

* **Framework**: Tauri 2.0 + Svelte 5 (Runes only, no stores).
* **Backend**: Hexagonal/Clean Architecture in `src-tauri/`.
* **Constraint**: No hardcoded strings (use Paraglide-JS). No `unwrap()` in Rust.
* **Type Safety**:
* Use `tauri-specta` bindings; never redefine types.
* **Strict TypeScript**: Avoid `any` at all costs. Use unknown, generics, or specific interfaces. Use `as const` for literals.

## Hard Constraints

* **Package Manager**: Always use `pnpm`. Never use `npm` or `yarn`.
* **Architecture**: Consult the `docs/adr/` directory before making architectural changes; follow established decisions.
* **Testing Integrity**: Never skip, disable, or delete a failing test to pass a check without explicit user permission.
* **Dependencies**: Never update, add, or change a version in `package.json` or `Cargo.toml` without asking first.

## Workflow Protocol

A task is **Complete** only when this sequence passes with **zero** errors/warnings:

1. **Plan**: Describe the logic and architectural impact.
2. **Implement**: Code follows Hexagonal (Backend) or Feature-modular (Frontend) patterns.
3. **Sync Bindings**: If any Rust types or commands changed, **run `pnpm tauri dev` to rebuild `tauri-specta` bindings.
4. **Format & Lint**: Run project formatters and Clippy/ESLint.
5. **Verify**: Run `svelte-check`, Vitest, and Cargo tests.

## Commit Convention

* Use **Conventional Commits** (e.g., `feat:`, `fix:`, `refactor:`).
* Only commit once the full Workflow Protocol has passed.
