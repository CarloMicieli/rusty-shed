# Contributing

Thanks for your interest in contributing to this project! The following guidelines will help you get started and ensure a smooth collaboration.

## Code of Conduct

Please read and follow the project's Code of Conduct: see [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md).

## How to File Issues

- Search existing issues before opening a new one.
- Use clear titles and provide a reproducible description with steps, expected vs actual behavior, and environment details.
- For feature requests, explain the use case and possible approaches.

## Development Setup

Prerequisites:

- Node.js (use nvm to manage versions)
- pnpm
- Rust and cargo (stable)

Common commands (adjust if your local layout differs):

```bash
pnpm install
pnpm dev          # run frontend dev server (Vite)
# For Tauri development (if configured):
pnpm tauri dev
# Build for production
pnpm build
pnpm tauri build
```

If the repository includes a `src-tauri` directory, run `cargo fmt` and `cargo clippy` as part of your workflow.

## Branching & PRs

- Create a topic branch from `main` named `feat/brief-description` or `fix/brief-description`.
- Keep changes focused and small; one logical change per PR.
- Include tests or screenshots where applicable.

## Commit Messages

This project follows Conventional Commits. Commit messages should follow the format:

```
<type>(<scope>): <subject>

<body>

(BREAKING CHANGE: )<footer>
```

Common types: `feat`, `fix`, `docs`, `style`, `refactor`, `perf`, `test`, `build`, `ci`, `chore`.

## Code Style & Tests

- Run `cargo fmt` for Rust code and `pnpm` formatting tools for frontend code if configured.
- Add or update tests for new features or bug fixes.

## Reviewing & Merging

- PRs should include a description of what changed and why.
- Maintain a respectful tone in reviews; reference the Code of Conduct when necessary.
- Maintainers may squash, rebase, or request changes before merging.

## Thank You

We appreciate your contributions. If you need help getting started, open an issue labeled `good first issue` or ask in the project's communication channels.
