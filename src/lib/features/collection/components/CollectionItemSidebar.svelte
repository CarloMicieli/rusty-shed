<script lang="ts">
  import { onMount } from 'svelte';
  import { Button, Card, CardContent, CardHeader, CardTitle } from '$lib/components';
  import { Separator } from '$lib/components/ui/separator';
  import InPlaceEdit from '$lib/components/InPlaceEdit.svelte';
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

  type EditableField =
    | null
    | 'seller'
    | 'price'
    | 'purchaseDate'
    | 'addedDate'
    | 'notes'
    | 'purchaseCondition'
    | 'modelCondition'
    | 'boxCondition';

  let editingField = $state<EditableField>(null);
  let savingField = $state<EditableField>(null);
  let errorMessage = $state<string | null>(null);
  let sellers = $state<SellerView[]>([]);

  let sellerDraftId = $state('');
  let priceDraft = $state('');
  let priceCurrencyDraft = $state('EUR');
  let purchaseDateDraft = $state('');
  let addedDateDraft = $state('');
  let purchaseConditionDraft = $state('');
  let modelConditionDraft = $state('');
  let boxConditionDraft = $state('');

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

  function startEditing(field: Exclude<EditableField, null>) {
    if (savingField) return;
    errorMessage = null;
    editingField = field;
    sellerDraftId = seller?.id ?? '';
    const currentPrice = purchasedInfo?.price;
    priceDraft = currentPrice ? (Number(currentPrice.amount) / 100).toFixed(2) : '';
    priceCurrencyDraft = currentPrice?.currency ?? 'EUR';
    purchaseDateDraft = toDateInputValue(purchasedInfo?.purchaseDate ?? null);
    addedDateDraft = toDateInputValue(item.addedDate);
    purchaseConditionDraft = item.purchaseCondition ?? '';
    modelConditionDraft = item.modelCondition ?? '';
    boxConditionDraft = item.boxCondition ?? '';
  }

  function cancelEditing() {
    editingField = null;
    errorMessage = null;
  }

  async function saveUpdate(
    field: Exclude<EditableField, null>,
    update: CollectionItemUpdateArgs
  ): Promise<void> {
    savingField = field;
    errorMessage = null;

    try {
      const result = await commands.updateCollectionItem({
        collectionItemId: item.id,
        update
      });
      if (result.status === 'error') {
        throw new Error('update_failed');
      }
      editingField = null;
      await onItemUpdated?.();
    } catch {
      errorMessage = m.collection_item_error();
    } finally {
      savingField = null;
    }
  }

  async function saveNotes(value: string): Promise<void> {
    const notes = value.trim();
    await saveUpdate('notes', {
      kind: 'notes',
      data: { notes: notes.length > 0 ? notes : null }
    });
  }

  async function savePrice(): Promise<void> {
    const normalized = priceDraft.trim();
    if (!normalized) {
      await saveUpdate('price', {
        kind: 'price',
        data: { amount: null, currency: null }
      });
      return;
    }

    const parsed = Number(normalized);
    if (Number.isNaN(parsed) || parsed < 0) {
      errorMessage = m.collection_item_error();
      return;
    }

    const amount = BigInt(Math.round(parsed * 100));
    await saveUpdate('price', {
      kind: 'price',
      data: { amount, currency: priceCurrencyDraft }
    });
  }
</script>

