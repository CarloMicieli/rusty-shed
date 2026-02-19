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

  interface InPlaceEditProps {
    /** Current value displayed and edited. */
    value: string;
    /** Placeholder shown when value is empty. */
    placeholder?: string;
    /** Whether to render a <textarea> instead of an <input>. */
    multiline?: boolean;
    /** Called when the user commits the new value (blur or Save click). */
    onSave: (value: string) => Promise<void>;
  }

  let { value, placeholder, multiline = false, onSave }: InPlaceEditProps = $props();

  let isEditing = $state(false);
  // Initialized to empty; always assigned from `value` prop inside startEditing()
  let editValue = $state('');
  let isSaving = $state(false);
  let error = $state<string | null>(null);

  /**
   * Set to true from onmousedown on pill buttons to prevent the blur
   * handler from triggering a duplicate save when a pill button is clicked.
   */
  let suppressBlurSave = false;

  let inputEl = $state<HTMLInputElement | HTMLTextAreaElement | null>(null);

  $effect(() => {
    if (isEditing && inputEl) {
      inputEl.focus();
    }
  });

  function startEditing() {
    editValue = value;
    error = null;
    isEditing = true;
  }

  function cancel() {
    editValue = value;
    error = null;
    isEditing = false;
  }

  async function save() {
    if (isSaving) return;
    isSaving = true;
    error = null;
    try {
      await onSave(editValue);
      isEditing = false;
    } catch {
      error = m.edit_save_error();
    } finally {
      isSaving = false;
    }
  }

  async function handleBlur() {
    if (suppressBlurSave) {
      suppressBlurSave = false;
      return;
    }
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
        class="w-full resize-none rounded border border-[#D48A42] bg-[#0F0F0F] p-1 text-sm text-[#E0E0E0] ring-1 ring-[#D48A42]/30 outline-none"
        bind:value={editValue}
        onblur={handleBlur}
        onkeydown={handleKeydown}
        disabled={isSaving}
        rows={3}
      ></textarea>
    {:else}
      <input
        bind:this={inputEl}
        type="text"
        class="w-full rounded border border-[#D48A42] bg-[#0F0F0F] p-1 text-sm text-[#E0E0E0] ring-1 ring-[#D48A42]/30 outline-none"
        bind:value={editValue}
        onblur={handleBlur}
        onkeydown={handleKeydown}
        disabled={isSaving}
      />
    {/if}

    <!-- Floating Save/Cancel pill — absolute positioned to avoid card layout shift -->
    <div
      class="absolute top-full left-0 z-50 mt-1 flex gap-1 rounded border border-[#1F1F1F] bg-[#0F0F0F] px-2 py-1 shadow-lg"
    >
      <button
        type="button"
        class="h-6 rounded bg-[#D48A42] px-2 text-xs font-medium text-black hover:bg-[#D48A42]/90 disabled:opacity-50"
        onmousedown={() => {
          suppressBlurSave = true;
        }}
        onclick={() => void save()}
        disabled={isSaving}
      >
        {m.edit_field_save()}
      </button>
      <button
        type="button"
        class="h-6 rounded px-2 text-xs text-[#E0E0E0] hover:bg-white/10 disabled:opacity-50"
        onmousedown={() => {
          suppressBlurSave = true;
        }}
        onclick={cancel}
        disabled={isSaving}
      >
        {m.edit_field_cancel()}
      </button>
    </div>

    {#if error}
      <p class="mt-7 text-xs text-red-400" role="alert">{error}</p>
    {/if}
  </div>
{:else}
  <div
    class="group relative -mx-1 cursor-pointer rounded p-1 transition-colors duration-150 hover:border hover:border-dashed hover:border-[#D48A42]/40 hover:bg-[rgba(212,138,66,0.15)]"
    onclick={startEditing}
    onkeydown={(e) => {
      if (e.key === 'Enter' || e.key === ' ') startEditing();
    }}
    role="button"
    tabindex="0"
  >
    {#if value}
      <span class="text-sm text-[#E0E0E0]">{value}</span>
    {:else}
      <span class="text-sm text-[#808080] italic"
        >{placeholder ?? m.edit_field_placeholder_empty()}</span
      >
    {/if}
  </div>
{/if}
