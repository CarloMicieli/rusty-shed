<script lang="ts">
  import { TrainFront } from 'lucide-svelte';
  import { convertFileSrc } from '@tauri-apps/api/core';
  import type { ModelCard as ModelCardType } from '$lib/bindings';
  import { Badge } from '$lib/components';
  import * as m from '$lib/paraglide/messages.js';

  let { card, onclick }: { card: ModelCardType; onclick?: () => void } = $props();

  const thumbnailSrc = $derived(card.thumbnailPath ? convertFileSrc(card.thumbnailPath) : null);

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
  </div>
</button>
