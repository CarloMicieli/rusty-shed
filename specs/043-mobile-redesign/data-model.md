# Data Model: Mobile Redesign

## Entity: MobileViewportProfile

Represents effective viewport-driven UI mode.

| Field | Type | Required | Validation | Notes |
| --- | --- | --- | --- | --- |
| `isMobile` | boolean | Yes | `true` only when viewport width `<768px` | Canonical redesign gate |
| `viewportWidth` | number | Yes | Positive integer | Runtime measured value |
| `safeAreaInsets` | object | Yes | `top/right/bottom/left` non-negative | From CSS env values when available |
| `reducedMotion` | boolean | Yes | Browser media query derived | Animation policy input |

## Entity: DrawerStackState

Tracks active sheet hierarchy for detail and quick actions.

| Field | Type | Required | Validation | Notes |
| --- | --- | --- | --- | --- |
| `stack` | DrawerLayer[] | Yes | Length `0..2` | Parent + one child max |
| `activeLayerId` | string \| null | No | Must reference top of stack when present | Current interactive layer |
| `sourceRoute` | string | Yes | Non-empty route path | Used for back-sync behavior |

`DrawerLayer`

| Field | Type | Required | Validation | Notes |
| --- | --- | --- | --- | --- |
| `id` | string | Yes | Unique per open cycle | Registry identifier |
| `kind` | `parent` \| `child` | Yes | Derived from stack position | Enforces one-level nesting |
| `dismissMode` | `gesture` \| `button` \| `back` | Yes | Enum | Telemetry/debug friendly |
| `payload` | Record<string, unknown> \| null | No | Optional and serializable | Sheet-specific context |

## Entity: MobileNavigationModel

Defines mobile navigation composition and secondary access.

| Field | Type | Required | Validation | Notes |
| --- | --- | --- | --- | --- |
| `primaryDestinations` | string[] | Yes | Exactly 4 items | Fixed bottom-nav items |
| `hasMoreEntry` | boolean | Yes | Must be `true` on mobile | Required for secondary destinations |
| `moreTopActions` | string[] | Yes | Must include `settings`, `debug` | Clarified decision |

## Entity: TouchTargetPolicy

Defines interactive hit target constraints.

| Field | Type | Required | Validation | Notes |
| --- | --- | --- | --- | --- |
| `defaultMinSizePx` | number | Yes | Must be `44` | Applies to mobile controls |
| `chipRemoveMinSizePx` | number | Yes | Must be `36` | Exception case |
| `scope` | string | Yes | `<768px` | Mobile-only enforcement |

## Entity: MediaAttachmentCapability

Represents camera availability and fallback mode in edit flows.

| Field | Type | Required | Validation | Notes |
| --- | --- | --- | --- | --- |
| `cameraAvailable` | boolean | Yes | Derived from capability probe or guarded fallback | Controls capture affordance |
| `probeStatus` | `success` \| `unsupported` \| `denied` \| `error` | Yes | Enum | Debug/UX mapping |
| `activeInputMode` | `camera_capture` \| `gallery_picker` | Yes | Enum | Must stay in same edit session |
| `captureAttributeEnabled` | boolean | Yes | False when no camera capability | DOM safety rule |

## Entity: MobileCollectionLayoutState

Defines collection rendering mode on phone-sized viewports.

| Field | Type | Required | Validation | Notes |
| --- | --- | --- | --- | --- |
| `itemMinWidth` | number | Yes | Must be `320` on mobile collection view | Prevents multi-column squeeze |
| `tableToggleVisible` | boolean | Yes | `false` on mobile | Layout control adaptation |
| `fabVisible` | boolean | Yes | `true` in collection mobile view | Contextual quick action |

## State Transitions

1. `DesktopMode` -> `MobileMode` when viewport enters `<768px` profile.
2. `DrawerStackState.stack=[]` -> `[parent]` on first sheet open.
3. `[parent]` -> `[parent, child]` on nested action open.
4. Attempt to push third layer -> rejected or replaced child (never exceed length 2).
5. Hardware back while child open -> pop child only.
6. Hardware back while parent open -> pop parent only.
7. Media probe unavailable/denied/error -> switch to `gallery_picker` without resetting edit form.
8. Collection mobile activation -> enforce one-column behavior and hide table toggle.

## Validation Rules Summary

- Mobile redesign rules must only apply when `isMobile=true`.
- `DrawerStackState.stack.length` must never exceed 2.
- Settings and Debug must remain accessible from More top actions on mobile.
- All mobile controls must meet 44x44 hit target except chip-remove controls at 36x36.
- Camera fallback must preserve user progress in the current edit flow.
