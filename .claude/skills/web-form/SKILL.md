---
name: web-form
description: Use this skill when architecting high-performance, accessible, and type-safe web forms for Tauri 2/Rust desktop apps. Specialized in Svelte 5 Runes and Superform-Zod-Garde validation cycles without SSR.
---

### 📋 Constraints & Design Patterns

- **Architecture:** Zero SSR. All forms must be client-side only.

- **Validation Cycle:**

1. **UI Level:** Use `Zod` within `superforms` for immediate, lightweight feedback.
2. **Backend Level:** Use the `garde` crate in Rust for the source-of-truth validation upon Tauri command invocation.

- **Display Logic:** Forms must be encapsulated in either a **Modal** (Dialog) or a **Drawer** component from `shadcn-svelte`.

---

### 🎨 UI/UX Specifications

The agent must implement the following visual and functional standards:

| Feature               | Requirement             | Implementation Detail                                                                                                  |
| --------------------- | ----------------------- | ---------------------------------------------------------------------------------------------------------------------- |
| **Required Fields**   | Clear visual indicators | Append a red asterisk `*` to the label. Use `aria-required="true"`.                                                    |
| **Field Hints**       | Contextual help         | Use `Form.Description` below the label; text should be muted/small.                                                    |
| **Error Feedback**    | Dual-mode highlighting  | 1. Show descriptive error text below the field. 2. Apply a **red border** (`border-destructive`) to the input element. |
| **Enums**             | Dropdown selection      | Use `Select` or `Combobox` components; never raw text inputs for fixed sets.                                           |
| **Validation Timing** | "Lightweight First"     | Trigger Zod validation `onBlur` or `onChange` (Superforms default) before calling the Rust backend.                    |

---

### 📝 Instructions

1. **Form Schema Definition:** Always start by defining a `Zod` schema that mirrors the backend `garde` struct. Use `.describe()` in Zod to store the "hints" for the UI.
2. **State Management:** Use Svelte 5 Runes (`$state`, `$derived`, `$props`) for local form state. Ensure `superForm` is initialized with `SPA: true` to prevent SSR errors.
3. **Component Structure:**

- Use `Form.Root`, `Form.Field`, and `Form.Control` hierarchy from `shadcn-svelte`.
- Inject `Select.Root` for enumerated types.
- Wrap the entire structure in a `<Dialog.Content>` (Modal) or `<Drawer.Content>` based on context.

4. **Error Handling:**

- Map backend `garde` errors back to the UI state using `superform`’s error reporting helper.
- Ensure the "Invalid" state is visually distinct: use `class={cn(errors && "border-destructive")}`.