<aside class="w-full shrink-0 space-y-4 lg:w-80">
  <Card>
    <CardHeader class="pb-3">
      <CardTitle class="text-xs font-semibold tracking-widest uppercase"
        >{m.collection_item_section_acquisition()}</CardTitle
      >
    </CardHeader>
    <CardContent class="space-y-3">
      <div class="grid grid-cols-[auto,1fr] items-start gap-2 text-sm">
        <span class="text-muted-foreground">{m.collection_item_seller()}</span>
        <div class="text-right">
          {#if editingField === 'seller'}
            <div class="space-y-2">
              <select
                bind:value={sellerDraftId}
                class="w-full rounded-md border bg-background px-2 py-1 text-sm"
              >
                <option value="">{m.collection_item_not_recorded()}</option>
                {#each sellers as sellerOption (sellerOption.id)}
                  <option value={sellerOption.id}>{sellerOption.name}</option>
                {/each}
              </select>
              <div class="flex justify-end gap-2">
                <Button
                  size="sm"
                  onclick={() =>
                    saveUpdate('seller', {
                      kind: 'seller',
                      data: { seller_id: sellerDraftId || null }
                    })}
                  disabled={savingField === 'seller'}>{m.edit_field_save()}</Button
                >
                <Button variant="ghost" size="sm" onclick={cancelEditing}
                  >{m.edit_field_cancel()}</Button
                >
              </div>
            </div>
          {:else}
            <button
              type="button"
              class="text-right text-primary hover:underline"
              onclick={() => startEditing('seller')}
            >
              {seller?.name ?? m.collection_item_not_recorded()}
            </button>
          {/if}
        </div>
      </div>

      <Separator />

      <div class="grid grid-cols-[auto,1fr] items-start gap-2 text-sm">
        <span class="text-muted-foreground">{m.collection_item_price()}</span>
        <div class="text-right">
          {#if editingField === 'price'}
            <div class="space-y-2">
              <div class="grid grid-cols-[1fr,auto] gap-2">
                <input
                  bind:value={priceDraft}
                  type="number"
                  min="0"
                  step="0.01"
                  class="rounded-md border bg-background px-2 py-1 text-sm"
                />
                <select
                  bind:value={priceCurrencyDraft}
                  class="rounded-md border bg-background px-2 py-1 text-sm"
                >
                  {#each currencyOptions as currencyCode (currencyCode)}
                    <option value={currencyCode}>{currencyCode}</option>
                  {/each}
                </select>
              </div>
              <div class="flex justify-end gap-2">
                <Button size="sm" onclick={savePrice} disabled={savingField === 'price'}
                  >{m.edit_field_save()}</Button
                >
                <Button variant="ghost" size="sm" onclick={cancelEditing}
                  >{m.edit_field_cancel()}</Button
                >
              </div>
            </div>
          {:else}
            <button
              type="button"
              class="text-right text-primary hover:underline"
              onclick={() => startEditing('price')}
            >
              {#if purchasedInfo?.price}
                {formatPrice(purchasedInfo.price.amount, purchasedInfo.price.currency)}
              {:else}
                {m.collection_item_not_recorded()}
              {/if}
            </button>
          {/if}
        </div>
      </div>

      <Separator />

      <div class="grid grid-cols-[auto,1fr] items-start gap-2 text-sm">
        <span class="text-muted-foreground">{m.collection_item_purchase_date()}</span>
        <div class="text-right">
          {#if editingField === 'purchaseDate'}
            <div class="space-y-2">
              <input
                bind:value={purchaseDateDraft}
                type="date"
                class="w-full rounded-md border bg-background px-2 py-1 text-sm"
              />
              <div class="flex justify-end gap-2">
                <Button
                  size="sm"
                  onclick={() =>
                    saveUpdate('purchaseDate', {
                      kind: 'purchaseDate',
                      data: { purchase_date: purchaseDateDraft || null }
                    })}
                  disabled={savingField === 'purchaseDate'}>{m.edit_field_save()}</Button
                >
                <Button variant="ghost" size="sm" onclick={cancelEditing}
                  >{m.edit_field_cancel()}</Button
                >
              </div>
            </div>
          {:else}
            <button
              type="button"
              class="text-right text-primary hover:underline"
              onclick={() => startEditing('purchaseDate')}
            >
              {#if purchasedInfo?.purchaseDate}
                {formatDate(purchasedInfo.purchaseDate)}
              {:else}
                {m.collection_item_not_recorded()}
              {/if}
            </button>
          {/if}
        </div>
      </div>

      <Separator />

      <div class="grid grid-cols-[auto,1fr] items-start gap-2 text-sm">
        <span class="text-muted-foreground">{m.collection_item_added_date()}</span>
        <div class="text-right">
          {#if editingField === 'addedDate'}
            <div class="space-y-2">
              <input
                bind:value={addedDateDraft}
                type="date"
                class="w-full rounded-md border bg-background px-2 py-1 text-sm"
              />
              <div class="flex justify-end gap-2">
                <Button
                  size="sm"
                  onclick={() =>
                    saveUpdate('addedDate', {
                      kind: 'addedDate',
                      data: { added_date: addedDateDraft || null }
                    })}
                  disabled={savingField === 'addedDate'}>{m.edit_field_save()}</Button
                >
                <Button variant="ghost" size="sm" onclick={cancelEditing}
                  >{m.edit_field_cancel()}</Button
                >
              </div>
            </div>
          {:else}
            <button
              type="button"
              class="text-right text-primary hover:underline"
              onclick={() => startEditing('addedDate')}
            >
              {formatDate(item.addedDate)}
            </button>
          {/if}
        </div>
      </div>

      <Separator />

      <div class="space-y-2">
        <span class="text-sm text-muted-foreground">{m.collection_item_notes()}</span>
        <InPlaceEdit
          value={item.notes ?? ''}
          placeholder={m.collection_item_not_recorded()}
          multiline={true}
          onSave={saveNotes}
        />
      </div>
    </CardContent>
  </Card>

  <Card>
    <CardHeader class="pb-3">
      <CardTitle class="text-xs font-semibold tracking-widest uppercase"
        >{m.collection_item_section_condition()}</CardTitle
      >
    </CardHeader>
    <CardContent class="space-y-3">
      <div class="grid grid-cols-[auto,1fr] items-start gap-2 text-sm">
        <span class="text-muted-foreground">{m.collection_item_purchase_condition()}</span>
        <div class="text-right">
          {#if editingField === 'purchaseCondition'}
            <div class="space-y-2">
              <select
                bind:value={purchaseConditionDraft}
                class="w-full rounded-md border bg-background px-2 py-1 text-sm"
              >
                <option value="">{m.collection_item_not_recorded()}</option>
                {#each purchaseConditionOptions as option (option)}
                  <option value={option}>{purchaseConditionLabel(option)}</option>
                {/each}
              </select>
              <div class="flex justify-end gap-2">
                <Button
                  size="sm"
                  onclick={() =>
                    saveUpdate('purchaseCondition', {
                      kind: 'purchaseCondition',
                      data: { purchase_condition: purchaseConditionDraft || null }
                    })}
                  disabled={savingField === 'purchaseCondition'}>{m.edit_field_save()}</Button
                >
                <Button variant="ghost" size="sm" onclick={cancelEditing}
                  >{m.edit_field_cancel()}</Button
                >
              </div>
            </div>
          {:else}
            <button
              type="button"
              class="text-right text-primary hover:underline"
              onclick={() => startEditing('purchaseCondition')}
            >
              {#if item.purchaseCondition}
                {purchaseConditionLabel(item.purchaseCondition)}
              {:else}
                {m.collection_item_not_recorded()}
              {/if}
            </button>
          {/if}
        </div>
      </div>

      <Separator />

      <div class="grid grid-cols-[auto,1fr] items-start gap-2 text-sm">
        <span class="text-muted-foreground">{m.collection_item_model_condition()}</span>
        <div class="text-right">
          {#if editingField === 'modelCondition'}
            <div class="space-y-2">
              <select
                bind:value={modelConditionDraft}
                class="w-full rounded-md border bg-background px-2 py-1 text-sm"
              >
                <option value="">{m.collection_item_not_recorded()}</option>
                {#each modelConditionOptions as option (option)}
                  <option value={option}>{modelConditionLabel(option)}</option>
                {/each}
              </select>
              <div class="flex justify-end gap-2">
                <Button
                  size="sm"
                  onclick={() =>
                    saveUpdate('modelCondition', {
                      kind: 'modelCondition',
                      data: { model_condition: modelConditionDraft || null }
                    })}
                  disabled={savingField === 'modelCondition'}>{m.edit_field_save()}</Button
                >
                <Button variant="ghost" size="sm" onclick={cancelEditing}
                  >{m.edit_field_cancel()}</Button
                >
              </div>
            </div>
          {:else}
            <button
              type="button"
              class="text-right text-primary hover:underline"
              onclick={() => startEditing('modelCondition')}
            >
              {#if item.modelCondition}
                {modelConditionLabel(item.modelCondition)}
              {:else}
                {m.collection_item_not_recorded()}
              {/if}
            </button>
          {/if}
        </div>
      </div>

      <Separator />

      <div class="grid grid-cols-[auto,1fr] items-start gap-2 text-sm">
        <span class="text-muted-foreground">{m.collection_item_box_condition()}</span>
        <div class="text-right">
          {#if editingField === 'boxCondition'}
            <div class="space-y-2">
              <select
                bind:value={boxConditionDraft}
                class="w-full rounded-md border bg-background px-2 py-1 text-sm"
              >
                <option value="">{m.collection_item_not_recorded()}</option>
                {#each boxConditionOptions as option (option)}
                  <option value={option}>{boxConditionLabel(option)}</option>
                {/each}
              </select>
              <div class="flex justify-end gap-2">
                <Button
                  size="sm"
                  onclick={() =>
                    saveUpdate('boxCondition', {
                      kind: 'boxCondition',
                      data: { box_condition: boxConditionDraft || null }
                    })}
                  disabled={savingField === 'boxCondition'}>{m.edit_field_save()}</Button
                >
                <Button variant="ghost" size="sm" onclick={cancelEditing}
                  >{m.edit_field_cancel()}</Button
                >
              </div>
            </div>
          {:else}
            <button
              type="button"
              class="text-right text-primary hover:underline"
              onclick={() => startEditing('boxCondition')}
            >
              {#if item.boxCondition}
                {boxConditionLabel(item.boxCondition)}
              {:else}
                {m.collection_item_not_recorded()}
              {/if}
            </button>
          {/if}
        </div>
      </div>
    </CardContent>
  </Card>

  {#if errorMessage}
    <p class="text-sm text-destructive">{errorMessage}</p>
  {/if}
</aside>
