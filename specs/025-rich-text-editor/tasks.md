# Tasks: Rich Text Editor for RailwayModelCard

**Input**: Design documents from `/specs/025-rich-text-editor/`
**Prerequisites**: plan.md ✓, spec.md ✓, research.md ✓, data-model.md ✓, contracts/ ✓, quickstart.md ✓

**Organization**: Tasks are grouped by user story so each story can be implemented, tested, and delivered independently.

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Install new dependencies and activate the typography plugin. Must complete before any component work can begin.

- [ ] T001 Install new frontend packages via `pnpm add @tiptap/core @tiptap/pm @tiptap/starter-kit @tiptap/markdown marked` — updates `package.json` and `pnpm-lock.yaml`
- [ ] T002 Activate `@tailwindcss/typography` plugin and add prose color token overrides in `src/routes/layout.css` — add `@plugin "@tailwindcss/typography";` after `@import 'tw-animate-css';`, then add `.prose { --tw-prose-body: var(--foreground); --tw-prose-headings: var(--foreground); --tw-prose-links: var(--primary); --tw-prose-bold: var(--foreground); --tw-prose-bullets: var(--primary); --tw-prose-counters: var(--muted-foreground); --tw-prose-hr: var(--border); --tw-prose-quote-borders: var(--primary); --tw-prose-code: var(--foreground); --tw-prose-pre-bg: var(--card); --tw-prose-th-borders: var(--border); --tw-prose-td-borders: var(--border); max-width: none; }` (see research.md Decision 4 for full token list)

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Shared i18n strings required by all user story components. Must complete before US1–US4.

**⚠️ CRITICAL**: No user story component work can begin until this phase is complete.

- [ ] T003 Add Paraglide message keys to `messages/en.json` — add `"details_placeholder": "Add maintenance notes, DCC addresses, or other details…"` and `"details_save_failed": "Failed to save details. Please try again."` — then run `pnpm prepare` to recompile the Paraglide output in `src/lib/paraglide/`

**Checkpoint**: Foundation ready — user story implementation can begin.

---

## Phase 3: User Story 1 — View and Edit Model Details in Rich Text (Priority: P1) 🎯 MVP

**Goal**: A collector can click the Details area to enter Editor Mode, type or edit content, click outside to auto-save, and see the formatted result in Display Mode. Error on save keeps the editor open with content preserved.

**Independent Test**: Load a model card with existing `details` text. Click the text area → editor opens. Type new content → click outside → verify auto-save and correct Markdown rendering in Display Mode. Load a model with null `details` → verify placeholder. Verify read-only mode (editable=false) shows no hover, no click-to-edit.

### Implementation for User Story 1

- [ ] T004 [US1] Create `src/lib/components/RichTextEditor.svelte` with Display Mode only — accept props `value: string | null`, `editable?: boolean`, `placeholder?: string`, `onSave: (value: string) => Promise<void>`; render `<div class="prose prose-invert max-w-none">` with `{@html marked.parse(localValue)}` when value is non-empty; render italic muted placeholder `<p>` when value is null/empty; add hover ring when `editable && !isEditing` (see quickstart.md Step 4 for full component skeleton); `isEditing` state remains `false` throughout this task — Editor Mode is scaffolded next
- [ ] T005 [US1] Extend `src/lib/components/RichTextEditor.svelte` with Editor Mode and Tiptap integration — add `isEditing = $state(false)`, `isDirty = $state(false)`, `editor = $state<Editor | null>(null)`, `editorElement` binding; implement `startEditing()` onclick handler (guard: `!editable || isEditing`); add `$effect` that mounts `new Editor({ element: editorElement, extensions: [StarterKit, Markdown.configure({ html: false, tightLists: true, bulletListMarker: '-', transformPastedText: true, transformCopiedText: true })], content: localValue, onTransaction: () => { editor = instance; isDirty = true; }, onBlur: handleBlur })` when `isEditing` becomes true, and calls `editor.destroy()` when `isEditing` becomes false; implement `handleBlur()` that skips save when `!isDirty`, calls `onSave(editor.storage.markdown.getMarkdown())`, updates `localValue` on success, keeps editor open and surfaces error on failure; add `onDestroy(() => editor?.destroy())` (see quickstart.md Step 4 and research.md Decision 1 for the `onTransaction` reactivity pattern)
- [ ] T006 [US1] Integrate `RichTextEditor` into `src/lib/components/RailwayModelCard.svelte` Details tab — import `RichTextEditor` at top of script block; locate the Details tab content section (approx. line 408–424) and replace the `InPlaceEdit` component used for `localDetails` with `<RichTextEditor value={localDetails} editable={editable} placeholder={m.details_placeholder()} onSave={saveDetails} />`; remove now-unused InPlaceEdit import if no longer needed elsewhere in the file
- [ ] T007 [US1] Write Vitest unit tests for `RichTextEditor` in `src/__tests__/components/RichTextEditor.test.ts` — cover: Display Mode renders Markdown HTML from `value` prop; placeholder `<p>` shown when `value` is null; no hover class when `editable` is false; click does nothing when `editable` is false; click sets `isEditing` true when `editable` is true; `onSave` called with correct Markdown string on blur after edit; `onSave` NOT called on blur when no changes made (`isDirty` stays false); `onSave` rejection keeps editor mounted and does not update `localValue`; after successful save of empty string, placeholder re-appears

