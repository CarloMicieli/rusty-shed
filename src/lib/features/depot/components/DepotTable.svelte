<script lang="ts" generics="T extends { id: string }">
  import type { Component } from 'svelte';
  import type { BadgeVariant } from '$lib/components/shadcn/badge/Badge.svelte';
  import * as m from '$lib/paraglide/messages.js';
  import { ArrowUpDown, ArrowUpNarrowWide, ArrowDownWideNarrow } from 'lucide-svelte';
  import { Badge, Button } from '$lib/components';

  let {
    title,
    items,
    icon: Icon,
    type,
    toneClass = 'secondary',
    stickyOffset = 'var(--header-offset, 4rem)',
    emptyMessage
  }: {
    title: string;
    items: T[];
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    icon: Component<any> | any;
    type: 'locomotive' | 'train' | 'car';
    toneClass?: BadgeVariant;
    stickyOffset?: string;
    emptyMessage: string;
  } = $props();

  let viewAll = $state(false);
  const hasOverflow = $derived(!viewAll && items.length > 100);

  // Sorting state
  let sortField = $state<string | null>(null);
  let sortDirection = $state<'asc' | 'desc'>('asc');

  function toggleSort(field: string) {
    if (sortField === field) {
      sortDirection = sortDirection === 'asc' ? 'desc' : 'asc';
    } else {
      sortField = field;
      sortDirection = 'asc';
    }
  }

  // Helper to safely access properties that might differ between types
  function getItemProps(item: T) {
    const it = item as unknown as Record<string, unknown>;
    const str = (v: unknown) => (v === undefined || v === null ? '-' : String(v));
    return {
      productCode: str(it.productCode ?? it.product_code ?? '-'),
      model: str(it.group ?? it.type ?? '-'),
      manufacturer: str(it.manufacturer ?? '-'),
      // series: str(it.seriesCode ?? it.series_code ?? '-'), // Removed as column is removed
      category: str(it.categoryLabel ?? it.category_label ?? '-'),
      roadNumber: str(it.roadNumber ?? it.road_number ?? '-'),
      railway: str(it.railwayCompany ?? it.railway_company ?? '-'),
      livery: str(it.livery ?? '-'),
      control: str(it.control ?? '-'),
      serviceLevel: str(it.serviceLevel ?? '-')
    };
  }

  const sortedItems = $derived.by(() => {
    if (!sortField) return items;

    // We sort a shallow copy to stay pure-ish regarding 'items'
    return [...items].sort((a, b) => {
      const propA = getItemProps(a)[sortField as keyof ReturnType<typeof getItemProps>];
      const propB = getItemProps(b)[sortField as keyof ReturnType<typeof getItemProps>];

      if (propA < propB) return sortDirection === 'asc' ? -1 : 1;
      if (propA > propB) return sortDirection === 'asc' ? 1 : -1;
      return 0;
    });
  });

  const visibleItems = $derived(
    viewAll || sortedItems.length <= 100 ? sortedItems : sortedItems.slice(0, 100)
  );

  // Table headers based on type
  const headers = $derived.by(() => {
    const base = [
      {
        label: 'Manufacturer',
        key: 'manufacturer',
        class: 'hidden sm:table-cell w-32 cursor-pointer hover:bg-surface-500/10'
      },
      {
        label: 'Product code',
        key: 'productCode',
        class: 'w-50 cursor-pointer hover:bg-surface-500/10'
      },
      {
        label: 'Category',
        key: 'category',
        class: 'hidden lg:table-cell w-32 cursor-pointer hover:bg-surface-500/10'
      },
      {
        label: 'Railway',
        key: 'railway',
        class: 'hidden sm:table-cell w-32 cursor-pointer hover:bg-surface-500/10'
      },
      {
        label: 'Road #',
        key: 'roadNumber',
        class: 'hidden md:table-cell w-full cursor-pointer hover:bg-surface-500/10'
      }
    ];

    // Add specific columns
    if (type === 'car') {
      base.push({
        label: 'Service',
        key: 'serviceLevel',
        class: 'hidden lg:table-cell w-24 cursor-pointer hover:bg-surface-500/10'
      });
    } else {
      // Control for locos/trains
      base.push({
        label: 'Control',
        key: 'control',
        class: 'hidden lg:table-cell w-24 text-center cursor-pointer hover:bg-surface-500/10'
      });
    }

    // Livery is always nice if space permits
    base.push({
      label: 'Livery',
      key: 'livery',
      class: 'hidden 2xl:table-cell w-32 cursor-pointer hover:bg-surface-500/10'
    });

    return base;
  });
