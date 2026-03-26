<script lang="ts">
  import { X, MoveRight } from 'lucide-svelte';
  import type { WishlistPreview } from '$lib/bindings';
  import * as m from '$lib/paraglide/messages.js';
  import * as Dialog from '$lib/components/ui/dialog';
  import * as Select from '$lib/components/ui/select';
  import { Button } from '$lib/components';

  interface Props {
    open: boolean;
    itemId: string;
    fromWishlistId: string;
    otherTargets: WishlistPreview[];
    onClose: () => void;
    onMove: (detail: { itemId: string; fromId: string; toId: string }) => void;
  }

  const { open, itemId, fromWishlistId, otherTargets, onClose, onMove }: Props = $props();

  let selectedTargetId = $state('');
  let error = $state('');

  $effect(() => {
    if (open) {
      selectedTargetId = '';
      error = '';
    }
  });

  const selectedLabel = $derived(
    otherTargets.find((t: WishlistPreview) => t.id === selectedTargetId)?.name ?? ''
  );

  function handleSubmit() {
    if (!selectedTargetId) {
      error = m.wishlists_move_destination_label();
      return;
    }
    onMove({ itemId, fromId: fromWishlistId, toId: selectedTargetId });
    onClose();
  }
</script>

<Dialog.Root {open} onOpenChange={() => onClose()}>
  <Dialog.Content
    class="overflow-hidden rounded-2xl border border-amber-500/20 bg-layout-surface p-0 text-white shadow-2xl"
  >
    <!-- Header -->
    <div class="flex items-center justify-between border-b border-white/5 p-6">
      <div class="flex items-center gap-3">
        <div
          class="flex h-10 w-10 items-center justify-center rounded-xl bg-amber-500/10 text-amber-500"
        >
          <MoveRight size={20} />
        </div>
        <h2 class="text-xl font-bold tracking-wider text-amber-500 uppercase">
          {m.wishlists_move_model_title()}
        </h2>
      </div>
      <button
        type="button"
        onclick={onClose}
        class="rounded-lg p-2 text-zinc-500 transition-colors hover:bg-white/5 hover:text-white"
        aria-label="close"
      >
        <X size={20} />
      </button>
    </div>

    <!-- Body -->
    <div class="space-y-3 p-6">
      <span class="text-[10px] font-bold tracking-[0.25em] text-muted-foreground uppercase">
        {m.wishlists_move_destination_label()}
      </span>
      <Select.Root
        type="single"
        value={selectedTargetId || undefined}
        onValueChange={(v) => {
          selectedTargetId = v ?? '';
          error = '';
        }}
      >
        <Select.Trigger class="w-full border-layout-border bg-layout-surface text-foreground">
          {#if selectedLabel}
            {selectedLabel}
          {:else}
            <span class="text-zinc-500">—</span>
          {/if}
        </Select.Trigger>
        <Select.Content>
          {#each otherTargets as target (target.id)}
            <Select.Item value={target.id} label={target.name}>
              <span class="flex-1">{target.name}</span>
              <span class="text-xs text-zinc-500"
                >{m.wishlists_move_items_count({ count: target.count })}</span
              >
            </Select.Item>
          {/each}
        </Select.Content>
      </Select.Root>

      {#if error}
        <p class="text-xs text-red-400">{error}</p>
      {/if}
    </div>

    <!-- Footer -->
    <div class="flex justify-end gap-3 border-t border-white/5 px-6 py-4">
      <Button
        variant="ghost"
        type="button"
        onclick={onClose}
        class="text-zinc-500 hover:text-white"
      >
        {m.wishlists_move_cancel()}
      </Button>
      <Button
        type="button"
        onclick={handleSubmit}
        disabled={!selectedTargetId}
        class="bg-primary font-bold text-primary-foreground hover:bg-primary/90 disabled:opacity-40"
      >
        {m.wishlists_move_submit()}
      </Button>
    </div>
  </Dialog.Content>
</Dialog.Root>
