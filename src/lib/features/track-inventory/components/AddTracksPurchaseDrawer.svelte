<script lang="ts">
  import * as m from '$lib/paraglide/messages';
  import type { TrackProductView, SellerView, Currency, Manufacturer } from '$lib/bindings';
  import { commands } from '$lib/bindings';
  import { getTrackInventoryContext } from '$lib/features/track-inventory';
  import { DrawerShell, DrawerHeader, DrawerFooter } from '$lib/components/drawer';
  import { ShoppingCart } from 'lucide-svelte';
  import CreateProductDialog from './CreateProductDialog.svelte';
  import PurchaseFormFields from './PurchaseFormFields.svelte';
  import PurchaseLoadingState from './PurchaseLoadingState.svelte';
  import { superForm } from 'sveltekit-superforms';
  import { zod4 as zod } from 'sveltekit-superforms/adapters';
  import { trackPurchaseSchema } from '$lib/schemas/track-purchase-form';
  import { localeState } from '$lib/stores/locale.svelte';

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
  let isSubmitting = $state(false);
  let asyncError = $state<string | null>(null);
  let formEl: HTMLFormElement | undefined = $state();

  function getInitialData() {
    return {
      selectedProductId: '',
      quantity: 1,
      priceAmount: null as number | null,
      priceCurrency: 'EUR' as Currency,
      selectedSellerId: '',
      purchaseDate: new Date().toISOString().split('T')[0]
    };
  }

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  const { form, errors, tainted, enhance, reset, isTainted } = superForm(getInitialData() as any, {
    SPA: true,
    dataType: 'json',
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    validators: zod(trackPurchaseSchema as any),
    onUpdate: async ({ form: fd }) => {
      if (!fd.valid) return;
      isSubmitting = true;
      asyncError = null;
      try {
        await service.addPurchase({
          id: inventoryId,
          trackId: $form.selectedProductId,
          quantity: $form.quantity,
          price: {
            amount: ($form.priceAmount as number) * $form.quantity,
            currency: $form.priceCurrency as Currency
          },
          sellerId: $form.selectedSellerId || null,
          purchaseDate: $form.purchaseDate
        });

        try {
          await onPurchaseAdded?.();
        } catch {
          // callback errors must not block the drawer from closing
        }

        handleClose();
      } catch (err) {
        console.error('Failed to add purchase:', err);
        asyncError = err instanceof Error ? err.message : m.track_purchase_toast_error();
      } finally {
        isSubmitting = false;
      }
    }
  });

  const hasChanges = $derived(isTainted($tainted));
  const formError = $derived(
    asyncError ??
      (($errors.selectedProductId?.[0] ?? $errors.quantity?.[0] ?? $errors.priceAmount?.[0]) as
        string | undefined) ??
      null
  );

  $effect.pre(() => {
    if (open) {
      asyncError = null;
      dataFetched = false;
      // eslint-disable-next-line @typescript-eslint/no-explicit-any
      reset({ data: getInitialData() as any });
    }
  });

  $effect(() => {
    if (open) {
      void loadData();
    }
  });

  async function loadData() {
    if (loadingData || dataFetched) return;
    try {
      loadingData = true;
      const [productsResult, sellersResult, manufacturersResult] = await Promise.all([
        commands.getTrackProducts(localeState.activeLocale),
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
    if (isSubmitting) return;
    open = false;
    onClose?.();
  }

  async function handleProductCreated(productId: string) {
    showCreateProduct = false;
    await loadData();
    $form.selectedProductId = productId;
  }

  function handleSubmit() {
    formEl?.requestSubmit();
  }
</script>

<DrawerShell {open} onClose={handleClose} size="xl" {hasChanges} labelledby="add-purchase-title">
  {#snippet header({ requestClose })}
    <DrawerHeader
      id="add-purchase-title"
      title={m.track_purchase_dialog_title()}
      icon={ShoppingCart}
      onClose={requestClose}
      disabled={isSubmitting}
    />
  {/snippet}

  {#if loadingData}
    <PurchaseLoadingState />
  {:else}
    <form bind:this={formEl} use:enhance class="contents">
      <!-- Hidden submit button to enable Enter-to-submit keyboard navigability -->
      <button type="submit" class="hidden" aria-hidden="true" tabindex="-1"></button>
      <PurchaseFormFields
        {products}
        {sellers}
        bind:selectedProductId={$form.selectedProductId}
        bind:quantity={$form.quantity}
        bind:priceAmount={$form.priceAmount}
        bind:priceCurrency={$form.priceCurrency}
        bind:selectedSellerId={$form.selectedSellerId}
        bind:purchaseDate={$form.purchaseDate}
        submitting={isSubmitting}
        error={formError}
        onCreateProduct={() => (showCreateProduct = true)}
      />
    </form>
  {/if}

  {#snippet footer({ requestClose: _requestClose })}
    <DrawerFooter
      cancelLabel={m.track_purchase_cancel()}
      submitLabel={m.track_purchase_submit()}
      onCancel={handleClose}
      onSubmit={handleSubmit}
      submitting={isSubmitting}
    />
  {/snippet}
</DrawerShell>

<CreateProductDialog
  bind:open={showCreateProduct}
  manufacturers={manufacturers.map((mfr) => ({ id: mfr.id, name: mfr.name }))}
  onClose={() => (showCreateProduct = false)}
  onProductCreated={handleProductCreated}
/>
