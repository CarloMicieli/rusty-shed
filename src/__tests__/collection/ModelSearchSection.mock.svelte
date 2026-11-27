<script lang="ts">
  import type { AddModelFormState } from '$lib/features/collection/types/AddModelFormTypes';

  interface Props {
    form: AddModelFormState;
    isLoading: boolean;
    showPurchaseSection?: boolean;
    isRollingStockExpanded?: boolean;
    onAddRollingStock: () => void;
    onRemoveRollingStock: (uid: string) => void;
    onQuickAddManufacturer?: () => void;
    onQuickAddSeller?: () => void;
  }

  let {
    form = $bindable(),
    isLoading = false,
    showPurchaseSection = $bindable(false),
    isRollingStockExpanded = $bindable(false),
    onAddRollingStock,
    onRemoveRollingStock,
    onQuickAddManufacturer,
    onQuickAddSeller
  }: Props = $props();

  function prepareValidForm() {
    form.manufacturerId = form.manufacturerId ?? 'mfg-1';
    form.productCode = 'HR2906';
    form.description = 'Electric Locomotive';
    form.category = 'LOCOMOTIVE';
    form.scale = 'H0';
    form.powerMethod = 'DC';
    form.epoch = 'IV';
    form.rollingStocks = [
      {
        uid: 'rs-1',
        railwayCompanyId: 'rc-1',
        seriesCode: 'E.656',
        category: 'LOCOMOTIVE',
        roadNumber: '656 001',
        subcategory: 'ELECTRIC_LOCOMOTIVE'
      }
    ];
    form.purchase.sellerId = form.purchase.sellerId ?? 'seller-1';
    form.purchase.priceAmount = 25999;
    form.purchase.priceCurrency = 'EUR';
    form.purchase.purchaseType = 'PREORDER';
    form.purchase.depositAmount = 5000;
    form.purchase.depositCurrency = 'EUR';
    form.purchase.preorderTotalAmount = 25999;
    form.purchase.preorderTotalCurrency = 'EUR';
    form.purchase.expectedDate = '2026-12-01';

    showPurchaseSection = true;
    isRollingStockExpanded = true;
  }

  function removeFirstRollingStock() {
    const firstUid = form.rollingStocks[0]?.uid;
    if (firstUid) {
      onRemoveRollingStock(firstUid);
    }
  }
</script>

<div data-testid="loading-state">{isLoading ? 'loading' : 'ready'}</div>
<div data-testid="rolling-stock-count">{form.rollingStocks.length}</div>
<button type="button" aria-label="add-rolling-stock" onclick={onAddRollingStock}></button>
<button type="button" aria-label="remove-rolling-stock" onclick={removeFirstRollingStock}></button>
<button type="button" aria-label="prepare-valid-form" onclick={prepareValidForm}></button>
<button
  type="button"
  aria-label="open-quick-add-manufacturer"
  onclick={() => onQuickAddManufacturer?.()}
></button>
<button type="button" aria-label="open-quick-add-seller" onclick={() => onQuickAddSeller?.()}
></button>
