<!-- Please use this template for PRs. Start the title with a Conventional Commit-style summary. -->

## Summary
Short description of what this PR changes.

## Type of change
- [ ] feat
- [ ] fix
- [ ] docs
- [ ] style
- [ ] refactor
- [ ] perf
- [ ] test
- [ ] build
- [ ] ci
- [ ] chore

## Related issues
Closes # (issue number) or relate to other issues.

## Changes
Describe the main changes and any important implementation notes.

## How to test / QA steps
Provide step-by-step instructions to verify the changes locally.

## Screenshots or recordings
If applicable, add screenshots to help reviewers.

## Checklist (required before merging)
- Local tests & checks
  - [ ] I ran `pnpm install` and `pnpm check`
  - [ ] I ran `pnpm lint` and fixed lint issues
  - [ ] I ran relevant frontend tests
- Rust checks (if touched)
  - [ ] I ran `cargo fmt`
  - [ ] I ran `cargo clippy -- -D warnings`
- Localization
  - [ ] UI text uses `* as m` paraglide messages (no hardcoded strings)
- Repository hygiene
  - [ ] I updated relevant docs (if applicable)
  - [ ] I added/updated tests where applicable
- PR metadata
  - [ ] The PR title follows Conventional Commits (`type(scope?): short description`)
  - [ ] I linked related issues and added labels where relevant

## Reviewer notes
Anything in particular reviewers should look at or test.


Conventional commit examples for PR title:
- feat(ui): add model details panel
- fix: correct date parsing in exporter
- docs: update contributing guide

If you have questions about which checks apply, ask in a comment and maintainers will advise.