</script>

<section class="space-y-2 pt-2">
  <div
    class="border-surface-500/10 bg-surface-50/80 sticky z-10 border-b backdrop-blur-sm"
    style:top={stickyOffset}
  >
    <div class="flex items-center gap-3 rounded-lg px-2 py-2">
      <Badge variant={toneClass} class="flex items-center justify-center p-1.5">
        {#if Icon}
          <Icon size={16} />
        {/if}
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
      <div
        class="table-container border-surface-500/10 bg-surface-900/40 overflow-hidden rounded-xl border"
      >
        <table class="table-hover table-compact table w-full">
          <thead class="bg-surface-900/60 text-surface-400 font-medium">
            <tr>
              {#each headers as col (col.label)}
                <th class="{col.class} group select-none" onclick={() => toggleSort(col.key)}>
                  <div class="flex items-center gap-1">
                    {col.label}
                    {#if sortField === col.key}
                      {#if sortDirection === 'asc'}
                        <ArrowUpNarrowWide size={14} class="opacity-70" />
                      {:else}
                        <ArrowDownWideNarrow size={14} class="opacity-70" />
                      {/if}
                    {:else}
                      <ArrowUpDown size={14} class="opacity-0 group-hover:opacity-30" />
                    {/if}
                  </div>
                </th>
              {/each}
            </tr>
          </thead>
          <tbody class="divide-surface-500/10 divide-y">
            {#each visibleItems as item (item.id)}
              {@const props = getItemProps(item)}
              <tr class="group">
                <td class="hidden align-middle text-surface-300 sm:table-cell">
                  {props.manufacturer}
                </td>
                <td class="align-middle font-medium text-surface-200">
                  <div class="font-mono text-sm">{props.productCode}</div>
                  <!-- Mobile-only details -->
                  <div class="text-surface-400 mt-0.5 space-x-1 text-xs font-normal sm:hidden">
                    <span>{props.manufacturer}</span>
                    {#if props.railway !== '-'}<span>• {props.railway}</span>{/if}
                    {#if props.roadNumber !== '-'}<span>• {props.roadNumber}</span>{/if}
                  </div>
                </td>
                <td
                  class="text-surface-400 hidden align-middle text-xs tracking-wide uppercase lg:table-cell"
                >
                  {props.category}
                </td>
                <td class="hidden align-middle text-surface-300 sm:table-cell">
                  {props.railway}
                </td>
                <td class="hidden align-middle font-mono text-sm text-surface-300 md:table-cell">
                  {props.roadNumber}
                </td>
                {#if type === 'car'}
                  <td class="text-surface-400 hidden align-middle text-sm lg:table-cell">
                    {props.serviceLevel}
                  </td>
                {:else}
                  <td class="hidden text-center align-middle lg:table-cell">
                    {#if props.control !== '-'}
                      <Badge
                        variant="secondary"
                        class="max-w-[120px] truncate font-mono text-xs"
                        title={props.control}>{props.control}</Badge
                      >
                    {:else}
                      <span class="text-surface-500">-</span>
                    {/if}
                  </td>
                {/if}
                <td class="text-surface-400 hidden align-middle text-sm 2xl:table-cell">
                  {props.livery}
                </td>
              </tr>
            {/each}
          </tbody>
        </table>
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
