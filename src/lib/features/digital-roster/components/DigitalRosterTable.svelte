<script lang="ts">
  import type { DigitalRollingStockView } from '$lib/bindings';
  import * as m from '$lib/paraglide/messages';
  import { Search, X, Filter, Microchip, Cpu, Trash2 } from 'lucide-svelte';
  import { Badge, Button } from '$lib/components';
  import InPlaceDccAddressEdit from './InPlaceDccAddressEdit.svelte';

  interface Props {
    rollingStocks: DigitalRollingStockView[];
    filterText?: string;
    loading?: boolean;
    onFilterChange?: (text: string) => void;
    onInstallDecoder?: () => void;
    onSaveAddress?: (id: string, address: number) => Promise<boolean>;
    onChangeDecoder?: (stock: DigitalRollingStockView) => void;
    onDelete?: (stock: DigitalRollingStockView) => void;
  }

  let {
    rollingStocks = [],
    filterText = $bindable(''),
    loading = false,
    onFilterChange,
    onInstallDecoder,
    onSaveAddress,
    onChangeDecoder,
    onDelete
  }: Props = $props();

  // Tracks which row is actively being edited so all others deactivate
  let editingId = $state<string | null>(null);

  // Shared grid column template applied identically to header and every data row.
  // Compact view (< @3xl): hides Scale and Power columns.
  // Wide view (>= @3xl): reveals Scale (60px) and Power (90px) between Railway and Decoder.
  const ROW =
    'grid items-center gap-x-4 px-4 py-2.5 grid-cols-[80px_minmax(90px,1fr)_110px_minmax(90px,1fr)_minmax(120px,1fr)_210px] @3xl:grid-cols-[80px_minmax(90px,1fr)_110px_minmax(90px,1fr)_60px_90px_minmax(120px,1fr)_210px]';

  function handleFilterInput(event: Event) {
    const target = event.target as HTMLInputElement;
    filterText = target.value;
    onFilterChange?.(target.value);
  }

  function clearFilter() {
    filterText = '';
    onFilterChange?.('');
  }
</script>

