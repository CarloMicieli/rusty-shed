<script lang="ts" generics="T extends { id: string }">
  import { ArrowUpNarrowWide, ArrowDownWideNarrow } from 'lucide-svelte';
  import * as m from '$lib/paraglide/messages.js';
  import { Button } from '$lib/components';
  import DepotThumbnail from './DepotThumbnail.svelte';

  interface Props {
    items: T[];
  }

  const { items }: Props = $props();

  let viewAll = $state(false);
  const hasOverflow = $derived(!viewAll && items.length > 50);

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

  // Helper to safely access properties
  function getItemProps(item: T) {
    const it = item as unknown as Record<string, unknown>;
    const str = (v: unknown) => (v === undefined || v === null ? '-' : String(v));

    return {
      id: str(it.id),
      productCode: str(it.productCode ?? ''),
      manufacturer: str(it.manufacturer ?? ''),
      railwayModelId: str(it.railwayModelId ?? ''),
      category: str(it.categoryLabel ?? '-'),
      roadNumber: str(it.roadNumber ?? '-'),
      railway: str(it.railwayCompany ?? '-'),
      livery: str(it.livery ?? '-'),
      control: str(it.control ?? '-'),
      dccAddress: it.dccAddress as number | null,
      seriesCode: str(it.seriesCode ?? '-'),
      series: str(it.series ?? '-')
    };
  }

  const sortedItems = $derived.by(() => {
    if (!sortField) return items;

    return [...items].sort((a, b) => {
      const propA = getItemProps(a)[sortField as keyof ReturnType<typeof getItemProps>];
      const propB = getItemProps(b)[sortField as keyof ReturnType<typeof getItemProps>];

      if (propA == null) return 1;
      if (propB == null) return -1;
      if (propA < propB) return sortDirection === 'asc' ? -1 : 1;
      if (propA > propB) return sortDirection === 'asc' ? 1 : -1;
      return 0;
    });
  });

  const visibleItems = $derived(
    viewAll || sortedItems.length <= 50 ? sortedItems : sortedItems.slice(0, 50)
  );

  const headers = [
    { label: '', key: '', class: 'w-20' },
    { label: m.depot_manufacturer(), key: 'manufacturer', class: 'w-32 hidden lg:table-cell' },
    { label: m.depot_product_code(), key: 'productCode', class: 'w-28 hidden lg:table-cell' },
    { label: m.depot_company(), key: 'railway', class: 'w-36' },
    { label: m.depot_series_code(), key: 'seriesCode', class: 'w-36 hidden xl:table-cell' },
    { label: m.depot_series(), key: 'series', class: 'w-40 hidden 2xl:table-cell' },
    { label: m.depot_road_number(), key: 'roadNumber', class: 'min-w-[140px]' },
    { label: m.depot_type(), key: 'category', class: 'w-28 hidden md:table-cell' },
    { label: m.depot_livery(), key: 'livery', class: 'w-36 hidden md:table-cell' },
    { label: m.depot_dcc_address(), key: 'dccAddress', class: 'w-20' }
  ];
</script>

