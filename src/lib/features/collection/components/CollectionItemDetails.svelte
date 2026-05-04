<script lang="ts">
  import { onMount } from 'svelte';
  import InPlaceEdit from '$lib/components/InPlaceEdit.svelte';
  import InPlaceSelectEdit from '$lib/components/InPlaceSelectEdit.svelte';
  import { Badge } from '$lib/components/ui/badge';
  import { commands } from '$lib/bindings';
  import * as m from '$lib/paraglide/messages.js';
  import { getLocale } from '$lib/paraglide/runtime.js';
  import { Button, CurrencyInput, DatePickerField } from '$lib/components';
  import SellCollectionItemDialog from './SellCollectionItemDialog.svelte';
  import { regionalManager } from '$lib/features/settings/RegionalManager.svelte';
  import type { CollectionItemView, CollectionItemUpdateArgs, SellerView } from '$lib/bindings';
  import { getCollectionContext } from '$lib/features/collection/CollectionState.svelte';

  interface Props {
    item: CollectionItemView;
    seller: SellerView | null;
    onItemUpdated?: () => Promise<void>;
  }

  let { item, seller, onItemUpdated }: Props = $props();

  const collectionService = getCollectionContext();
  let isReceiving = $state(false);

  let isPriceEditing = $state(false);
  let isSavingPrice = $state(false);
  let priceError = $state<string | null>(null);
  let priceDraftCents = $state<number | null>(null);
  let priceCurrencyDraft = $state('EUR');
  let showSellDialog = $state(false);

  let sellers = $state<SellerView[]>([]);

  function formatPrice(amount: number, currency: string): string {
    return new Intl.NumberFormat(getLocale(), {
      style: 'currency',
      currency
    }).format(Number(amount) / 100);
  }

  const purchasedInfo = $derived(
    item.purchaseInfo?.kind === 'purchased' ? item.purchaseInfo.data : null
  );
  const soldInfo = $derived(item.purchaseInfo?.kind === 'sold' ? item.purchaseInfo.data : null);
  const preorderedInfo = $derived(
    item.purchaseInfo?.kind === 'preOrdered' ? item.purchaseInfo.data : null
  );
  const isPreordered = $derived(item.purchaseInfo?.kind === 'preOrdered');
  const isSold = $derived(item.removedDate !== null || item.purchaseInfo?.kind === 'sold');
  const acquisitionPrice = $derived.by(() => {
    if (purchasedInfo?.price) return purchasedInfo.price;
    if (soldInfo?.purchasePrice) return soldInfo.purchasePrice;
    return null;
  });
  const acquisitionDate = $derived.by(() => {
    if (purchasedInfo?.purchaseDate) return purchasedInfo.purchaseDate;
    if (soldInfo?.purchaseDate) return soldInfo.purchaseDate;
    return null;
  });
  const preorderRemainingBalance = $derived.by(() => {
    if (!preorderedInfo) return null;
    return Number(preorderedInfo.totalPrice.amount) - Number(preorderedInfo.deposit.amount);
  });

  onMount(async () => {
    console.log('[CollectionItemDetails] Mounted with item:', JSON.parse(JSON.stringify(item)));
    const result = await commands.getSellers();
    if (result.status === 'ok') {
      sellers = result.data;
    }
  });

  async function saveUpdate(update: CollectionItemUpdateArgs): Promise<void> {
    console.log(
      `[CollectionItemDetails] Saving update for item ${item.id}:`,
      JSON.parse(JSON.stringify(update))
    );
    try {
      const result = await commands.updateCollectionItem({
        collectionItemId: item.id,
        update
      });
      if (result.status === 'error') {
        console.error('[CollectionItemDetails] Update failed with error:', result.error);
        throw new Error('update_failed');
      }
      console.log('[CollectionItemDetails] Update reported success from backend');
      if (onItemUpdated) {
        console.log('[CollectionItemDetails] Triggering onItemUpdated callback...');
        await onItemUpdated();
      }
    } catch (e) {
      console.error('[CollectionItemDetails] Exception during saveUpdate:', e);
      throw e;
    }
  }

  async function saveNotes(value: string): Promise<void> {
    const notes = value.trim();
    await saveUpdate({
      kind: 'notes',
      data: { notes: notes.length > 0 ? notes : null }
    });
  }

  function startEditingPrice() {
    if (isSavingPrice || isSold) return;
    priceError = null;
    const currentPrice = acquisitionPrice;
    priceDraftCents = currentPrice ? Number(currentPrice.amount) : null;
    priceCurrencyDraft = currentPrice?.currency ?? 'EUR';
    isPriceEditing = true;
  }

  async function handlePriceKeydown(e: KeyboardEvent) {
    if (e.key === 'Enter') {
      void savePrice();
    } else if (e.key === 'Escape') {
      isPriceEditing = false;
      priceError = null;
    }
  }

  async function savePrice(): Promise<void> {
    if (isSavingPrice || !isPriceEditing) return;

    // We use a local capture to avoid race conditions during async calls
    const amountToSave = priceDraftCents;
    const currencyToSave = priceCurrencyDraft;

    console.log(
      `[CollectionItemDetails] savePrice called. Amount: ${amountToSave}, Currency: ${currencyToSave}`
    );

    isSavingPrice = true;
    priceError = null;

    try {
      if (isSold) {
        return;
      }

      if (amountToSave === null) {
        console.log('[CollectionItemDetails] Amount is null, clearing price');
        await saveUpdate({ kind: 'price', data: { amount: null, currency: null } });
      } else {
        if (amountToSave < 0) {
          priceError = m.collection_item_error();
          isSavingPrice = false;
          return;
        }

        const amount = amountToSave as number;
        console.log(`[CollectionItemDetails] Sending amount: ${amount} (as cents)`);
        await saveUpdate({ kind: 'price', data: { amount, currency: currencyToSave } });
      }

      console.log('[CollectionItemDetails] Save sequence completed successfully');
      isPriceEditing = false;
      priceDraftCents = null;
    } catch (e) {
      console.error('[CollectionItemDetails] Error in savePrice:', e);
      priceError = m.collection_item_error();
    } finally {
      isSavingPrice = false;
    }
  }

  const sellerOptions = $derived([
    { value: '', label: m.collection_item_not_recorded() },
    ...sellers.map((s) => ({ value: s.id, label: s.name }))
  ]);

  async function handleReceivePreorder() {
    if (isReceiving) return;
    isReceiving = true;
    try {
      const today = new Date().toISOString().split('T')[0];
      const success = await collectionService.receivePreorder({
        itemId: item.id,
        receivedDate: today
      });
      if (success && onItemUpdated) {
        await onItemUpdated();
      }
    } finally {
      isReceiving = false;
    }
  }
