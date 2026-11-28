---
name: web-form
description: Use when architecting, implementing, refactoring, or auditing forms in the Rusty Shed Tauri 2 application. Covers Svelte 5, Superforms SPA mode, Zod schemas, Rust Garde validation, type safety, error handling, accessibility, and frontend/backend validation contracts.
---

# Web Form Architecture

## 1. Purpose

Implement accessible, type-safe, predictable forms for the Rusty Shed Tauri 2 application.

This skill governs:
- Form architecture and component composition.
- Frontend validation using Zod.
- Backend validation using Rust Garde.
- Frontend/backend data contracts.
- Form state management.
- Validation error handling.
- Form submission and asynchronous states.
- Form accessibility.

It does not define an independent visual system or dictate whether every form must live in a Dialog or Drawer.

## 2. Mandatory Project References

Before implementing or modifying a form, consult:

1. `docs/ux/design-system.md`
2. `docs/frontend-architecture.md`

Authority:
- `design-system.md` is the single source of truth for all visual decisions, including field styling, labels, descriptions, required indicators, invalid states, and form layout appearance.
- `frontend-architecture.md` defines the project's frontend execution model, Svelte conventions, routing, component organization, and Tauri integration patterns.
- This skill defines form-specific architecture, validation, and data-contract requirements.

Inspect the existing form implementation, Rust command, input types, and package versions before selecting an implementation.

## 3. Execution Model and Superforms

Follow the project's zero-SSR architecture.

Requirements:
1. Forms must execute client-side in the Tauri application.
2. Use Superforms SPA mode where the project uses Superforms.
3. Do not introduce server actions or server-side form dependencies.
4. Follow the existing form initialization and data-loading patterns.
5. Do not assume that `SPA: true` alone configures the entire application for client-side execution.

Use the installed Superforms version and verify its API before implementing configuration. A typical SPA configuration may use:

```ts
superForm(initialData, {
  SPA: true
});
```

Treat this as an illustration, not a replacement for checking the actual project integration.

## 4. Validation Architecture

Use two validation layers with distinct responsibilities.

### Frontend: Zod

Zod provides:
- Immediate user feedback.
- Client-side validation before command invocation.
- Structured form data validation.
- Field-level error information.

### Backend: Rust Garde

Garde provides authoritative validation when a Tauri command is invoked.

The backend must validate untrusted input independently. Never assume that frontend validation has run or that it cannot be bypassed.

Validation flow:

```text
User Input
    |
    v
Svelte Form State
    |
    v
Zod Validation
    |
    v
Tauri Command
    |
    v
Rust Garde Validation
    |
    v
Business Logic / Persistence
```

Frontend validation improves usability; backend validation protects application correctness and data integrity.

## 5. Schema and Type Safety

### Zod Schemas

Define or reuse a Zod schema for structured form input.

Requirements:
1. Mirror the corresponding Rust command input contract.
2. Use `.describe()` for useful field descriptions or metadata where appropriate.
3. Distinguish optional fields from nullable fields.
4. Match enum values to the Rust serialization contract.
5. Define numeric, date, identifier, and nested-object serialization explicitly.
6. Avoid implicit coercion unless it is intentional and documented.
7. Keep transformations explicit so the submitted payload has a predictable shape.
8. Reuse shared schemas when the same contract is used by multiple forms.

Do not assume that Zod and Garde support identical rules or produce identical error messages. Implement equivalent behavior deliberately where required.

### Rust Contract Inspection

Before implementing a form, inspect the corresponding Rust command and data structures.

Verify:
- Required and optional fields.
- Nullable values and defaults.
- Enum variants and serialization names.
- String and numeric constraints.
- Nested structures and collections.
- Date/time and identifier representations.
- Garde validation rules.
- Any domain-level constraints enforced outside Garde.

Do not invent frontend constraints that contradict backend behavior. If a mismatch is discovered, identify the intended contract and update the appropriate layer deliberately.

## 6. Svelte 5 State Management

Use Svelte 5 runes for component-local state, consistent with `frontend-architecture.md`.

Requirements:
- `$props()` for component properties and defaults.
- `$state()` for mutable local UI state.
- `$derived()` or `$derived.by()` for computed values.
- `$effect()` only for genuine side effects.
- Superforms as the form-state and validation integration when applicable.

Avoid:
- Maintaining a second independent copy of form values.
- Using `$effect()` to calculate values that belong in `$derived`.
- Creating reactive loops between form state and component state.
- Mutating derived values.
- Hiding submission behavior in unrelated presentational components.

## 7. Component Structure

Use the project's existing shadcn-svelte form primitives and wrappers.

Prefer the established composition for:
- Form root.
- Form field.
- Form control.
- Form description.
- Field-level errors.

Use the actual exports and APIs available in the installed shadcn-svelte version. Do not assume illustrative names such as `Form.Root` or `Form.Field` are valid without checking the local project.

