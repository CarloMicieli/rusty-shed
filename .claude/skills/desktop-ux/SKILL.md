---
name: desktop-ux
description: Use when planning, designing, implementing, or auditing user workflows and interaction patterns for the Rusty Shed Tauri 2 desktop application. Focuses on desktop-native usability, affordances, navigation, information hierarchy, keyboard workflows, and interaction ergonomics.
---

# Desktop UX

## 1. Purpose

Make Rusty Shed feel and behave like a coherent desktop application rather than a web document placed inside a window.

This skill governs:
- Workflow design and interaction behavior.
- Desktop affordances and signifiers.
- Navigation and information architecture at the interaction level.
- Information hierarchy and density.
- Keyboard-first operation.
- Feedback, confirmations, and error recovery.
- Desktop ergonomics and interaction consistency.

This skill does not define colors, typography, spacing, radii, shadows, or component appearance. Those decisions belong exclusively to `docs/ux/design-system.md`.

## 2. Mandatory Project References

Before designing or changing a workflow, consult:

1. `docs/ux/design-system.md`
2. `docs/frontend-architecture.md`

Use `design-system.md` for all visual decisions and `frontend-architecture.md` for the actual application shell, navigation structure, routing, and frontend conventions.

Inspect the existing workflow before proposing a new pattern. Prefer consistency with established application behavior unless there is a clear usability reason to change it.

## 3. Desktop-First Mental Model

Design for a user working in an application, not reading a responsive web page.

Prioritize:
- Clear work areas and stable navigation.
- Persistent context where it helps users complete tasks.
- Dense but legible presentation of operational information.
- Predictable placement of common actions.
- Efficient repeated workflows.
- Clear state transitions and recoverable errors.
- Keyboard and pointer operation.

Do not apply desktop conventions mechanically. Choose patterns based on the task, frequency, risk, and complexity of the workflow.

## 4. Affordances and Signifiers

Interactive elements must communicate that they are interactive.

Requirements:
1. Use established button, menu, link, tab, and control components.
2. Ensure actions have a recognizable interactive treatment consistent with `design-system.md`.
3. Provide hover, focus, pressed, selected, and disabled feedback where applicable.
4. Do not make plain descriptive text appear actionable.
5. Do not use a clickable generic container when a semantic button or link is appropriate.
6. Use pointer cursors for pointer-interactive elements when required by the project's component conventions.
7. Make destructive and irreversible actions distinguishable from ordinary actions.

Not every interactive element needs a heavy border or filled background. Follow the design system and preserve the visual distinction between primary actions, secondary actions, navigation, and low-emphasis controls.

## 5. Desktop Ergonomics

Consider Fitts's Law and the practical realities of pointer use.

- Make frequent and important actions easy to locate and activate.
- Avoid unnecessarily small hit targets.
- Place related actions near the content they affect.
- Keep primary workflow actions in predictable locations.
- Avoid requiring precise pointer movement for common operations.
- Do not move important controls unexpectedly between states.

Use consistent action placement when it reduces cognitive load. Do not force every primary action into the same corner if the workflow context calls for a different placement.

## 6. Keyboard-First Interaction

Support efficient keyboard operation.

Requirements:
1. Ensure all functionality is reachable by keyboard.
2. Preserve logical tab order.
3. Keep focus visible.
4. Use native keyboard behavior for semantic controls and accessible primitives.
5. Provide shortcuts for frequent actions only when they are useful and do not conflict with platform conventions.
6. Display shortcut hints where users can discover them, such as tooltips or command menus.
7. Respect platform differences between macOS, Windows, and Linux when defining shortcut labels and behavior.
8. Avoid intercepting common browser or operating-system shortcuts without a deliberate product decision.

Do not invent shortcuts without checking for conflicts and existing application conventions.

## 7. Navigation and Information Hierarchy

Users should be able to answer quickly:
- Where am I?
- What can I do here?
- What is the current task or selected object?
- What happens if I activate this action?
- How do I return or recover?

Requirements:
- Distinguish global navigation from local navigation and contextual actions.
- Keep navigation labels specific and consistent.
- Preserve context when moving into detail or editing workflows where practical.
- Make the active location or selection apparent.
- Group related information and actions.
- Put the most decision-relevant information before secondary metadata.
- Avoid decorative content that competes with operational information.

Use established project layout patterns. Do not independently prescribe a three-panel shell or other fixed layout; consult `frontend-architecture.md`.

## 8. Layering, Dialogs, and Overlays

Overlays must preserve legibility and interaction context.

Requirements:
1. Use the project's established Dialog, Drawer, Popover, and Menu primitives.
2. Ensure overlay content remains legible against the underlying interface.
3. Prevent unintended interaction with obscured content when the interaction is modal.
4. Use non-modal popovers only when the user should reasonably retain access to the surrounding interface.
5. Keep overlay content focused on the task.
6. Provide predictable close, cancel, and escape behavior.
7. Avoid stacking multiple overlays unless the workflow clearly requires it.
8. Return focus appropriately when an overlay closes.

The actual surface, backdrop, opacity, blur, border, and elevation values must come from `design-system.md`.

## 9. Feedback and Error Recovery

Every meaningful action should provide understandable feedback.

Consider:
- Immediate feedback for direct manipulation.
- Progress feedback for operations that take noticeable time.
- Confirmation for destructive or difficult-to-reverse actions.
- Clear success feedback after completion.
- Actionable error messages.
- Retry or recovery paths when possible.
- Empty states that explain what is absent and what the user can do next.

Do not show success messages before an operation has actually succeeded. Do not hide errors that prevent the user's task from completing.

## 10. Forms and Specialized Workflows

When designing a form workflow, use the `web-form` skill for schema, validation, submission, and backend error handling.

When designing a drawer workflow, use the `drawer-design` skill for drawer-specific structure and behavior.

This skill may define the interaction requirements for those workflows, but must not duplicate their implementation rules or visual specifications.

## 11. Tauri-Specific Considerations

- Treat the application as a desktop runtime with explicit operating-system integration.
- Follow the project's existing window, navigation, and platform conventions.
- Do not assume browser-only behavior is appropriate in the Tauri webview.
- Consider window resizing and the supported minimum window dimensions.
- Ensure keyboard shortcuts and system interactions do not conflict with platform conventions.
- Use Tauri capabilities and APIs only through the project's established, permission-aware integration patterns.
- Do not introduce new permissions or native integrations without understanding their security implications.

## 12. UX Audit Checklist

When auditing a workflow, evaluate:

1. Is the user's goal and current context clear?
2. Are interactive elements recognizable?
3. Are primary and secondary actions distinguishable?
4. Can the workflow be completed efficiently with a pointer?
5. Can it be completed with a keyboard?
6. Are focus, hover, pressed, disabled, and selected states understandable?
7. Is navigation predictable?
8. Are overlays legible and behaviorally appropriate?
9. Are loading, success, empty, and error states handled?
10. Can users recover from mistakes?
11. Does the workflow follow the project's visual and frontend architecture?

For each finding, describe the usability impact and recommend a concrete correction. Prioritize task blockers, accessibility issues, and error-prone interactions over subjective polish.
