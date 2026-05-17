<script lang="ts">
  import * as m from '$lib/paraglide/messages.js';
  import * as Select from '$lib/components/ui/select';
  import { CurrencyInput } from '$lib/components';
  import { DrawerInput, EpochPicker } from '$lib/components/drawer';
  import { regionalManager } from '$lib/features/settings/RegionalManager.svelte';
  import { Copy, Plus, Trash2 } from 'lucide-svelte';
  import type { Category, Manufacturer } from '$lib/bindings';
  import type { AcquisitionItemEntry, AcquisitionItemErrors } from '../types.js';
  import { categoryOptions, categoryLabel } from '$lib/utils/enum-options';

  interface Props {
    item: AcquisitionItemEntry;
    index: number;
    manufacturers: Manufacturer[];
    currency: string;
    errors: AcquisitionItemErrors;
    canRemove: boolean;
    onUpdate: (uid: string, patch: Partial<AcquisitionItemEntry>) => void;
    onDuplicate: (uid: string) => void;
    onRemove: (uid: string) => void;
    onQuickAddManufacturer: (uid: string) => void;
  }

  let {
    item,
    index,
    manufacturers,
    currency,
    errors,
    canRemove,
    onUpdate,
    onDuplicate,
    onRemove,
    onQuickAddManufacturer
  }: Props = $props();

  const LABEL_CLASS = 'text-[10px] font-bold tracking-[0.2em] text-muted-foreground uppercase';
  const TRIGGER_CLASS =
    'h-10 w-full border-border bg-background text-foreground data-[placeholder]:text-muted-foreground focus-visible:border-primary/60 focus-visible:ring-1 focus-visible:ring-primary';
  const TRIGGER_ERROR_CLASS =
    'h-10 w-full border-destructive bg-background text-foreground ring-1 ring-destructive';
  const SELECT_CONTENT_CLASS = 'border-border bg-background text-foreground';

  const selectedManufacturer = $derived(
    manufacturers.find((mfg) => mfg.id === item.manufacturerId)
  );
</script>

