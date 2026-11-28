<script lang="ts">
  interface RollingStockLike {
    owned_rolling_stock_id: string;
  }

  interface Props {
    rollingStocks?: RollingStockLike[];
    selectedId: string | null;
    onChange: (id: string | null) => void;
  }

  let { rollingStocks = [], selectedId, onChange }: Props = $props();
</script>

<div data-testid="rolling-stock-picker-stub">
  <p data-testid="selected-stock-id">{selectedId ?? 'none'}</p>
  {#each rollingStocks as stock (stock.owned_rolling_stock_id)}
    <button
      type="button"
      data-testid={`stock-${stock.owned_rolling_stock_id}`}
      onclick={() => onChange(stock.owned_rolling_stock_id)}
    >
      {stock.owned_rolling_stock_id}
    </button>
  {/each}
</div>