**Checkpoint**: US1 fully functional — open a model card, click the Details area, type, click outside, verify save and rendering. Read-only mode shows no interactive affordance.

---

## Phase 4: User Story 2 — Apply Rich Formatting (Priority: P2)

**Goal**: A collector can apply Bold, Italic, Bullet List, and Ordered List formatting using toolbar buttons while in Editor Mode. The stored value contains correct Markdown syntax. Toolbar buttons reflect active state.

**Independent Test**: Open Editor Mode on an existing model. Select text → click Bold → verify `**text**` in stored Markdown. Click Italic → verify `*text*`. Insert Bullet List → verify `- item`. Insert Ordered List → verify `1. item`. Reload the card → verify all formatting preserved in Display Mode.

### Implementation for User Story 2

- [ ] T008 [P] [US2] Create `src/lib/components/RichTextToolbar.svelte` — accept `editor: Editor | null` prop; declare `$derived` active states: `isBold`, `isItalic`, `isBullet`, `isOrdered` using `editor?.isActive(...)` (see quickstart.md Step 3 for full component code); render a horizontal flex bar with shadcn `Button` (variant `ghost`/`secondary`) and lucide-svelte icons `Bold`, `Italic`, `List`, `ListOrdered`; `onclick` handlers call `editor?.chain().focus().toggleBold().run()` etc.; add a 1px vertical divider between emphasis buttons and list buttons; use `h-7 w-7` icon buttons to keep the toolbar compact
- [ ] T009 [US2] Import and render `RichTextToolbar` inside the Editor Mode branch of `src/lib/components/RichTextEditor.svelte` — add `import RichTextToolbar from './RichTextToolbar.svelte';` to script block; in the `{#if isEditing}` branch, render `<RichTextToolbar {editor} />` above the `<div bind:this={editorElement}>` inside a wrapping `<div class="rounded-md ring-1 ring-primary/40">` (see quickstart.md Step 4 for the full Editor Mode template)
- [ ] T010 [US2] Write Vitest unit tests for `RichTextToolbar` in `src/__tests__/components/RichTextToolbar.test.ts` — cover: all four buttons render; Bold button has `secondary` variant class when `editor.isActive('bold')` returns true, `ghost` when false; Italic, BulletList, OrderedList buttons follow same pattern; clicking Bold calls `editor.chain().focus().toggleBold().run()`; buttons are accessible (aria-label present)

**Checkpoint**: US1 + US2 both work — formatting toolbar visible in Editor Mode, active states update correctly, Markdown round-trips.

---

## Phase 5: User Story 3 — Plain Text Paste (Priority: P3)

**Goal**: Pasting HTML-rich content from a manufacturer website inserts clean text and basic structure (no HTML tags, no inline styles). Plain text paste is unmodified.

**Independent Test**: Copy styled HTML text from a website (or simulate a paste event with `text/html` clipboard data). Paste into Editor Mode. Save. Verify the stored Markdown contains no `<` or `>` characters (no HTML tags). Verify plain text paste inserts as-is.

> **Note**: The implementation for US3 is fully provided by `Markdown.configure({ transformPastedText: true, html: false })` already configured in T005. This phase is pure test coverage and configuration verification.

### Implementation for User Story 3

- [ ] T011 [US3] Add paste normalisation tests to `src/__tests__/components/RichTextEditor.test.ts` — simulate a paste event with `text/html` clipboard content containing `<strong>BR Standard</strong> Class 4MT <span style="color:red">limited edition</span>` and verify: the Tiptap editor stores `**BR Standard** Class 4MT limited edition` (or equivalent clean Markdown); simulate a plain text paste and verify content is inserted as-is without modification

**Checkpoint**: US3 complete — paste from external HTML sources produces clean Markdown output.

---

## Phase 6: User Story 4 — Placeholder Text (Priority: P4)

**Goal**: New model cards with no details show a helpful placeholder. Placeholder disappears in Editor Mode and reappears if the user saves without entering content.

**Independent Test**: Render `RichTextEditor` with `value={null}`. Verify placeholder text is visible in Display Mode. Click to enter Editor Mode — verify placeholder is absent (empty editable area). Save without typing. Verify placeholder reappears in Display Mode.

> **Note**: Placeholder rendering is implemented in T004 (Display Mode branch). This phase adds dedicated test coverage for the placeholder lifecycle edge cases.

### Implementation for User Story 4

