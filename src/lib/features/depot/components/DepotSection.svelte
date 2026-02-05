<script lang="ts" generics="T extends { id: string }">
  import type { Component } from 'svelte';
  import * as m from '$lib/paraglide/messages.js';
  import { Badge, Button } from '$lib/components';

  interface CardProps<ItemType> {
    item: ItemType;
  }

  let {
    title,
    items,
    icon: Icon,
    card: Card,
    toneClass = 'secondary',
    stickyOffset = 'var(--header-offset, 4rem)',
    emptyMessage
  }: {
    title: string;
    items: T[];
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    icon: Component<any> | any;
    card: Component<CardProps<T>>;
    toneClass?: string;
    stickyOffset?: string;
    emptyMessage: string;
  } = $props();

  let viewAll = $state(false);
  const visibleItems = $derived(viewAll || items.length <= 100 ? items : items.slice(0, 100));
  const hasOverflow = $derived(!viewAll && items.length > 100);
</script>

<section class="space-y-2 pt-2">
  <div
    class="border-surface-500/10 bg-surface-50/80 sticky z-10 border-b backdrop-blur-sm"
    style:top={stickyOffset}
  >
    <div class="flex items-center gap-3 rounded-lg px-2 py-2">
      <Badge variant={toneClass} class="flex items-center justify-center p-1.5">
        <Icon size={16} />
      </Badge>
      <div class="flex items-center gap-2">
        <h2 class="text-lg font-semibold tracking-tight">{title}</h2>
        <Badge variant="outline" class="font-mono text-xs">{items.length}</Badge>
      </div>
    </div>
  </div>

  <div class="space-y-3">
    {#if items.length === 0}
      <div class="border-surface-500/20 rounded-xl border border-dashed p-8 text-center">
        <p class="text-surface-500 text-sm">{emptyMessage}</p>
      </div>
    {:else}
      <div class="grid grid-cols-1 gap-4 lg:grid-cols-2">
        {#each visibleItems as item (item.id)}
          <Card {item} />
        {/each}
      </div>

      {#if hasOverflow}
        <div
          class="border-surface-500/10 text-surface-400 flex flex-wrap items-center justify-between gap-2 border-t pt-4 text-xs"
        >
          <p>{m.depot_overflow_note({ showing: 100, total: items.length })}</p>
          <Button type="button" variant="ghost" size="sm" onclick={() => (viewAll = true)}>
            {m.depot_view_all()}
          </Button>
        </div>
      {/if}
    {/if}
  </div>
</section>
