# 🧬 Project

- **Framework**: Tauri 2.0 + Svelte 5 (**Runes only**, strictly no legacy stores).
- **Runtime & Environment**: Deno 2.x native execution layer.
- **UI Architecture**: `shadcn-svelte` (bits-ui) + Tailwind CSS.
  - **Rule**: All components must use `$props()` and `$state()`. Refactor any `export let` or `$:` from shadcn CLI imports immediately.
- **Backend**: Clean Architecture in `src-tauri/`.
  - **Layers**: Domain (Logic/Traits), Application (Services), Infrastructure (Adapters/Tauri Commands).
- **Localization (i18n)**:
  - **No Hardcoded Strings**: All user-facing text must use **Paraglide-JS**.
  - **Dual-Language**: Every new key **must** be defined in both English (`en`) and Italian (`it`) before a task is considered complete.
- **Error Handling**: No `unwrap()` in Rust. Use `Result` mapping with custom serializable error enums for the frontend.

# 🛑 Hard Constraints

- **Package Manager**: Strictly use **Deno native package management** (`deno install`, `deno add`). Never use `npm`, `pnpm`, or `yarn` directly.
- **Dependency Resolution**: All frontend dependencies must be managed in `deno.json` using native `npm:` or `jsr:` specifiers. Never install unconfirmed packages.
- **Type Safety**:
  - Use `tauri-specta` bindings; **never** redefine types manually on the frontend.
  - **Strict TypeScript**: No `any`. Use `unknown` or specific interfaces. Use `as const` for literals.
- **Architecture**: Consult `docs/adr/` before changes; adhere to established decisions.
- **Testing Integrity**: Never skip, disable, or delete a failing test. Fix the logic, not the check.

# 🔄 Workflow Protocol

A task is **Complete** only when this sequence passes with **zero** errors/warnings:

1. **Plan**: Describe logic and architectural impact. Identify the layer affected.
2. **Implement**:
   - **Backend**: Logic in Domain/Application before Infrastructure.
   - **Frontend**: Modular structure. Ensure shadcn components are Runes-compliant.
   - **i18n**: Add keys to messages/en.json and messages/it.json.
3. **Sync Bindings**: If Rust types or commands change, run `deno task specta:generate` and verify the `.ts` binding updates.
4. **Format & Lint**:
   - **Frontend**: Run `deno fmt` and `deno lint`.
   - **Backend**: **Clippy warnings are errors** (`cargo clippy -- -D warnings`).
5. **Verify**: Run `deno task svelte-check`, `deno task test` (Vitest), and `cargo test`.

# 📝 Commit Convention

- Use **Conventional Commits** (e.g., `feat:`, `fix:`, `refactor:`, `chore:`).
- Only commit once the full **Workflow Protocol** has been satisfied.
