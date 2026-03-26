<script lang="ts">
  /**
   * InPlaceBooleanEdit Component
   *
   * A click-to-edit primitive for three-state FeatureFlag fields (YES / NO / null).
   * In view mode shows a compact chip; clicking opens a 3-button inline picker.
   * Saves immediately on selection; Escape cancels without saving.
   *
   * @component
   * @example
   * ```svelte
   * <InPlaceBooleanEdit
   *   value={localFlywheelFitted}
   *   onSave={async (v) => { ... }}
   * />
   * ```
   */
  import * as m from '$lib/paraglide/messages';

  interface InPlaceBooleanEditProps {
    /** Current value: 'YES', 'NO', or null (not applicable / unknown). */
    value: 'YES' | 'NO' | null;
    /** Placeholder shown when value is null in view mode. */
    placeholder?: string;
    /** Called with 'YES', 'NO', or null when the user picks an option. */
    onSave: (value: 'YES' | 'NO' | null) => Promise<void>;
    /** Called when the user activates edit mode (for cross-card coordination). */
    onActivate?: () => void;
    /** Called when the user exits edit mode (for cross-card coordination). */
    onDeactivate?: () => void;
  }

  let {
    value,
    placeholder = '—',
    onSave,
    onActivate,
    onDeactivate
  }: InPlaceBooleanEditProps = $props();

  let isEditing = $state(false);
  let isSaving = $state(false);
  let error = $state<string | null>(null);

  function startEditing() {
    if (isSaving) return;
    error = null;
    isEditing = true;
    onActivate?.();
  }

  function cancel() {
    if (isSaving) return;
    error = null;
    isEditing = false;
    onDeactivate?.();
  }

  async function pick(v: 'YES' | 'NO' | null) {
    isSaving = true;
    error = null;
    try {
      await onSave(v);
      isEditing = false;
      onDeactivate?.();
    } catch {
      error = m.edit_save_error();
    } finally {
      isSaving = false;
    }
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === 'Escape') {
      e.preventDefault();
      cancel();
    }
  }
</script>

{#if isEditing}
  <div class="relative">
    <div class="flex items-center gap-1">
      <button
        type="button"
        disabled={isSaving}
        onclick={() => pick(null)}
        onkeydown={handleKeydown}
        class="rounded px-2 py-0.5 text-xs font-medium transition-colors disabled:opacity-50
               {value === null
          ? 'bg-zinc-600 text-zinc-200'
          : 'bg-zinc-800 text-zinc-400 hover:bg-zinc-700'}"
      >
        —
      </button>
      <button
        type="button"
        disabled={isSaving}
        onclick={() => pick('YES')}
        onkeydown={handleKeydown}
        class="rounded px-2 py-0.5 text-xs font-medium transition-colors disabled:opacity-50
               {value === 'YES'
          ? 'bg-emerald-700 text-emerald-100'
          : 'bg-zinc-800 text-zinc-400 hover:bg-zinc-700'}"
      >
        Yes
      </button>
      <button
        type="button"
        disabled={isSaving}
        onclick={() => pick('NO')}
        onkeydown={handleKeydown}
        class="rounded px-2 py-0.5 text-xs font-medium transition-colors disabled:opacity-50
               {value === 'NO'
          ? 'bg-zinc-600 text-zinc-200'
          : 'bg-zinc-800 text-zinc-400 hover:bg-zinc-700'}"
      >
        No
      </button>
      {#if isSaving}
        <span
          class="ml-1 inline-block h-3 w-3 animate-spin self-center rounded-full border-2 border-primary border-t-transparent"
        ></span>
      {/if}
    </div>
    {#if error}
      <p class="mt-1 text-xs text-red-400" role="alert">{error}</p>
    {/if}
  </div>
{:else}
  <div
    class="group relative -mx-1 cursor-pointer rounded p-1 transition-colors duration-150 hover:border hover:border-dashed hover:border-primary/40 hover:bg-primary/15 focus-visible:outline-none"
    onclick={startEditing}
    onkeydown={(e) => {
      if (e.key === 'Enter' || e.key === ' ') startEditing();
    }}
    role="button"
    tabindex="0"
  >
    {#if value === 'YES'}
      <div class="flex items-center gap-2">
        <div class="h-3 w-3 rounded-sm bg-primary"></div>
        <span class="text-xs font-medium text-foreground">{m.boolean_yes()}</span>
      </div>
    {:else if value === 'NO'}
      <div class="flex items-center gap-2">
        <div class="h-3 w-3 rounded-sm border border-layout-border bg-layout-surface"></div>
        <span class="text-xs font-medium text-muted-foreground">{m.boolean_no()}</span>
      </div>
    {:else}
      <span class="text-sm text-muted-foreground italic">{placeholder}</span>
    {/if}
  </div>
{/if}
