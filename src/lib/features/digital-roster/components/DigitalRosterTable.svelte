<script lang="ts">
  import type { DigitalRollingStockView } from '$lib/bindings';
  import * as m from '$lib/paraglide/messages';
  import { Search, X } from 'lucide-svelte';

  interface Props {
    rollingStocks: DigitalRollingStockView[];
    filterText?: string;
    loading?: boolean;
    onFilterChange?: (text: string) => void;
    onEdit?: (stock: DigitalRollingStockView) => void;
  }

  let {
    rollingStocks = [],
    filterText = $bindable(''),
    loading = false,
    onFilterChange,
    onEdit
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

<div class="card space-y-4 p-6">
  <!-- Filter Input -->
  <div class="flex items-center gap-2">
    <div class="input-group-divider input-group flex-1 grid-cols-[auto_1fr_auto]">
      <div class="input-group-shim">
        <Search class="h-5 w-5" />
      </div>
      <input
        type="text"
        placeholder={m.digital_roster_filter_placeholder()}
        value={filterText}
        oninput={handleFilterInput}
        class="input"
      />
      {#if filterText}
        <button type="button" class="variant-filled-surface btn-icon" onclick={clearFilter}>
          <X class="h-4 w-4" />
        </button>
      {/if}
    </div>
  </div>

  <!-- Table -->
  {#if loading}
    <div class="flex items-center justify-center py-12">
      <div class="border-primary-500 h-12 w-12 animate-spin rounded-full border-b-2"></div>
    </div>
  {:else if rollingStocks.length === 0}
    <div class="space-y-2 py-12 text-center">
      <p class="text-xl font-semibold opacity-75">{m.digital_roster_empty()}</p>
      <p class="text-sm opacity-60">{m.digital_roster_empty_message()}</p>
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
                <span class="variant-soft-primary badge">
                  {stock.category}
                </span>
              </td>
              <td>{stock.railway_company_name ?? '-'}</td>
              <td>{stock.scale ?? '-'}</td>
              <td>{stock.power_method ?? '-'}</td>
              <td class="text-sm">
                {stock.decoder.manufacturer}
                {stock.decoder.product_code}
              </td>
              <td class="text-right">
                <button
                  type="button"
                  class="variant-ghost-surface btn btn-sm"
                  onclick={() => handleEdit(stock)}
                >
                  {m.digital_roster_edit_address()}
                </button>
              </td>
            </tr>
          {/each}
        </tbody>
      </table>
    </div>
  {/if}
</div>
