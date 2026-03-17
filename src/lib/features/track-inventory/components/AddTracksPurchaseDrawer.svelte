<script lang="ts">
  import * as m from '$lib/paraglide/messages';
  import type { TrackProductView, SellerView, Currency, Manufacturer } from '$lib/bindings';
  import { commands } from '$lib/bindings';
  import { getTrackInventoryContext } from '$lib/features/track-inventory';
  import { DrawerShell, DrawerHeader, DrawerFooter, createDrawerForm } from '$lib/components/drawer';
  import { ShoppingCart } from 'lucide-svelte';
  import CreateProductDialog from './CreateProductDialog.svelte';
  import PurchaseFormFields from './PurchaseFormFields.svelte';
  import PurchaseLoadingState from './PurchaseLoadingState.svelte';

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
  let submitting = $state(false);
  let error = $state<string | null>(null);

  const f = createDrawerForm({
    initial: () => ({
      selectedProductId: '',
      quantity: 1,
      priceAmount: null as number | null,
      priceCurrency: 'EUR' as Currency,
      selectedSellerId: '',
      purchaseDate: new Date().toISOString().split('T')[0]
    }),
    validate: (v) => ({
      selectedProductId: !v.selectedProductId
        ? m.track_purchase_validation_product()
        : undefined,
      quantity: v.quantity <= 0 ? m.track_purchase_validation_quantity() : undefined,
      priceAmount:
        v.priceAmount === null || v.priceAmount < 0
          ? m.track_purchase_validation_price()
          : undefined
    })
  });

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
    f.reset();
    error = null;
    dataFetched = false;
  }

  async function handleProductCreated(productId: string) {
    showCreateProduct = false;
    await loadData();
    f.values.selectedProductId = productId;
  }

  async function handleSubmit(e?: Event) {
    e?.preventDefault();
    f.touch();

    if (!f.isValid) {
      error = Object.values(f.errors).find((e) => !!e) ?? null;
      return;
    }

    try {
      submitting = true;
      error = null;

      await service.addPurchase({
        id: inventoryId,
        trackId: f.values.selectedProductId,
        quantity: f.values.quantity,
        price: {
          amount: f.values.priceAmount! * f.values.quantity,
          currency: f.values.priceCurrency
        },
        sellerId: f.values.selectedSellerId || null,
        purchaseDate: f.values.purchaseDate
      });

      try {
        await onPurchaseAdded?.();
      } catch {
        // callback errors must not block the drawer from closing
      }

      submitting = false;
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
    }
  });
</script>

<DrawerShell {open} onClose={handleClose} size="xl" labelledby="add-purchase-title">
  {#snippet header({ requestClose })}
    <DrawerHeader
      id="add-purchase-title"
      title={m.track_purchase_dialog_title()}
      icon={ShoppingCart}
      onClose={requestClose}
      disabled={submitting}
    />
  {/snippet}

  {#if loadingData}
    <PurchaseLoadingState />
  {:else}
    <PurchaseFormFields
      {products}
      {sellers}
      bind:selectedProductId={f.values.selectedProductId}
      bind:quantity={f.values.quantity}
      bind:priceAmount={f.values.priceAmount}
      bind:priceCurrency={f.values.priceCurrency}
      bind:selectedSellerId={f.values.selectedSellerId}
      bind:purchaseDate={f.values.purchaseDate}
      {submitting}
      {error}
      onCreateProduct={() => (showCreateProduct = true)}
    />
  {/if}

  {#snippet footer({ requestClose: _requestClose })}
    <DrawerFooter
      cancelLabel={m.track_purchase_cancel()}
      submitLabel={m.track_purchase_submit()}
      onCancel={handleClose}
      onSubmit={handleSubmit}
      {submitting}
    />
  {/snippet}
</DrawerShell>

<CreateProductDialog
  bind:open={showCreateProduct}
  manufacturers={manufacturers.map((mfr) => ({ id: mfr.id, name: mfr.name }))}
  onClose={() => (showCreateProduct = false)}
  onProductCreated={handleProductCreated}
/>
