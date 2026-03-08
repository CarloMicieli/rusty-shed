<script lang="ts">
  import * as m from '$lib/paraglide/messages.js';
  import type { InstallableRollingStockView } from '$lib/bindings';

  interface Props {
    rollingStocks: InstallableRollingStockView[];
    selectedId: string | null;
    error?: string;
    touched?: boolean;
    onChange: (id: string | null) => void;
  }

  const { rollingStocks, selectedId, error, touched = false, onChange }: Props = $props();

  function formatLabel(rs: InstallableRollingStockView): string {
    const parts = [];
    if (rs.series_code) parts.push(rs.series_code);
    if (rs.road_number) parts.push(rs.road_number);
    if (rs.railway_company_name) parts.push(`(${rs.railway_company_name})`);
    return parts.join(' ') || rs.owned_rolling_stock_id;
  }
</script>

<div>
  <label for="rolling-stock" class="block space-y-1">
    <span class="text-sm text-muted-foreground">
      {m.digital_roster_rolling_stock_label()}
    </span>
  </label>
  <select
    id="rolling-stock"
    value={selectedId ?? ''}
    onchange={(e) => onChange(e.currentTarget.value || null)}
    class="h-9 w-full rounded-md border border-input bg-background px-3 py-2 text-sm transition-colors outline-none focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/40"
    class:input-error={touched && error}
  >
    <option value="">{m.form_new_model_select_placeholder()}</option>
    {#each rollingStocks as rs (rs.owned_rolling_stock_id)}
      <option value={rs.owned_rolling_stock_id}>
        {formatLabel(rs)}
        {#if rs.has_decoder}
          ({m.digital_roster_has_decoder()})
        {/if}
      </option>
    {/each}
  </select>
  {#if touched && error}
    <p class="text-error-500 mt-1 text-xs">{error}</p>
  {/if}
</div>

<style>
  .input-error {
    border-color: rgb(var(--color-error-500));
  }
</style>