Choose controls based on the data:
- Select or Combobox for fixed enumerated values.
- Numeric controls for numeric input.
- Date/time controls for date and time values.
- Textarea for longer text.
- Appropriate specialized controls for domain-specific data.

Never use a free-text input for a fixed enum when a suitable selection control exists.

### Container Selection

Choose the form container according to workflow complexity:
- Dialog: short, focused interactions.
- Drawer: contextual editing while retaining the underlying view.
- Dedicated page: complex, multi-step, or information-dense forms.
- Inline form: lightweight edits.

Consult `drawer-design` for drawer-specific composition and `desktop-ux` for workflow ergonomics.

## 8. Validation Timing and User Feedback

Choose validation timing according to field complexity and user experience.

Guidance:
- Validate on blur for fields where feedback during typing would be distracting.
- Validate on change for simple fields where immediate feedback is useful.
- Validate on submit before invoking the backend.
- Avoid displaying errors for untouched fields too aggressively unless the project has an established convention.

### Required Fields

Required fields must:
- Be clearly identified visually.
- Expose required state programmatically when appropriate.
- Have labels correctly associated with their controls.

The exact indicator and styling must come from `design-system.md`.

### Invalid Fields

Invalid fields must:
- Display descriptive, actionable error messages.
- Expose invalid state programmatically, such as with `aria-invalid`.
- Associate the error text with the corresponding control.
- Follow the invalid-state styling defined by `design-system.md`.

Do not rely exclusively on a red border or color to communicate an error.

### Field Descriptions

Use descriptions for constraints, examples, units, or contextual help when they help users enter valid data.

Descriptions must be programmatically associated with the corresponding field where supported by the component primitives.

## 9. Backend Error Mapping

Map Rust Garde validation errors and other expected command failures back into the form.

Requirements:
1. Preserve field-level error information.
2. Map backend field identifiers to the corresponding frontend fields.
3. Display errors that cannot be associated with a field as form-level errors.
4. Handle multiple errors predictably.
5. Do not silently discard validation failures.
6. Avoid exposing internal implementation details or sensitive data in user-facing errors.
7. Preserve entered values when a submission fails, where safe.

Use the existing Superforms error-reporting integration when appropriate and supported by the installed version.

## 10. Submission Lifecycle

Every asynchronous form submission must handle the relevant states:

1. Initial.
2. Editing.
3. Client validation.
4. Submitting.
5. Success.
6. Backend validation failure.
7. Unexpected failure.

Requirements:
- Prevent accidental duplicate submissions.
- Provide clear progress feedback for operations that take noticeable time.
- Disable or guard actions appropriately while submitting.
- Preserve user input when submission fails, where safe.
- Handle rejected Tauri command promises.
- Show actionable error feedback.
- Update or invalidate affected application data after success using the project's established data-flow pattern.
- Do not report success until the operation has actually succeeded.

## 11. Accessibility

All forms must support:
- Explicit labels and correct label-control associations.
- Accessible required-field communication.
- Programmatically exposed invalid states.
- Error messages associated with their fields.
- Keyboard navigation.
- Visible focus indicators.
- Appropriate input types and autocomplete attributes where relevant.
- Accessible submission and loading feedback.
- Semantic grouping for related controls.

Use native semantics first. Add ARIA attributes when needed to communicate relationships or states that native HTML does not already provide.

## 12. Form Audit Workflow

When auditing a form, report findings in this order:

1. **Contract and validation:** Zod/Rust mismatch, missing Garde validation, incorrect enum or serialization contracts.
2. **Submission and error handling:** missing states, swallowed errors, duplicate submissions, incorrect backend error mapping.
3. **State management:** duplicated state, inappropriate runes, unstable form initialization.
4. **Component architecture:** incorrect primitives, unsuitable controls, unnecessary custom abstractions.
5. **Accessibility:** labels, descriptions, invalid states, keyboard behavior, focus.
6. **Design-system compliance:** field styling, required indicators, error appearance, spacing, and surfaces.

For each finding, include:
- What is wrong.
- Why it matters.
- The affected file or contract, when known.
- A concrete correction consistent with the project architecture.

## 13. Completion Checklist

Before considering form work complete, verify:
- [ ] `design-system.md` was followed for all visual decisions.
- [ ] `frontend-architecture.md` was followed.
- [ ] The corresponding Rust command and input types were inspected.
- [ ] Zod schema matches the intended command contract.
- [ ] Rust Garde validation remains authoritative.
- [ ] Enum, optional, nullable, date, and numeric serialization is explicit.
- [ ] Superforms is configured for the project's client-side architecture.
- [ ] Svelte 5 runes are used appropriately.
- [ ] Appropriate shadcn-svelte controls are used.
- [ ] Backend field and form-level errors are handled.
- [ ] Loading, success, and failure states are implemented.
- [ ] Keyboard and accessibility requirements are satisfied.
- [ ] User input is preserved on recoverable failures.