<div class="space-y-3 rounded-xl border border-white/10 bg-zinc-900/50 p-4">
  <!-- Top row: item title + action icons -->
  <div class="flex items-center justify-between">
    <span class="text-[10px] font-bold tracking-[0.2em] text-muted-foreground uppercase">
      Item {index + 1}
    </span>
    <div class="flex items-center gap-1">
      <button
        type="button"
        onclick={() => onDuplicate(item.uid)}
        class="rounded p-1.5 text-zinc-500 transition-colors hover:bg-white/5 hover:text-zinc-300"
        aria-label={m.action_duplicate_item()}
      >
        <Copy size={14} />
      </button>
      {#if canRemove}
        <button
          type="button"
          onclick={() => onRemove(item.uid)}
          class="rounded p-1.5 text-zinc-500 transition-colors hover:bg-destructive/10 hover:text-destructive"
          aria-label={m.action_remove_item()}
        >
          <Trash2 size={14} />
        </button>
      {/if}
    </div>
  </div>

  <!-- Fields in 2-column grid -->
  <div class="grid grid-cols-2 items-start gap-3">
    <!-- Manufacturer -->
    <div class="min-w-0 space-y-1">
      <div class="flex items-center justify-between gap-2">
        <label for="item-{item.uid}-manufacturer" class={LABEL_CLASS}>
          {m.acquisition_item_manufacturer_label()}
        </label>
        <button
          type="button"
          class="rounded-sm border border-border p-1 text-muted-foreground hover:text-foreground"
          aria-label={m.quick_add_drawer_title_manufacturer()}
          onclick={() => onQuickAddManufacturer(item.uid)}
        >
          <Plus size={12} />
        </button>
      </div>
      <Select.Root
        type="single"
        value={item.manufacturerId ?? undefined}
        onValueChange={(v) => onUpdate(item.uid, { manufacturerId: v || null })}
      >
        <Select.Trigger
          id="item-{item.uid}-manufacturer"
          class={errors.manufacturerId ? TRIGGER_ERROR_CLASS : TRIGGER_CLASS}
          aria-invalid={!!errors.manufacturerId}
          aria-describedby={errors.manufacturerId
            ? `item-${item.uid}-manufacturer-error`
            : undefined}
        >
          {#if selectedManufacturer}
            {selectedManufacturer.name}
          {:else}
            <span class="text-muted-foreground"
              >{m.acquisition_item_manufacturer_placeholder()}</span
            >
          {/if}
        </Select.Trigger>
        <Select.Content class={SELECT_CONTENT_CLASS}>
          {#each manufacturers as mfg (mfg.id)}
            <Select.Item value={mfg.id} label={mfg.name} />
          {/each}
        </Select.Content>
      </Select.Root>
      {#if errors.manufacturerId}
        <p id="item-{item.uid}-manufacturer-error" class="mt-1 text-xs text-destructive">
          {errors.manufacturerId}
        </p>
      {/if}
    </div>

    <!-- Product Code -->
    <div class="min-w-0 space-y-1">
      <label for="item-{item.uid}-product-code" class={LABEL_CLASS}>
        {m.acquisition_item_product_code_label()}
      </label>
      <DrawerInput
        id="item-{item.uid}-product-code"
        type="text"
        placeholder={m.acquisition_item_product_code_placeholder()}
        value={item.productCode}
        oninput={(e) =>
          onUpdate(item.uid, { productCode: (e.currentTarget as HTMLInputElement).value })}
        aria-invalid={!!errors.productCode}
        aria-describedby={errors.productCode ? `item-${item.uid}-product-code-error` : undefined}
      />
      {#if errors.productCode}
        <p id="item-{item.uid}-product-code-error" class="mt-1 text-xs text-destructive">
          {errors.productCode}
        </p>
      {/if}
    </div>

    <!-- Description (full width) -->
    <div class="col-span-2 space-y-1">
      <label for="item-{item.uid}-description" class={LABEL_CLASS}>
        {m.acquisition_item_description_label()}
      </label>
      <DrawerInput
        id="item-{item.uid}-description"
        type="text"
        placeholder={m.acquisition_item_description_placeholder()}
        value={item.description}
        oninput={(e) =>
          onUpdate(item.uid, { description: (e.currentTarget as HTMLInputElement).value })}
      />
    </div>

    <!-- Category -->
    <div class="space-y-1">
      <label for="item-{item.uid}-category" class={LABEL_CLASS}>
        {m.acquisition_item_category_label()}
      </label>
      <Select.Root
        type="single"
        value={item.category ?? undefined}
        onValueChange={(v) => onUpdate(item.uid, { category: (v as Category) || null })}
      >
        <Select.Trigger
          id="item-{item.uid}-category"
          class={errors.category ? TRIGGER_ERROR_CLASS : TRIGGER_CLASS}
          aria-invalid={!!errors.category}
          aria-describedby={errors.category ? `item-${item.uid}-category-error` : undefined}
        >
          {#if item.category}
            {categoryLabel(item.category)}
          {:else}
            <span class="text-muted-foreground">{m.acquisition_item_category_placeholder()}</span>
          {/if}
        </Select.Trigger>
        <Select.Content class={SELECT_CONTENT_CLASS}>
          {#each categoryOptions() as opt (opt.value)}
            <Select.Item value={opt.value} label={opt.label} />
          {/each}
        </Select.Content>
      </Select.Root>
      {#if errors.category}
        <p id="item-{item.uid}-category-error" class="mt-1 text-xs text-destructive">
          {errors.category}
        </p>
      {/if}
    </div>

    <!-- Epoch -->
    <div class="col-span-2 space-y-1">
      <EpochPicker
        label={m.acquisition_item_epoch_label()}
        value={item.epoch}
        onchange={(v) => onUpdate(item.uid, { epoch: v })}
      />
    </div>

    <!-- Price (full width) -->
    <div class="col-span-2 space-y-1">
      <label for="item-{item.uid}-price" class={LABEL_CLASS}>
        {m.acquisition_item_price_label()}
      </label>
      <CurrencyInput
        id="item-{item.uid}-price"
        value={item.priceAmount}
        symbol={regionalManager.getCurrencySymbol(currency)}
        label={m.acquisition_item_price_label()}
        inputClass="h-10 rounded-sm border-border bg-background text-foreground placeholder:text-muted-foreground focus-visible:border-primary/60 focus-visible:ring-1 focus-visible:ring-primary"
        onchange={(val) => onUpdate(item.uid, { priceAmount: val })}
      />
    </div>
  </div>
</div>
