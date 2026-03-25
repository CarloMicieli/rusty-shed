<script lang="ts">
  /**
   * BadgePicker Component
   *
   * A constrained-selection popover anchored to a trigger element.
   * Clicking the trigger opens a picker panel with a list of options;
   * selecting one calls `onSelect` and closes the panel.
   *
   * @component
   * @example
   * ```svelte
   * <BadgePicker
   *   value="H0"
   *   options={[{ id: 'H0', label: 'H0 (1:87)' }, { id: 'N', label: 'N (1:160)' }]}
   *   onSelect={async (id) => { await commands.updateRailwayModelClassification({...}); }}
   * />
   * ```
   */
  import * as m from '$lib/paraglide/messages';
  import { Pencil } from 'lucide-svelte';

  interface Option {
    id: string;
    label: string;
  }

  interface BadgePickerProps {
    /** Currently selected display value. */
    value: string;
    /** Available options to pick from. */
    options: Option[];
    /** Called with the selected option id when the user picks a value. */
    onSelect: (id: string) => Promise<void>;
  }

  let { value, options, onSelect }: BadgePickerProps = $props();

  let isOpen = $state(false);
  let isSaving = $state(false);
  /** Optimistic selected id/value — reverted if onSelect rejects. */
  let selectedValue = $derived(value);
  let focusedIndex = $state(0);
  let triggerEl = $state<HTMLDivElement | null>(null);
  let listEl = $state<HTMLUListElement | null>(null);

  let displayValue = $derived.by(() => {
    const option = options.find((o) => o.id === selectedValue || o.label === selectedValue);
    return option?.label ?? selectedValue;
  });

  $effect(() => {
    if (isOpen && listEl) {
      listEl.focus();
    }
  });

  function open() {
    if (isSaving) return;
    focusedIndex = options.findIndex((o) => o.id === selectedValue || o.label === selectedValue);
    if (focusedIndex < 0) focusedIndex = 0;
    isOpen = true;
  }

  function close() {
    isOpen = false;
  }

  async function select(id: string) {
    if (isSaving) return;
    const previous = selectedValue;
    selectedValue = id;
    close();
    isSaving = true;
    try {
      await onSelect(id);
    } catch {
      selectedValue = previous;
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
      if (opt) void select(opt.id);
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

<svelte:window onclick={handleOutsideClick} onkeydown={(e) => e.key === 'Escape' && close()} />

<div class="relative inline-block">
  <!-- Trigger -->
  <div
    bind:this={triggerEl}
    class="group inline-flex cursor-pointer items-center gap-1 rounded px-1.5 py-0.5 transition-colors hover:bg-primary/15"
    role="button"
    tabindex="0"
    aria-haspopup="listbox"
    aria-expanded={isOpen}
    onclick={open}
    onkeydown={handleTriggerKeydown}
  >
    <span class="font-mono text-xs text-foreground">{displayValue}</span>
    <Pencil
      size={10}
      class="text-muted-foreground opacity-0 transition-opacity duration-150 group-hover:opacity-100"
    />
  </div>

  <!-- Picker panel -->
  {#if isOpen}
    <div
      class="absolute top-full left-0 z-50 mt-1 min-w-[10rem] overflow-hidden rounded-lg border border-border bg-card shadow-xl"
    >
      <div class="flex items-center justify-between border-b border-border px-2 py-1">
        <span class="text-[10px] font-medium tracking-wider text-muted-foreground uppercase"
          >{m.badge_picker_close()}</span
        >
      </div>
      <ul
        bind:this={listEl}
        role="listbox"
        tabindex="-1"
        class="max-h-48 overflow-y-auto py-1 outline-none"
        onkeydown={handleListKeydown}
      >
        {#each options as option, i (option.id)}
          <li
            role="option"
            aria-selected={option.id === selectedValue || option.label === selectedValue}
            class="flex cursor-pointer items-center px-3 py-1.5 text-xs transition-colors
              {option.id === selectedValue || option.label === selectedValue
              ? 'bg-primary/20 text-primary'
              : 'text-foreground'}
              {i === focusedIndex ? 'bg-primary/10' : ''}
              hover:bg-primary/15 hover:text-primary"
            onclick={() => void select(option.id)}
            onkeydown={(e) => {
              if (e.key === 'Enter' || e.key === ' ') {
                e.preventDefault();
                void select(option.id);
              }
            }}
          >
            {option.label}
          </li>
        {/each}
      </ul>
    </div>
  {/if}
</div>
