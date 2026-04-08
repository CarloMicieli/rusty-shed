<script lang="ts">
  /**
   * InPlaceSelectEdit Component
   *
   * A click-to-edit primitive for select/dropdown fields.
   * Opens a floating overlay panel (not a native <select>) so the dropdown
   * overlays content rather than expanding the container height.
   * Saves immediately on selection, no Save/Cancel buttons needed.
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
    /** Additional CSS classes applied to the trigger element. */
    class?: string;
    /** Called when the user opens the picker (for cross-card coordination). */
    onActivate?: () => void;
    /** Called when the user closes the picker (for cross-card coordination). */
    onDeactivate?: () => void;
    /** Accessible label for the trigger element. */
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

  let isOpen = $state(false);
  let isSaving = $state(false);
  let error = $state<string | null>(null);
  let focusedIndex = $state(0);
  let panelTop = $state(0);
  let panelLeft = $state(0);

  let triggerEl = $state<HTMLDivElement | null>(null);
  let listEl = $state<HTMLUListElement | null>(null);

  $effect(() => {
    if (isOpen && listEl) {
      listEl.focus();
    }
  });

  function open() {
    if (isSaving) return;
    error = null;
    focusedIndex = options.findIndex((o) => o.value === value);
    if (focusedIndex < 0) focusedIndex = 0;
    // Compute position from trigger's viewport rect so the panel renders
    // via position:fixed — completely outside the document flow.
    if (triggerEl) {
      const rect = triggerEl.getBoundingClientRect();
      panelTop = rect.bottom + 4;
      panelLeft = rect.left;
    }
    isOpen = true;
    onActivate?.();
  }

  function close() {
    isOpen = false;
    onDeactivate?.();
  }

  async function select(newValue: string) {
    if (isSaving) return;
    isSaving = true;
    error = null;
    try {
      await onSave(newValue);
      close();
    } catch {
      error = m.edit_save_error();
    } finally {
      isSaving = false;
    }
  }

  function handleTriggerKeydown(e: KeyboardEvent) {
    if (e.key === 'Enter' || e.key === ' ') {
      e.preventDefault();
      open();
    }
  }

  function handleListKeydown(e: KeyboardEvent) {
    if (e.key === 'Escape') {
      e.preventDefault();
      close();
      triggerEl?.focus();
    } else if (e.key === 'ArrowDown') {
      e.preventDefault();
      focusedIndex = (focusedIndex + 1) % options.length;
    } else if (e.key === 'ArrowUp') {
      e.preventDefault();
      focusedIndex = (focusedIndex - 1 + options.length) % options.length;
    } else if (e.key === 'Enter') {
      e.preventDefault();
      const opt = options[focusedIndex];
      if (opt) void select(opt.value);
    }
  }

  function handleOutsideClick(e: MouseEvent) {
    if (
      isOpen &&
      triggerEl &&
      !triggerEl.contains(e.target as Node) &&
      listEl &&
      !listEl.contains(e.target as Node)
    ) {
      close();
    }
  }
</script>

<svelte:window
  onclick={handleOutsideClick}
  onkeydown={(e) => e.key === 'Escape' && isOpen && close()}
/>

<!-- Trigger: always visible, shows current value -->
<div
  bind:this={triggerEl}
  class="group -mx-1 cursor-pointer rounded p-1 transition-colors duration-150 hover:border hover:border-dashed hover:border-primary/40 hover:bg-primary/15 {extraClass}"
  role="button"
  tabindex="0"
  aria-label={ariaLabel}
  aria-haspopup="listbox"
  aria-expanded={isOpen}
  onclick={open}
  onkeydown={handleTriggerKeydown}
>
  {#if value}
    <span class="text-sm text-foreground">{displayLabel}</span>
  {:else}
    <span class="text-sm text-muted-foreground italic"
      >{placeholder ?? m.edit_field_placeholder_empty()}</span
    >
  {/if}
</div>

<!-- Floating panel: position:fixed so it never affects document flow or container size -->
{#if isOpen}
  <div
    class="fixed z-50 min-w-[10rem] overflow-hidden rounded-lg border border-border bg-card shadow-xl"
    style="top: {panelTop}px; left: {panelLeft}px;"
  >
    <ul
      bind:this={listEl}
      role="listbox"
      tabindex="-1"
      class="max-h-48 overflow-y-auto py-1 outline-none"
      onkeydown={handleListKeydown}
    >
      {#each options as option, i (option.value)}
        <li
          role="option"
          aria-selected={option.value === value}
          class="flex cursor-pointer items-center px-3 py-1.5 text-sm transition-colors
            {option.value === value ? 'bg-primary/20 text-primary' : 'text-foreground'}
            {i === focusedIndex ? 'bg-primary/10' : ''}
            hover:bg-primary/15 hover:text-primary"
          onclick={() => void select(option.value)}
          onkeydown={(e) => {
            if (e.key === 'Enter' || e.key === ' ') {
              e.preventDefault();
              void select(option.value);
            }
          }}
        >
          {option.label}
        </li>
      {/each}
    </ul>
    {#if error}
      <p class="px-3 pb-1 text-xs text-destructive" role="alert">{error}</p>
    {/if}
  </div>
{/if}
