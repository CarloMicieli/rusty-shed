<script lang="ts">
  import { onMount } from 'svelte';
  import InPlaceEdit from '$lib/components/InPlaceEdit.svelte';
  import InPlaceSelectEdit from '$lib/components/InPlaceSelectEdit.svelte';
  import { commands } from '$lib/bindings';
  import * as m from '$lib/paraglide/messages.js';
  import { getLocale } from '$lib/paraglide/runtime.js';
  import type { CollectionItemView, CollectionItemUpdateArgs, SellerView } from '$lib/bindings';

  interface Props {
    item: CollectionItemView;
    seller: SellerView | null;
    onItemUpdated?: () => Promise<void>;
  }

  let { item, seller, onItemUpdated }: Props = $props();

  const dateFormatter = new Intl.DateTimeFormat(getLocale(), {
    year: 'numeric',
    month: 'short',
    day: 'numeric'
  });

  let isPriceEditing = $state(false);
  let isSavingPrice = $state(false);
  let priceError = $state<string | null>(null);
  let priceDraft = $state('');
  let priceCurrencyDraft = $state('EUR');

  let sellers = $state<SellerView[]>([]);

  function formatDate(iso: string): string {
    return dateFormatter.format(new Date(iso));
  }

  function formatPrice(amount: bigint, currency: string): string {
    return new Intl.NumberFormat(getLocale(), {
      style: 'currency',
      currency
    }).format(Number(amount) / 100);
  }

  const purchasedInfo = $derived(
    item.purchaseInfo?.kind === 'purchased' ? item.purchaseInfo.data : null
  );

  const currencyOptions = ['EUR', 'USD', 'GBP', 'JPY'];

  onMount(async () => {
    const result = await commands.getSellers();
    if (result.status === 'ok') {
      sellers = result.data;
    }
  });

  function toDateInputValue(value: string | null | undefined): string {
    if (!value) return '';
    return value.slice(0, 10);
  }

  async function saveUpdate(update: CollectionItemUpdateArgs): Promise<void> {
    const result = await commands.updateCollectionItem({
      collectionItemId: item.id,
      update
    });
    if (result.status === 'error') {
      throw new Error('update_failed');
    }
    await onItemUpdated?.();
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
    priceDraft = currentPrice ? (Number(currentPrice.amount) / 100).toFixed(2) : '';
    priceCurrencyDraft = currentPrice?.currency ?? 'EUR';
    isPriceEditing = true;
  }

  async function savePrice(): Promise<void> {
    if (isSavingPrice) return;
    isSavingPrice = true;
    priceError = null;

    try {
      const normalized = priceDraft.trim();
      if (!normalized) {
        await saveUpdate({ kind: 'price', data: { amount: null, currency: null } });
        isPriceEditing = false;
        return;
      }

      const parsed = Number(normalized);
      if (Number.isNaN(parsed) || parsed < 0) {
        priceError = m.collection_item_error();
        return;
      }

      const amount = BigInt(Math.round(parsed * 100));
      await saveUpdate({ kind: 'price', data: { amount, currency: priceCurrencyDraft } });
      isPriceEditing = false;
    } catch {
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
          <div
            class="col-span-3 mt-1 flex flex-col gap-2"
            onfocusout={(e) => {
              if (!e.currentTarget.contains(e.relatedTarget as Node)) {
                void savePrice();
              }
            }}
          >
            <div class="grid grid-cols-[1fr,auto] gap-1">
              <input
                bind:value={priceDraft}
                type="number"
                min="0"
                step="0.01"
                disabled={isSavingPrice}
                onkeydown={(e) => {
                  if (e.key === 'Escape') {
                    isPriceEditing = false;
                    priceError = null;
                  }
                }}
                class="rounded border border-[#D48A42] bg-[#0F0F0F] px-2 py-1 text-sm text-[#E0E0E0] ring-1 ring-[#D48A42]/30 outline-none"
              />
              <select
                bind:value={priceCurrencyDraft}
                disabled={isSavingPrice}
                onkeydown={(e) => {
                  if (e.key === 'Escape') {
                    isPriceEditing = false;
                    priceError = null;
                  }
                }}
                class="rounded border border-[#D48A42] bg-[#0F0F0F] px-1 py-1 text-sm text-[#E0E0E0] ring-1 ring-[#D48A42]/30 outline-none"
              >
                {#each currencyOptions as currencyCode (currencyCode)}
                  <option value={currencyCode}>{currencyCode}</option>
                {/each}
              </select>
            </div>
            {#if priceError}
              <p class="text-xs text-red-400" role="alert">{priceError}</p>
            {/if}
          </div>
        {:else}
          <div
            class="-mx-1 cursor-pointer rounded p-1 transition-colors duration-150 hover:border hover:border-dashed hover:border-[#D48A42]/40 hover:bg-[rgba(212,138,66,0.15)]"
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
        <InPlaceEdit
          type="date"
          value={toDateInputValue(purchasedInfo?.purchaseDate ?? null)}
          displayValue={purchasedInfo?.purchaseDate
            ? formatDate(purchasedInfo.purchaseDate)
            : undefined}
          placeholder="—"
          onSave={(v) => saveUpdate({ kind: 'purchaseDate', data: { purchase_date: v || null } })}
        />
      </div>

      <!-- Added Date -->
      <div class="flex flex-col gap-0.5">
        <span class="text-[9px] font-medium tracking-wider text-[#808080] uppercase">
          {m.collection_item_added_date()}
        </span>
        <InPlaceEdit
          type="date"
          value={toDateInputValue(item.addedDate)}
          displayValue={formatDate(item.addedDate)}
          placeholder="—"
          onSave={(v) => saveUpdate({ kind: 'addedDate', data: { added_date: v || null } })}
        />
      </div>
    </div>
  </div>
</div>
