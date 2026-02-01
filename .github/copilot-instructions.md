# Global Project Context: Tauri 2 + Svelte 5

## 1. Directory Map

- **Frontend:** `/src` (Svelte 5 Runes, Tailwind 4, Skeleton 4.x)
- **Backend:** `/src-tauri` (Rust, Tauri 2, Workspace Root)

## 2. Model Context & Tools

- **MCP TOOLS:** You MUST use MCP tools to read files and execute terminal commands.
- **Verification:** Before completion, you MUST run the verification scripts defined in the language-specific instructions.

## 3. Communication

- Always use Conventional Commits (e.g., `feat:`, `fix:`).
- Use Paraglide-JS for all user-facing strings; hardcoded text is forbidden.

## 4. Coding Standards

- Follow the specific coding standards defined in the language-specific instruction files.
  - Rust: Refer to `.github/copilot-rust-instructions.md`
  - Svelte/JS: Refer to `.github/copilot-svelte-instructions.md`
- Ensure code is modular, well-documented, and adheres to best practices for Svelte 5 and Rust.
- Write unit tests for all new features and bug fixes.
- Ensure compatibility with Tauri 2 and Svelte 5 frameworks.
- Use Tailwind 4 and Skeleton 4.x for styling; avoid custom CSS unless absolutely necessary.
- Maintain a clean and organized project structure as outlined in the directory map.
