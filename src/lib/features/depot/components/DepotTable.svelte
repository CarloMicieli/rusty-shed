<script lang="ts" generics="T extends { id: string }">
  import {
    ArrowUpNarrowWide,
    ArrowDownWideNarrow,
    Cpu,
    Trash2,
    Settings2,
    Info,
    TrainFront
  } from 'lucide-svelte';
  import * as m from '$lib/paraglide/messages.js';
  import { Button } from '$lib/components';
  import { convertFileSrc } from '@tauri-apps/api/core';
  import { Sheet } from '$lib/components/ui/sheet';

  let {
    items
  }: {
    items: T[];
  } = $props();

  let viewAll = $state(false);
  const hasOverflow = $derived(!viewAll && items.length > 50);

  let selectedItemId = $state<string | null>(null);

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

    // Construct image path: models/Manufacturer_ProductCode.jpg
    const manufacturer = str(it.manufacturer ?? '');
    const productCode = str(it.productCode ?? '');
    const imagePath = `models/${manufacturer}_${productCode}.jpg`;

    return {
      id: str(it.id),
      productCode,
      manufacturer,
      category: str(it.categoryLabel ?? '-'),
      roadNumber: str(it.roadNumber ?? '-'),
      railway: str(it.railwayCompany ?? '-'),
      epoch: str(it.epoch ?? '-'),
      livery: str(it.livery ?? '-'),
      control: str(it.control ?? '-'),
      dccAddress: it.dccAddress as number | null,
      serviceLevel: str(it.serviceLevel ?? '-'),
      imagePath,
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
    { label: 'VISUAL', key: 'imagePath', class: 'w-24' },
    { label: 'ROAD NUMBER / MOD. INFO', key: 'roadNumber', class: 'min-w-[200px]' },
    { label: 'DCC ADDR', key: 'dccAddress', class: 'w-32' },
    { label: 'SYSTEM', key: 'control', class: 'w-32 hidden md:table-cell' },
    { label: 'RAILWAY', key: 'railway', class: 'w-40 hidden lg:table-cell' },
    { label: 'ACTIONS', key: null, class: 'w-32 text-right' }
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
                  <ArrowUpNarrowWide size={12} class="text-[#f59e0b]" />
                {:else}
                  <ArrowDownWideNarrow size={12} class="text-[#f59e0b]" />
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
            class="relative rounded-l-xl border-y border-l border-white/5 bg-white/5 px-4 py-4 group-hover:border-[#f59e0b]/30"
          >
            <div
              class="absolute top-1/4 bottom-1/4 left-0 w-1 bg-transparent transition-all group-hover:bg-[#f59e0b]"
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
          <td class="border-y border-white/5 bg-white/5 px-2 py-4 group-hover:border-[#f59e0b]/30">
            <div
              class="flex h-10 w-16 items-center justify-center overflow-hidden rounded border border-white/10 bg-black"
            >
              <img
                src={convertFileSrc(props.imagePath)}
                alt={props.productCode}
                class="h-full w-full object-cover contrast-125 grayscale transition-all group-hover:contrast-100 group-hover:grayscale-0"
                onerror={(e) => ((e.currentTarget as HTMLImageElement).style.display = 'none')}
              />
              <TrainFront size={16} class="text-zinc-800" />
            </div>
          </td>

          <!-- Model Info -->
          <td class="border-y border-white/5 bg-white/5 px-4 py-4 group-hover:border-[#f59e0b]/30">
            <div class="flex flex-col">
              <span
                class="font-mono text-base font-bold text-white transition-colors group-hover:text-[#f59e0b]"
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
          <td class="border-y border-white/5 bg-white/5 px-4 py-4 group-hover:border-[#f59e0b]/30">
            {#if props.dccAddress !== null}
              <div class="flex items-center gap-2">
                <div
                  class="flex h-7 w-12 items-center justify-center rounded border border-[#f59e0b]/30 bg-[#f59e0b]/10"
                >
                  <span class="font-mono text-sm font-bold text-[#f59e0b]">{props.dccAddress}</span>
                </div>
              </div>
            {:else}
              <span class="font-mono text-xs text-zinc-700">---</span>
            {/if}
          </td>

          <!-- Control System -->
          <td
            class="hidden border-y border-white/5 bg-white/5 px-4 py-4 group-hover:border-[#f59e0b]/30 md:table-cell"
          >
            <span class="font-mono text-[10px] font-bold tracking-widest text-zinc-400 uppercase"
              >{props.control.replace('_', ' ')}</span
            >
          </td>

          <!-- Railway -->
          <td
            class="hidden border-y border-white/5 bg-white/5 px-4 py-4 group-hover:border-[#f59e0b]/30 lg:table-cell"
          >
            <span class="text-xs font-semibold text-zinc-400">{props.railway}</span>
          </td>

          <!-- Actions -->
          <td
            class="rounded-r-xl border-y border-r border-white/5 bg-white/5 px-4 py-4 text-right group-hover:border-[#f59e0b]/30"
          >
            <Button
              variant="ghost"
              size="icon"
              class="h-8 w-8 text-zinc-500 hover:bg-[#f59e0b]/10 hover:text-[#f59e0b]"
              onclick={() => (selectedItemId = props.id)}
            >
              <Settings2 size={16} />
            </Button>

            <Sheet
              open={selectedItemId === props.id}
              onOpenChange={(open) => !open && (selectedItemId = null)}
              class="border-white/10 bg-[#0c0c0c]/90 backdrop-blur-xl"
            >
              <div class="p-6 text-white">
                <div class="mb-6">
                  <h3 class="font-mono text-sm tracking-widest text-[#f59e0b] uppercase">
                    System Operations
                  </h3>
                  <p class="text-xs text-zinc-500">
                    {props.roadNumber} ({props.manufacturer}
                    {props.productCode})
                  </p>
                </div>
                <div class="space-y-8 py-10">
                  <div class="rounded-lg border border-white/5 bg-white/5 p-4">
                    <h4 class="mb-4 text-[10px] font-bold tracking-widest text-zinc-500 uppercase">
                      DCC Controller
                    </h4>
                    <div class="flex items-center justify-between">
                      <div class="flex flex-col gap-1">
                        <span class="text-xs text-zinc-400">{m.model_rolling_stock_digital_address()}</span>
                        <span class="font-mono text-2xl font-bold text-[#f59e0b]"
                          >{props.dccAddress ?? '—'}</span
                        >
                      </div>
                      <Button class="bg-[#f59e0b] text-black">Update ADDR</Button>
                    </div>
                  </div>

                  <div class="grid grid-cols-2 gap-4">
                    <Button variant="outline" class="border-white/10 hover:bg-white/5">
                      <Cpu size={14} class="mr-2" />
                      Diagnostics
                    </Button>
                    <Button variant="outline" class="border-white/10 hover:bg-white/5">
                      <Info size={14} class="mr-2" />
                      Model Logs
                    </Button>
                  </div>

                  <div class="pt-10">
                    <Button
                      variant="outline"
                      class="w-full border-red-500/20 text-red-500 hover:bg-red-500/10"
                    >
                      <Trash2 size={14} class="mr-2" />
                      Decommission from Depot
                    </Button>
                  </div>
                </div>
                <div class="mt-8 flex justify-end gap-3 border-t border-white/5 pt-8">
                  <Button variant="ghost" onclick={() => (selectedItemId = null)}>{m.common_cancel()}</Button>
                </div>
              </div>
            </Sheet>
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
        class="text-[#f59e0b] hover:bg-[#f59e0b]/5"
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
