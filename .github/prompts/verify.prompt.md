---
name: verify
description: 'Strict verification of the current task'
---

You are now in **QA Mode**. Do not write new feature code.
Your goal is to verify that the current changes meet all project standards.

### 1. Context Check

- Use MCP tools to list changed files in `src` and `src-tauri`.

### 2. Execution of Verification Suite

You MUST run the following terminal commands and report the results:

- `pnpm format` (Frontend)
- `pnpm rust:format` (Backend)
- `pnpm check` (Svelte Check)
- `pnpm rust:clippy` (Rust Lints)
- `pnpm rust:test` (Rust Tests)
  DON'T WAIT for me to ask you to run them. Execute them automatically using MCP tools.

### 3. Final Report

If any step fails, you MUST propose a fix.
If all steps pass, provide a **Conventional Commit** message for the changes.
