<script lang="ts" generics="T extends { id: string }">
  import { ArrowUpNarrowWide, ArrowDownWideNarrow } from 'lucide-svelte';
  import * as m from '$lib/paraglide/messages.js';
  import { Button } from '$lib/components';
  import DepotThumbnail from './DepotThumbnail.svelte';

  let {
    items
  }: {
    items: T[];
  } = $props();

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

    const manufacturer = str(it.manufacturer ?? '');
    const productCode = str(it.productCode ?? '');
    const railwayModelId = str(it.railwayModelId ?? '');

    return {
      id: str(it.id),
      productCode,
      manufacturer,
      railwayModelId,
      category: str(it.categoryLabel ?? '-'),
      roadNumber: str(it.roadNumber ?? '-'),
      railway: str(it.railwayCompany ?? '-'),
      epoch: str(it.epoch ?? '-'),
      livery: str(it.livery ?? '-'),
      control: str(it.control ?? '-'),
      dccAddress: it.dccAddress as number | null,
      serviceLevel: str(it.serviceLevel ?? '-'),
      // Status simulation (or use real data if available)
      status: it.depot ? 'On Track' : 'In Storage'
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

  // Table headers
  const headers = [
    { label: 'STATUS', key: 'status', class: 'w-16' },
    { label: 'VISUAL', key: '', class: 'w-24' },
    { label: m.depot_company(), key: 'railway', class: 'w-40 hidden lg:table-cell' },
    { label: m.depot_road_number(), key: 'roadNumber', class: 'min-w-[200px]' },
    { label: m.depot_dcc_address(), key: 'dccAddress', class: 'w-32' },
    { label: m.depot_type(), key: 'control', class: 'w-32 hidden md:table-cell' }
  ];
</script>

<div class="px-6 py-4">
  <table class="w-full border-separate border-spacing-y-3">
    <thead>
      <tr>
        {#each headers as col (col.label)}
          <th
            class="px-4 py-2 text-left text-[10px] font-bold tracking-[0.2em] text-zinc-500 uppercase {col.class}"
            onclick={col.key ? () => toggleSort(col.key) : undefined}
            class:cursor-pointer={col.key}
          >
            <div class="flex items-center gap-2">
              {col.label}
              {#if sortField === col.key && col.key}
                {#if sortDirection === 'asc'}
                  <ArrowUpNarrowWide size={12} class="text-amber-500" />
                {:else}
                  <ArrowDownWideNarrow size={12} class="text-amber-500" />
                {/if}
              {/if}
            </div>
          </th>
        {/each}
      </tr>
    </thead>
    <tbody>
      {#each visibleItems as item (item.id)}
        {@const props = getItemProps(item)}
        <tr class="group transition-all duration-300">
          <!-- Status LED -->
          <td
            class="relative rounded-l-xl border-y border-l border-white/5 bg-white/5 px-4 py-4 group-hover:border-amber-500/30"
          >
            <div
              class="absolute top-1/4 bottom-1/4 left-0 w-1 bg-transparent transition-all group-hover:bg-amber-500"
            ></div>
            <div class="flex items-center justify-center">
              <div
                class="h-2.5 w-2.5 rounded-full shadow-[0_0_8px_rgba(0,0,0,0.5)] transition-all"
                class:bg-emerald-500={props.status === 'On Track'}
                class:shadow-emerald-500-glow={props.status === 'On Track'}
                class:bg-zinc-700={props.status !== 'On Track'}
              ></div>
            </div>
          </td>

          <!-- Thumbnail -->
          <td class="border-y border-white/5 bg-white/5 px-2 py-4 group-hover:border-amber-500/30">
            <DepotThumbnail railwayModelId={props.railwayModelId} productCode={props.productCode} />
          </td>

          <!-- Railway -->
          <td
            class="hidden border-y border-white/5 bg-white/5 px-4 py-4 group-hover:border-amber-500/30 lg:table-cell"
          >
            <span class="text-xs font-semibold text-zinc-400">{props.railway}</span>
          </td>

          <!-- Model Info -->
          <td class="border-y border-white/5 bg-white/5 px-4 py-4 group-hover:border-amber-500/30">
            <div class="flex flex-col">
              <span
                class="font-mono text-base font-bold text-white transition-colors group-hover:text-amber-500"
                >{props.roadNumber}</span
              >
              <div class="mt-0.5 flex items-center gap-1.5">
                <span class="text-[10px] font-bold tracking-wider text-zinc-500 uppercase"
                  >{props.manufacturer}</span
                >
                <span class="h-1 w-1 rounded-full bg-zinc-700"></span>
                <span class="font-mono text-[10px] tracking-tighter text-zinc-500 uppercase"
                  >{props.productCode}</span
                >
              </div>
            </div>
          </td>

          <!-- DCC Address -->
          <td class="border-y border-white/5 bg-white/5 px-4 py-4 group-hover:border-amber-500/30">
            {#if props.dccAddress !== null}
              <div class="flex items-center gap-2">
                <div
                  class="flex h-7 w-12 items-center justify-center rounded border border-amber-500/30 bg-amber-500/10"
                >
                  <span class="font-mono text-sm font-bold text-amber-500">{props.dccAddress}</span>
                </div>
              </div>
            {:else}
              <span class="font-mono text-xs text-zinc-700">---</span>
            {/if}
          </td>

          <!-- Control System -->
          <td
            class="hidden rounded-r-xl border-y border-r border-white/5 bg-white/5 px-4 py-4 group-hover:border-amber-500/30 md:table-cell"
          >
            <span class="font-mono text-[10px] font-bold tracking-widest text-zinc-400 uppercase"
              >{props.control.replace('_', ' ')}</span
            >
          </td>
        </tr>
      {/each}
    </tbody>
  </table>

  {#if hasOverflow}
    <div class="mt-8 flex items-center justify-between border-t border-white/5 py-4">
      <p class="font-mono text-[10px] tracking-widest text-zinc-500 uppercase">
        {m.depot_overflow_note({ showing: visibleItems.length, total: items.length })}
      </p>
      <Button
        variant="ghost"
        size="sm"
        onclick={() => (viewAll = true)}
        class="text-amber-500 hover:bg-amber-500/5"
      >
        {m.depot_view_all()}
      </Button>
    </div>
  {/if}
</div>

<style>
  .shadow-emerald-500-glow {
    box-shadow:
      0 0 10px rgba(16, 185, 129, 0.4),
      0 0 20px rgba(16, 185, 129, 0.2);
  }
</style>
