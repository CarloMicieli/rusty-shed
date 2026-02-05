<script lang="ts">
  import * as m from '$lib/paraglide/messages';
  import { X, Plus } from 'lucide-svelte';
  import type { TrackProductView, SellerView, Currency, Manufacturer } from '$lib/bindings';
  import { commands } from '$lib/bindings';
  import CreateProductDialog from './CreateProductDialog.svelte';

  interface Props {
    open?: boolean;
    inventoryId: string;
    onClose?: () => void;
    onPurchaseAdded?: () => void;
  }

  const { open = $bindable(false), inventoryId, onClose, onPurchaseAdded }: Props = $props();

  let products = $state<TrackProductView[]>([]);
  let sellers = $state<SellerView[]>([]);
  let manufacturers = $state<Manufacturer[]>([]);
  let loadingData = $state(false);
  let showCreateProduct = $state(false);

  let selectedProductId = $state('');
  let quantity = $state(1);
  let priceAmount = $state('');
  let priceCurrency = $state<Currency>('EUR');
  let selectedSellerId = $state('');
  let purchaseDate = $state(new Date().toISOString().split('T')[0]);
  let submitting = $state(false);
  let error = $state<string | null>(null);

  async function loadData() {
    try {
      loadingData = true;
      const [productsResult, sellersResult, manufacturersResult] = await Promise.all([
        commands.getTrackProducts(),
        commands.getSellers(),
        commands.getManufacturers()
      ]);

      if (productsResult.status === 'ok') {
        products = productsResult.data;
      }
      if (sellersResult.status === 'ok') {
        sellers = sellersResult.data;
      }
      if (manufacturersResult.status === 'ok') {
        manufacturers = manufacturersResult.data;
      }
    } catch (err) {
      console.error('Failed to load data:', err);
    } finally {
      loadingData = false;
    }
  }

  function handleClose() {
    if (submitting) return;
    selectedProductId = '';
    quantity = 1;
    priceAmount = '';
    priceCurrency = 'EUR';
    selectedSellerId = '';
    purchaseDate = new Date().toISOString().split('T')[0];
    error = null;
    onClose?.();
  }

  async function handleProductCreated(productId: string) {
    showCreateProduct = false;
    // Reload products to include the newly created one
    await loadData();
    // Auto-select the newly created product
    selectedProductId = productId;
  }

  async function handleSubmit(e: Event) {
    e.preventDefault();

    // Validation
    if (!selectedProductId) {
      error = m.track_purchase_validation_product();
      return;
    }
    if (quantity <= 0) {
      error = m.track_purchase_validation_quantity();
      return;
    }
    if (!priceAmount || parseFloat(priceAmount) < 0) {
      error = m.track_purchase_validation_price();
      return;
    }

    try {
      submitting = true;
      error = null;

      const result = await commands.addTrackPurchase({
        id: inventoryId,
        trackId: selectedProductId,
        quantity: BigInt(quantity),
        price: {
          amount: BigInt(Math.round(parseFloat(priceAmount) * 100)), // Convert to cents
          currency: priceCurrency
        },
        sellerId: selectedSellerId || null,
        purchaseDate: purchaseDate
      });

      if (result.status === 'error') {
        let errorMsg: string;
        if ('DatabaseError' in result.error) {
          errorMsg = result.error.DatabaseError;
        } else if ('NotFound' in result.error) {
          errorMsg = result.error.NotFound;
        } else if ('ValidationError' in result.error) {
          errorMsg = JSON.stringify(result.error.ValidationError);
        } else if ('BusinessRule' in result.error) {
          errorMsg = result.error.BusinessRule;
        } else if ('Unknown' in result.error) {
          errorMsg = result.error.Unknown;
        } else {
          errorMsg = m.track_purchase_toast_error();
        }
        throw new Error(errorMsg);
      }

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
    if (open && products.length === 0) {
      loadData();
    }
  });
</script>

