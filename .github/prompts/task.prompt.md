---
name: 'task'
description: 'Start a new task with full planning and verification'
---

I want you to implement a new feature/fix.
You MUST strictly follow the workflow defined in our project instructions:

1. **READ CONTEXT:** Use MCP tools to analyze `src-tauri` (Rust) or `src` (Svelte) depending on the task.
2. **PLAN FIRST:** Present a plan. Do not write code until I approve the plan.
3. **EXECUTION:** - Use Svelte 5 Runes for UI.
   - Use Safe Rust (no unwrap) for Backend.
   - Avoid unnecessary cloning.
4. **DOCUMENT:** Use JSDoc for TS and `///` for Rust.
5. **VERIFY:** You MUST run `pnpm format` and the relevant `rust:check`/`rust:test` commands using your terminal tool.

What are we building today?
