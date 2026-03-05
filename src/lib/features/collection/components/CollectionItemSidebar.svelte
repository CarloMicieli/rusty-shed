<script lang="ts">
  import { onMount } from 'svelte';
  import InPlaceEdit from '$lib/components/InPlaceEdit.svelte';
  import InPlaceSelectEdit from '$lib/components/InPlaceSelectEdit.svelte';
  import { commands } from '$lib/bindings';
  import * as m from '$lib/paraglide/messages.js';
  import { getLocale } from '$lib/paraglide/runtime.js';
  import type {
    CollectionItemView,
    CollectionItemUpdateArgs,
    SellerView,
    ModelCondition,
    BoxCondition,
    PurchaseCondition
  } from '$lib/bindings';

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

  // Price field state (composite field, managed inline)
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

  function modelConditionLabel(c: ModelCondition): string {
    switch (c) {
      case 'MINT':
        return m.collection_item_condition_mint();
      case 'NEAR_MINT':
        return m.collection_item_condition_near_mint();
      case 'EXCELLENT':
        return m.collection_item_condition_excellent();
      case 'VERY_GOOD':
        return m.collection_item_condition_very_good();
      case 'GOOD':
        return m.collection_item_condition_good();
      case 'FAIR':
        return m.collection_item_condition_fair();
      case 'POOR':
        return m.collection_item_condition_poor();
      case 'FOR_PARTS':
        return m.collection_item_condition_for_parts();
    }
  }

  function boxConditionLabel(c: BoxCondition): string {
    switch (c) {
      case 'ORIGINAL_MINT':
        return m.collection_item_box_original_mint();
      case 'ORIGINAL_GOOD':
        return m.collection_item_box_original_good();
      case 'ORIGINAL_WORN':
        return m.collection_item_box_original_worn();
      case 'REPLACEMENT_BOX':
        return m.collection_item_box_replacement();
      case 'NO_BOX':
        return m.collection_item_box_no_box();
    }
  }

  function purchaseConditionLabel(c: PurchaseCondition): string {
    switch (c) {
      case 'NEW':
        return m.collection_item_purchase_condition_new();
      case 'PRE_OWNED':
        return m.collection_item_purchase_condition_pre_owned();
    }
  }

  const purchasedInfo = $derived(
    item.purchaseInfo?.kind === 'purchased' ? item.purchaseInfo.data : null
  );

  const purchaseConditionOptions: PurchaseCondition[] = ['NEW', 'PRE_OWNED'];
  const modelConditionOptions: ModelCondition[] = [
    'MINT',
    'NEAR_MINT',
    'EXCELLENT',
    'VERY_GOOD',
    'GOOD',
    'FAIR',
    'POOR',
    'FOR_PARTS'
  ];
  const boxConditionOptions: BoxCondition[] = [
    'ORIGINAL_MINT',
    'ORIGINAL_GOOD',
    'ORIGINAL_WORN',
    'REPLACEMENT_BOX',
    'NO_BOX'
  ];

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

  function isPositiveCondition(value: string): boolean {
    return ['NEW', 'MINT', 'ORIGINAL_MINT'].includes(value);
  }

  // Seller options for InPlaceSelectEdit
  const sellerOptions = $derived([
    { value: '', label: m.collection_item_not_recorded() },
    ...sellers.map((s) => ({ value: s.id, label: s.name }))
  ]);

  // Condition options with empty "not recorded" entry
  const purchaseConditionSelectOptions = $derived([
    { value: '', label: m.collection_item_not_recorded() },
    ...purchaseConditionOptions.map((o) => ({ value: o, label: purchaseConditionLabel(o) }))
  ]);
  const modelConditionSelectOptions = $derived([
    { value: '', label: m.collection_item_not_recorded() },
    ...modelConditionOptions.map((o) => ({ value: o, label: modelConditionLabel(o) }))
  ]);
  const boxConditionSelectOptions = $derived([
    { value: '', label: m.collection_item_not_recorded() },
    ...boxConditionOptions.map((o) => ({ value: o, label: boxConditionLabel(o) }))
  ]);
</script>

<aside class="w-full shrink-0 space-y-4 lg:w-80">
  <!-- ═══ Acquisition Card ═══════════════════════════════════════════════ -->
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

  <!-- ═══ Condition Card ══════════════════════════════════════════════════ -->
  <div class="overflow-hidden rounded-lg border border-[#1F1F1F] bg-[#0F0F0F]">
    <div class="border-b border-[#1F1F1F] px-4 py-3">
      <h3 class="text-xs font-semibold tracking-widest text-[#808080] uppercase">
        {m.collection_item_section_condition()}
      </h3>
    </div>

    <div class="grid grid-cols-3 grid-rows-[auto_auto] items-start gap-x-3 gap-y-0.5 p-4">
      <span class="text-[9px] font-medium tracking-wider text-[#808080] uppercase">
        {m.collection_item_purchase_condition()}
      </span>
      <span class="text-[9px] font-medium tracking-wider text-[#808080] uppercase">
        {m.collection_item_model_condition()}
      </span>
      <span class="text-[9px] font-medium tracking-wider text-[#808080] uppercase">
        {m.collection_item_box_condition()}
      </span>

      <InPlaceSelectEdit
        value={item.purchaseCondition ?? ''}
        displayLabel={item.purchaseCondition
          ? purchaseConditionLabel(item.purchaseCondition)
          : m.collection_item_not_recorded()}
        options={purchaseConditionSelectOptions}
        placeholder={m.collection_item_not_recorded()}
        class={item.purchaseCondition && isPositiveCondition(item.purchaseCondition)
          ? 'text-[#D48A42]'
          : ''}
        onSave={(v) =>
          saveUpdate({ kind: 'purchaseCondition', data: { purchase_condition: v || null } })}
      />
      <InPlaceSelectEdit
        value={item.modelCondition ?? ''}
        displayLabel={item.modelCondition
          ? modelConditionLabel(item.modelCondition)
          : m.collection_item_not_recorded()}
        options={modelConditionSelectOptions}
        placeholder={m.collection_item_not_recorded()}
        class={item.modelCondition && isPositiveCondition(item.modelCondition)
          ? 'text-[#D48A42]'
          : ''}
        onSave={(v) => saveUpdate({ kind: 'modelCondition', data: { model_condition: v || null } })}
      />
      <InPlaceSelectEdit
        value={item.boxCondition ?? ''}
        displayLabel={item.boxCondition
          ? boxConditionLabel(item.boxCondition)
          : m.collection_item_not_recorded()}
        options={boxConditionSelectOptions}
        placeholder={m.collection_item_not_recorded()}
        class={item.boxCondition && isPositiveCondition(item.boxCondition) ? 'text-[#D48A42]' : ''}
        onSave={(v) => saveUpdate({ kind: 'boxCondition', data: { box_condition: v || null } })}
      />
    </div>
  </div>
</aside>