<div class="space-y-4">
  {#if rollingStocks.length > 0 || filterText || loading}
    <!-- Integrated Search Toolbar -->
    <div class="rounded-lg bg-zinc-900/50 p-3">
      <div class="flex items-center gap-2">
        <div
          class="flex flex-1 items-center gap-2 rounded-md border border-border bg-background px-3"
        >
          <Search class="h-5 w-5 text-muted-foreground" />
          <input
            type="text"
            class="flex-1 bg-transparent py-2 text-sm outline-none placeholder:text-muted-foreground"
            placeholder={m.digital_roster_filter_placeholder()}
            value={filterText}
            oninput={handleFilterInput}
          />
          {#if filterText}
            <button
              type="button"
              class="rounded-sm p-1 transition-colors hover:bg-zinc-800"
              onclick={clearFilter}
            >
              <X class="h-4 w-4 text-muted-foreground" />
            </button>
          {/if}
        </div>
        <Button variant="outline" size="sm">
          <Filter class="h-4 w-4" />
          <span>Filter</span>
        </Button>
      </div>
    </div>
  {/if}

  <!-- Content Area -->
  <div class="space-y-4 rounded-lg border border-white/10 bg-black/20 p-4">
    {#if loading}
      <div class="flex items-center justify-center py-12">
        <div class="border-primary-500 h-12 w-12 animate-spin rounded-full border-b-2"></div>
      </div>
    {:else if rollingStocks.length === 0}
      <!-- Empty State -->
      <div
        class="flex flex-col items-center justify-center gap-8 rounded-3xl border border-white/5 bg-[#0c0c0c]/50 px-4 py-24 text-center"
      >
        <div class="relative">
          <div class="absolute inset-0 rounded-full bg-zinc-500/10 blur-3xl"></div>
          <div
            class="relative flex h-32 w-32 items-center justify-center rounded-full border border-white/10 bg-zinc-900/50"
          >
            <Microchip size={56} class="text-zinc-600 opacity-50" />
          </div>
        </div>
        <div class="flex max-w-sm flex-col items-center gap-3 text-center">
          <h3 class="text-2xl font-bold text-zinc-200">{m.digital_roster_empty()}</h3>
          <p class="text-sm leading-relaxed text-zinc-500">{m.digital_roster_empty_message()}</p>
        </div>
        {#if onInstallDecoder}
          <button
            type="button"
            class="group relative mt-2 inline-flex cursor-pointer items-center gap-3 overflow-hidden rounded-full bg-amber-500 px-8 py-4 font-bold tracking-wide text-black transition-all hover:scale-105 hover:bg-amber-400 hover:shadow-[0_0_20px_rgba(245,158,11,0.4)] active:scale-95"
            onclick={onInstallDecoder}
          >
            <div
              class="absolute inset-0 translate-y-full bg-white/20 transition-transform duration-300 group-hover:translate-y-0"
            ></div>
            <Microchip class="h-5 w-5" />
            <span>{m.digital_roster_install_decoder()}</span>
          </button>
        {/if}
      </div>
    {:else}
      <!-- CSS Grid data table -->
      <div class="@container overflow-hidden rounded-lg">
        <!-- Header row -->
        <div
          class="{ROW} border-b border-white/10 bg-zinc-900/60 text-xs font-medium tracking-wide text-zinc-500 uppercase"
          role="row"
        >
          <div role="columnheader">{m.digital_roster_table_address()}</div>
          <div role="columnheader">{m.digital_roster_table_road_number()}</div>
          <div role="columnheader">{m.digital_roster_table_category()}</div>
          <div role="columnheader">{m.digital_roster_table_railway()}</div>
          <div role="columnheader" class="hidden @3xl:block">{m.digital_roster_table_scale()}</div>
          <div role="columnheader" class="hidden @3xl:block">{m.digital_roster_table_power()}</div>
          <div role="columnheader">{m.digital_roster_table_decoder()}</div>
          <div role="columnheader"></div>
        </div>

        <!-- Data rows -->
        <div class="divide-y divide-white/5" role="rowgroup">
          {#each rollingStocks as stock (stock.id)}
            {@const isRowEditing = editingId === stock.id}
            <div
              class="{ROW} transition-colors {isRowEditing
                ? 'bg-amber-950/20'
                : 'hover:bg-white/[0.02]'}"
              role="row"
            >
              <!-- DCC Address (editable) -->
              <div role="cell">
                <InPlaceDccAddressEdit
                  value={stock.dcc_address}
                  excludeId={stock.id}
                  allAddresses={rollingStocks}
                  onSave={async (addr) => onSaveAddress?.(stock.id, addr) ?? false}
                  deactivate={editingId !== null && editingId !== stock.id}
                  onEditStart={() => (editingId = stock.id)}
                  onEditEnd={() => {
                    if (editingId === stock.id) editingId = null;
                  }}
                />
              </div>

              <!-- Road Number -->
              <div role="cell" class="font-bold text-white">
                {stock.road_number ?? '-'}
              </div>

              <!-- Category -->
              <div role="cell">
                <Badge variant="secondary">{stock.category}</Badge>
              </div>

              <!-- Railway -->
              <div role="cell" class="truncate text-sm text-zinc-400">
                {stock.railway_company_name ?? '-'}
              </div>

              <!-- Scale (hidden below @3xl) -->
              <div role="cell" class="hidden text-sm text-zinc-400 @3xl:block">
                {stock.scale ?? '-'}
              </div>

              <!-- Power Method (hidden below @3xl) -->
              <div role="cell" class="hidden text-sm text-zinc-400 @3xl:block">
                {stock.power_method ?? '-'}
              </div>

              <!-- Decoder -->
              <div role="cell" class="text-sm text-zinc-400">
                {stock.decoder.manufacturer}
                {stock.decoder.product_code}
              </div>

              <!-- Settings column -->
              <div role="cell">
                <div class="flex items-center justify-end gap-1">
                  <!-- Change Decoder button -->
                  <button
                    type="button"
                    class="inline-flex items-center gap-1.5 rounded-md border border-amber-600/40 bg-amber-950/40 px-2.5 py-1 text-xs font-medium text-amber-400 transition-colors hover:border-amber-500/60 hover:bg-amber-900/50"
                    onclick={() => onChangeDecoder?.(stock)}
                  >
                    <Cpu class="h-3.5 w-3.5" />
                    {m.digital_roster_change_decoder()}
                  </button>

                  <!-- Delete button -->
                  <button
                    type="button"
                    class="rounded p-1 text-zinc-600 transition-colors hover:bg-red-950/50 hover:text-red-400"
                    onclick={() => onDelete?.(stock)}
                    aria-label={m.common_delete()}
                  >
                    <Trash2 class="h-4 w-4" />
                  </button>
                </div>
              </div>
            </div>
          {/each}
        </div>
      </div>
    {/if}
  </div>
</div>
