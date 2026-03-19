<script lang="ts">
  import type { DigitalRollingStockView } from '$lib/bindings';
  import * as m from '$lib/paraglide/messages';
  import { Search, X, Filter, Microchip } from 'lucide-svelte';
  import { Badge, Button } from '$lib/components';

  interface Props {
    rollingStocks: DigitalRollingStockView[];
    filterText?: string;
    loading?: boolean;
    onFilterChange?: (text: string) => void;
    onEdit?: (stock: DigitalRollingStockView) => void;
    onInstallDecoder?: () => void;
  }

  let {
    rollingStocks = [],
    filterText = $bindable(''),
    loading = false,
    onFilterChange,
    onEdit,
    onInstallDecoder
  }: Props = $props();

  function handleFilterInput(event: Event) {
    const target = event.target as HTMLInputElement;
    filterText = target.value;
    onFilterChange?.(target.value);
  }

  function clearFilter() {
    filterText = '';
    onFilterChange?.('');
  }

  function handleEdit(stock: DigitalRollingStockView) {
    onEdit?.(stock);
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
      <div class="table-container">
        <table class="table-hover table">
          <thead>
            <tr>
              <th>{m.digital_roster_table_address()}</th>
              <th>{m.digital_roster_table_road_number()}</th>
              <th>{m.digital_roster_table_category()}</th>
              <th>{m.digital_roster_table_railway()}</th>
              <th>{m.digital_roster_table_scale()}</th>
              <th>{m.digital_roster_table_power()}</th>
              <th>{m.digital_roster_table_decoder()}</th>
              <th class="text-right">{m.app_settings()}</th>
            </tr>
          </thead>
          <tbody>
            {#each rollingStocks as stock (stock.id)}
              <tr>
                <td class="font-mono font-semibold">{stock.dcc_address}</td>
                <td>{stock.road_number ?? '-'}</td>
                <td>
                  <Badge variant="secondary">
                    {stock.category}
                  </Badge>
                </td>
                <td>{stock.railway_company_name ?? '-'}</td>
                <td>{stock.scale ?? '-'}</td>
                <td>{stock.power_method ?? '-'}</td>
                <td class="text-sm">
                  {stock.decoder.manufacturer}
                  {stock.decoder.product_code}
                </td>
                <td class="text-right">
                  <Button type="button" variant="ghost" size="sm" onclick={() => handleEdit(stock)}>
                    {m.digital_roster_edit_address()}
                  </Button>
                </td>
              </tr>
            {/each}
          </tbody>
        </table>
      </div>
    {/if}
  </div>
</div>
