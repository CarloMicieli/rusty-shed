# Data Model: Rich Text Editor for RailwayModelCard

**Feature**: 025-rich-text-editor
**Date**: 2026-02-20

---

## Entities

### ModelDetails (existing — no change)

The `details` field on `RailwayModel` is the only persistent entity affected by this feature.

| Attribute | Type                    | Constraints  | Notes                                             |
| --------- | ----------------------- | ------------ | ------------------------------------------------- |
| `id`      | `RailwayModelId` (UUID) | PK, not null | Identifies the parent model                       |
| `details` | `TEXT` (Markdown)       | nullable     | Stores the rich text as Markdown; NULL when empty |

**State transitions**:

- `NULL / empty` → `has content`: user types in editor and saves (auto-save on blur)
- `has content` → `NULL / empty`: user clears all text and saves (empty string → stored as NULL)
- `has content` → `has content`: user edits and saves

**No schema migration required.** The column already exists as `TEXT NULL` in the SQLite database.

---

## Frontend Component Entities

These are UI components introduced by this feature. They have no database representation.

### RichTextEditor (new Svelte component)

Manages the Display/Editor Mode state machine for the `details` field.

| Prop          | Type                               | Required              | Notes                                                 |
| ------------- | ---------------------------------- | --------------------- | ----------------------------------------------------- |
| `value`       | `string \| null`                   | Yes                   | Current Markdown content from the model               |
| `editable`    | `boolean`                          | No (default: `false`) | Enables click-to-edit when true                       |
| `placeholder` | `string`                           | No                    | Text shown in Display Mode when content is null/empty |
| `onSave`      | `(value: string) => Promise<void>` | Yes                   | Called with Markdown string on blur/save              |

**Internal state machine**:

```
DisplayMode (idle)
  │  ← user clicks (editable = true)
  ▼
EditorMode (active)
  │  ← user clicks outside (blur event)
  ▼
Saving (transient)
  │  ← save succeeds
  ├──► DisplayMode (content updated)
  │  ← save fails
  └──► EditorMode (error shown, content preserved)
```

### RichTextToolbar (new Svelte component)

Formatting controls rendered inside `RichTextEditor` when in Editor Mode.

| Prop     | Type             | Required | Notes                         |
| -------- | ---------------- | -------- | ----------------------------- |
| `editor` | `Editor \| null` | Yes      | Active Tiptap editor instance |

**Controls**:
| Button | Tiptap Command | Keyboard Shortcut | Active Check |
|--------|---------------|-------------------|--------------|
| Bold | `toggleBold` | `Ctrl+B` | `editor.isActive('bold')` |
| Italic | `toggleItalic` | `Ctrl+I` | `editor.isActive('italic')` |
| Bullet List | `toggleBulletList` | `Ctrl+Shift+8` | `editor.isActive('bulletList')` |
| Ordered List | `toggleOrderedList` | `Ctrl+Shift+7` | `editor.isActive('orderedList')` |

---

## Key Validations

| Rule                                              | Where enforced                | Error handling                                            |
| ------------------------------------------------- | ----------------------------- | --------------------------------------------------------- |
| Empty `details` value is allowed (stored as NULL) | Backend (existing)            | No error; editor returns to Display Mode with placeholder |
| `details` with content MUST be valid Markdown     | `@tiptap/markdown` (frontend) | Extension guarantees valid output                         |
| `description` field (header) must be non-empty    | Backend (existing)            | Out of scope — not touched by this feature                |

---

## Persistence Flow

```
User types/formats content in Tiptap editor
         │
         │ blur event fires
         ▼
editor.storage.markdown.getMarkdown()
         │
         │ Markdown string
         ▼
commands.updateRailwayModelText({
  railwayModelId: model.id,
  field: 'Details',
  value: markdownString     // empty string → stored as NULL
})
         │
         ├── OK → local state updated, DisplayMode restored
         └── Err → error toast shown, EditorMode retained, content preserved
```

---

## Display Mode Rendering Flow

```
model.details (Markdown string | null)
         │
         │ null/empty?
         ├── Yes → show placeholder text
         │
         │ has content?
         └── No → marked.parse(markdownString)
                        │
                        │ HTML string
                        ▼
              <div class="prose prose-invert max-w-none"
                   {@html html}></div>
```

---

## State Synchronisation

The `RichTextEditor` component uses optimistic local state (mirroring the `InPlaceEdit` pattern):

```typescript
let localValue = $state(props.value ?? '');

// Sync when prop changes from parent (e.g., model reloaded)
$effect(() => {
  if (!isEditing) {
    localValue = props.value ?? '';
  }
});
```

The Tiptap editor is mounted **only when `isEditing` is true** (lazy mount) to conserve memory when viewing many model cards.
