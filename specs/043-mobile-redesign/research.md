# Phase 0 Research: Mobile Redesign

## Decision 1: Use CSS-first responsive switching for `<768px` scope

- Decision: Mobile behavior is implemented with Tailwind base classes for mobile and `md:` overrides for desktop/tablet, avoiding JS-driven resize loops for structural layout.
- Rationale: CSS media-query evaluation avoids runtime layout thrashing during Tauri webview warm-up and keeps transitions deterministic.
- Alternatives considered:
  - `window.innerWidth`/resize listener branching: rejected due to reflow churn and startup pop-in risk.
  - Fully JS media orchestration for all responsive behavior: rejected due to complexity and regression risk.
- Implementation notes:
  - Use a minimal `matchMedia` state helper only for cases CSS cannot express cleanly (for example conditional slot positioning and behavior toggles).
  - Keep CSS as source of truth for geometry and visibility.

## Decision 2: Enforce stateless, depth-limited sheet registry

- Decision: Implement `DrawerRegistry` as a bounded stack with maximum depth 2 (`parent + one child`), rejecting or replacing attempts to open a third layer.
- Rationale: Matches clarified requirement FR-017 and prevents stale nested state when users dismiss via gesture or hardware back.
- Alternatives considered:
  - Unbounded stack: rejected due to UX ambiguity and bug-prone dismiss chains.
  - Single-layer only: rejected because one nested child is required by product decision.
- Implementation notes:
  - On mobile back action, close child first, then parent, then allow route-level back.
  - Synchronize registry with route/history state so hardware back unwinds overlays before leaving page.

## Decision 3: Prefer capability-aware camera flow with resilient fallback

- Decision: Provide camera-first affordance with explicit media capability interrogation when available; if camera capability is unavailable, denied, or blocked, downgrade to file picker/gallery in the same flow.
- Rationale: WebView camera behavior is platform-variable; explicit capability checks with graceful fallback reduce dead-end interactions.
- Alternatives considered:
  - Camera-only path: rejected because permission or device constraints can fail silently.
  - File-picker-only path: rejected because camera-first remains preferred UX on mobile.
- Implementation notes:
  - Attempt `navigator.mediaDevices.enumerateDevices()` in guarded async flow.
  - If API is missing, throws, or no `videoinput` exists, switch copy and remove capture-specific attributes.
  - Preserve edit form state when fallback path is selected.

## Decision 4: Harden bottom-sheet animation for low-end Android webviews

- Decision: Use transform-based animations (`translateY`) with compositor hints (`will-change: transform` lifecycle usage and `translate3d(0,0,0)`/equivalent GPU promotion), plus reduced-motion fallbacks.
- Rationale: Prevents stutter during sheet transitions under constrained WebView GPUs while keeping interaction smooth.
- Alternatives considered:
  - Position-based animation (`top/bottom`): rejected due to layout recalculation cost.
  - JS `requestAnimationFrame` animation orchestration: rejected due to complexity and main-thread contention.
- Implementation notes:
  - Apply compositor hints only during transition/gesture phases.
  - Ensure overlay/backdrop and sheet layers avoid costly nested effects.
  - Respect `prefers-reduced-motion` with simplified or instant transitions.

## Decision 5: Preserve desktop behavior through route-sliced rollout gates

- Decision: Ship redesign in milestone slices (shell/navigation, collection, detail sheets, supplementary sheets) with viewport-gated behavior and explicit desktop parity checks at each slice.
- Rationale: Reduces regression blast radius and keeps app releasable at all times.
- Alternatives considered:
  - Big-bang rewrite: rejected due to high integration and regression risk.
  - Per-component random migration: rejected due to inconsistent UX and validation gaps.

## Decision 6: Keep transport and persistence architecture unchanged

- Decision: Reuse existing Tauri IPC commands and settings/media flows; no new persistence model introduced for this feature.
- Rationale: Mobile redesign is interaction/layout-focused; preserving transport boundaries aligns with constitution and reduces backend risk.
- Alternatives considered:
  - New backend endpoints for all mobile flows: rejected unless gaps are found during implementation.
  - Frontend-only persistence bypass: rejected by architectural laws.
