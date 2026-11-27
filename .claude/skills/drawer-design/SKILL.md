---
name: drawer-design
description: Use when designing, implementing, refactoring, or auditing drawer-based workflows in the Rusty Shed Tauri 2 application. Covers drawer anatomy, headers, sectioning, field grouping, contextual editing, footer actions, and accessible drawer behavior.
---

# Drawer Design

## 1. Purpose

Create consistent, task-focused drawers for contextual viewing, creation, and editing workflows.

This skill governs:
- Drawer structure and anatomy.
- Header and footer organization.
- Content sectioning and grouping.
- Field layout and contextual actions.
- Collapsible sections.
- Drawer interaction and accessibility.
- Reuse of established drawer patterns.

It does not define an independent visual theme or form-validation architecture.

## 2. Mandatory Project References

Before implementing or modifying a drawer, consult:

1. `docs/ux/DESIGN_SYSTEM.md`
2. `docs/FRONTEND_ARCHITECTURE.md`

Authority:
- `DESIGN_SYSTEM.md` defines all drawer surfaces, colors, typography, spacing, borders, radii, shadows, icon treatments, and interaction states.
- `FRONTEND_ARCHITECTURE.md` defines the project's component organization, routing, and frontend implementation conventions.
- This skill defines drawer-specific composition and behavior.

Inspect existing drawers and shared components before creating a new pattern. Reuse established project components where possible.

If this skill conflicts with either reference document, follow the reference document.

## 3. When to Use a Drawer

Use a drawer when the user benefits from editing or inspecting an object while retaining context from the underlying view.

Good use cases:
- Creating or editing a domain object in context.
- Inspecting details without navigating away from a list or workspace.
- Performing a focused workflow associated with a selected item.
- Showing contextual settings or metadata.

Prefer another container when appropriate:
- Use a Dialog for a short, focused decision or confirmation.
- Use a dedicated page for complex, multi-step, or information-dense workflows.
- Use inline editing for small changes that do not need a separate workspace.

Do not force every form or detail view into a drawer.

## 4. Drawer Anatomy

A standard drawer should have clearly separated regions:

1. **Header:** Identifies the task or object and provides an appropriate close action.
2. **Content:** Holds the task's fields, information, and grouped sections.
3. **Footer:** Holds the primary completion action and relevant secondary actions when a persistent footer is appropriate.

Keep these regions predictable. The content region should own scrolling when the drawer's height is constrained; avoid making the header and footer scroll away when persistent controls are needed.

Use the project's existing Drawer primitives and composition patterns.

## 5. Header

The header should establish context immediately.

Requirements:
- Use an icon only when it helps identify the object or task.
- Provide a concise, descriptive title.
- Add a subtitle only when it provides useful context, such as the selected object's identity or the purpose of the workflow.
- Provide a clearly accessible close control.
- Avoid duplicating information already prominent in the content.
- Keep header actions limited to actions relevant to the drawer itself.

Icon size, container treatment, colors, typography, spacing, and close-button styling must follow `DESIGN_SYSTEM.md`. Do not hardcode pixel sizes or colors from older drawer examples.

## 6. Content Organization

Group content according to the user's mental model and task sequence.

Requirements:
1. Put the most important or frequently edited information first.
2. Group related fields into meaningful sections.
3. Use clear section headings.
4. Use collapsible sections only when they reduce clutter without hiding essential information.
5. Keep descriptions and help text close to the field or section they explain.
6. Avoid excessive nested cards, borders, and containers.
7. Preserve a clear visual hierarchy between section headings, field labels, values, and supporting information.

Do not create sections merely to fill space. Each section should represent a meaningful category or stage of the workflow.

## 7. Field Grouping and Layout

Choose field layouts based on the relationship and expected length of the values.

Guidance:
- Place short, closely related fields together when side-by-side entry improves scanning.
- Give long text, descriptions, notes, and other wide content sufficient width.
- Avoid placing unrelated fields in the same row just to reduce drawer height.
- Keep field order aligned with the user's task sequence.
- Ensure layouts remain usable at the application's supported window sizes.

A two-column layout can work for short related fields, while long descriptions generally benefit from a full-width layout. These are layout guidelines, not mandatory fixed grids.

Field components, labels, hints, required indicators, and error states must follow the project's form conventions and `DESIGN_SYSTEM.md`. Use `web-form` for validation and submission behavior.

## 8. Selection and Specialized Inputs

Use established controls appropriate to the data.

Examples:
- Use a searchable command-style selection when users choose from a large catalog.
- Use a standard select for a small, fixed set of options.
- Show the current selection clearly.
- Make search, clear, and selection states accessible.
- Use appropriate numeric and currency controls for numeric values.
- Explain domain-specific units or scales where they are not self-evident.

Do not hardcode a particular dropdown style, currency placement, placeholder example, or scale-label format in this skill. Follow domain conventions already established in the application and the visual rules in `DESIGN_SYSTEM.md`.

## 9. Footer and Actions

The footer should make the next step clear.

Requirements:
1. Provide one visually dominant primary action when the workflow has a clear completion action.
2. Place secondary actions so they are easy to distinguish from the primary action.
3. Use action labels that describe the outcome, such as "Save changes" or "Add item", rather than vague labels where possible.
4. Prevent accidental duplicate submission.
5. Reflect disabled and submitting states.
6. Confirm or warn before destructive actions when appropriate.
7. Avoid placing unrelated navigation or low-priority actions in the footer.

Button variants, colors, dimensions, alignment, and spacing must follow `DESIGN_SYSTEM.md`.

## 10. Drawer Behavior and Accessibility

Requirements:
- Use the project's accessible Drawer primitive.
- Provide an accessible name for the drawer.
- Ensure keyboard focus enters the drawer appropriately.
- Keep focus within a modal drawer while it is open, when required by the primitive's modal behavior.
- Restore focus to the triggering control when the drawer closes.
- Support expected Escape and close-button behavior.
- Ensure all controls are keyboard accessible.
- Preserve visible focus indicators.
- Give icon-only controls accessible labels.
- Avoid relying on color alone to communicate status.
- Ensure scrolling content does not obscure important actions.

Do not override accessibility behavior provided by the underlying primitive without a specific, tested reason.

## 11. State and Lifecycle

Represent drawer states explicitly where relevant:
- Initial or empty state.
- Loading state.
- Populated state.
- Editing state.
- Submitting state.
- Success or completion state.
- Error state.

Requirements:
- Avoid showing stale data when the selected object changes.
- Handle asynchronous loading and submission errors.
- Prevent duplicate operations.
- Preserve user input when an operation fails, where safe.
- Clean up subscriptions and listeners.
- Keep business logic outside purely presentational drawer primitives.

## 12. Relationship to Other Skills

- Use `frontend-design` for Svelte 5 implementation, component composition, tokens, and shadcn-svelte conventions.
- Use `desktop-ux` for workflow ergonomics, navigation context, and desktop interaction decisions.
- Use `web-form` when the drawer contains a form requiring schema, validation, or backend submission.

Do not duplicate the detailed requirements of those skills.

## 13. Drawer Audit Checklist

When auditing a drawer, verify:

1. The drawer is the appropriate container for the workflow.
2. Header title and context are clear.
3. Header, content, and footer have predictable behavior.
4. Information is grouped meaningfully.
5. Field layout reflects the content and task.
6. Existing selection and input components are reused.
7. Primary and secondary actions are clear.
8. Loading, submitting, success, and error states are handled.
9. Keyboard focus and close behavior are correct.
10. All visual decisions follow `DESIGN_SYSTEM.md`.

For each issue, explain the impact and provide a concrete correction using existing project components and tokens.
