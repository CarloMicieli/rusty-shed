# Quickstart: Rich Text Editor for RailwayModelCard

**Feature**: 025-rich-text-editor
**Date**: 2026-02-20

---

## Prerequisites

- Branch `025-rich-text-editor` checked out (done)
- Node.js / pnpm available
- Rust toolchain available (though no Rust changes are needed for this feature)

---

## Step 1: Install New Frontend Dependencies

```bash
pnpm add @tiptap/core @tiptap/pm @tiptap/starter-kit @tiptap/markdown marked
```

Verify installation:

```bash
pnpm list @tiptap/core @tiptap/markdown marked
```

**Note**: `@tailwindcss/typography` is **already installed** (v0.5.19). Do NOT run `pnpm add` for it.

---

## Step 2: Activate the Typography Plugin

Add `@plugin "@tailwindcss/typography";` to [src/routes/layout.css](../../src/routes/layout.css), immediately after `@import 'tw-animate-css';`:

```css
@import 'tailwindcss';

@import 'tw-animate-css';

@plugin '@tailwindcss/typography'; /* ← add this line */
```

Then add custom prose color overrides below the existing `:root { }` block to integrate with the steampunk zinc/copper palette. See [research.md](research.md) for the full set of `--tw-prose-*` overrides.

---

## Step 3: Create the Formatting Toolbar Component

Create `src/lib/components/RichTextToolbar.svelte`:

```svelte
<script lang="ts">
  import type { Editor } from '@tiptap/core';
  import { Bold, Italic, List, ListOrdered } from 'lucide-svelte';
  import { Button } from '$lib/components/ui/button';

  let { editor }: { editor: Editor | null } = $props();

  let isBold = $derived(editor?.isActive('bold') ?? false);
  let isItalic = $derived(editor?.isActive('italic') ?? false);
  let isBullet = $derived(editor?.isActive('bulletList') ?? false);
  let isOrdered = $derived(editor?.isActive('orderedList') ?? false);
</script>

<div class="flex items-center gap-1 border-b border-border/40 px-2 py-1">
  <Button
    variant={isBold ? 'secondary' : 'ghost'}
    size="icon"
    class="h-7 w-7"
    onclick={() => editor?.chain().focus().toggleBold().run()}
    aria-label="Bold"
  >
    <Bold class="h-4 w-4" />
  </Button>

  <Button
    variant={isItalic ? 'secondary' : 'ghost'}
    size="icon"
    class="h-7 w-7"
    onclick={() => editor?.chain().focus().toggleItalic().run()}
    aria-label="Italic"
  >
    <Italic class="h-4 w-4" />
  </Button>

  <div class="mx-1 h-4 w-px bg-border/40"></div>

  <Button
    variant={isBullet ? 'secondary' : 'ghost'}
    size="icon"
    class="h-7 w-7"
    onclick={() => editor?.chain().focus().toggleBulletList().run()}
    aria-label="Bullet list"
  >
    <List class="h-4 w-4" />
  </Button>

  <Button
    variant={isOrdered ? 'secondary' : 'ghost'}
    size="icon"
    class="h-7 w-7"
    onclick={() => editor?.chain().focus().toggleOrderedList().run()}
    aria-label="Ordered list"
  >
    <ListOrdered class="h-4 w-4" />
  </Button>
</div>
```

---

## Step 4: Create the RichTextEditor Component

Create `src/lib/components/RichTextEditor.svelte`:

