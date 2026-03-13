<script lang="ts">
  import * as m from '$lib/paraglide/messages';
  import type { TrackProductView, SellerView, Currency, Manufacturer } from '$lib/bindings';
  import { commands } from '$lib/bindings';
  import { getTrackInventoryContext } from '$lib/features/track-inventory';
  import CreateProductDialog from './CreateProductDialog.svelte';
  import PurchaseFormFields from './PurchaseFormFields.svelte';
  import PurchaseDialogHeader from './PurchaseDialogHeader.svelte';
  import PurchaseLoadingState from './PurchaseLoadingState.svelte';
  import PurchaseActionFooter from './PurchaseActionFooter.svelte';

  interface Props {
    open?: boolean;
    inventoryId: string;
    onClose?: () => void;
    onPurchaseAdded?: () => void;
  }

  let { open = $bindable(false), inventoryId, onClose, onPurchaseAdded }: Props = $props();

  const service = getTrackInventoryContext();

  let products = $state<TrackProductView[]>([]);
  let sellers = $state<SellerView[]>([]);
  let manufacturers = $state<Manufacturer[]>([]);
  let loadingData = $state(false);
  let dataFetched = $state(false);
  let showCreateProduct = $state(false);

  let selectedProductId = $state('');
  let quantity = $state(1);
  let priceAmount = $state<number | null>(null);
  let priceCurrency = $state<Currency>('EUR');
  let selectedSellerId = $state('');
  let purchaseDate = $state(new Date().toISOString().split('T')[0]);
  let submitting = $state(false);
  let error = $state<string | null>(null);

  async function loadData() {
    if (loadingData || dataFetched) return;
    try {
      loadingData = true;
      const [productsResult, sellersResult, manufacturersResult] = await Promise.all([
        commands.getTrackProducts(),
        commands.getSellers(),
        commands.getManufacturers()
      ]);

      if (productsResult.status === 'ok') products = productsResult.data;
      if (sellersResult.status === 'ok') sellers = sellersResult.data;
      if (manufacturersResult.status === 'ok') manufacturers = manufacturersResult.data;
      dataFetched = true;
    } catch (err) {
      console.error('Failed to load data:', err);
    } finally {
      loadingData = false;
    }
  }

  function handleClose() {
    if (submitting) return;
    open = false;
    resetForm();
    onClose?.();
  }

  function resetForm() {
    selectedProductId = '';
    quantity = 1;
    priceAmount = null;
    priceCurrency = 'EUR';
    selectedSellerId = '';
    purchaseDate = new Date().toISOString().split('T')[0];
    error = null;
    dataFetched = false;
  }

  async function handleProductCreated(productId: string) {
    showCreateProduct = false;
    await loadData();
    selectedProductId = productId;
  }

  async function handleSubmit(e: Event) {
    e.preventDefault();

    if (!selectedProductId) {
      error = m.track_purchase_validation_product();
      return;
    }
    if (quantity <= 0) {
      error = m.track_purchase_validation_quantity();
      return;
    }
    if (priceAmount === null || priceAmount < 0) {
      error = m.track_purchase_validation_price();
      return;
    }

    try {
      submitting = true;
      error = null;

      await service.addPurchase({
        id: inventoryId,
        trackId: selectedProductId,
        quantity,
        price: {
          amount: priceAmount * quantity,
          currency: priceCurrency
        },
        sellerId: selectedSellerId || null,
        purchaseDate
      });

      await onPurchaseAdded?.();
      handleClose();
    } catch (err) {
      console.error('Failed to add purchase:', err);
      error = err instanceof Error ? err.message : m.track_purchase_toast_error();
    } finally {
      submitting = false;
    }
  }

  $effect(() => {
    if (open) {
      loadData();
      document.body.style.overflow = 'hidden';
      return () => {
        document.body.style.overflow = '';
      };
    }
  });

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === 'Escape' && open && !showCreateProduct) handleClose();
  }
</script>

<svelte:window onkeydown={handleKeydown} />

{#if open}
  <!-- Overlay -->
  <div
    class="fixed inset-0 z-[100] bg-black/80 backdrop-blur-md transition-all duration-300"
    onclick={handleClose}
    aria-hidden="true"
  ></div>

  <!-- Dialog -->
  <div
    class="fixed top-1/2 left-1/2 z-[101] w-full max-w-2xl -translate-x-1/2 -translate-y-1/2 p-4"
    role="dialog"
    aria-modal="true"
    aria-labelledby="add-purchase-title"
  >
    <div
      class="flex flex-col overflow-hidden rounded-2xl border border-amber-500/20 bg-[#0c0c0c] shadow-2xl"
    >
      <PurchaseDialogHeader onClose={handleClose} disabled={submitting} />

      <div class="p-6">
        {#if loadingData}
          <PurchaseLoadingState />
        {:else}
          <form onsubmit={handleSubmit} class="space-y-6">
            <PurchaseFormFields
              {products}
              {sellers}
              bind:selectedProductId
              bind:quantity
              bind:priceAmount
              bind:priceCurrency
              bind:selectedSellerId
              bind:purchaseDate
              {submitting}
              {error}
              onCreateProduct={() => (showCreateProduct = true)}
            />

            <PurchaseActionFooter onCancel={handleClose} onSubmit={handleSubmit} {submitting} />
          </form>
        {/if}
      </div>
    </div>
  </div>

  <!-- Create Product Dialog -->
  <CreateProductDialog
    bind:open={showCreateProduct}
    manufacturers={manufacturers.map((m) => ({ id: m.id, name: m.name }))}
    onClose={() => (showCreateProduct = false)}
    onProductCreated={handleProductCreated}
  />
{/if}