{#if open}
  <div
    class="fixed inset-0 z-50 flex items-center justify-center bg-black/50 p-4"
    role="dialog"
    aria-modal="true"
    aria-labelledby="add-purchase-title"
    tabindex="-1"
    onclick={(e) => {
      if (e.target === e.currentTarget) handleClose();
    }}
    onkeydown={(e) => {
      if (e.key === 'Escape') handleClose();
    }}
  >
    <div class="variant-filled-surface card w-full max-w-2xl space-y-4 p-6">
      <div class="flex items-center justify-between">
        <h2 id="add-purchase-title" class="h3 font-bold">
          {m.track_purchase_dialog_title()}
        </h2>
        <button
          onclick={handleClose}
          class="variant-ghost-surface btn-icon btn-icon-sm"
          disabled={submitting}
        >
          <X size={20} />
        </button>
      </div>

      {#if loadingData}
        <div class="flex items-center justify-center py-8">
          <div
            class="variant-filled-primary h-8 w-8 animate-spin rounded-full border-4 border-t-transparent"
          ></div>
        </div>
      {:else}
        <form onsubmit={handleSubmit} class="space-y-4">
          <!-- Product selector with create button -->
          <div class="flex gap-2">
            <label class="label flex-1">
              <span>{m.track_purchase_field_product()}</span>
              <select class="select" bind:value={selectedProductId} disabled={submitting} required>
                <option value="">Select a product...</option>
                {#each products as product (product.track_id)}
                  <option value={product.track_id}>
                    {product.description || product.product_code}
                  </option>
                {/each}
              </select>
            </label>
            <div class="flex items-end">
              <button
                type="button"
                onclick={() => (showCreateProduct = true)}
                class="variant-ghost-primary btn-icon"
                disabled={submitting}
                title="Create new product"
              >
                <Plus size={20} />
              </button>
            </div>
          </div>

          <!-- Quantity -->
          <label class="label">
            <span>{m.track_purchase_field_quantity()}</span>
            <input
              type="number"
              class="input"
              bind:value={quantity}
              min="1"
              disabled={submitting}
              required
            />
          </label>

          <!-- Price -->
          <div class="grid grid-cols-2 gap-4">
            <label class="label">
              <span>{m.track_purchase_field_price()}</span>
              <input
                type="number"
                class="input"
                bind:value={priceAmount}
                min="0"
                step="0.01"
                disabled={submitting}
                required
              />
            </label>
            <label class="label">
              <span>Currency</span>
              <select class="select" bind:value={priceCurrency} disabled={submitting}>
                <option value="EUR">EUR (€)</option>
                <option value="USD">USD ($)</option>
                <option value="GBP">GBP (£)</option>
              </select>
            </label>
          </div>

          <!-- Seller -->
          <label class="label">
            <span>{m.track_purchase_field_seller()}</span>
            <select class="select" bind:value={selectedSellerId} disabled={submitting}>
              <option value="">None</option>
              {#each sellers as seller (seller.id)}
                <option value={seller.id}>{seller.name}</option>
              {/each}
            </select>
          </label>

          <!-- Purchase date -->
          <label class="label">
            <span>{m.track_purchase_field_date()}</span>
            <input
              type="date"
              class="input"
              bind:value={purchaseDate}
              disabled={submitting}
              required
            />
          </label>

          {#if error}
            <div class="variant-filled-error rounded-lg p-3 text-sm">
              {error}
            </div>
          {/if}

          <div class="flex justify-end gap-2">
            <button
              type="button"
              onclick={handleClose}
              class="variant-ghost-surface btn"
              disabled={submitting}
            >
              {m.track_purchase_cancel()}
            </button>
            <button type="submit" class="variant-filled-primary btn" disabled={submitting}>
              {#if submitting}
                <span class="animate-pulse">...</span>
              {:else}
                {m.track_purchase_submit()}
              {/if}
            </button>
          </div>
        </form>
      {/if}
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
