---
name: Bug report
about: Create a report to help us fix a bug
title: "[bug] "
labels: ['bug']
assignees: []
---

<!-- Please replace the sections below with the requested information. -->

## Summary
A short description of the problem.

## Steps to reproduce
1. Step one to reproduce
2. Step two
3. ...

## Expected behavior
What you expected to happen.

## Actual behavior
What actually happened.

## Environment
- OS: (e.g. Ubuntu 22.04)
- Node: (e.g. 18.17)
- pnpm: (e.g. 8.x)
- Rust: (e.g. 1.74.0)
- Branch: (e.g. main)

## Logs / Screenshots
Paste relevant logs, stack traces or attach screenshots.

## Reproducible example
If possible provide a small reproduction repository or steps to reproduce in an isolated environment.

## Additional context
Any other context about the problem here.

---

## Contributor checklist (please check before submitting)
- [ ] I searched existing issues and PRs and this is not already reported
- [ ] I can reproduce the issue on the latest `main` branch
- Frontend checks
  - [ ] I ran `pnpm check` locally
  - [ ] I ran `pnpm lint` and fixed linting issues
  - [ ] I ran the relevant frontend tests (if applicable)
- Backend (Rust) checks
  - [ ] I ran `cargo fmt`
  - [ ] I ran `cargo clippy -- -D warnings` and addressed warnings
- Localization
  - [ ] No hardcoded UI strings — UI text uses `* as m` paraglide messages


If you are unsure which checks apply, leave a note and maintainers will advise.
