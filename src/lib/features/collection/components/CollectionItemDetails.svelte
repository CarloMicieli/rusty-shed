<script lang="ts">
  import { onMount } from 'svelte';
  import InPlaceEdit from '$lib/components/InPlaceEdit.svelte';
  import InPlaceSelectEdit from '$lib/components/InPlaceSelectEdit.svelte';
  import { commands } from '$lib/bindings';
  import * as m from '$lib/paraglide/messages.js';
  import { getLocale } from '$lib/paraglide/runtime.js';
  import { CurrencyInput, DatePickerField } from '$lib/components';
  import { getCurrencySymbol } from '$lib/utils/currency';
  import type { CollectionItemView, CollectionItemUpdateArgs, SellerView } from '$lib/bindings';

  interface Props {
    item: CollectionItemView;
    seller: SellerView | null;
    onItemUpdated?: () => Promise<void>;
  }

  let { item, seller, onItemUpdated }: Props = $props();

  let isPriceEditing = $state(false);
  let isSavingPrice = $state(false);
  let priceError = $state<string | null>(null);
  let priceDraftCents = $state<number | null>(null);
  let priceCurrencyDraft = $state('EUR');

  let sellers = $state<SellerView[]>([]);

  function formatPrice(amount: bigint, currency: string): string {
    return new Intl.NumberFormat(getLocale(), {
      style: 'currency',
      currency
    }).format(Number(amount) / 100);
  }

  const purchasedInfo = $derived(
    item.purchaseInfo?.kind === 'purchased' ? item.purchaseInfo.data : null
  );

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
    if (isSavingPrice) return;
    priceError = null;
    const currentPrice = purchasedInfo?.price;
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
      if (amountToSave === null) {
        console.log('[CollectionItemDetails] Amount is null, clearing price');
        await saveUpdate({ kind: 'price', data: { amount: null, currency: null } });
      } else {
        if (amountToSave < 0) {
          priceError = m.collection_item_error();
          isSavingPrice = false;
          return;
        }

        const amount = amountToSave as unknown as bigint;
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
</script>

<div class="overflow-hidden rounded-lg border border-[#1F1F1F] bg-[#0F0F0F]">
  <div class="border-b border-[#1F1F1F] px-4 py-3">
    <h3 class="text-xs font-semibold tracking-widest text-[#808080] uppercase">
      {m.collection_item_section_acquisition()}
    </h3>
  </div>

  <div class="space-y-3 p-4">
    <!-- Seller -->
    <div class="space-y-1">
      <span class="text-[9px] font-medium tracking-wider text-[#808080] uppercase">
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
      <span class="text-[9px] font-medium tracking-wider text-[#808080] uppercase">
        {m.collection_item_notes()}
      </span>
      <InPlaceEdit
        value={item.notes ?? ''}
        placeholder={m.collection_item_not_recorded()}
        multiline={true}
        onSave={saveNotes}
      />
    </div>

    <!-- Three-column footer: Price | Purchase Date | Added Date -->
    <div class="grid grid-cols-3 gap-x-3 border-t border-[#1F1F1F] pt-3">
      <!-- Price -->
      <div class="flex flex-col gap-0.5">
        <span class="text-[9px] font-medium tracking-wider text-[#808080] uppercase">
          {m.collection_item_price()}
        </span>
        {#if isPriceEditing}
          <div class="col-span-3 mt-1">
            <CurrencyInput
              bind:value={priceDraftCents}
              symbol={getCurrencySymbol(priceCurrencyDraft)}
              disabled={isSavingPrice}
              onblur={savePrice}
              onkeydown={handlePriceKeydown}
              class="w-full"
              inputClass="border-[#D48A42] ring-[#D48A42]/30"
              label={m.collection_item_price()}
              autofocus
            />
            {#if priceError}
              <p class="mt-1 text-xs text-red-400" role="alert">{priceError}</p>
            {/if}
          </div>
        {:else}
          <div
            class="-mx-1 flex h-6 cursor-pointer items-center rounded px-1 transition-colors duration-150 hover:border hover:border-dashed hover:border-[#D48A42]/40 hover:bg-[rgba(212,138,66,0.15)]"
            onclick={startEditingPrice}
            onkeydown={(e) => {
              if (e.key === 'Enter' || e.key === ' ') startEditingPrice();
            }}
            role="button"
            tabindex="0"
          >
            {#if purchasedInfo?.price}
              <span class="text-xs font-semibold text-[#E0E0E0]">
                {formatPrice(purchasedInfo.price.amount, purchasedInfo.price.currency)}
              </span>
            {:else}
              <span class="text-xs text-[#808080] italic">—</span>
            {/if}
          </div>
        {/if}
      </div>

      <!-- Purchase Date -->
      <div class="flex flex-col gap-0.5">
        <span class="text-[9px] font-medium tracking-wider text-[#808080] uppercase">
          {m.collection_item_purchase_date()}
        </span>
        <DatePickerField
          value={purchasedInfo?.purchaseDate ?? null}
          onSelect={(iso: string | null) =>
            saveUpdate({ kind: 'purchaseDate', data: { purchase_date: iso } })}
          placeholder="—"
          class="h-6"
        />
      </div>

      <!-- Added Date -->
      <div class="flex flex-col gap-0.5">
        <span class="text-[9px] font-medium tracking-wider text-[#808080] uppercase">
          {m.collection_item_added_date()}
        </span>
        <DatePickerField
          value={item.addedDate ?? null}
          onSelect={(iso: string | null) =>
            saveUpdate({ kind: 'addedDate', data: { added_date: iso } })}
          placeholder="—"
          class="h-6"
        />
      </div>
    </div>
  </div>
</div>
