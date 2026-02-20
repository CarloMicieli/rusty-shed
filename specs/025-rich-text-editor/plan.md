# Implementation Plan: Rich Text Editor for RailwayModelCard

**Branch**: `025-rich-text-editor` | **Date**: 2026-02-20 | **Spec**: [spec.md](spec.md)
**Input**: Feature specification from `/specs/025-rich-text-editor/spec.md`

---

## Summary

Replace the plain `InPlaceEdit` textarea in the Details tab of `RailwayModelCard` with a headless Tiptap rich-text editor that stores content as Markdown. The editor renders formatted Markdown in Display Mode (via `marked`) and switches to an editable Tiptap instance on click. Auto-save on blur commits Markdown to the existing `update_railway_model_text` IPC command. No backend or schema changes are required.

---

## Technical Context

**Language/Version**: TypeScript 5.9.3 (frontend), Rust edition 2024 / 1.93.0 (backend — no changes needed)
**Primary Dependencies**: Tiptap (`@tiptap/core`, `@tiptap/pm`, `@tiptap/starter-kit`, `@tiptap/markdown`), `marked` (Markdown→HTML display renderer), `@tailwindcss/typography` (already installed)
**Storage**: SQLite via sqlx — existing `details TEXT NULL` column on `railway_models` table; no migration required
**Testing**: Vitest v4 with happy-dom (frontend), cargo test (backend — no new tests needed)
**Target Platform**: Tauri 2.9.x desktop app (macOS / Linux / Windows)
**Project Type**: Tauri 2 desktop with SvelteKit frontend
**Performance Goals**: Display→Editor transition ≤ 200ms; blur-to-save round-trip ≤ 500ms
**Constraints**: No Rust changes; no schema migrations; no new Tauri commands; all strings via Paraglide; TypeScript strict mode; ESLint/Prettier must pass

---

## Constitution Check

_GATE: Must pass before Phase 0 research. Re-check after Phase 1 design._

| Principle                                    | Status  | Notes                                                                                        |
| -------------------------------------------- | ------- | -------------------------------------------------------------------------------------------- |
| **Modular, Library-First Design**            | ✅ PASS | `RichTextEditor` and `RichTextToolbar` are self-contained, independently testable components |
| **Deterministic Interfaces & Observability** | ✅ PASS | Reuses existing specta-generated `updateRailwayModelText` IPC contract; no new commands      |
| **Test-First Emphasis**                      | ✅ PASS | Vitest unit tests required for RichTextEditor; acceptance scenarios testable independently   |
| **Code Quality**                             | ✅ PASS | TypeScript strict; `pnpm lint` + `pnpm check` + Prettier enforced                            |
| **Testing Standards**                        | ✅ PASS | Component unit tests at `src/__tests__/components/RichTextEditor.test.ts`                    |
| **User Experience Consistency**              | ✅ PASS | Paraglide for all user-facing strings; design tokens from existing CSS variables             |
| **Performance Requirements**                 | ✅ PASS | Tiptap mounted lazily (only when editing); `marked` renders synchronously for Display Mode   |
| **Safe Rust Practices**                      | ✅ N/A  | No Rust code changes in this feature                                                         |
| **Database (Persistence)**                   | ✅ PASS | No schema change; existing sqlx migration handles `details` column                           |
| **State Management / Domain Event Tracking** | ✅ PASS | Existing `UpdateRailwayModelText` domain event and repository handle persistence             |
| **API Design & Transport Boundary**          | ✅ PASS | Existing specta-typed command reused; no new IPC endpoints                                   |
| **Domain Logic Location**                    | ✅ PASS | Pure frontend feature; no business logic on frontend                                         |

**Constitution Check result: ALL GATES PASS — no violations.**

---

## Project Structure

### Documentation (this feature)

```text
specs/025-rich-text-editor/
├── plan.md              # This file
├── research.md          # Phase 0 — all NEEDS CLARIFICATION resolved
├── data-model.md        # Phase 1 — entity model + state machine
├── quickstart.md        # Phase 1 — step-by-step dev setup
├── contracts/
│   └── ipc-commands.md  # Phase 1 — existing IPC contract reference
├── checklists/
│   └── requirements.md  # Spec quality validation (all pass)
└── tasks.md             # Phase 2 output (/speckit.tasks — not yet created)
```

### Source Code (affected files)

```text
src/
├── routes/
│   └── layout.css                        # + @plugin "@tailwindcss/typography"
│                                         # + .prose color overrides
├── lib/
│   ├── components/
│   │   ├── RichTextEditor.svelte         # NEW — display/editor mode wrapper
│   │   ├── RichTextToolbar.svelte        # NEW — Bold/Italic/UL/OL toolbar
│   │   └── RailwayModelCard.svelte       # MODIFIED — replace details InPlaceEdit
│   └── paraglide/
│       └── messages/en.json              # + details_placeholder, details_save_failed
└── __tests__/
    └── components/
        └── RichTextEditor.test.ts        # NEW — unit tests

# No changes in src-tauri/ (backend unchanged)
# No changes in migrations/ (schema unchanged)
```

