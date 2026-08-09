# Mobile Redesign Success Criteria Measurement Protocol

## Scope

Defines how to measure SC-001, SC-002, and SC-006 for feature 043.

## Sample Design

- Runs per scenario: 10
- Mobile baseline viewport: 375x812
- Desktop parity viewport: 1280x800
- Locales: en and it
- Environments: local debug build and CI test build

## Timing Method

- Use browser test timing (`performance.now`) around route render + startup placeholder transitions.
- For interaction checks, measure from action dispatch to visible stable UI state.
- Record median and p95 for each scenario.

## Pass/Fail Thresholds

- SC-001 (navigation/title readability at 375): pass when 0 clipping/overlap defects across 10 runs for en+it.
- SC-002 (collection mobile interaction reliability): pass when 100% successful filter/remove/add interactions across 10 runs.
- SC-006 (startup placeholder): pass when startup loading surface becomes visible immediately and first stable shell is <1s median.

## Evidence Requirements

- Test command output excerpt.
- Screenshot or annotated note per failed run.
- Final pass/fail decision per criterion.
- Commit hash and date.