</script>

<div class="overflow-hidden rounded-lg border border-border bg-card">
  <div class="border-b border-border px-4 py-3">
    <div class="flex items-center justify-between gap-3">
      <h3 class="text-xs font-semibold tracking-widest text-muted-foreground uppercase">
        {m.collection_item_section_acquisition()}
      </h3>
      {#if isSold}
        <Badge variant="destructive">{m.collection_item_status_sold()}</Badge>
      {:else if isPreordered}
        <Badge variant="outline" class="border-amber-500/60 text-amber-400">
          {m.collection_item_status_preordered()}
        </Badge>
      {:else}
        <Badge variant="success">{m.collection_item_status_owned()}</Badge>
      {/if}
    </div>
  </div>

  <div class="space-y-3 p-4">
    {#if !isSold && !isPreordered}
      <div class="flex justify-end">
        <Button variant="rusty" size="sm" onclick={() => (showSellDialog = true)}>
          {m.collection_item_sell_action()}
        </Button>
      </div>
    {/if}

    {#if isPreordered && preorderedInfo}
      <div class="space-y-3 rounded-lg border border-amber-500/30 bg-amber-500/5 p-3">
        <div class="flex items-center justify-between">
          <p class="text-[9px] font-semibold tracking-widest text-amber-400 uppercase">
            {m.collection_item_status_preordered()}
          </p>
          <Button
            variant="outline"
            size="sm"
            class="h-6 border-amber-500/40 px-2 text-xs text-amber-300 hover:bg-amber-500/10"
            onclick={handleReceivePreorder}
            disabled={isReceiving}
          >
            {isReceiving ? '...' : m.collection_item_receive_action()}
          </Button>
        </div>
        <div class="grid grid-cols-2 gap-3">
          <div class="space-y-0.5">
            <p class="text-[9px] text-muted-foreground uppercase">
              {m.collection_item_preorder_deposit()}
            </p>
            <p class="text-xs font-semibold text-foreground">
              {formatPrice(preorderedInfo.deposit.amount, preorderedInfo.deposit.currency)}
            </p>
          </div>
          <div class="space-y-0.5">
            <p class="text-[9px] text-muted-foreground uppercase">
              {m.collection_item_preorder_total()}
            </p>
            <p class="text-xs font-semibold text-foreground">
              {formatPrice(preorderedInfo.totalPrice.amount, preorderedInfo.totalPrice.currency)}
            </p>
          </div>
          {#if preorderedInfo.expectedDate}
            <div class="space-y-0.5">
              <p class="text-[9px] text-muted-foreground uppercase">
                {m.collection_item_preorder_expected_date()}
              </p>
              <p class="font-mono text-xs text-foreground">{preorderedInfo.expectedDate}</p>
            </div>
          {/if}
          {#if preorderRemainingBalance !== null && preorderRemainingBalance > 0}
            <div class="space-y-0.5">
              <p class="text-[9px] text-muted-foreground uppercase">
                {m.collection_item_preorder_remaining_balance()}
              </p>
              <p class="text-xs font-semibold text-amber-300">
                {formatPrice(preorderRemainingBalance, preorderedInfo.totalPrice.currency)}
              </p>
            </div>
          {/if}
        </div>
      </div>
    {/if}

    <!-- Seller -->
    <div class="space-y-1">
      <span class="text-[9px] font-medium tracking-wider text-muted-foreground uppercase">
        {m.collection_item_seller()}
      </span>
      <InPlaceSelectEdit
        value={seller?.id ?? ''}
        displayLabel={seller?.name ?? m.collection_item_not_recorded()}
        options={sellerOptions}
        placeholder={m.collection_item_not_recorded()}
        onSave={(id) => saveUpdate({ kind: 'seller', data: { seller_id: id || null } })}
      />
    </div>

    <!-- Notes -->
    <div class="space-y-1">
      <span class="text-[9px] font-medium tracking-wider text-muted-foreground uppercase">
        {m.collection_item_notes()}
      </span>
      <InPlaceEdit
        value={item.notes ?? ''}
        placeholder={m.collection_item_not_recorded()}
        multiline={true}
        onSave={saveNotes}
      />
    </div>

    <!-- Two-column footer: Price | Purchase Date -->
    <div class="grid grid-cols-2 gap-x-3 border-t border-border pt-3">
      <!-- Price -->
      <div class="flex flex-col gap-0.5">
        <span class="text-[9px] font-medium tracking-wider text-muted-foreground uppercase">
          {m.collection_item_price()}
        </span>
        {#if isPriceEditing && !isSold}
          <div class="col-span-3 mt-1">
            <CurrencyInput
              bind:value={priceDraftCents}
              symbol={regionalManager.getCurrencySymbol(priceCurrencyDraft)}
              disabled={isSavingPrice}
              onblur={savePrice}
              onkeydown={handlePriceKeydown}
              class="w-full"
              inputClass="border-primary ring-primary/30"
              label={m.collection_item_price()}
              autofocus
            />
            {#if priceError}
              <p class="mt-1 text-xs text-red-400" role="alert">{priceError}</p>
            {/if}
          </div>
        {:else if isSold}
          <div class="-mx-1 flex h-6 items-center rounded px-1">
            {#if acquisitionPrice}
              <span class="text-xs font-semibold text-foreground">
                {formatPrice(acquisitionPrice.amount, acquisitionPrice.currency)}
              </span>
            {:else}
              <span class="text-xs text-muted-foreground italic">—</span>
            {/if}
          </div>
        {:else}
          <button
            type="button"
            class="-mx-1 flex h-6 w-full cursor-pointer items-center rounded px-1 text-left transition-colors duration-150 hover:border hover:border-dashed hover:border-primary/40 hover:bg-primary/15"
            onclick={startEditingPrice}
          >
            {#if acquisitionPrice}
              <span class="text-xs font-semibold text-foreground">
                {formatPrice(acquisitionPrice.amount, acquisitionPrice.currency)}
              </span>
            {:else}
              <span class="text-xs text-muted-foreground italic">—</span>
            {/if}
          </button>
        {/if}
      </div>

      <!-- Purchase Date -->
      <div class="flex flex-col gap-0.5">
        <span class="text-[9px] font-medium tracking-wider text-muted-foreground uppercase">
          {m.collection_item_purchase_date()}
        </span>
        {#if isSold}
          <span class="h-6 font-mono text-xs text-foreground">{acquisitionDate ?? '—'}</span>
        {:else}
          <DatePickerField
            value={acquisitionDate}
            onSelect={(iso: string | null) =>
              saveUpdate({ kind: 'purchaseDate', data: { purchase_date: iso } })}
            placeholder="—"
            class="h-6"
          />
        {/if}
      </div>
    </div>

    {#if isSold}
      <div class="space-y-2 border-t border-border pt-3">
        <h4 class="text-[10px] font-semibold tracking-wider text-muted-foreground uppercase">
          {m.collection_item_disposition_title()}
        </h4>
        <div class="grid grid-cols-2 gap-3">
          <div class="space-y-0.5">
            <p class="text-[9px] text-muted-foreground uppercase">
              {m.collection_item_sale_date()}
            </p>
            <p class="font-mono text-xs text-foreground">
              {soldInfo?.saleDate ?? item.removedDate ?? '—'}
            </p>
          </div>
          <div class="space-y-0.5">
            <p class="text-[9px] text-muted-foreground uppercase">
              {m.collection_item_sale_price()}
            </p>
            <p class="text-xs font-semibold text-foreground">
              {#if soldInfo?.salePrice}
                {formatPrice(soldInfo.salePrice.amount, soldInfo.salePrice.currency)}
              {:else}
                —
              {/if}
            </p>
          </div>
        </div>
      </div>
    {/if}

    {#if isSold && soldInfo?.salePrice && soldInfo?.purchasePrice}
      {@const profitLoss =
        Number(soldInfo.salePrice.amount) - Number(soldInfo.purchasePrice.amount)}
      <div
        class="space-y-1 rounded-lg border p-3 {profitLoss >= 0
          ? 'border-green-500/30 bg-green-500/5'
          : 'border-red-500/30 bg-red-500/5'}"
      >
        <p
          class="text-[9px] font-semibold tracking-widest uppercase {profitLoss >= 0
            ? 'text-green-400'
            : 'text-red-400'}"
        >
          {m.collection_item_profit_loss_title()}
        </p>
        <p class="text-sm font-bold {profitLoss >= 0 ? 'text-green-300' : 'text-red-300'}">
          {profitLoss >= 0 ? '+' : ''}{formatPrice(profitLoss, soldInfo.salePrice.currency)}
        </p>
      </div>
    {/if}
  </div>
</div>

<SellCollectionItemDialog
  open={showSellDialog}
  itemId={item.id}
  defaultCurrency={acquisitionPrice?.currency ?? 'EUR'}
  onClose={() => (showSellDialog = false)}
  onSold={onItemUpdated}
/>
