<script lang="ts">
  /**
   * InPlaceSelectEdit Component
   *
   * A click-to-edit primitive for select/dropdown fields.
   * Saves immediately on selection change (onChange), no Save/Cancel buttons needed.
   *
   * @component
   * @example
   * ```svelte
   * <InPlaceSelectEdit
   *   value={item.condition}
   *   displayLabel={conditionLabel(item.condition)}
   *   options={conditionOptions.map(o => ({ value: o, label: conditionLabel(o) }))}
   *   placeholder="Not recorded"
   *   onSave={async (v) => { await commands.updateCondition({ value: v }); }}
   * />
   * ```
   */
  import * as m from '$lib/paraglide/messages';

  interface SelectOption {
    value: string;
    label: string;
  }

  interface InPlaceSelectEditProps {
    /** Current selected value. */
    value: string;
    /** Label text shown in view mode. */
    displayLabel: string;
    /** Available options. */
    options: SelectOption[];
    /** Placeholder shown when value is empty. */
    placeholder?: string;
    /** Called with the newly selected value when user picks an option. */
    onSave: (value: string) => Promise<void>;
    /** Additional CSS classes applied to the view-mode element. */
    class?: string;
    /** Called when the user activates edit mode (for cross-card coordination). */
    onActivate?: () => void;
    /** Called when the user exits edit mode (for cross-card coordination). */
    onDeactivate?: () => void;
    /** Accessible label for the trigger element in view mode. */
    ariaLabel?: string;
  }

  let {
    value,
    displayLabel,
    options,
    placeholder,
    onSave,
    class: extraClass = '',
    onActivate,
    onDeactivate,
    ariaLabel
  }: InPlaceSelectEditProps = $props();

  let isEditing = $state(false);
  let isSaving = $state(false);
  let error = $state<string | null>(null);

  let selectEl = $state<HTMLSelectElement | null>(null);

  $effect(() => {
    if (isEditing && selectEl) {
      selectEl.focus();
    }
  });

  function startEditing() {
    if (isSaving) return;
    error = null;
    isEditing = true;
    onActivate?.();
  }

  function cancel() {
    // Guard: if a save is in progress (change fired before blur), don't cancel
    if (isSaving) return;
    error = null;
    isEditing = false;
    onDeactivate?.();
  }

  async function handleChange(e: Event) {
    const newValue = (e.target as HTMLSelectElement).value;
    isSaving = true;
    error = null;
    try {
      await onSave(newValue);
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
    <select
      bind:this={selectEl}
      {value}
      onchange={handleChange}
      onkeydown={handleKeydown}
      onblur={cancel}
      disabled={isSaving}
      class="w-full rounded border border-primary bg-card px-2 py-1 text-sm text-foreground ring-1 ring-primary/30 outline-none"
    >
      {#each options as option (option.value)}
        <option value={option.value}>{option.label}</option>
      {/each}
    </select>
    {#if error}
      <p class="mt-1 text-xs text-red-400" role="alert">{error}</p>
    {/if}
  </div>
{:else}
  <div
    class="group relative -mx-1 cursor-pointer rounded p-1 transition-colors duration-150 hover:border hover:border-dashed hover:border-primary/40 hover:bg-primary/15 {extraClass}"
    onclick={startEditing}
    onkeydown={(e) => {
      if (e.key === 'Enter' || e.key === ' ') startEditing();
    }}
    role="button"
    tabindex="0"
    aria-label={ariaLabel}
    aria-haspopup="listbox"
    aria-expanded="false"
  >
    {#if value}
      <span class="text-sm text-foreground">{displayLabel}</span>
    {:else}
      <span class="text-sm text-muted-foreground italic"
        >{placeholder ?? m.edit_field_placeholder_empty()}</span
      >
    {/if}
  </div>
{/if}
