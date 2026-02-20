# IPC Command Contracts: Rich Text Editor

**Feature**: 025-rich-text-editor
**Date**: 2026-02-20

---

## Summary

This feature requires **no new Tauri IPC commands**. The existing `update_railway_model_text` command handles the `Details` field correctly. All types are already generated in `src/lib/bindings.ts` via specta.

---

## Existing Command: `update_railway_model_text`

**Already registered in** `src-tauri/src/lib.rs`.
**Already typed in** `src/lib/bindings.ts`.

### TypeScript Signature

```typescript
async updateRailwayModelText(
  args: UpdateRailwayModelTextArgs
): Promise<Result<null, CommandError>>
```

### Args Type

```typescript
type UpdateRailwayModelTextArgs = {
  railwayModelId: RailwayModelId; // UUID string
  field: RailwayModelTextField; // "Description" | "Details"
  value: string;
};
```

### Field Behaviour

| `field`         | Empty string behaviour               | Valid values                   |
| --------------- | ------------------------------------ | ------------------------------ |
| `"Description"` | Rejected by domain (ValidationError) | Non-empty string               |
| `"Details"`     | Stored as NULL (acceptable)          | Any string, including Markdown |

### Usage in this feature

```typescript
// Save Markdown content from RichTextEditor
const markdownValue = editor.storage.markdown.getMarkdown();

const result = await commands.updateRailwayModelText({
  railwayModelId: model.id,
  field: 'Details',
  value: markdownValue // empty string stores NULL
});

if (result.status === 'error') {
  // Surface error, preserve editor state
  toaster.error(m.details_save_failed());
} else {
  localValue = markdownValue;
  isEditing = false;
}
```

### Error Responses

| Error variant                   | When it occurs                                           | Frontend handling                  |
| ------------------------------- | -------------------------------------------------------- | ---------------------------------- |
| `CommandError::NotFound`        | Model ID doesn't exist in DB                             | Show error toast, log              |
| `CommandError::ValidationError` | Empty string for Description (not applicable to Details) | N/A for this feature               |
| `CommandError::DatabaseError`   | SQLite write failure                                     | Show error toast, keep editor open |

---

## Read Path (no change)

The `details` field is already returned by the existing model fetch commands:

```typescript
// RailwayModelView includes:
type RailwayModelView = {
  id: RailwayModelId;
  description: string;
  details: string | null; // ← consumed by RichTextEditor as `value` prop
  // ...other fields
};
```

The `RailwayModelCard` already receives the full model and passes `model.details` down. No new query command is needed.
