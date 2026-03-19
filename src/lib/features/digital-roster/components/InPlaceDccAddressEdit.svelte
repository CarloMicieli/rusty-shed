<script lang="ts">
  import * as m from '$lib/paraglide/messages';
  import { Check, X, Loader2 } from 'lucide-svelte';

  interface Props {
    value: number;
    excludeId: string;
    allAddresses: { id: string; dcc_address: number }[];
    onSave: (newAddress: number) => Promise<boolean>;
    deactivate?: boolean;
    onEditStart?: () => void;
    onEditEnd?: () => void;
  }

  let {
    value,
    excludeId,
    allAddresses,
    onSave,
    deactivate = false,
    onEditStart,
    onEditEnd
  }: Props = $props();

  let isEditing = $state(false);
  let editValue = $state(0);
  let isSaving = $state(false);
  let inputEl = $state<HTMLInputElement | null>(null);

  $effect(() => {
    if (isEditing && inputEl) {
      inputEl.focus();
    }
  });

  // Deactivate when parent signals another row is being edited, but not mid-save
  $effect(() => {
    if (deactivate && isEditing && !isSaving) {
      isEditing = false;
    }
  });

  let isDuplicate = $derived(
    editValue !== value &&
      allAddresses.some((a) => a.id !== excludeId && a.dcc_address === editValue)
  );
  let isOutOfRange = $derived(editValue < 1 || editValue > 9999);
  let canSave = $derived(!isDuplicate && !isOutOfRange && editValue !== value);

  function activate() {
    editValue = value;
    isEditing = true;
    onEditStart?.();
  }

  function cancel() {
    isEditing = false;
    onEditEnd?.();
  }

  async function save() {
    if (!canSave || isSaving) return;
    isSaving = true;
    try {
      const success = await onSave(editValue);
      if (success) {
        isEditing = false;
        onEditEnd?.();
      }
    } finally {
      isSaving = false;
    }
  }

  function handleKeydown(event: KeyboardEvent) {
    if (event.key === 'Enter') save();
    else if (event.key === 'Escape') cancel();
  }

  let inputBorderClass = $derived(
    isDuplicate ? 'border-red-500' : isOutOfRange ? 'border-orange-500' : 'border-amber-500/60'
  );
</script>

{#if isEditing}
  <div class="flex flex-col gap-1">
    <div class="flex items-center gap-1">
      <input
        type="number"
        min="1"
        max="9999"
        class="w-20 rounded border {inputBorderClass} bg-black/60 px-2 py-0.5 font-mono text-sm text-amber-400 tabular-nums outline-none focus:ring-1 focus:ring-amber-500/40"
        bind:value={editValue}
        bind:this={inputEl}
        onkeydown={handleKeydown}
      />
      {#if isSaving}
        <Loader2 class="h-4 w-4 animate-spin text-amber-400" />
      {:else}
        <button
          type="button"
          class="rounded p-0.5 text-amber-400 transition-colors hover:bg-amber-950/60 disabled:cursor-not-allowed disabled:opacity-40"
          onclick={save}
          disabled={!canSave}
          title="Save"
        >
          <Check class="h-4 w-4" />
        </button>
      {/if}
      <button
        type="button"
        class="rounded p-0.5 text-zinc-500 transition-colors hover:bg-zinc-800 hover:text-zinc-300"
        onclick={cancel}
        title="Cancel"
      >
        <X class="h-4 w-4" />
      </button>
    </div>
    {#if isDuplicate}
      <span class="text-xs text-red-400">{m.digital_roster_duplicate_address()}</span>
    {:else if isOutOfRange}
      <span class="text-xs text-orange-400">{m.digital_roster_address_range_hint()}</span>
    {/if}
  </div>
{:else}
  <button
    type="button"
    class="cursor-pointer rounded border border-black/60 bg-black/40 px-2 py-0.5 font-mono text-sm text-amber-400/90 tabular-nums transition-colors hover:border-amber-500/40"
    onclick={activate}
    title="Click to edit"
  >
    {value}
  </button>
{/if}
