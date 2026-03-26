<script lang="ts">
  import * as m from '$lib/paraglide/messages.js';
  import * as Select from '$lib/components/ui/select';
  import { CurrencyInput, DatePickerField } from '$lib/components';
  import { regionalManager } from '$lib/features/settings/RegionalManager.svelte';
  import { CalendarDate } from '@internationalized/date';
  import type { SellerView } from '$lib/bindings';

  interface Props {
    priceAmount: number | null;
    priceCurrency: string;
    purchaseDate: string;
    selectedSellerId: string;
    selectedPurchaseCondition: string;
    selectedModelCondition: string;
    selectedBoxCondition: string;
    sellers: SellerView[];
    isSubmitting: boolean;
    today: string;
  }

  let {
    priceAmount = $bindable(),
    priceCurrency = $bindable(),
    purchaseDate = $bindable(),
    selectedSellerId = $bindable(),
    selectedPurchaseCondition = $bindable(),
    selectedModelCondition = $bindable(),
    selectedBoxCondition = $bindable(),
    sellers,
    isSubmitting,
    today
  }: Props = $props();

  const todayCalendar = $derived.by(() => {
    const [y, mo, d] = today.split('-').map(Number);
    return new CalendarDate(y, mo, d);
  });

  const PURCHASE_CONDITION_OPTIONS = [
    { value: 'NEW', label: m.purchase_dialog_purchase_condition_new() },
    { value: 'PRE_OWNED', label: m.purchase_dialog_purchase_condition_pre_owned() }
  ];

  const MODEL_CONDITION_OPTIONS = [
    { value: 'MINT', label: m.purchase_dialog_model_condition_mint() },
    { value: 'NEAR_MINT', label: m.purchase_dialog_model_condition_near_mint() },
    { value: 'EXCELLENT', label: m.purchase_dialog_model_condition_excellent() },
    { value: 'VERY_GOOD', label: m.purchase_dialog_model_condition_very_good() },
    { value: 'GOOD', label: m.purchase_dialog_model_condition_good() },
    { value: 'FAIR', label: m.purchase_dialog_model_condition_fair() },
    { value: 'POOR', label: m.purchase_dialog_model_condition_poor() },
    { value: 'FOR_PARTS', label: m.purchase_dialog_model_condition_for_parts() }
  ];

  const BOX_CONDITION_OPTIONS = [
    { value: 'ORIGINAL_MINT', label: m.purchase_dialog_box_condition_original_mint() },
    { value: 'ORIGINAL_GOOD', label: m.purchase_dialog_box_condition_original_good() },
    { value: 'ORIGINAL_WORN', label: m.purchase_dialog_box_condition_original_worn() },
    { value: 'REPLACEMENT_BOX', label: m.purchase_dialog_box_condition_replacement_box() },
    { value: 'NO_BOX', label: m.purchase_dialog_box_condition_no_box() }
  ];

  const purchaseConditionLabel = $derived(
    PURCHASE_CONDITION_OPTIONS.find((o) => o.value === selectedPurchaseCondition)?.label ??
      m.purchase_dialog_purchase_condition_placeholder()
  );

  const modelConditionLabel = $derived(
    MODEL_CONDITION_OPTIONS.find((o) => o.value === selectedModelCondition)?.label ??
      m.purchase_dialog_model_condition_placeholder()
  );

  const boxConditionLabel = $derived(
    BOX_CONDITION_OPTIONS.find((o) => o.value === selectedBoxCondition)?.label ??
      m.purchase_dialog_box_condition_placeholder()
  );
</script>

<!-- Price -->
<div class="space-y-2">
  <label
    for="purchase-price"
    class="ml-1 text-[10px] font-bold tracking-[0.2em] text-zinc-500 uppercase"
  >
    {m.purchase_dialog_price_label()}
  </label>
  <div class="flex gap-2">
    <CurrencyInput
      id="purchase-price"
      bind:value={priceAmount}
      symbol={regionalManager.getCurrencySymbol(priceCurrency)}
      placeholder={m.purchase_dialog_price_placeholder()}
      disabled={isSubmitting}
      required
      class="flex-1"
      label={m.purchase_dialog_price_label()}
    />
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
  <DatePickerField
    id="purchase-date"
    bind:value={purchaseDate}
    maxValue={todayCalendar}
    disabled={isSubmitting}
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