- [ ] T012 [US4] Add placeholder lifecycle tests to `src/__tests__/components/RichTextEditor.test.ts` — cover: placeholder `<p>` is rendered when `value` is null; placeholder is rendered when `value` is empty string; placeholder is absent when `value` has content; clicking the component when `editable=true` and value is null activates Editor Mode (no placeholder shown); after saving an empty string (clearing all content), Display Mode shows placeholder again

**Checkpoint**: All four user stories independently functional and tested.

---

## Phase 7: Polish & Cross-Cutting Concerns

**Purpose**: Code quality gates and manual acceptance validation across all user stories.

- [ ] T013 [P] Run `pnpm lint` and resolve any ESLint warnings across all new and modified files (`src/lib/components/RichTextEditor.svelte`, `src/lib/components/RichTextToolbar.svelte`, `src/lib/components/RailwayModelCard.svelte`, `src/routes/layout.css`, `src/__tests__/components/RichTextEditor.test.ts`, `src/__tests__/components/RichTextToolbar.test.ts`)
- [ ] T014 [P] Run `pnpm check` and resolve any TypeScript/Svelte type errors — ensure `Editor | null` is handled safely in all `?.` chains, all Tiptap imports are correctly typed, `marked.parse()` return is cast to `string`, `onSave` prop is typed as `(value: string) => Promise<void>`
- [ ] T015 Run `pnpm test` and ensure all new unit tests pass — fix any test failures before proceeding
- [ ] T016 Manual acceptance test walkthrough per `specs/025-rich-text-editor/quickstart.md` Step 7 — start `pnpm tauri dev`; verify US1 (click to edit, auto-save, Display Mode render); verify US2 (Bold/Italic/UL/OL toolbar); verify US3 (paste from a manufacturer website); verify US4 (placeholder on new/empty model card); verify editable=false shows no interactive affordance

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies — can start immediately
- **Foundational (Phase 2)**: Depends on Phase 1 — BLOCKS all component work
- **US1 (Phase 3)**: Depends on Phase 2 — BLOCKS US2 (toolbar wires into editor), US3 (config is set here), US4 (placeholder is set here)
- **US2 (Phase 4)**: Depends on US1 (T005 for Tiptap instance, T006 for card integration) — T008 can start in parallel with US1 implementation
- **US3 (Phase 5)**: Depends on T005 (paste config set there) — only test tasks remain, can run after T007
- **US4 (Phase 6)**: Depends on T004 (placeholder Display Mode set there) — only test tasks remain, can run after T007
- **Polish (Phase 7)**: Depends on all story phases complete

### User Story Dependencies

- **US1 (P1)**: Can start after Phase 2 — no dependencies on other stories
- **US2 (P2)**: T008 (`RichTextToolbar.svelte` creation) can start in **parallel** with US1 work (different file); T009 (wiring) depends on T005 (Tiptap instance exists)
- **US3 (P3)**: Test only — depends on T005 and T007 (editor created, initial tests written)
- **US4 (P4)**: Test only — depends on T004 and T007 (Display Mode created, initial tests written)

### Parallel Opportunities Within US1

T004 and T008 can run in parallel (different files):

- `RichTextEditor.svelte` Display Mode (T004)
- `RichTextToolbar.svelte` creation (T008, US2)

---

## Parallel Example: US1 + US2 Kickoff (after Phase 2)

```text
# After T003 completes:

Parallel track A (US1):
  T004 → T005 → T006 → T007

Parallel track B (US2, starts after T004):
  T008 → T009 → T010

# T009 depends on T005 (needs Tiptap in editor)
# T008 can start as soon as T004 is in progress (different file)
```

---

## Implementation Strategy

### MVP First (US1 Only)

1. Complete Phase 1: Setup (T001, T002)
2. Complete Phase 2: Foundational (T003)
3. Complete Phase 3: US1 (T004 → T005 → T006 → T007)
4. **STOP and VALIDATE**: Click Details tab — edit, save, verify Markdown round-trip
5. **Deliverable**: Working Display/Editor toggle with auto-save and error handling

### Incremental Delivery

1. Setup + Foundational → Foundation ready (T001–T003)
2. US1 → Display/Edit/Save loop works (T004–T007) — MVP
3. US2 → Formatting toolbar added (T008–T010)
4. US3 → Paste test coverage added (T011)
5. US4 → Placeholder test coverage added (T012)
6. Polish → All quality gates pass (T013–T016)

---

## Notes

- `[P]` tasks touch different files and have no blocking inter-dependencies
- `[US#]` label maps each task to its user story for traceability
- T008 (`RichTextToolbar.svelte`) is the only US2 task that can run in parallel with US1 work
- US3 and US4 implementation is subsumed into US1 (paste config in T005, placeholder in T004) — their phases are test-coverage phases only
- No Rust/backend changes in any phase
- No database migrations in any phase
- Commit after each completed task or logical group (T004 Display Mode, T005+T006 Editor Mode + integration, T007 tests, etc.)
