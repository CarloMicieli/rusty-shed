<script lang="ts">
  import * as m from '$lib/paraglide/messages';
  import { X, Plus, Loader2, Calendar, ShoppingCart, Tag, Store } from 'lucide-svelte';
  import type { TrackProductView, SellerView, Currency, Manufacturer } from '$lib/bindings';
  import { commands } from '$lib/bindings';
  import CreateProductDialog from './CreateProductDialog.svelte';
  import { Input, Button } from '$lib/components';

  interface Props {
    open?: boolean;
    inventoryId: string;
    onClose?: () => void;
    onPurchaseAdded?: () => void;
  }

  let { open = $bindable(false), inventoryId, onClose, onPurchaseAdded }: Props = $props();

  let products = $state<TrackProductView[]>([]);
  let sellers = $state<SellerView[]>([]);
  let manufacturers = $state<Manufacturer[]>([]);
  let loadingData = $state(false);
  let dataFetched = $state(false);
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
    priceAmount = '';
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
          amount: BigInt(Math.round(parseFloat(priceAmount) * 100)),
          currency: priceCurrency
        },
        sellerId: selectedSellerId || null,
        purchaseDate: purchaseDate
      });

      if (result.status === 'error') {
        throw new Error(JSON.stringify(result.error));
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
      <div class="flex items-center justify-between border-b border-white/5 p-6">
        <div class="flex items-center gap-3">
          <div
            class="flex h-10 w-10 items-center justify-center rounded-xl bg-amber-500/10 text-amber-500"
          >
            <ShoppingCart size={20} />
          </div>
          <h2
            id="add-purchase-title"
            class="text-xl font-bold tracking-tight text-zinc-100 uppercase"
          >
            {m.track_purchase_dialog_title()}
          </h2>
        </div>
        <button
          onclick={handleClose}
          class="rounded-lg p-2 text-zinc-500 transition-colors hover:bg-white/5 hover:text-white"
          disabled={submitting}
        >
          <X size={20} />
        </button>
      </div>

      <div class="p-6">
        {#if loadingData}
          <div class="flex flex-col items-center justify-center gap-4 py-20">
            <Loader2 size={32} class="animate-spin text-amber-500" />
            <p class="text-sm font-bold tracking-widest text-zinc-500 uppercase">
              Indexing Ledger...
            </p>
          </div>
        {:else}
          <form onsubmit={handleSubmit} class="space-y-6">
            <!-- Product selector with create button -->
            <div class="space-y-2">
              <label
                for="track-product-select"
                class="ml-1 text-[10px] font-bold tracking-[0.2em] text-zinc-500 uppercase"
              >
                {m.track_purchase_field_product()}
              </label>
              <div class="flex gap-2">
                <div class="relative flex-1">
                  <select
                    id="track-product-select"
                    class="h-12 w-full appearance-none rounded-xl border border-white/10 bg-zinc-950 px-4 text-sm text-zinc-100 focus:border-white/20 focus:outline-none"
                    bind:value={selectedProductId}
                    disabled={submitting}
                    required
                  >
                    <option value="" disabled selected>Select a track component...</option>
                    {#each products as product (product.track_id)}
                      <option value={product.track_id}>
                        {product.manufacturer_name} • {product.description || product.product_code}
                      </option>
                    {/each}
                  </select>
                  <div
                    class="pointer-events-none absolute inset-y-0 right-4 flex items-center text-zinc-600"
                  >
                    <Plus size={16} class="rotate-45" />
                  </div>
                </div>
                <Button
                  type="button"
                  variant="outline"
                  onclick={() => (showCreateProduct = true)}
                  class="h-12 w-12 border-white/10 bg-zinc-900/50 p-0 text-zinc-400 hover:bg-zinc-800 hover:text-white"
                  disabled={submitting}
                >
                  <Plus size={20} />
                </Button>
              </div>
            </div>

            <div class="grid grid-cols-1 gap-6 sm:grid-cols-2">
              <!-- Quantity -->
              <div class="space-y-2">
                <label
                  for="purchase-quantity"
                  class="ml-1 text-[10px] font-bold tracking-[0.2em] text-zinc-500 uppercase"
                >
                  {m.track_purchase_field_quantity()}
                </label>
                <div class="relative">
                  <Input
                    id="purchase-quantity"
                    type="number"
                    value={String(quantity)}
                    oninput={(e) => (quantity = parseInt(e.currentTarget.value) || 1)}
                    min="1"
                    class="h-12 rounded-xl border-white/10 bg-zinc-950 pl-10 text-zinc-100 focus:border-white/20 focus:ring-0"
                    disabled={submitting}
                    required
                  />
                  <Tag size={16} class="absolute top-1/2 left-4 -translate-y-1/2 text-zinc-600" />
                </div>
              </div>

              <!-- Price -->
              <div class="space-y-2">
                <label
                  for="purchase-price"
                  class="ml-1 text-[10px] font-bold tracking-[0.2em] text-zinc-500 uppercase"
                >
                  Market Price (Total)
                </label>
                <div class="flex gap-2">
                  <div class="relative flex-1">
                    <Input
                      id="purchase-price"
                      type="number"
                      value={priceAmount}
                      oninput={(e) => (priceAmount = e.currentTarget.value)}
                      min="0"
                      step="0.01"
                      class="h-12 rounded-xl border-white/10 bg-zinc-950 pl-10 text-zinc-100 focus:border-white/20 focus:ring-0"
                      disabled={submitting}
                      required
                      placeholder="0.00"
                    />
                    <ShoppingCart
                      size={16}
                      class="absolute top-1/2 left-4 -translate-y-1/2 text-zinc-600"
                    />
                  </div>
                  <select
                    class="h-12 rounded-xl border border-white/10 bg-zinc-900 px-3 text-xs font-bold text-zinc-400 focus:outline-none"
                    bind:value={priceCurrency}
                    disabled={submitting}
                  >
                    <option value="EUR">EUR</option>
                    <option value="USD">USD</option>
                    <option value="GBP">GBP</option>
                  </select>
                </div>
              </div>

              <!-- Seller -->
              <div class="space-y-2">
                <label
                  for="purchase-seller"
                  class="ml-1 text-[10px] font-bold tracking-[0.2em] text-zinc-500 uppercase"
                >
                  {m.track_purchase_field_seller()}
                </label>
                <div class="relative">
                  <select
                    id="purchase-seller"
                    class="h-12 w-full appearance-none rounded-xl border border-white/10 bg-zinc-950 pr-4 pl-10 text-sm text-zinc-100 focus:border-white/20 focus:outline-none"
                    bind:value={selectedSellerId}
                    disabled={submitting}
                  >
                    <option value="">No vendor specified</option>
                    {#each sellers as seller (seller.id)}
                      <option value={seller.id}>{seller.name}</option>
                    {/each}
                  </select>
                  <Store size={16} class="absolute top-1/2 left-4 -translate-y-1/2 text-zinc-600" />
                </div>
              </div>

              <!-- Purchase date -->
              <div class="space-y-2">
                <label
                  for="purchase-date"
                  class="ml-1 text-[10px] font-bold tracking-[0.2em] text-zinc-500 uppercase"
                >
                  Transaction Date
                </label>
                <div class="relative">
                  <Input
                    id="purchase-date"
                    type="date"
                    value={purchaseDate}
                    oninput={(e) => (purchaseDate = e.currentTarget.value)}
                    class="h-12 rounded-xl border-white/10 bg-zinc-950 pl-10 text-zinc-100 [color-scheme:dark] focus:border-white/20 focus:ring-0"
                    disabled={submitting}
                    required
                  />
                  <Calendar
                    size={16}
                    class="absolute top-1/2 left-4 -translate-y-1/2 text-zinc-600"
                  />
                </div>
              </div>
            </div>

            {#if error}
              <div
                class="rounded-lg border border-red-500/20 bg-red-500/10 p-4 text-xs font-bold tracking-wider text-red-500 uppercase"
              >
                System Error: {error}
              </div>
            {/if}

            <div class="flex justify-end gap-3 pt-4">
              <Button
                type="button"
                variant="ghost"
                onclick={handleClose}
                class="px-6 text-zinc-500 hover:bg-transparent hover:text-white"
                disabled={submitting}
              >
                {m.track_purchase_cancel()}
              </Button>
              <Button
                type="submit"
                variant="rusty"
                class="h-12 min-w-[160px] px-8"
                disabled={submitting}
              >
                {#if submitting}
                  <Loader2 size={18} class="mr-2 animate-spin" />
                  <span>Recording...</span>
                {:else}
                  {m.track_purchase_submit()}
                {/if}
              </Button>
            </div>
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
