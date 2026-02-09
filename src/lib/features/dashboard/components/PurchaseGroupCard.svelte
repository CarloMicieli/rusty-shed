<script lang="ts">
  import { Calendar, Store } from 'lucide-svelte';
  import { format } from 'date-fns';
  import type { PurchaseGroup } from '$lib/bindings';
  import ModelCard from './ModelCard.svelte';
  import * as m from '$lib/paraglide/messages.js';

  let { group, onModelClick }: { group: PurchaseGroup; onModelClick?: (modelId: string) => void } =
    $props();

  const formattedDate = $derived(format(new Date(group.purchaseDate), 'MMMM d, yyyy'));
  const sellerDisplay = $derived(group.sellerName ?? m.dashboard_unknown_source());
  const hasMoreModels = $derived(Number(group.totalCount) > group.modelCards.length);
  const additionalCount = $derived(Number(group.totalCount) - group.modelCards.length);
</script>

<div class="space-y-4 rounded-lg border border-white/10 bg-black/20 p-4">
  <!-- Purchase Header -->
  <div class="space-y-2">
    <!-- Purchase Date -->
    <div class="flex items-center gap-2">
      <Calendar class="h-4 w-4 text-zinc-400" />
      <span class="text-sm font-semibold text-zinc-300">{formattedDate}</span>
    </div>

    <!-- Seller Name -->
    <div class="flex items-center gap-2">
      <Store class="h-4 w-4 text-zinc-400" />
      <span class="text-sm text-zinc-400">{sellerDisplay}</span>
    </div>

    <!-- Notes (if present) -->
    {#if group.notes}
      <p class="mt-2 text-xs text-zinc-500 italic">{group.notes}</p>
    {/if}
  </div>

  <!-- Model Cards Grid -->
  <div class="grid grid-cols-1 gap-4 md:grid-cols-3">
    {#each group.modelCards as card (card.id)}
      <ModelCard {card} onclick={() => onModelClick?.(card.id)} />
    {/each}
  </div>

  <!-- "+N more models..." indicator -->
  {#if hasMoreModels}
    <div class="text-center">
      <p class="text-xs text-zinc-500">
        {m.dashboard_more_models({ count: additionalCount })}
      </p>
    </div>
  {/if}
</div>
