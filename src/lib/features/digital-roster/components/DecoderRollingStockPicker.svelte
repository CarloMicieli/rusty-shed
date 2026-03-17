<script lang="ts">
  import * as m from '$lib/paraglide/messages.js';
  import * as Select from '$lib/components/ui/select';
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
    const base = parts.join(' ') || rs.owned_rolling_stock_id;
    return rs.has_decoder ? `${base} — ${m.digital_roster_has_decoder()}` : base;
  }
</script>

<div>
  <label for="rolling-stock" class="block space-y-1">
    <span class="text-sm text-muted-foreground">{m.digital_roster_rolling_stock_label()}</span>
  </label>
  <Select.Root
    type="single"
    value={selectedId ?? undefined}
    onValueChange={(v) => onChange(v || null)}
  >
    <Select.Trigger id="rolling-stock" class="w-full">
      {#if selectedId}
        {formatLabel(rollingStocks.find((rs) => rs.owned_rolling_stock_id === selectedId)!)}
      {:else}
        <span class="text-muted-foreground">{m.form_new_model_select_placeholder()}</span>
      {/if}
    </Select.Trigger>
    <Select.Content>
      {#each rollingStocks as rs (rs.owned_rolling_stock_id)}
        <Select.Item value={rs.owned_rolling_stock_id} label={formatLabel(rs)} />
      {/each}
    </Select.Content>
  </Select.Root>
  {#if touched && error}
    <p class="text-error-500 mt-1 text-xs">{error}</p>
  {/if}
</div>
