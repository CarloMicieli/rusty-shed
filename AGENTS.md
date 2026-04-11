# Agent Instructions

You are an expert developer working on this Tauri 2.0 project.

## Quality Assurance Policy
Before declaring any task "complete," you MUST verify the project's health. Run the following commands to ensure both frontend and backend compliance:

### 1. Verification Command
Execute the following to run all linting, formatting, and type checks:

`pnpm format:check && pnpm lint && pnpm check && pnpm rust:fmt -- --check && pnpm rust:clippy`

### 2. Testing
Ensure business logic is sound by running the full test suite before finishing:

`pnpm test && pnpm rust:test`

## CI/CD Parity
This project enforces quality via GitHub Actions. Your local state must match the CI requirements:

* **Frontend**: Must pass `pnpm format:check`, `pnpm lint`, and `pnpm check`.
* **Rust**: Must pass `pnpm rust:fmt` (check mode) and `pnpm rust:clippy` (with -D warnings).
* **i18n**: Ensure `pnpm prepare` is run to compile Paraglide translations if you have modified `project.inlang`.

## Quality Gate
Do not confirm a task is finished until the Verification and Testing steps above have passed with exit code 0. Always prefer the provided pnpm script aliases.
