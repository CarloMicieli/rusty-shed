<script lang="ts">
  interface Props {
    selectedProductId?: string;
    quantity?: number;
    priceAmount?: number | null;
    priceCurrency?: string;
    selectedSellerId?: string;
    purchaseDate?: string;
    submitting?: boolean;
    error?: string | null;
    onCreateProduct?: () => void;
  }

  let {
    selectedProductId = $bindable(''),
    quantity = $bindable(1),
    priceAmount = $bindable(null),
    priceCurrency = $bindable('EUR'),
    selectedSellerId = $bindable(''),
    purchaseDate = $bindable('2026-11-27'),
    submitting = false,
    error = null,
    onCreateProduct
  }: Props = $props();

  function fillValidData() {
    selectedProductId = 'track-1';
    quantity = 2;
    priceAmount = 1500;
    priceCurrency = 'EUR';
    selectedSellerId = 'seller-1';
    purchaseDate = '2026-11-15';
  }
</script>

<div>
  <button type="button" data-testid="fill-valid-data" onclick={fillValidData}>fill</button>
  <button type="button" data-testid="open-create-product" onclick={onCreateProduct}>create</button>
  {#if submitting}
    <span data-testid="stub-submitting">submitting</span>
  {/if}
  {#if error}
    <span data-testid="stub-error">{error}</span>
  {/if}
</div>