**Structure Decision**: Tauri 2 desktop app (frontend in `src/`, backend in `src-tauri/`). Only frontend changes. No new feature directories needed — components follow the existing flat `src/lib/components/` pattern for shared UI.

---

## Architecture Design

### Component Hierarchy

```
RailwayModelCard.svelte
└── (Details tab)
    └── RichTextEditor.svelte          ← replaces InPlaceEdit
        ├── (Display Mode)
        │   ├── <div class="prose prose-invert ..."> (rendered Markdown via marked)
        │   └── <p class="text-muted-foreground italic"> (placeholder)
        └── (Editor Mode)
            ├── RichTextToolbar.svelte  ← Bold, Italic, UL, OL buttons
            └── <div bind:this={editorElement}> (Tiptap mount point)
```

### Tiptap Integration Pattern (Svelte 5)

**Direct instantiation** — no wrapper package. Editor lifecycle managed by `$effect`:

```typescript
// $state variable holds Editor instance — reassigned on every transaction
// to trigger Svelte 5 fine-grained reactivity for toolbar button states
let editor = $state<Editor | null>(null);

$effect(() => {
  if (isEditing && editorElement && !editor) {
    const instance = new Editor({
      element: editorElement,
      extensions: [StarterKit, Markdown.configure({ ... })],
      content: localValue,   // Markdown string parsed automatically
      onTransaction: () => { editor = instance; /* force $state reactivity */ },
      onBlur: () => { handleBlur(); },
    });
    editor = instance;
  }
  if (!isEditing && editor) {
    editor.destroy();
    editor = null;   // GC on exit
  }
});
```

**Toolbar active states** use `$derived` to avoid per-keystroke overhead:

```typescript
let isBold = $derived(editor?.isActive('bold') ?? false);
```

### Markdown Data Flow

```
Database                Frontend                     Editor
─────────               ─────────                    ──────
details TEXT            model.details                @tiptap/markdown
(Markdown | NULL)  →→→  → RichTextEditor.value  →→→  content prop (parsed)
                        localValue                   editor.storage
                        ← markdown string ←←←←←←←←  .getMarkdown()
                        ↓
                     commands.updateRailwayModelText({
                       field: 'Details', value: markdownString
                     })
```

### Display Mode Rendering

```typescript
// marked.parse() is synchronous and fast — safe for $derived
const displayHtml = $derived(localValue ? (marked.parse(localValue) as string) : '');
```

Styled with `<div class="prose prose-invert max-w-none">` — `@tailwindcss/typography` registered via `@plugin` in `layout.css`. Prose colors overridden to match the steampunk zinc/copper design tokens.

### Paste Normalisation

`Markdown.configure({ transformPastedText: true })` intercepts `paste` events at the ProseMirror level and converts HTML clipboard content to Markdown before insertion. No custom paste handler needed.

---

## Dependency Installation

```bash
pnpm add @tiptap/core @tiptap/pm @tiptap/starter-kit @tiptap/markdown marked
```

`@tailwindcss/typography` is already installed (v0.5.19) — no additional install needed.

---

## Testing Strategy

### Unit Tests (`src/__tests__/components/RichTextEditor.test.ts`)

| Test                                                       | Coverage       |
| ---------------------------------------------------------- | -------------- |
| Display Mode renders Markdown HTML correctly               | FR-001, SC-005 |
| Display Mode shows placeholder when content is null        | FR-013, US4    |
| Click activates Editor Mode (editable=true)                | FR-002, SC-001 |
| Click does nothing in read-only mode (editable=false)      | FR-016, SC-007 |
| Hover class present when editable, absent when not         | FR-003, SC-007 |
| onSave called with correct Markdown on blur                | FR-004         |
| onSave NOT called when no changes made                     | FR-005         |
| Editor stays open and error is surfaced when onSave throws | FR-006, SC-008 |
| Returns to placeholder after saving empty content          | US4, FR-013    |

### Manual Acceptance Tests

Covered by acceptance scenarios in `spec.md`. Key flows:

1. US1: Click → edit → blur → verify save + render (core loop)
2. US2: Bold/Italic/UL/OL toolbar interactions
3. US3: Paste from hornby.com → verify clean Markdown output
4. US4: Empty model → placeholder visible → click → empty editor → save → placeholder returns

---

## Rollout Notes

- **No feature flag needed** — the change is localised to the Details tab of RailwayModelCard
- **Backwards compatible** — existing `details` TEXT data continues to be valid Markdown (or plain text, which renders correctly in Display Mode)
- **No migration** — schema unchanged

---

## Phase 2 Next Steps

Run `/speckit.tasks` to generate `tasks.md` with dependency-ordered implementation tasks.

Expected task count: 8–12 tasks covering:

1. Dependency installation
2. Typography plugin activation + prose theme
3. `RichTextToolbar.svelte` component
4. `RichTextEditor.svelte` component (Display Mode)
5. `RichTextEditor.svelte` component (Editor Mode + Tiptap)
6. Paraglide message keys
7. `RailwayModelCard.svelte` integration
8. Unit tests
9. Lint + type check + manual verification
