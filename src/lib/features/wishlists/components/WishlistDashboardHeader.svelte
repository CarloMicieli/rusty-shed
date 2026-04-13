<script lang="ts">
  import * as m from '$lib/paraglide/messages.js';
  import { Settings, Star, Trash2, Edit2, X } from 'lucide-svelte';
  import { Button, Input } from '$lib/components';
  import * as Select from '$lib/components/ui/select';
  import type { WishlistPreview, WishlistItem } from '$lib/bindings';
  import { regionalManager } from '$lib/features/settings/RegionalManager.svelte';

  interface Props {
    wishlists: WishlistPreview[];
    activeWishlist?: WishlistPreview | null;
    activeWishlistId: string | null;
    items: WishlistItem[];
    onSelect: (id: string) => void;
    onRename?: (name: string) => void;
    onSetDefault?: () => void;
    onDelete?: (id: string) => void;
  }

  const {
    wishlists,
    activeWishlist,
    activeWishlistId,
    items,
    onSelect,
    onRename,
    onSetDefault,
    onDelete
  }: Props = $props();

  const selectedWishlist = $derived(
    activeWishlist ?? wishlists.find((w) => w.id === activeWishlistId) ?? null
  );

  const activeWishlistName = $derived(selectedWishlist?.name ?? '');

  const pricedItems = $derived(items.filter((item) => item.desiredPrice != null));

  let isEditing = $state(false);
  let nameDraft = $state('');
  let showSettings = $state(false);

  $effect(() => {
    if (!selectedWishlist) return;
    if (!isEditing) nameDraft = selectedWishlist.name;
  });

  const totals = $derived.by(() => {
    let high = 0;
    let normal = 0;
    let low = 0;
    for (const item of pricedItems) {
      const amount = Number(item.desiredPrice!.amount);
      if (item.priority === 'HIGH') high += amount;
      else if (item.priority === 'NORMAL') normal += amount;
      else if (item.priority === 'LOW') low += amount;
    }
    return { high, normal, low, total: high + normal + low };
  });

  const percentages = $derived.by(() => {
    if (totals.total === 0) return { high: 0, normal: 0, low: 0 };
    return {
      high: (totals.high / totals.total) * 100,
      normal: (totals.normal / totals.total) * 100,
      low: (totals.low / totals.total) * 100
    };
  });

  const currency = $derived(pricedItems[0]?.desiredPrice?.currency ?? 'EUR');

  function formatAmount(cents: number, curr: string): string {
    return regionalManager.formatCurrencyWith(cents, curr);
  }

  function handleRenameSubmit() {
    if (!selectedWishlist) return;
    const nextName = nameDraft.trim();
    if (nextName && nextName !== selectedWishlist.name) {
      onRename?.(nextName);
    }
    isEditing = false;
  }

  function handleRenameKeydown(e: KeyboardEvent) {
    if (e.key === 'Enter') handleRenameSubmit();
    if (e.key === 'Escape') {
      isEditing = false;
      if (selectedWishlist) nameDraft = selectedWishlist.name;
    }
  }
</script>

<div
  class="flex flex-col gap-4 rounded-[8px] border border-layout-border bg-layout-surface px-4 py-3"
