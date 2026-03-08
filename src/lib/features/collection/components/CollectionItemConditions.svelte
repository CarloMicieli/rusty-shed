<script lang="ts">
  import InPlaceSelectEdit from '$lib/components/InPlaceSelectEdit.svelte';
  import * as m from '$lib/paraglide/messages.js';
  import { commands } from '$lib/bindings';
  import type {
    CollectionItemView,
    CollectionItemUpdateArgs,
    ModelCondition,
    BoxCondition,
    PurchaseCondition
  } from '$lib/bindings';

  interface Props {
    item: CollectionItemView;
    onItemUpdated?: () => Promise<void>;
  }

  let { item, onItemUpdated }: Props = $props();

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

  function isPositiveCondition(value: string): boolean {
    return ['NEW', 'MINT', 'ORIGINAL_MINT'].includes(value);
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
