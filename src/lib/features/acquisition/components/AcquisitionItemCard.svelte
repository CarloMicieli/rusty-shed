<script lang="ts">
  import * as m from '$lib/paraglide/messages.js';
  import * as Select from '$lib/components/ui/select';
  import { CurrencyInput } from '$lib/components';
  import { DrawerInput } from '$lib/components/drawer';
  import { regionalManager } from '$lib/features/settings/RegionalManager.svelte';
  import { Copy, Trash2 } from 'lucide-svelte';
  import type { Manufacturer } from '$lib/bindings';
  import type { AcquisitionItemEntry, AcquisitionItemErrors } from '../types.js';
  import { EPOCHS } from '$lib/features/wishlists/constants.js';

  const SCALE_OPTIONS = ['H0', 'H0m', 'H0e', 'N', 'TT', 'Z', '0', 'G', 'S', 'II'] as const;
  const POWER_METHOD_OPTIONS = ['AC', 'DC', 'TRIX_EXPRESS'] as const;
  const CATEGORY_OPTIONS = [
    'LOCOMOTIVES',
    'FREIGHT_CARS',
    'PASSENGER_CARS',
    'ELECTRIC_MULTIPLE_UNITS',
    'RAILCARS',
    'TRAIN_SETS',
    'STARTER_SETS'
  ] as const;

  const SCALE_DISPLAY_MAP: Record<string, string> = {
    H0: 'H0 (1:87)',
    H0m: 'H0m (1:87)',
    H0e: 'H0e (1:87)',
    N: 'N (1:160)',
    TT: 'TT (1:120)',
    Z: 'Z (1:220)',
    '0': '0 (1:43)',
    G: 'G (1:22.5)',
    S: 'S (1:64)',
    II: 'II (1:22.5)'
  };

  const CATEGORY_LABELS: Record<string, () => string> = {
    LOCOMOTIVES: m.wishlist_category_locomotives,
    FREIGHT_CARS: m.wishlist_category_freight_cars,
    PASSENGER_CARS: m.wishlist_category_passenger_cars,
    ELECTRIC_MULTIPLE_UNITS: m.wishlist_category_electric_multiple_units,
    RAILCARS: m.wishlist_category_railcars,
    TRAIN_SETS: m.wishlist_category_train_sets,
    STARTER_SETS: m.wishlist_category_starter_sets
  };

  function getCategoryLabel(category: string): string {
    return CATEGORY_LABELS[category]?.() ?? category;
  }

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
    onRemove
  }: Props = $props();

  const LABEL_CLASS = 'ml-1 text-[10px] font-bold tracking-[0.2em] text-zinc-500 uppercase';
  const TRIGGER_CLASS = 'w-full border-border bg-card text-foreground';
  const TRIGGER_ERROR_CLASS =
    'w-full border-destructive bg-card text-foreground ring-1 ring-destructive';

  const selectedManufacturer = $derived(
    manufacturers.find((mfg) => mfg.id === item.manufacturerId)
  );
</script>

