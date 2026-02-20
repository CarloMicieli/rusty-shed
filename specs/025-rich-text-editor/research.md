# Research: Rich Text Editor for RailwayModelCard

**Feature**: 025-rich-text-editor
**Date**: 2026-02-20
**Status**: Complete — all unknowns resolved

---

## Decision 1: Svelte 5 + Tiptap Integration Strategy

**Decision**: Use Tiptap directly (no wrapper package) by instantiating `new Editor()` inside Svelte's `onMount`, bound to a `div` element, with `$state` for reactivity.

**Rationale**:

- `svelte-tiptap` (the community wrapper) targets Svelte 4 only. Its v3.x peer dependency is `"svelte": "^4.0.0"` — it explicitly excludes Svelte 5.
- There is no official `@tiptap/svelte` package from ueberdosis.
- Direct instantiation using the `element` option on `new Editor()` is the framework-agnostic approach documented by Tiptap and confirmed as the correct Svelte 5 pattern by community examples.

**Svelte 5 Reactivity Pattern**:
Tiptap's `Editor` instance is a mutable class — Svelte 5's `$state` does not deep-track mutations on arbitrary class instances. The standard workaround is to reassign the `editor` `$state` variable inside `onTransaction` to force reactive re-evaluations:

```typescript
let editor = $state<Editor | null>(null);
onMount(() => {
  const instance = new Editor({
    element: editorElement,
    extensions: [...],
    onTransaction: () => {
      editor = instance; // triggers $state reactivity for toolbar button states
    },
  });
  editor = instance;
});
```

Use `$derived` to memoize toolbar active-state checks to avoid per-keystroke overhead:

```typescript
let isBold = $derived(editor?.isActive('bold') ?? false);
let isItalic = $derived(editor?.isActive('italic') ?? false);
let isBullet = $derived(editor?.isActive('bulletList') ?? false);
let isOrdered = $derived(editor?.isActive('orderedList') ?? false);
```

**Alternatives Considered**:

- `svelte-tiptap` — rejected (Svelte 4 only)
- Official `@tiptap/svelte` — does not exist

---

## Decision 2: Markdown Extension

**Decision**: Use `@tiptap/markdown` (the official Tiptap Markdown extension) for bidirectional Markdown ↔ editor content conversion.

**Package name**: `@tiptap/markdown` (NOT `@tiptap/extension-markdown` — that package does not exist).

**API surface**:
| Operation | API |
|-----------|-----|
| Get Markdown from editor | `editor.storage.markdown.getMarkdown()` |
| Initialize editor with Markdown string | Pass Markdown string as the `content` option |
| Set content at runtime | `editor.commands.setContent('# Heading\n\n...')` |
| Strip HTML on paste | `Markdown.configure({ transformPastedText: true })` |

**Recommended configuration** for this project:

```typescript
Markdown.configure({
  html: false, // strip raw HTML — keeps database clean
  tightLists: true, // no <p> inside <li>
  bulletListMarker: '-', // canonical Markdown style
  transformPastedText: true, // pastes from websites converted to Markdown
  transformCopiedText: true // copied text leaves editor as Markdown
});
```

**Supported formatting** (all in scope for this feature):

- Bold: `**text**` ✓
- Italic: `*text*` ✓
- Bullet list: `- item` ✓
- Ordered list: `1. item` ✓
- Headings: supported but **out of scope** per spec assumptions

**Known limitations**:

- Custom node types require manually registered serializers — not relevant here (StarterKit covers all needed types).
- Tables not supported by default — out of scope per spec.
- `tightLists: true` is required to prevent verbose `<p>`-wrapped list items in Markdown output.

**Alternatives Considered**:

- Manual Markdown serialization — rejected (fragile, maintenance burden)
- Custom ProseMirror schema — rejected (unnecessary complexity)

---

## Decision 3: Display Mode (Read-Only) Renderer

**Decision**: Use `marked` for rendering Markdown to HTML in Display Mode. Do NOT use a Tiptap editor instance for the read-only view.

**Rationale**:

- Creating a headless `Editor` instance just to render HTML wastes memory per card — unacceptable when browsing a large collection.
- `marked` is a lightweight, battle-tested Markdown-to-HTML library with zero dependencies and an extremely small bundle footprint.
- The existing project already has `@tailwindcss/typography` (v0.5.19) installed, which provides the `prose` class for beautiful rendered Markdown styling.
- The `@tiptap/markdown` extension has no static renderer API — `editor.storage.markdown.getMarkdown()` is only accessible on a live editor instance.

**Usage pattern**:

```svelte
<script lang="ts">
  import { marked } from 'marked';
  let { content }: { content: string } = $props();
  const html = $derived(marked.parse(content) as string);
</script>
<div class="prose prose-invert max-w-none" {@html html}></div>
```

