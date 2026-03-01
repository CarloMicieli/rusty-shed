<script lang="ts">
  import type { DigitalRollingStockView } from '$lib/bindings';
  import * as m from '$lib/paraglide/messages';
  import { Search, X, Filter, Microchip } from 'lucide-svelte';
  import { Badge, Button, Card, CardContent } from '$lib/components';

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

<div class="space-y-4">
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

  <!-- Content Area -->
  <div class="space-y-4 rounded-lg border border-white/10 bg-black/20 p-4">
    {#if loading}
      <div class="flex items-center justify-center py-12">
        <div class="border-primary-500 h-12 w-12 animate-spin rounded-full border-b-2"></div>
      </div>
    {:else if rollingStocks.length === 0}
      <!-- Enhanced Empty State -->
      <Card class="border-2 border-dashed border-white/20 bg-transparent">
        <CardContent class="flex flex-col items-center justify-center py-16">
          <div class="mb-4 rounded-full bg-zinc-900/50 p-6">
            <Microchip class="h-16 w-16 text-muted-foreground" />
          </div>
          <p class="text-xl font-semibold opacity-75">{m.digital_roster_empty()}</p>
          <p class="mt-2 text-sm opacity-60">{m.digital_roster_empty_message()}</p>
        </CardContent>
      </Card>
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