<!-- Condition inputs (2×2: Purchase + Model on row 1, Box spanning row 2) -->
<div class="grid grid-cols-2 gap-4">
  <!-- Purchase Condition -->
  <div class="flex flex-col gap-1.5">
    <label
      for="purchase-condition"
      class="text-[10px] font-bold tracking-widest text-muted-foreground uppercase"
    >
      {m.purchase_dialog_purchase_condition_label()}
    </label>
    <Select.Root
      type="single"
      value={selectedPurchaseCondition || undefined}
      onValueChange={(v) => (selectedPurchaseCondition = v ?? '')}
      disabled={isSubmitting}
    >
      <Select.Trigger
        id="purchase-condition"
        class="!h-10 w-full border-layout-border bg-layout-surface text-foreground focus-visible:border-primary/60 focus-visible:ring-primary/20 data-[state=open]:border-primary"
      >
        {purchaseConditionLabel}
      </Select.Trigger>
      <Select.Content
        class="z-[200] border-layout-border bg-layout-surface text-foreground [&_[data-highlighted]]:bg-primary/10 [&_[data-highlighted]]:text-primary"
      >
        {#each PURCHASE_CONDITION_OPTIONS as opt (opt.value)}
          <Select.Item value={opt.value} label={opt.label} />
        {/each}
      </Select.Content>
    </Select.Root>
  </div>

  <!-- Model Condition -->
  <div class="flex flex-col gap-1.5">
    <label
      for="model-condition"
      class="text-[10px] font-bold tracking-widest text-muted-foreground uppercase"
    >
      {m.purchase_dialog_model_condition_label()}
    </label>
    <Select.Root
      type="single"
      value={selectedModelCondition || undefined}
      onValueChange={(v) => (selectedModelCondition = v ?? '')}
      disabled={isSubmitting}
    >
      <Select.Trigger
        id="model-condition"
        class="!h-10 w-full border-layout-border bg-layout-surface text-foreground focus-visible:border-primary/60 focus-visible:ring-primary/20 data-[state=open]:border-primary"
      >
        {modelConditionLabel}
      </Select.Trigger>
      <Select.Content
        class="z-[200] border-layout-border bg-layout-surface text-foreground [&_[data-highlighted]]:bg-primary/10 [&_[data-highlighted]]:text-primary"
      >
        {#each MODEL_CONDITION_OPTIONS as opt (opt.value)}
          <Select.Item value={opt.value} label={opt.label} />
        {/each}
      </Select.Content>
    </Select.Root>
  </div>

  <!-- Box Condition (full-width second row) -->
  <div class="col-span-2 flex flex-col gap-1.5">
    <label
      for="box-condition"
      class="text-[10px] font-bold tracking-widest text-muted-foreground uppercase"
    >
      {m.purchase_dialog_box_condition_label()}
    </label>
    <Select.Root
      type="single"
      value={selectedBoxCondition || undefined}
      onValueChange={(v) => (selectedBoxCondition = v ?? '')}
      disabled={isSubmitting}
    >
      <Select.Trigger
        id="box-condition"
        class="!h-10 w-full border-layout-border bg-layout-surface text-foreground focus-visible:border-primary/60 focus-visible:ring-primary/20 data-[state=open]:border-primary"
      >
        {boxConditionLabel}
      </Select.Trigger>
      <Select.Content
        class="z-[200] border-layout-border bg-layout-surface text-foreground [&_[data-highlighted]]:bg-primary/10 [&_[data-highlighted]]:text-primary"
      >
        {#each BOX_CONDITION_OPTIONS as opt (opt.value)}
          <Select.Item value={opt.value} label={opt.label} />
        {/each}
      </Select.Content>
    </Select.Root>
  </div>
</div>