```svelte
<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { Editor } from '@tiptap/core';
  import StarterKit from '@tiptap/starter-kit';
  import { Markdown } from '@tiptap/markdown';
  import { marked } from 'marked';
  import RichTextToolbar from './RichTextToolbar.svelte';

  interface Props {
    value: string | null;
    editable?: boolean;
    placeholder?: string;
    onSave: (value: string) => Promise<void>;
  }

  let {
    value,
    editable = false,
    placeholder = 'Click to add details...',
    onSave,
  }: Props = $props();

  let isEditing = $state(false);
  let isSaving = $state(false);
  let localValue = $state(value ?? '');
  let editorElement: HTMLDivElement;
  let editor = $state<Editor | null>(null);
  let isDirty = $state(false);

  // Sync when external prop changes (model reload)
  $effect(() => {
    if (!isEditing) {
      localValue = value ?? '';
    }
  });

  const displayHtml = $derived(
    localValue ? (marked.parse(localValue) as string) : ''
  );

  async function startEditing() {
    if (!editable || isEditing) return;
    isEditing = true;
    isDirty = false;
  }

  async function handleBlur() {
    if (!isEditing || isSaving) return;
    if (!isDirty) {
      isEditing = false;
      return;
    }
    await save();
  }

  async function save() {
    if (!editor) return;
    isSaving = true;
    const markdown = editor.storage.markdown.getMarkdown();
    try {
      await onSave(markdown);
      localValue = markdown;
      isEditing = false;
      isDirty = false;
    } catch {
      // Keep editor open on error — onSave should surface the error toast
    } finally {
      isSaving = false;
    }
  }

  $effect(() => {
    if (isEditing && editorElement && !editor) {
      const instance = new Editor({
        element: editorElement,
        extensions: [
          StarterKit,
          Markdown.configure({
            html: false,
            tightLists: true,
            bulletListMarker: '-',
            transformPastedText: true,
            transformCopiedText: true,
          }),
        ],
        content: localValue,
        editorProps: {
          attributes: {
            class: 'prose prose-invert max-w-none focus:outline-none min-h-[4rem]',
          },
        },
        onTransaction: () => {
          editor = instance;
          isDirty = true;
        },
        onBlur: () => {
          handleBlur();
        },
      });
      editor = instance;
    }

    if (!isEditing && editor) {
      editor.destroy();
      editor = null;
    }
  });

  onDestroy(() => {
    editor?.destroy();
  });
</script>

<!-- svelte-ignore a11y_click_events_have_key_events -->
<!-- svelte-ignore a11y_no_static_element_interactions -->
<div
  class="relative rounded-md transition-colors"
  class:hover:bg-white/5={editable && !isEditing}
  class:hover:ring-1={editable && !isEditing}
  class:ring-border/40={editable && !isEditing}
  onclick={startEditing}
>
  {#if isEditing}
    <div class="rounded-md ring-1 ring-primary/40">
      <RichTextToolbar {editor} />
      <div class="p-3">
        <div bind:this={editorElement}></div>
      </div>
    </div>
  {:else if localValue}
    <div class="prose prose-invert max-w-none px-1 py-0.5 text-sm">
      {@html displayHtml}
    </div>
  {:else}
    <p class="px-1 py-0.5 text-sm text-muted-foreground italic">
      {placeholder}
    </p>
  {/if}
</div>
```

---

## Step 5: Integrate into RailwayModelCard

In [src/lib/components/RailwayModelCard.svelte](../../src/lib/components/RailwayModelCard.svelte), replace the `InPlaceEdit` used for the `details` field in the Details tab with `RichTextEditor`.

Find the details tab section (approximately line 408–424) and replace the `InPlaceEdit` with:

```svelte
<RichTextEditor
  value={localDetails}
  {editable}
  placeholder={m.details_placeholder()}
  onSave={saveDetails}
/>
```

Also add the import at the top of the `<script>` block:

```typescript
import RichTextEditor from '$lib/components/RichTextEditor.svelte';
```

The `saveDetails` function already exists and calls `commands.updateRailwayModelText`.

---

## Step 6: Add Paraglide Message Keys

Add the new i18n key to `messages/en.json`:

```json
{
  "details_placeholder": "Add maintenance notes, DCC addresses, or other details...",
  "details_save_failed": "Failed to save details. Please try again."
}
```

Then run the Paraglide compile step:

```bash
pnpm prepare
```

---

## Step 7: Verify

```bash
pnpm lint       # must pass
pnpm check      # must pass
pnpm test       # must pass
```

For manual testing:

1. Run `pnpm tauri dev`
2. Open a railway model card
3. Click the Details tab — verify placeholder is shown for empty models
4. Click the placeholder area — editor should open with toolbar
5. Type content, apply bold/italic/lists — verify Markdown syntax stored
6. Click outside the card — verify auto-save and Display Mode
7. Paste from a web page — verify HTML stripping

---

## File Change Summary

| File                                              | Change                                                           |
| ------------------------------------------------- | ---------------------------------------------------------------- |
| `src/routes/layout.css`                           | Add `@plugin "@tailwindcss/typography";` + prose color overrides |
| `src/lib/components/RichTextToolbar.svelte`       | **New** — formatting toolbar (Bold, Italic, UL, OL)              |
| `src/lib/components/RichTextEditor.svelte`        | **New** — Display/Editor mode wrapper with Tiptap                |
| `src/lib/components/RailwayModelCard.svelte`      | Replace Details `InPlaceEdit` with `RichTextEditor`              |
| `messages/en.json`                                | Add `details_placeholder`, `details_save_failed` keys            |
| `src/__tests__/components/RichTextEditor.test.ts` | **New** — unit tests                                             |
| `package.json`                                    | Add Tiptap + marked dependencies                                 |

**No Rust/backend changes.** No database migrations.
