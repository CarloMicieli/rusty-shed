<script lang="ts">
  import type { ComponentType } from 'svelte';
  import * as m from '$lib/paraglide/messages.js';

  type WithId = { id: string };

  let {
    title,
    items,
    icon: Icon,
    card: Card,
    toneClass = 'variant-filled-surface',
    stickyOffset = 'var(--header-offset, 4rem)',
    emptyMessage
  } = $props<{
    title: string;
    items: WithId[];
    icon: ComponentType | any;
    card: ComponentType | any;
    toneClass?: string;
    stickyOffset?: string;
    emptyMessage: string;
  }>();

  let viewAll = $state(false);

  const visibleItems = $derived(viewAll || items.length <= 100 ? items : items.slice(0, 100));

  const hasOverflow = $derived(!viewAll && items.length > 100);
</script>

<section class="space-y-2 pt-2">
  <div class="sticky z-10 bg-surface-50/80 backdrop-blur-sm" style={`top: ${stickyOffset}`}>
    <div class="flex items-center gap-3 rounded-lg px-2 py-2">
      <span class={`badge ${toneClass} flex items-center justify-center`}>
        <Icon size={16} />
      </span>
      <div class="flex items-center gap-2">
        <h2 class="text-lg font-semibold tracking-tight">{title}</h2>
        <span class="variant-soft-surface badge font-mono text-xs">{items.length}</span>
      </div>
    </div>
  </div>

  <div class="space-y-3">
    {#if items.length === 0}
      <p class="text-sm text-surface-500">{emptyMessage}</p>
    {:else}
      <div class="grid grid-cols-1 gap-4 lg:grid-cols-2">
        {#each visibleItems as item (item.id)}
          <Card {item} />
        {/each}
      </div>
      {#if hasOverflow}
        <div class="flex flex-wrap items-center justify-between gap-2 text-xs text-surface-400">
          <span>{m.depot_overflow_note({ showing: 100, total: items.length })}</span>
          <button class="variant-ghost-primary btn btn-sm" onclick={() => (viewAll = true)}>
            {m.depot_view_all()}
          </button>
        </div>
      {/if}
    {/if}
  </div>
</section>
