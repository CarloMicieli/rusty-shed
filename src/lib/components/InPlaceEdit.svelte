<script lang="ts">
  /**
   * InPlaceEdit Component
   *
   * A click-to-edit primitive that allows inline editing of text fields.
   * Supports four visual states: idle, hover affordance, active edit, and
   * a floating Save/Cancel pill.
   *
   * @component
   * @example
   * ```svelte
   * <InPlaceEdit
   *   value={model.description}
   *   multiline={true}
   *   onSave={async (v) => { await commands.updateRailwayModelText({...}); }}
   * />
   * ```
   */
  import * as m from '$lib/paraglide/messages';
  import { createMobileMatchMediaState } from '$lib/state/match-media.svelte';
  import { onMount } from 'svelte';

  interface InPlaceEditProps {
    /** Current value displayed and edited. */
    value: string;
    /** Override the text shown in view mode (e.g. a formatted date). Falls back to value. */
    displayValue?: string;
    /** Placeholder shown when value is empty. */
    placeholder?: string;
    /** Whether to render a <textarea> instead of an <input>. */
    multiline?: boolean;
    /** Input type — 'text' (default) or 'date'. */
    type?: 'text' | 'date';
    /** Called when the user commits the new value (blur or Save click). */
    onSave: (value: string) => Promise<void>;
    /** Called when the user activates edit mode (for cross-card coordination). */
    onActivate?: () => void;
    /** Called when the user exits edit mode (for cross-card coordination). */
    onDeactivate?: () => void;
    /** Disable inline editing for mobile and delegate to sheet flow. */
    disableInlineOnMobile?: boolean;
    /** Optional callback when mobile edit is requested while inline is disabled. */
    onRequestMobileEdit?: () => void;
  }

  let {
    value,
    displayValue,
    placeholder,
    multiline = false,
    type = 'text',
    onSave,
    onActivate,
    onDeactivate,
    disableInlineOnMobile = false,
    onRequestMobileEdit
  }: InPlaceEditProps = $props();

  let isEditing = $state(false);
  // Initialized to empty; always assigned from `value` prop inside startEditing()
  let editValue = $state('');
  let isSaving = $state(false);
  let error = $state<string | null>(null);
  let isMobileViewport = $state(false);

  let inputEl = $state<HTMLInputElement | HTMLTextAreaElement | null>(null);

  const mobileMedia = createMobileMatchMediaState();

  $effect(() => {
    const unsubscribe = mobileMedia.subscribe((matches) => {
      isMobileViewport = matches;
    });

    return () => {
      unsubscribe();
    };
  });

  onMount(() => {
    return () => {
      mobileMedia.destroy();
    };
  });

  $effect(() => {
    if (isEditing && inputEl) {
      inputEl.focus();
    }
  });

  function startEditing() {
    if (disableInlineOnMobile && isMobileViewport) {
      onRequestMobileEdit?.();
      return;
    }

    editValue = value;
    error = null;
    isEditing = true;
    onActivate?.();
  }

  function cancel() {
    editValue = value;
    error = null;
    isEditing = false;
    onDeactivate?.();
  }

  async function save() {
    if (isSaving) return;
    isSaving = true;
    error = null;
    try {
      await onSave(editValue);
      isEditing = false;
      onDeactivate?.();
    } catch {
      error = m.edit_save_error();
    } finally {
      isSaving = false;
    }
  }

  async function handleBlur() {
    await save();
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === 'Escape') {
      e.preventDefault();
      cancel();
    } else if (!multiline && e.key === 'Enter') {
      e.preventDefault();
      void save();
    }
  }
</script>

{#if isEditing}
  <div class="relative">
    {#if multiline}
      <textarea
        bind:this={inputEl}
        class="w-full resize-none rounded border border-primary bg-card p-1 text-sm text-foreground ring-1 ring-primary/30 outline-none"
        bind:value={editValue}
        onblur={handleBlur}
        onkeydown={handleKeydown}
        disabled={isSaving}
        rows={3}
      ></textarea>
    {:else}
      <input
        bind:this={inputEl}
        {type}
        class="w-full rounded border border-primary bg-card p-1 text-sm text-foreground ring-1 ring-primary/30 outline-none"
        bind:value={editValue}
        onblur={handleBlur}
        onkeydown={handleKeydown}
        disabled={isSaving}
      />
    {/if}

    {#if isSaving}
      <span
        class="absolute top-1/2 right-1.5 inline-block h-3 w-3 -translate-y-1/2 animate-spin rounded-full border-2 border-primary border-t-transparent"
      ></span>
    {/if}
    {#if error}
      <p class="mt-1 text-xs text-red-400" role="alert">{error}</p>
    {/if}
  </div>
{:else}
  <button
    type="button"
    class="group relative -mx-1 w-full cursor-pointer rounded border-0 bg-transparent p-1 text-left transition-colors duration-150 hover:border hover:border-dashed hover:border-primary/40 hover:bg-primary/15 focus-visible:outline-none"
    onclick={startEditing}
  >
    {#if value}
      <span class="text-sm text-foreground">{displayValue ?? value}</span>
    {:else}
      <span class="text-sm text-muted-foreground italic"
        >{placeholder ?? m.edit_field_placeholder_empty()}</span
      >
    {/if}
  </button>
{/if}
