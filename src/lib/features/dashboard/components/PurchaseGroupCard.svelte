<script lang="ts">
  import { Calendar, Store } from 'lucide-svelte';
  import { format, isValid } from 'date-fns';
  import type { PurchaseGroup } from '$lib/bindings';
  import ModelCard from './ModelCard.svelte';
  import * as m from '$lib/paraglide/messages.js';

  let {
    group,
    onModelClick
  }: { group: PurchaseGroup; onModelClick?: (collectionItemId: string) => void } = $props();

  const formattedDate = $derived.by(() => {
    const d = new Date(group.purchaseDate);
    return isValid(d) ? format(d, 'MMMM d, yyyy') : '---';
  });

  const sellerDisplay = $derived(group.sellerName ?? m.dashboard_unknown_source());
  const additionalCount = $derived(Number(group.totalCount) - group.modelCards.length);
</script>

<div
  class="space-y-4 rounded-xl border border-border/50 bg-card p-5 shadow-[0_2px_10px_rgba(42,34,24,0.08)] transition-all hover:border-border/70"
>
  <header
    class="flex flex-wrap items-start justify-between gap-4 border-l-2 border-primary/30 pl-4"
  >
    <div class="space-y-1">
      <div class="flex items-center gap-2 text-foreground">
        <Calendar class="h-3.5 w-3.5 text-primary/70" />
        <span class="font-mono text-xs font-bold tracking-tight">{formattedDate}</span>
      </div>

      <div class="flex items-center gap-2 text-muted-foreground">
        <Store class="h-3.5 w-3.5" />
        <span class="text-xs tracking-wider uppercase">{sellerDisplay}</span>
      </div>

      {#if group.notes}
        <p class="max-w-md text-[11px] leading-relaxed text-muted-foreground italic">
          // {group.notes}
        </p>
      {/if}
    </div>

    <div class="hidden sm:block">
      <span
        class="rounded bg-muted/20 px-2 py-1 font-mono text-[9px] text-muted-foreground uppercase"
      >
        ID: {group.id.split('-')[0]}
      </span>
    </div>
  </header>

  <div class="grid grid-cols-1 gap-4 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4">
    {#each group.modelCards as card (card.id)}
      <ModelCard {card} onclick={() => onModelClick?.(card.id)} />
    {/each}
  </div>

  {#if additionalCount > 0}
    <div class="flex items-center gap-4 py-2">
      <div class="h-px flex-1 bg-gradient-to-r from-transparent via-border/30 to-transparent"></div>
      <p class="font-mono text-[10px] tracking-widest text-muted-foreground uppercase">
        + {m.dashboard_more_models({ count: additionalCount })}
      </p>
      <div class="h-px flex-1 bg-gradient-to-r from-transparent via-border/30 to-transparent"></div>
    </div>
  {/if}
</div>
