---
applyTo: "src-tauri/**/*.rs"
---

# Rust & Tauri Standards

## Scope
Apply these rules to all files within the `src-tauri/` directory.

## 1. Project Context & Pathing
- **WORKSPACE ROOT:** The Rust workspace and `Cargo.toml` are located in the `src-tauri/` directory.
- **EXECUTION:** All `cargo` commands MUST be run from within `src-tauri/` or via `pnpm` scripts from the root.
- **MCP TOOLS:** You have access to MCP tools. Use them to read files, search the codebase, and execute terminal commands efficiently.

## 2. Mandatory Workflow (The Execution Loop)
Before a task is considered "Done", you MUST execute this sequence:
1. **PLAN:** Detail the logic in chat. Mention if you'll use `sqlx` or specific crates.
2. **EXECUTE:** Implement logic in `src-tauri/src`.
3. **DOCUMENT:** Add `///` docstrings to public APIs.
4. **FORMAT:** Run `pnpm rust:format` (or `cd src-tauri && cargo fmt`).
5. **VERIFY:** You MUST run and pass:
   - `pnpm rust:check`
   - `pnpm rust:clippy` (Treat warnings as errors)
   - `pnpm rust:test` (Ensure all inline tests pass)

## 3. Troubleshooting
If a command fails, use your MCP tools to read the compiler error, fix the code, and re-run the Verification step. Do not skip verification.

## 4. Safety & Error Handling
- **NO UNWRAP/EXPECT:** Never use `.unwrap()` or `.expect()` in production code. 
- **ERROR HANDLING:** Use the `?` operator and return `Result<T, E>`.
- **TEST EXCEPTION:** You may use `.expect("clear message")` inside test modules ONLY.

## 5. Memory & Performance
- **MINIMIZE CLONING:** Avoid `.clone()` and `.to_owned()` unless absolutely necessary for ownership requirements. Favor references `&T` where possible.
- **BORROW CHECKER:** Solve ownership issues through structural changes before resorting to cloning.

## 6. Testing Standards
- **LOCATION:** Write tests inline within a `mod tests { ... }` block at the bottom of the file using `#[cfg(test)]`.
- **NAMING:** Test functions MUST follow the pattern: `it_should_{scenario}` (e.g., `fn it_should_return_error_when_id_is_missing()`).
- **ASSERTIONS:** Always use `pretty_assertions::assert_eq!` for better diff output.
- **DATABASE:** Use `#[sqlx::test]` instead of `#[test]` when a database connection is required.