**XSS risk**: In a Tauri desktop app, all content originates from the user's own SQLite database. `DOMPurify` is unnecessary overhead for this use case, but can be added later if content ever originates from external/untrusted sources.

**Alternatives Considered**:

- Headless Tiptap `Editor` for rendering — rejected (memory cost, instantiation latency)
- `@tiptap/core` `generateHTML` — rejected (same problem, requires parsed ProseMirror JSON)
- `micromark` — considered, but `marked` has better TypeScript support and broader ecosystem familiarity

---

## Decision 4: Typography Plugin Integration

**Decision**: `@tailwindcss/typography` v0.5.19 is **already installed** in the project. No `pnpm add` needed. The single change required is registering it in `layout.css` with the `@plugin` directive.

**Tailwind v4 registration** (CSS-first, no `tailwind.config.js` needed):

```css
/* In src/routes/layout.css — add after @import 'tw-animate-css'; */
@plugin '@tailwindcss/typography';
```

The `@tailwindcss/vite` plugin (already wired up in this project) automatically discovers and applies `@plugin` directives.

**Custom prose tokens** for the steampunk dark theme (zinc/copper palette):

```css
/* Override prose colors to match the existing design tokens */
.prose {
  --tw-prose-body: var(--foreground);
  --tw-prose-headings: var(--foreground);
  --tw-prose-lead: var(--muted-foreground);
  --tw-prose-links: var(--primary);
  --tw-prose-bold: var(--foreground);
  --tw-prose-counters: var(--muted-foreground);
  --tw-prose-bullets: var(--primary); /* copper/amber bullets */
  --tw-prose-hr: var(--border);
  --tw-prose-quotes: var(--muted-foreground);
  --tw-prose-quote-borders: var(--primary);
  --tw-prose-code: var(--foreground);
  --tw-prose-pre-code: var(--muted-foreground);
  --tw-prose-pre-bg: var(--card);
  --tw-prose-th-borders: var(--border);
  --tw-prose-td-borders: var(--border);
}
```

The `prose-invert` modifier also needs invert tokens but since the app is dark-only, `prose-invert` is applied unconditionally in the component via `prose dark:prose-invert max-w-none` (the `dark:` class is always active).

---

## Decision 5: Backend/IPC Impact

**Decision**: No backend changes required. The existing `update_railway_model_text` Tauri command already handles the `Details` field correctly.

**Existing IPC command** (already registered in `lib.rs`):

```typescript
// From src/lib/bindings.ts
async updateRailwayModelText(args: UpdateRailwayModelTextArgs): Promise<Result<null, CommandError>>

type UpdateRailwayModelTextArgs = {
  railwayModelId: RailwayModelId;
  field: RailwayModelTextField;  // "Description" | "Details"
  value: string;
}
```

The `Details` variant accepts an empty string (stored as NULL) — this correctly handles the "save empty → show placeholder" scenario from US4.

**No schema migration needed**: the `details` column is already a nullable TEXT column. Markdown is valid text.

---

## Decision 6: New Frontend Packages Required

| Package               | Purpose                                                      | Version |
| --------------------- | ------------------------------------------------------------ | ------- |
| `@tiptap/core`        | Tiptap editor core engine                                    | `^2.x`  |
| `@tiptap/pm`          | ProseMirror peer re-export shim                              | `^2.x`  |
| `@tiptap/starter-kit` | Bundles Bold, Italic, BulletList, OrderedList, History, etc. | `^2.x`  |
| `@tiptap/markdown`    | Bidirectional Markdown ↔ editor content                      | `^2.x`  |
| `marked`              | Lightweight Markdown→HTML renderer for Display Mode          | `^15.x` |

**Install command**:

```bash
pnpm add @tiptap/core @tiptap/pm @tiptap/starter-kit @tiptap/markdown marked
```

**No new backend (Rust/Tauri) dependencies required.**

---

## Summary of Resolved Unknowns

| Unknown                                         | Resolution                                                       |
| ----------------------------------------------- | ---------------------------------------------------------------- |
| Which Svelte 5 + Tiptap integration strategy?   | Direct `new Editor()` in `onMount` — no wrapper package          |
| Which Markdown package?                         | `@tiptap/markdown` — `editor.storage.markdown.getMarkdown()`     |
| How to handle Svelte 5 `$state` reactivity?     | `onTransaction: () => { editor = instance; }` trick + `$derived` |
| How to render Markdown in Display Mode?         | `marked` library with `prose prose-invert max-w-none` classes    |
| Tailwind typography plugin — already installed? | **Yes** (v0.5.19) — only needs `@plugin` directive in layout.css |
| Does backend need changes?                      | **No** — existing `update_railway_model_text` handles `Details`  |
| Does schema need migration?                     | **No** — `details` column is already nullable TEXT               |