<div class="px-6 py-4">
  <table class="w-full border-separate border-spacing-y-2">
    <thead>
      <tr>
        {#each headers as col (col.label)}
          <th
            class="px-4 py-2 text-left text-[10px] font-bold tracking-[0.2em] text-muted-foreground uppercase {col.class}"
            onclick={col.key ? () => toggleSort(col.key) : undefined}
            class:cursor-pointer={!!col.key}
          >
            <div class="flex items-center gap-2">
              {col.label}
              {#if sortField === col.key && col.key}
                {#if sortDirection === 'asc'}
                  <ArrowUpNarrowWide size={12} class="text-primary" />
                {:else}
                  <ArrowDownWideNarrow size={12} class="text-primary" />
                {/if}
              {/if}
            </div>
          </th>
        {/each}
      </tr>
    </thead>
    <tbody>
      {#each visibleItems as item (item.id)}
        {@render rowItem(item)}
      {/each}
    </tbody>
  </table>

  {#if hasOverflow}
    <div class="mt-8 flex items-center justify-between border-t border-border py-4">
      <p class="font-mono text-[10px] tracking-widest text-muted-foreground uppercase">
        {m.depot_overflow_note({ showing: visibleItems.length, total: items.length })}
      </p>
      <Button
        variant="ghost"
        size="sm"
        onclick={() => (viewAll = true)}
        class="text-primary hover:bg-primary/5"
      >
        {m.depot_view_all()}
      </Button>
    </div>
  {/if}
</div>

{#snippet rowItem(item: T)}
  {@const props = getItemProps(item)}
  <tr class="group transition-all duration-150">
    <!-- Image Preview -->
    <td
      class="rounded-l-sm border-y border-l-2 border-border border-l-transparent bg-card px-2 py-3 group-hover:border-l-primary group-hover:bg-primary/15"
    >
      <DepotThumbnail railwayModelId={props.railwayModelId} productCode={props.productCode} />
    </td>

    <!-- Manufacturer -->
    <td
      class="hidden border-y border-border bg-card px-4 py-3 group-hover:bg-primary/15 lg:table-cell"
    >
      <span class="text-sm text-foreground">{props.manufacturer}</span>
    </td>

    <!-- Product Code -->
    <td
      class="hidden border-y border-border bg-card px-4 py-3 group-hover:bg-primary/15 lg:table-cell"
    >
      <span class="font-mono text-sm text-foreground">{props.productCode}</span>
    </td>

    <!-- Railway Company -->
    <td class="border-y border-border bg-card px-4 py-3 group-hover:bg-primary/15">
      <span class="text-sm font-bold text-primary">{props.railway}</span>
    </td>

    <!-- Series Code -->
    <td
      class="hidden border-y border-border bg-card px-4 py-3 group-hover:bg-primary/15 xl:table-cell"
    >
      <span class="font-mono text-sm text-foreground">{props.seriesCode}</span>
    </td>

    <!-- Series -->
    <td
      class="hidden border-y border-border bg-card px-4 py-3 group-hover:bg-primary/15 2xl:table-cell"
    >
      <span class="text-sm text-muted-foreground">{props.series}</span>
    </td>

    <!-- Road Number -->
    <td class="border-y border-border bg-card px-4 py-3 group-hover:bg-primary/15">
      <span class="font-mono text-lg font-bold text-foreground">{props.roadNumber}</span>
    </td>

    <!-- Type -->
    <td
      class="hidden border-y border-border bg-card px-4 py-3 group-hover:bg-primary/15 md:table-cell"
    >
      <span class="text-[10px] tracking-tighter text-muted-foreground uppercase"
        >{props.category}</span
      >
    </td>

    <!-- Livery Pill -->
    <td
      class="hidden border-y border-border bg-card px-4 py-3 group-hover:bg-primary/15 md:table-cell"
    >
      {#if props.livery !== '-'}
        <span
          class="rounded-full border border-primary px-2 py-0.5 font-mono text-[10px] text-primary"
        >
          {props.livery}
        </span>
      {:else}
        <span class="font-mono text-[10px] text-muted-foreground">—</span>
      {/if}
    </td>

    <!-- DCC Gauge (last column) -->
    <td
      class="rounded-r-sm border-y border-r border-border bg-card px-4 py-3 group-hover:bg-primary/15"
    >
      <div
        class="variant-steampunk-gauge h-8 w-8 {props.dccAddress !== null
          ? 'border-emerald-500 text-emerald-500'
          : props.control !== '-' && props.control !== 'analog' && props.control !== 'ANALOG'
            ? 'border-primary text-primary'
            : 'border-border text-muted-foreground'}"
      >
        <span class="font-mono text-[10px]">
          {props.dccAddress !== null
            ? String(props.dccAddress)
            : props.control !== '-' && props.control !== 'analog' && props.control !== 'ANALOG'
              ? 'DCC'
              : '—'}
        </span>
      </div>
    </td>
  </tr>
{/snippet}
