<script lang="ts">
  import { untrack, onDestroy } from 'svelte';
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
    onSave
  }: Props = $props();

  let isEditing = $state(false);
  let isSaving = $state(false);
  // Capture initial value without creating a reactive dependency on the prop
  let localValue = $state(untrack(() => value ?? ''));
  let editorElement = $state<HTMLDivElement | undefined>(undefined);
  let editor = $state<Editor | null>(null);
  let isDirty = $state(false);

  // Sync when external prop changes (model reload) — but not on isEditing transitions
  $effect(() => {
    const newValue = value ?? ''; // tracked: effect reruns when `value` prop changes
    if (untrack(() => !isEditing)) {
      localValue = newValue;
    }
  });

  const displayHtml = $derived(localValue ? (marked.parse(localValue) as string) : '');

  function startEditing() {
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
    const markdown = editor.getMarkdown();
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
        extensions: [StarterKit, Markdown],
        content: localValue,
        contentType: 'markdown',
        editorProps: {
          attributes: {
            class: 'prose prose-invert max-w-none focus:outline-none min-h-[4rem]'
          }
        },
        onTransaction: () => {
          editor = instance;
          isDirty = true;
        },
        onBlur: () => {
          handleBlur();
        }
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
  class="relative rounded-md transition-colors {editable && !isEditing
    ? 'ring-border/40 hover:bg-white/5 hover:ring-1'
    : ''}"
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
    <div class="prose max-w-none px-1 py-0.5 text-sm prose-invert">
      <!-- eslint-disable-next-line svelte/no-at-html-tags -->
      {@html displayHtml}
    </div>
  {:else}
    <p class="px-1 py-0.5 text-sm text-muted-foreground italic">
      {placeholder}
    </p>
  {/if}
</div>
