# Mobile Redesign Test Matrix

## Scope

This matrix tracks manual and automated verification for feature 043 across mobile and desktop breakpoints.

## Viewport Baselines

- Mobile baseline: 375x812
- Mobile large: 430x932
- Tablet boundary: 767x1024
- Desktop baseline: 1280x800

## Coverage Matrix

| Area           | Scenario                                  | Breakpoint | Locale | Test Type          | Status  | Notes                                       |
| -------------- | ----------------------------------------- | ---------- | ------ | ------------------ | ------- | ------------------------------------------- |
| Shell          | Safe-area header/footer spacing           | 375x812    | en/it  | automated + manual | planned | Validate top and bottom inset handling      |
| Navigation     | 4 primary tabs + More behavior            | 375x812    | en/it  | automated + manual | planned | Ensure Settings/Debug reachable in <=2 taps |
| Collection     | One-column layout + add action visibility | 375x812    | en/it  | automated + manual | planned | Confirm min card width and no clipping      |
| Editing        | Parent/child sheet unwind                 | 375x812    | en/it  | automated + manual | planned | Hardware/back must close child then parent  |
| Media          | Camera fallback to picker                 | 375x812    | en/it  | automated + manual | planned | Preserve form state on fallback             |
| Desktop parity | Existing layout parity                    | 1280x800   | en/it  | automated          | planned | No behavior regression at >=768px           |

## Execution Log

- 2026-06-28: Initial matrix created for Phase 1 setup.
