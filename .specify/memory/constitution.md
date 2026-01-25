<!--
Sync Impact Report

- Version change: 1.0.0 → 1.1.0
- Modified principles: Architecture Rules → Architecture Rules + Architectural Laws (codified ADRs)
- Added sections: Architectural Laws (codifying ADRs: database, state management, API/transport)
- Removed sections: none
- Templates requiring verification: .specify/templates/plan-template.md ⚠ pending, .specify/templates/spec-template.md ⚠ pending, .specify/templates/tasks-template.md ⚠ pending
- Follow-up TODOs:
  - Update templates to include new `Constitution Check` gates for DB/State/API (see .specify/templates/) — ACTION REQUIRED
  - RATIFICATION_DATE (deferred) — TODO(RATIFICATION_DATE)
-->

# Rusty Shed Constitution

## Core Principles

### Modular, Library-First Design

All production functionality SHOULD be implemented as a small, well-scoped module or library that is
self-contained, documented, and independently testable. Public interfaces SHOULD be stable and
reviewed; public API changes SHOULD follow Semantic Versioning.

### Deterministic Interfaces & Observability

External-facing interfaces (IPC via Tauri `invoke`, CLI, file formats) MUST be explicitly
documented and machine-readable where appropriate. Structured logging and runtime traces
are expected to enable debugging and telemetry (JSON or structured logger output).

### Test-First Emphasis

Tests are required for new features and bug fixes. The repository enforces unit and integration
testing practices via `vitest` on the frontend and `cargo test` on the Rust side. Authors SHOULD
follow Red–Green–Refactor practices when feasible.

### Safe Rust Practices (Backend Specific)

Rust code in `src-tauri/` uses safety-first practices: avoid panics in production flows, prefer
`Result<T, E>` error handling, and run `cargo clippy` and `cargo fmt` as part of CI. `unsafe` code
is allowed only when documented and reviewed in a PR.

### Simplicity & Semantic Versioning

Design decisions favor simplicity and clear upgrade paths. Releases follow Semantic Versioning
and breaking governance changes require a MAJOR bump and a documented migration plan.

## Tech Stack

- Frontend: SvelteKit (Svelte v5.48.2) running on Vite (v7.3.1)
- Styling: Tailwind CSS v4.1.18, Skeleton UI v4.x
- Package manager: pnpm (packageManager string indicates pnpm@10.27.0)
- Language: TypeScript (v5.9.3) with `tsconfig.json` using `"strict": true`
- Testing (frontend): Vitest v4.0.18 with `happy-dom` environment; `vitest.config.ts` present
- Translation/messages: Paraglide (`@inlang/paraglide-js` 2.7.1) with project keys under `messages/`
- Backend: Tauri (Rust) crate in `src-tauri/` — Rust edition 2024, `rust-version = 1.93.0`, `tauri` v2.9.x
- Backend tooling: `cargo fmt`, `cargo clippy` (CI runs clippy with `-D warnings`), `cargo test`

Notes: Exact package versions are taken from `package.json` and `src-tauri/Cargo.toml` as of this commit.

## Architecture Rules

- Workspace layout:
  - Frontend app lives in `src/` (SvelteKit). UI components and app code under `src/lib/`.
  - Rust Tauri crate lives in `src-tauri/` and exposes IPC commands consumed by the frontend.
  - Translations/messages live in `messages/` and are compiled via the `prepare` script using Paraglide.

- Communication:
  - Frontend ↔ Rust communication MUST use Tauri's `invoke`/command pattern and well-defined
    payload types. Shared types may be derived/checked via `specta`/specta-typescript where present.

- Packaging & Builds:
  - Frontend builds via `pnpm build` (Vite) and Tauri packaging via `pnpm tauri build`.

## Architectural Laws

The following rules are derived from accepted ADRs and are MANDATORY for all future specs and feature plans. Any new spec or plan that touches persistence, state, or the transport boundary MUST reference these laws and demonstrate compliance in the `Constitution Check` (see .specify/templates/plan-template.md).

- **Database (Persistence) — REQUIRED**: All local persistent storage MUST use SQLite accessed from Rust via `sqlx`.
  - Every new feature that requires local storage MUST include `sqlx` migrations in `/migrations` and embed them in the binary using `sqlx::migrate!().run(&pool).await`.
  - Every database connection MUST enable foreign key enforcement immediately after opening via `PRAGMA foreign_keys = ON;` to preserve referential integrity.
  - Schema changes MUST be performed through `sqlx` migration files; ad-hoc schema changes are forbidden.

