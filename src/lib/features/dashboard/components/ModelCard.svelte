<script lang="ts">
  import { TrainFront } from 'lucide-svelte';
  import { readFile } from '@tauri-apps/plugin-fs';
  import type { ModelCard as ModelCardType } from '$lib/bindings';
  import { Badge } from '$lib/components';
  import * as m from '$lib/paraglide/messages.js';
  import { regionalManager } from '$lib/features/settings/RegionalManager.svelte';

  let { card, onclick }: { card: ModelCardType; onclick?: () => void } = $props();

  let thumbnailSrc = $state<string | null>(null);

  $effect(() => {
    const path = card.thumbnailPath;
    if (!path) {
      thumbnailSrc = null;
      return;
    }

    let stale = false;
    const ext = path.split('.').pop()?.toLowerCase() ?? 'jpg';
    const mimes: Record<string, string> = {
      jpg: 'image/jpeg',
      jpeg: 'image/jpeg',
      png: 'image/png',
      webp: 'image/webp'
    };

    void readFile(path)
      .then((bytes) => {
        if (stale) return;
        const prev = thumbnailSrc;
        thumbnailSrc = URL.createObjectURL(new Blob([bytes], { type: mimes[ext] ?? 'image/jpeg' }));
        if (prev) URL.revokeObjectURL(prev);
      })
      .catch(() => {
        if (!stale) thumbnailSrc = null;
      });

    return () => {
      stale = true;
    };
  });

  function formatCondition(condition: string): string {
    switch (condition) {
      case 'NEW':
        return m.dashboard_condition_new();
      case 'PRE_OWNED':
        return m.dashboard_condition_preowned();
      default:
        return condition;
    }
  }

  function formatDescription(description: string | null): string {
    if (!description) return '';
    if (description.length <= 100) return description;
    return description.slice(0, 100) + '...';
  }

  const formattedPrice = $derived(
    card.price ? regionalManager.formatCurrencyWith(card.price.amount, card.price.currency) : null
  );
</script>

<button
  type="button"
  class="group flex gap-3 rounded-lg border border-border/50 bg-card/80 p-3 text-left shadow-[0_1px_4px_rgba(42,34,24,0.06)] transition-all hover:border-primary/50 hover:bg-card"
  {onclick}
>
  <!-- 3:1 Thumbnail — wide enough for locomotives, contain-fit with blurred backdrop -->
  <div class="relative aspect-[3/1] w-48 flex-shrink-0 overflow-hidden rounded">
    {#if thumbnailSrc}
      <img
        src={thumbnailSrc}
        alt=""
        aria-hidden="true"
        class="absolute inset-0 h-full w-full scale-110 object-cover opacity-60 blur-xl"
      />
      <img
        src={thumbnailSrc}
        alt={`${card.manufacturer} ${card.productCode}`}
        class="absolute inset-0 z-10 h-full w-full object-contain transition-transform duration-300 group-hover:scale-105"
      />
    {:else}
      <div
        class="flex h-full w-full items-center justify-center bg-gradient-to-br from-muted to-muted/70"
        style="background-image: linear-gradient(rgba(255,255,255,0.04) 1px, transparent 1px), linear-gradient(90deg, rgba(255,255,255,0.04) 1px, transparent 1px); background-size: 20px 20px;"
      >
        <TrainFront class="h-12 w-12 text-muted-foreground" />
      </div>
    {/if}

    <!-- Condition Badge (top-right corner) -->
    <div class="absolute top-1 right-1 z-20">
      <Badge
        variant={card.condition === 'NEW' ? 'default' : 'secondary'}
        class="text-[0.65rem] font-semibold"
      >
        {formatCondition(card.condition)}
      </Badge>
    </div>
  </div>

  <!-- Model Details -->
  <div class="flex min-w-0 flex-1 flex-col gap-1">
    <!-- Manufacturer -->
    <p class="text-xs font-bold tracking-wider text-primary uppercase">
      {card.manufacturer}
    </p>

    <!-- Product Code -->
    <p class="text-sm font-semibold text-foreground">{card.productCode}</p>

    <!-- Description (truncated to 100 chars) -->
    {#if card.description}
      <p class="text-xs text-muted-foreground">
        {formatDescription(card.description)}
      </p>
    {/if}

    {#if card.scale || card.era}
      <div class="mt-2 grid grid-cols-2 gap-1 border-t border-border/20 pt-2">
        {#each [{ label: m.depot_scale(), val: card.scale }, { label: m.depot_era(), val: card.era }] as col (col.label)}
          <div class="flex flex-col items-center gap-0.5">
            <span class="text-[9px] tracking-wider text-muted-foreground uppercase"
              >{col.label}</span
            >
            <span class="font-mono text-[11px] text-foreground">{col.val ?? '—'}</span>
          </div>
        {/each}
      </div>
    {/if}
    <div class="mt-2 flex items-baseline justify-between border-t border-border/20 pt-2">
      <span class="text-[9px] tracking-wider text-muted-foreground uppercase"
        >{m.dashboard_card_price()}</span
      >
      <span class="font-mono text-[11px] font-semibold text-primary">{formattedPrice ?? '—'}</span>
    </div>
  </div>
</button>