<div class="space-y-3 rounded-xl border border-white/10 bg-zinc-900/50 p-4">
  <!-- Top row: item title + action icons -->
  <div class="flex items-center justify-between">
    <span class="text-xs font-semibold tracking-wide text-zinc-400 uppercase">
      Item {index + 1}
    </span>
    <div class="flex items-center gap-1">
      <button
        type="button"
        onclick={() => onDuplicate(item.uid)}
        class="rounded p-1.5 text-zinc-500 transition-colors hover:bg-white/5 hover:text-zinc-300"
        aria-label="Duplicate item"
      >
        <Copy size={14} />
      </button>
      {#if canRemove}
        <button
          type="button"
          onclick={() => onRemove(item.uid)}
          class="rounded p-1.5 text-zinc-500 transition-colors hover:bg-destructive/10 hover:text-destructive"
          aria-label="Remove item"
        >
          <Trash2 size={14} />
        </button>
      {/if}
    </div>
  </div>

  <!-- Fields in 2-column grid -->
  <div class="grid grid-cols-2 gap-3">
    <!-- Manufacturer -->
    <div class="space-y-1">
      <label for="item-{item.uid}-manufacturer" class={LABEL_CLASS}>
        {m.acquisition_item_manufacturer_label()}
      </label>
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
            <span class="text-zinc-500">—</span>
          {/if}
        </Select.Trigger>
        <Select.Content>
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
    <div class="space-y-1">
      <label for="item-{item.uid}-product-code" class={LABEL_CLASS}>
        {m.acquisition_item_product_code_label()}
      </label>
      <DrawerInput
        id="item-{item.uid}-product-code"
        type="text"
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
        onValueChange={(v) => onUpdate(item.uid, { category: v || null })}
      >
        <Select.Trigger
          id="item-{item.uid}-category"
          class={errors.category ? TRIGGER_ERROR_CLASS : TRIGGER_CLASS}
          aria-invalid={!!errors.category}
          aria-describedby={errors.category ? `item-${item.uid}-category-error` : undefined}
        >
          {#if item.category}
            {getCategoryLabel(item.category)}
          {:else}
            <span class="text-zinc-500">—</span>
          {/if}
        </Select.Trigger>
        <Select.Content>
          {#each CATEGORY_OPTIONS as cat (cat)}
            <Select.Item value={cat} label={getCategoryLabel(cat)} />
          {/each}
        </Select.Content>
      </Select.Root>
      {#if errors.category}
        <p id="item-{item.uid}-category-error" class="mt-1 text-xs text-destructive">
          {errors.category}
        </p>
      {/if}
    </div>

    <!-- Scale -->
    <div class="space-y-1">
      <label for="item-{item.uid}-scale" class={LABEL_CLASS}>
        {m.acquisition_item_scale_label()}
      </label>
      <Select.Root
        type="single"
        value={item.scale ?? undefined}
        onValueChange={(v) => onUpdate(item.uid, { scale: v || null })}
      >
        <Select.Trigger id="item-{item.uid}-scale" class={TRIGGER_CLASS}>
          {#if item.scale}
            {SCALE_DISPLAY_MAP[item.scale] ?? item.scale}
          {:else}
            <span class="text-zinc-500">—</span>
          {/if}
        </Select.Trigger>
        <Select.Content>
          {#each SCALE_OPTIONS as scale (scale)}
            <Select.Item value={scale} label={SCALE_DISPLAY_MAP[scale] ?? scale} />
          {/each}
        </Select.Content>
      </Select.Root>
    </div>

    <!-- Epoch -->
    <div class="space-y-1">
      <label for="item-{item.uid}-epoch" class={LABEL_CLASS}>
        {m.acquisition_item_epoch_label()}
      </label>
      <Select.Root
        type="single"
        value={item.epoch ?? undefined}
        onValueChange={(v) => onUpdate(item.uid, { epoch: v || null })}
      >
        <Select.Trigger id="item-{item.uid}-epoch" class={TRIGGER_CLASS}>
          {#if item.epoch}
            {item.epoch}
          {:else}
            <span class="text-zinc-500">—</span>
          {/if}
        </Select.Trigger>
        <Select.Content>
          {#each EPOCHS as epoch (epoch)}
            <Select.Item value={epoch} label={epoch} />
          {/each}
        </Select.Content>
      </Select.Root>
    </div>

    <!-- Power Method -->
    <div class="space-y-1">
      <label for="item-{item.uid}-power" class={LABEL_CLASS}>
        {m.acquisition_item_power_label()}
      </label>
      <Select.Root
        type="single"
        value={item.powerMethod ?? undefined}
        onValueChange={(v) => onUpdate(item.uid, { powerMethod: v || null })}
      >
        <Select.Trigger id="item-{item.uid}-power" class={TRIGGER_CLASS}>
          {#if item.powerMethod}
            {item.powerMethod}
          {:else}
            <span class="text-zinc-500">—</span>
          {/if}
        </Select.Trigger>
        <Select.Content>
          {#each POWER_METHOD_OPTIONS as pm (pm)}
            <Select.Item value={pm} label={pm} />
          {/each}
        </Select.Content>
      </Select.Root>
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
        inputClass="bg-card border-border rounded-[8px] text-foreground placeholder:text-muted-foreground"
        onchange={(val) => onUpdate(item.uid, { priceAmount: val })}
      />
    </div>
  </div>
</div>