- **State Management / Persistence Strategy — REQUIRED**: The persistence of domain aggregates MUST follow the Domain Event Tracking pattern.
  - Aggregates MUST record domain events (e.g., `pending_events: Vec<DomainEvent>`) and the repository MUST drain and persist those events atomically inside a transaction.
  - Repositories MUST map events to precise SQL mutations; repositories are responsible for executing the minimal set of DB operations that reflect the event stream for an Aggregate.
  - The Domain layer MUST NOT depend on SQL types or persistence details; persistence concerns remain in repository/infrastructure layers.

- **API Design & Transport Boundary — REQUIRED**: The frontend ↔ backend transport MUST use Tauri IPC and specta-based type generation.
  - All transport DTOs follow the ADR 8 conventions: `Args` for write payloads, `Input` for use-case inputs, `Query`/`Criteria` for read paths. `Args` MUST derive `Debug, Clone, validator::Validate, specta::Type, serde::Deserialize`.
  - Command handlers MUST validate transport `Args` at the boundary (call `args.validate()`), map to validated `Input` types, and only then invoke use cases.
  - Type generation via `specta` (or equivalent approved tooling) MUST be included in the build pipeline so that TypeScript types are generated from Rust types and type mismatches fail fast during development.
  - Opening local network ports for frontend-backend communication (e.g., sidecar HTTP) is disallowed for regular IPC; any exception MUST be explicitly reviewed and approved by maintainers.

- **Domain Logic Location — REQUIRED**: All business and domain rules that affect data integrity MUST live in Rust (backend). The frontend is strictly for rendering, validation hints, and UX workflows.

These Architectural Laws are binding: any deviation must be documented in a spec, include explicit risk/migration plans, and receive a MAJOR-level governance approval per the Versioning Policy.

## Coding Standards

- Formatting: Prettier is configured and run via `pnpm format` / `prettier --write .`.
- Linting: ESLint is configured; `pnpm lint` and `eslint` plugins for Svelte are present.
- TypeScript: `tsconfig.json` enables `strict` mode; authors should address compiler errors reported by `svelte-check`/TypeScript.
- Rust: `cargo fmt` and `cargo clippy` (project uses `-D warnings` for CI) are required before merging Rust changes.
- Paraglide: All user-facing strings MUST be supplied via the Paraglide message system per existing project guidance; avoid hardcoded UI text.

Explicit enforcement observed in repo (do not invent additional bans):

- Rust CI runs `cargo clippy` with `-D warnings` so warnings are treated as errors for Rust code.
- The TypeScript toolchain uses `strict: true`; this produces compiler errors for many unsafe patterns.

## Testing Requirements

- Frontend tests live under `src/__tests__/` and the `vitest` configuration (`vitest.config.ts`) sets
  `environment: 'happy-dom'`, global test setup at `src/__tests__/setup.ts`, and coverage via the V8 provider.
- Test scripts in `package.json`: `test`, `test:unit`, `test:coverage` — use them to run and collect coverage.
- Rust tests: run `pnpm run rust:test` (runs `cargo test --manifest-path src-tauri/Cargo.toml`).
- CI: Continuous Integration should run formatting, linting, `pnpm check`, `pnpm test`, and Rust checks (`rust:clippy`, `rust:test`).

## Constraints & Security Requirements

- All user-facing strings MUST be provided through the Paraglide messaging system — no
  hardcoded UI strings.
- Follow Tauri security guidance: limit exposed commands, validate inputs server-side, and avoid executing shell commands with user input.
- Secrets MUST not be committed; use OS-provided secure stores where needed.

## Development Workflow

- Commits: follow Conventional Commits (repo uses Commitizen flows via `pnpm commit`).
- Pre-PR checks: run `pnpm format`, `pnpm lint`, `pnpm check`, `pnpm test` and the applicable Rust scripts before opening PRs.

## Governance

Amendments to this constitution follow the documented process:

1. Draft the proposed amendment in a spec or docs PR referencing the constitution.
2. Obtain technical approval from two maintainers and at least one impacted stakeholder.
3. Publish a migration plan for any breaking governance changes.
4. Merge and increment the constitution version according to the Versioning Policy below.

Versioning Policy:

- BUMP MAJOR when removing or redefining a principle or when an amendment is
  backward-incompatible for existing workflows.
- BUMP MINOR when adding a new principle or materially expanding guidance.
- BUMP PATCH for clarifications, typo fixes, or non-semantic wording changes.

Compliance and Review:

- PRs touching code or docs that affect governance MUST reference the relevant constitution principles in the PR description.
- Periodic compliance reviews SHOULD run annually or after major releases.

**Version**: 1.1.0 | **Ratified**: TODO(RATIFICATION_DATE): specify original adoption date | **Last Amended**: 2026-01-25
