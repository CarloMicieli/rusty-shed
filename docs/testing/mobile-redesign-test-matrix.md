# Mobile Redesign Test Matrix

## Scope

This matrix tracks manual and automated verification for feature 043 across mobile and desktop breakpoints.

## Viewport Baselines

- Mobile baseline: 375x812
- Mobile large: 430x932
- Tablet boundary: 767x1024
- Desktop baseline: 1280x800

## Touch-Target Audit Results

**Date**: 2026-06-29  
**Status**: ✓ PASS

### Findings

- **BottomNavigation items**: 64x64px (h-16, w-full per item) - Exceeds 44px requirement ✓
- **Filter chip remove buttons**: 36x36px (h-9, w-9) - Matches chip-remove exception ✓
- **Primary controls** (buttons, toggles): Minimum 44x44px - All verified ✓
- **Exceptions documented**: Chip-remove at 36x36px as specified ✓

### Components Verified

- BottomNavigation.svelte: Navigation items (64px)
- CollectionDashboard.svelte: Scale/Epoch/Category/Company/Tag chip remove buttons (36px)
- RailwayModelPreviewCard.svelte: Interactive elements (44px+)
- DrawerShell.svelte: Close/dismiss buttons (44px+)

**Exceptions**: Chip remove buttons (36x36px) are intentionally sized below the 44px standard per specification to maintain compact filter UI while remaining usable on touch devices.

## Coverage Matrix

| Area           | Scenario                                  | Breakpoint | Locale | Test Type          | Status | Notes                                       |
| -------------- | ----------------------------------------- | ---------- | ------ | ------------------ | ------ | ------------------------------------------- |
| Shell          | Safe-area header/footer spacing           | 375x812    | en/it  | automated + manual | ✓ pass | Validate top and bottom inset handling      |
| Navigation     | 4 primary tabs + More behavior            | 375x812    | en/it  | automated + manual | ✓ pass | Ensure Settings/Debug reachable in <=2 taps |
| Collection     | One-column layout + add action visibility | 375x812    | en/it  | automated + manual | ✓ pass | Confirm min card width and no clipping      |
| Editing        | Parent/child sheet unwind                 | 375x812    | en/it  | automated + manual | ✓ pass | Hardware/back must close child then parent  |
| Media          | Camera fallback to picker                 | 375x812    | en/it  | automated + manual | ✓ pass | Preserve form state on fallback             |
| Desktop parity | Existing layout parity                    | 1280x800   | en/it  | automated          | ✓ pass | No behavior regression at >=768px           |
| Touch targets  | 44x44px controls, 36x36px exceptions      | 375x812    | en     | manual             | ✓ pass | All interactive controls verified           |

## Execution Log

- 2026-06-28: Initial matrix created for Phase 1 setup.
- 2026-06-29: Touch-target audit completed; all components verified for mobile touch policy.
