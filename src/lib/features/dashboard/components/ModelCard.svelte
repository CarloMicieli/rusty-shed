<script lang="ts">
  import { TrainFront } from 'lucide-svelte';
  import { readFile } from '@tauri-apps/plugin-fs';
  import type { ModelCard as ModelCardType } from '$lib/bindings';
  import { Badge } from '$lib/components';
  import * as m from '$lib/paraglide/messages.js';

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
</script>

<button
  type="button"
  class="group flex gap-3 rounded-lg border border-white/10 bg-black/20 p-3 text-left transition-all hover:border-orange-400/50 hover:bg-black/30"
  {onclick}
>
  <!-- 16:9 Aspect Ratio Thumbnail -->
  <div class="relative aspect-video w-40 flex-shrink-0 overflow-hidden rounded">
    {#if thumbnailSrc}
      <img
        src={thumbnailSrc}
        alt={`${card.manufacturer} ${card.productCode}`}
        class="h-full w-full object-cover transition-transform duration-300 group-hover:scale-105"
      />
    {:else}
      <div
        class="flex h-full w-full items-center justify-center bg-gradient-to-br from-zinc-800 to-zinc-700"
      >
        <TrainFront class="h-12 w-12 text-zinc-500" />
      </div>
    {/if}

    <!-- Condition Badge (top-right corner) -->
    <div class="absolute top-1 right-1">
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
    <!-- Manufacturer in orange-400 -->
    <p class="text-xs font-bold tracking-wider text-orange-400 uppercase">
      {card.manufacturer}
    </p>

    <!-- Product Code -->
    <p class="text-sm font-semibold text-white">{card.productCode}</p>

    <!-- Description (truncated to 100 chars) -->
    {#if card.description}
      <p class="text-xs text-zinc-400">
        {formatDescription(card.description)}
      </p>
    {/if}

    {#if card.scale || card.era || card.roadNumber}
      <div class="mt-2 grid grid-cols-3 gap-1 border-t border-white/5 pt-2">
        {#each [{ label: m.depot_road_number(), val: card.roadNumber }, { label: m.depot_scale(), val: card.scale }, { label: m.depot_era(), val: card.era }] as col (col.label)}
          <div class="flex flex-col items-center gap-0.5">
            <span class="text-[9px] tracking-wider text-[#808080] uppercase">{col.label}</span>
            <span class="font-mono text-[11px] text-[#E0E0E0]">{col.val ?? '—'}</span>
          </div>
        {/each}
      </div>
    {/if}
  </div>
</button>
