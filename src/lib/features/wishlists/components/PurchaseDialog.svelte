<script lang="ts">
  import { onMount } from 'svelte';
  import { X, ShoppingCart, Loader2 } from 'lucide-svelte';
  import { commands } from '$lib/bindings';
  import type { SellerView } from '$lib/bindings';
  import * as m from '$lib/paraglide/messages.js';

  interface Props {
    open: boolean;
    wishlistId: string;
    wishlistItemId: string;
    itemName: string;
    onClose: () => void;
    onSuccess: () => void;
  }

  let { open, wishlistId, wishlistItemId, itemName, onClose, onSuccess }: Props = $props();

  // ── Form state ───────────────────────────────────────────────────────────────
  let priceAmount = $state('');
  let priceCurrency = $state('EUR');
  let purchaseDate = $state(new Date().toISOString().split('T')[0]);
  let selectedSellerId = $state('');
  let selectedCondition = $state('');
  let isSubmitting = $state(false);
  let error = $state<string | null>(null);

  // ── Data state ───────────────────────────────────────────────────────────────
  let sellers = $state<SellerView[]>([]);
  let loadingSellers = $state(false);

  // ── Computed today's date for max date validation ────────────────────────────
  const today = new Date().toISOString().split('T')[0];

  const CONDITION_OPTIONS = [
    { value: 'New', label: m.purchase_dialog_condition_new() },
    { value: 'PreOwnedLikeNew', label: m.purchase_dialog_condition_pre_owned_like_new() },
    { value: 'PreOwnedVeryGood', label: m.purchase_dialog_condition_pre_owned_very_good() },
    { value: 'PreOwnedGood', label: m.purchase_dialog_condition_pre_owned_good() },
    { value: 'PreOwnedAcceptable', label: m.purchase_dialog_condition_pre_owned_acceptable() }
  ];

  // ── Load initial data ─────────────────────────────────────────────────────────
  onMount(async () => {
    loadingSellers = true;
    try {
      const [sellersResult, settingsResult] = await Promise.all([
        commands.getSellers(),
        commands.getSettings()
      ]);
      if (sellersResult.status === 'ok') sellers = sellersResult.data;
      if (settingsResult.status === 'ok') priceCurrency = settingsResult.data.currency;
    } catch (e) {
      console.warn('Failed to load purchase dialog data', e);
    } finally {
      loadingSellers = false;
    }
  });

  // ── Reset form on close ───────────────────────────────────────────────────────
  function resetForm() {
    priceAmount = '';
    purchaseDate = new Date().toISOString().split('T')[0];
    selectedSellerId = '';
    selectedCondition = '';
    error = null;
    isSubmitting = false;
  }

  function handleClose() {
    if (isSubmitting) return;
    resetForm();
    onClose();
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === 'Escape' && open && !isSubmitting) {
      handleClose();
    }
  }

  // ── Submit ────────────────────────────────────────────────────────────────────
  async function handleSubmit(e: Event) {
    e.preventDefault();
    error = null;

    const amountCents = Math.round(parseFloat(priceAmount) * 100);
    if (!priceAmount || isNaN(amountCents) || amountCents < 0) {
      error = m.purchase_dialog_error_price_required();
      return;
    }

    if (purchaseDate > today) {
      error = m.purchase_dialog_error_future_date_forbidden();
      return;
    }

    isSubmitting = true;
    try {
      const result = await commands.purchaseWishlistItem({
        wishlistId,
        wishlistItemId,
        priceAmount: BigInt(amountCents),
        priceCurrency,
        purchaseDate,
        sellerId: selectedSellerId || null,
        condition: selectedCondition || null
      });

      if (result.status === 'error') {
        error = m.purchase_dialog_error_save_failed();
        return;
      }

      resetForm();
      onSuccess();
    } catch (e) {
      console.error('Purchase failed:', e);
      error = m.purchase_dialog_error_save_failed();
    } finally {
      isSubmitting = false;
    }
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
    class="fixed top-1/2 left-1/2 z-[101] w-full max-w-lg -translate-x-1/2 -translate-y-1/2 p-4"
    role="dialog"
    aria-modal="true"
    aria-labelledby="purchase-dialog-title"
  >
    <div
      class="flex flex-col overflow-hidden rounded-2xl border border-amber-500/20 bg-[#0c0c0c] shadow-2xl"
    >
      <!-- Header -->
      <div class="flex items-center justify-between border-b border-white/5 p-6">
        <div class="flex items-center gap-3">
          <div
            class="flex h-10 w-10 items-center justify-center rounded-xl bg-amber-500/10 text-amber-500"
          >
            <ShoppingCart size={20} />
          </div>
          <div>
            <h2 id="purchase-dialog-title" class="text-xl font-bold tracking-tight text-zinc-100">
              {m.purchase_dialog_title()}
            </h2>
            <p class="truncate text-sm text-zinc-500">{itemName}</p>
          </div>
        </div>
        <button
          onclick={handleClose}
          class="rounded-lg p-2 text-zinc-500 transition-colors hover:bg-white/5 hover:text-white"
          disabled={isSubmitting}
          type="button"
          aria-label={m.purchase_dialog_cancel()}
        >
          <X size={20} />
        </button>
      </div>

      <!-- Form -->
      <div class="p-6">
        {#if loadingSellers}
          <div class="flex items-center justify-center py-10">
            <Loader2 size={28} class="animate-spin text-amber-500" />
          </div>
        {:else}
          <form onsubmit={handleSubmit} class="space-y-5">
            <!-- Price -->
            <div class="space-y-2">
              <label
                for="purchase-price"
                class="ml-1 text-[10px] font-bold tracking-[0.2em] text-zinc-500 uppercase"
              >
                {m.purchase_dialog_price_label()}
              </label>
              <div class="flex gap-2">
                <input
                  id="purchase-price"
                  type="number"
                  min="0"
                  step="0.01"
                  placeholder={m.purchase_dialog_price_placeholder()}
                  bind:value={priceAmount}
                  disabled={isSubmitting}
                  required
                  class="h-12 flex-1 appearance-none rounded-xl border border-white/10 bg-zinc-950 px-4 text-sm text-zinc-100 placeholder-zinc-600 focus:border-white/20 focus:outline-none"
                />
                <select
                  bind:value={priceCurrency}
                  disabled={isSubmitting}
                  class="h-12 appearance-none rounded-xl border border-white/10 bg-zinc-950 px-3 text-sm text-zinc-100 focus:border-white/20 focus:outline-none"
                >
                  <option value="EUR">EUR</option>
                  <option value="USD">USD</option>
                  <option value="GBP">GBP</option>
                  <option value="JPY">JPY</option>
                </select>
              </div>
            </div>

            <!-- Purchase Date -->
            <div class="space-y-2">
              <label
                for="purchase-date"
                class="ml-1 text-[10px] font-bold tracking-[0.2em] text-zinc-500 uppercase"
              >
                {m.purchase_dialog_date_label()}
              </label>
              <input
                id="purchase-date"
                type="date"
                max={today}
                bind:value={purchaseDate}
                disabled={isSubmitting}
                required
                class="h-12 w-full appearance-none rounded-xl border border-white/10 bg-zinc-950 px-4 text-sm text-zinc-100 focus:border-white/20 focus:outline-none"
              />
            </div>

            <!-- Seller (optional) -->
            <div class="space-y-2">
              <label
                for="purchase-seller"
                class="ml-1 text-[10px] font-bold tracking-[0.2em] text-zinc-500 uppercase"
              >
                {m.purchase_dialog_seller_label()}
              </label>
              <select
                id="purchase-seller"
                bind:value={selectedSellerId}
                disabled={isSubmitting}
                class="h-12 w-full appearance-none rounded-xl border border-white/10 bg-zinc-950 px-4 text-sm text-zinc-100 focus:border-white/20 focus:outline-none"
              >
                <option value="">{m.purchase_dialog_seller_placeholder()}</option>
                {#each sellers as seller (seller.id)}
                  <option value={seller.id}>{seller.name}</option>
                {/each}
              </select>
            </div>

            <!-- Condition (optional) -->
            <div class="space-y-2">
              <label
                for="purchase-condition"
                class="ml-1 text-[10px] font-bold tracking-[0.2em] text-zinc-500 uppercase"
              >
                {m.purchase_dialog_condition_label()}
              </label>
              <select
                id="purchase-condition"
                bind:value={selectedCondition}
                disabled={isSubmitting}
                class="h-12 w-full appearance-none rounded-xl border border-white/10 bg-zinc-950 px-4 text-sm text-zinc-100 focus:border-white/20 focus:outline-none"
              >
                <option value="">{m.purchase_dialog_condition_placeholder()}</option>
                {#each CONDITION_OPTIONS as opt (opt.value)}
                  <option value={opt.value}>{opt.label}</option>
                {/each}
              </select>
            </div>

            <!-- Error message -->
            {#if error}
              <p
                class="rounded-xl border border-red-500/20 bg-red-500/10 px-4 py-3 text-sm text-red-400"
              >
                {error}
              </p>
            {/if}

            <!-- Actions -->
            <div class="flex justify-end gap-3 pt-2">
              <button
                type="button"
                onclick={handleClose}
                disabled={isSubmitting}
                class="rounded-xl border border-white/10 bg-transparent px-5 py-2.5 text-sm font-bold text-zinc-400 transition-colors hover:bg-white/5 hover:text-white disabled:opacity-50"
              >
                {m.purchase_dialog_cancel()}
              </button>
              <button
                type="submit"
                disabled={isSubmitting}
                class="flex items-center gap-2 rounded-xl bg-amber-500 px-5 py-2.5 text-sm font-bold text-black transition-colors hover:bg-amber-400 disabled:cursor-not-allowed disabled:opacity-50"
              >
                {#if isSubmitting}
                  <Loader2 size={16} class="animate-spin" />
                {/if}
                {m.purchase_dialog_submit()}
              </button>
            </div>
          </form>
        {/if}
      </div>
    </div>
  </div>
{/if}
