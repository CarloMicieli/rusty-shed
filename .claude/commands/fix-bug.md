---
description: acts as an autonomous bridge between your Rust backend and Web frontend, designed to move bugs from a Markdown backlog to a verified Git commit without manual intervention.
---

### **Command Description: `fix-bug`**

## 🤖 Role & Persona
Operate as a Senior Developer. Read `docs/BUGS.md`, propose a cross-layer fix (Rust + UI), and wait for approval. Upon approval, apply changes, run the defined validation scripts, and only if successful, commit with a `fix: ` prefix and mark the bug as `Resolved` in Markdown. Always verify Tauri 2 capability files if backend commands are modified.

---

## Trigger
Activated when the user requests to "add a bug," "report an issue," or uses the `@agent fix-bug` command.

---

## 📝 Command
**Summary:** An autonomous, multi-stage workflow that executes the end-to-end resolution of a bug. It spans the Rust backend and Web frontend, performs validation, and finalizes the work with a standardized Git commit.

**Execution Logic:**
1.  **Ingestion & Selection:**
    * The agent scans `docs/BUGS.md` to find the highest-priority `Pending` bug. 
    * It reads the "Observed vs. Expected" fields to establish a "Success Criterion."
2.  **Cross-Layer Discovery:**
    * It performs a `@workspace` scan to map the "Bridge." It identifies which Rust `#[tauri::command]` matches the frontend `invoke()` call and checks `src-tauri/capabilities/` for relevant permission files.
3.  **The Mandatory Plan (Dry Run):**
    * Before editing, the agent **must** output a bulleted plan:
        * *Example:* "1. Update `main.rs` signature. 2. Update `api.ts` caller. 3. Add `fs:allow-write` to `default.json`."
    * **Pause:** It waits for a "Proceed" or "LGTM" from the user.
4.  **Surgical Execution:**
    * It modifies the code in a single session to ensure the Rust and JS sides remain in sync (atomic fix).
5.  **Automated Validation:**
    * It executes the **Validation Command** defined in the bug report (e.g., `cargo test` and `npm run lint`).
    * If validation fails, the agent attempts **one** self-correction loop before reporting the error to the user.
6.  **Finalization (The Cleanup):**
    * **Git:** Stages changes and commits using the `fix: [ID] [Description]` convention.
    * **Documentation:** Updates the bug’s status in `docs/BUGS.md` from `Pending` to `Resolved` and appends the commit hash.

**Goal:** To eliminate the manual overhead of "context switching" between Rust and TypeScript, ensuring that every bug fix is technically sound, verified by tests, and properly documented in the project history.