>
  <div class="flex flex-col gap-3 lg:flex-row lg:items-start lg:justify-between">
    <div class="flex min-w-0 items-start gap-2">
      {#if isEditing}
        <div class="flex flex-wrap items-center gap-2">
          <Input
            bind:value={nameDraft}
            onkeydown={handleRenameKeydown}
            class="h-9 min-w-[240px] bg-card font-bebas text-xl tracking-wider text-card-foreground uppercase shadow-inner focus-visible:ring-2 focus-visible:ring-primary"
            autofocus
          />
          <Button size="sm" class="h-9" onclick={handleRenameSubmit}
            >{m.wishlist_modal_save()}</Button
          >
          <Button size="sm" variant="outline" class="h-9" onclick={() => (isEditing = false)}
            >{m.wishlist_modal_cancel()}</Button
          >
        </div>
      {:else}
        <h1
          class="truncate font-bebas text-3xl tracking-widest text-foreground uppercase lg:text-4xl"
        >
          {activeWishlistName || m.wishlists_select_list_placeholder()}
        </h1>

        {#if selectedWishlist}
          <div class="relative">
            <Button
              variant="outline"
              size="icon"
              onclick={() => (showSettings = !showSettings)}
              class="h-9 w-9 cursor-pointer border-border text-muted-foreground transition-all duration-150 ease-out hover:bg-muted hover:text-foreground"
            >
              {#if showSettings}
                <X size={18} />
              {:else}
                <Settings size={18} />
              {/if}
            </Button>

            {#if showSettings}
              <div
                class="absolute top-11 left-0 z-50 w-56 animate-in rounded-[8px] border border-border bg-card p-1 shadow-2xl duration-200 fade-in zoom-in"
                onmouseleave={() => (showSettings = false)}
                role="menu"
                tabindex="-1"
              >
                <button
                  onclick={() => {
                    isEditing = true;
                    showSettings = false;
                  }}
                  class="flex w-full items-center rounded-[8px] px-3 py-2 text-sm text-foreground transition-colors hover:bg-muted"
                >
                  <Edit2 size={14} class="mr-2" />
                  {m.wishlist_header_rename()}
                </button>
                <button
                  onclick={() => {
                    onSetDefault?.();
                    showSettings = false;
                  }}
                  class="flex w-full items-center rounded-[8px] px-3 py-2 text-sm text-foreground transition-colors hover:bg-muted"
                >
                  <Star size={14} class="mr-2" />
                  {m.wishlist_header_set_default()}
                </button>
                <div class="my-1 h-px bg-border"></div>
                <button
                  onclick={() => {
                    onDelete?.(selectedWishlist.id);
                    showSettings = false;
                  }}
                  class="flex w-full items-center rounded-[8px] px-3 py-2 text-sm text-red-400 transition-colors hover:bg-red-900/20"
                >
                  <Trash2 size={14} class="mr-2" />
                  {m.wishlist_header_delete_list()}
                </button>
              </div>
            {/if}
          </div>
        {/if}
      {/if}
    </div>

    <div class="w-full lg:w-[320px]">
      <div class="flex flex-col gap-1.5">
        <span class="font-mono text-[10px] tracking-[0.2em] text-muted-foreground uppercase">
          {m.wishlists_switch_list_label()}
        </span>
        <Select.Root
          type="single"
          value={activeWishlistId ?? undefined}
          onValueChange={(v) => {
            if (v) onSelect(v);
          }}
        >
          <Select.Trigger class="h-9 w-full border-layout-border bg-layout-surface">
            {#if activeWishlistName}
              <span class="font-bold text-primary">{activeWishlistName}</span>
            {:else}
              <span class="text-muted-foreground">{m.wishlists_select_list_placeholder()}</span>
            {/if}
          </Select.Trigger>
          <Select.Content>
            {#each wishlists as wl (wl.id)}
              <Select.Item value={wl.id} label={wl.name} />
            {/each}
          </Select.Content>
        </Select.Root>
      </div>
    </div>
  </div>

  <div class="hidden items-stretch border-t border-layout-border pt-4 lg:flex">
    <div class="flex w-[300px] shrink-0 flex-col border-r border-layout-border pr-4">
      {#if pricedItems.length === 0}
        <div class="flex items-center justify-center py-2">
          <span class="font-mono text-[10px] tracking-widest text-muted-foreground uppercase">
            {m.wishlist_value_bar_no_price_data()}
          </span>
        </div>
      {:else}
        <div class="grid grid-cols-2 gap-4">
          <div class="flex flex-col gap-0.5">
            <span
              class="flex min-h-[2em] items-start font-mono text-[10px] leading-snug tracking-widest text-muted-foreground uppercase"
            >
              {m.wishlist_stat_total_cost()}
            </span>
            <span class="font-mono text-base leading-none font-bold text-primary">
              {formatAmount(totals.total, currency)}
            </span>
          </div>
          <div class="flex flex-col gap-0.5">
            <span
              class="flex min-h-[2em] items-start font-mono text-[10px] leading-snug tracking-widest text-muted-foreground uppercase"
            >
              {m.wishlist_stat_item_count()}
            </span>
            <span class="font-mono text-base leading-none font-bold text-foreground">
              {items.length}
            </span>
          </div>
        </div>
      {/if}
    </div>

    <div class="flex flex-1 flex-col gap-2 pl-4">
      <span class="font-mono text-[10px] tracking-[0.2em] text-muted-foreground uppercase">
        {m.wishlist_procurement_summary()}
      </span>
      {#if pricedItems.length > 0}
        <!-- Gauge bar -->
        <div
          data-testid="gauge-bar"
          class="relative h-2.5 overflow-hidden rounded-full bg-layout-border"
        >
          <div class="absolute inset-y-0 left-0 flex h-full" style="width: 100%">
            {#if percentages.high > 0}
              <div
                class="h-full bg-red-500 transition-all duration-500"
                style="width: {percentages.high}%; box-shadow: 0 0 8px rgba(239,68,68,0.5), 0 0 16px rgba(239,68,68,0.2);"
              ></div>
            {/if}
            {#if percentages.normal > 0}
              <div
                class="h-full bg-orange-500 transition-all duration-500"
                style="width: {percentages.normal}%"
              ></div>
            {/if}
            {#if percentages.low > 0}
              <div
                class="h-full bg-primary/25 transition-all duration-500"
                style="width: {percentages.low}%"
              ></div>
            {/if}
          </div>
          <div class="pointer-events-none absolute inset-0 flex items-stretch justify-around">
            {#each [0, 1, 2] as _ (_)}
              <div class="h-full w-px bg-layout-surface/70"></div>
            {/each}
          </div>
        </div>

        <!-- Priority legend -->
        <div class="flex justify-between gap-2">
          <div class="flex flex-col gap-0.5">
            <div class="flex items-center gap-1.5">
              <div class="h-1.5 w-1.5 rounded-full bg-red-500"></div>
              <span class="font-mono text-[10px] tracking-widest text-muted-foreground uppercase">
                {m.wishlist_priority_high()}
              </span>
            </div>
            <span class="font-mono text-xs text-foreground/70">
              {formatAmount(totals.high, currency)}
            </span>
          </div>
          <div class="flex flex-col items-center gap-0.5">
            <div class="flex items-center gap-1.5">
              <div class="h-1.5 w-1.5 rounded-full bg-orange-500"></div>
              <span class="font-mono text-[10px] tracking-widest text-muted-foreground uppercase">
                {m.wishlist_priority_normal()}
              </span>
            </div>
            <span class="font-mono text-xs text-foreground/70">
              {formatAmount(totals.normal, currency)}
            </span>
          </div>
          <div class="flex flex-col items-end gap-0.5">
            <div class="flex items-center gap-1.5">
              <div class="h-1.5 w-1.5 rounded-full bg-primary/25"></div>
              <span class="font-mono text-[10px] tracking-widest text-muted-foreground uppercase">
                {m.wishlist_priority_low()}
              </span>
            </div>
            <span class="font-mono text-xs text-foreground/70">
              {formatAmount(totals.low, currency)}
            </span>
          </div>
        </div>
      {/if}
    </div>
  </div>
</div>
