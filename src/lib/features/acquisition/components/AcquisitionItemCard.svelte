<script lang="ts">
  import * as m from '$lib/paraglide/messages.js';
  import { CurrencyInput } from '$lib/components';
  import { getCurrencySymbol } from '$lib/utils/currency';
  import { Copy, Trash2 } from 'lucide-svelte';
  import type { Manufacturer } from '$lib/bindings';
  import type { AcquisitionItemEntry, AcquisitionItemErrors } from '../types.js';

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

  const SELECT_CLASS =
    'h-10 w-full appearance-none rounded-xl border border-white/10 bg-zinc-950 px-4 text-sm text-zinc-100 focus:border-white/20 focus:outline-none';
  const SELECT_ERROR_CLASS =
    'h-10 w-full appearance-none rounded-xl border border-destructive bg-zinc-950 px-4 text-sm text-zinc-100 ring-1 ring-destructive focus:outline-none';
  const INPUT_CLASS =
    'h-10 w-full rounded-xl border border-white/10 bg-zinc-950 px-4 text-sm text-zinc-100 placeholder:text-zinc-600 focus:border-white/20 focus:outline-none';
  const INPUT_ERROR_CLASS =
    'h-10 w-full rounded-xl border border-destructive bg-zinc-950 px-4 text-sm text-zinc-100 placeholder:text-zinc-600 ring-1 ring-destructive focus:outline-none';
  const LABEL_CLASS = 'ml-1 text-[10px] font-bold tracking-[0.2em] text-zinc-500 uppercase';
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
      <select
        id="item-{item.uid}-manufacturer"
        value={item.manufacturerId ?? ''}
        onchange={(e) =>
          onUpdate(item.uid, {
            manufacturerId: (e.currentTarget as HTMLSelectElement).value || null
          })}
        class={errors.manufacturerId ? SELECT_ERROR_CLASS : SELECT_CLASS}
        aria-invalid={!!errors.manufacturerId}
        aria-describedby={errors.manufacturerId ? `item-${item.uid}-manufacturer-error` : undefined}
      >
        <option value="">—</option>
        {#each manufacturers as mfg (mfg.id)}
          <option value={mfg.id}>{mfg.name}</option>
        {/each}
      </select>
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
      <input
        id="item-{item.uid}-product-code"
        type="text"
        value={item.productCode}
        oninput={(e) =>
          onUpdate(item.uid, { productCode: (e.currentTarget as HTMLInputElement).value })}
        class={errors.productCode ? INPUT_ERROR_CLASS : INPUT_CLASS}
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
      <input
        id="item-{item.uid}-description"
        type="text"
        value={item.description}
        oninput={(e) =>
          onUpdate(item.uid, { description: (e.currentTarget as HTMLInputElement).value })}
        class={INPUT_CLASS}
      />
    </div>

    <!-- Category -->
    <div class="space-y-1">
      <label for="item-{item.uid}-category" class={LABEL_CLASS}>
        {m.acquisition_item_category_label()}
      </label>
      <select
        id="item-{item.uid}-category"
        value={item.category ?? ''}
        onchange={(e) =>
          onUpdate(item.uid, { category: (e.currentTarget as HTMLSelectElement).value || null })}
        class={errors.category ? SELECT_ERROR_CLASS : SELECT_CLASS}
        aria-invalid={!!errors.category}
        aria-describedby={errors.category ? `item-${item.uid}-category-error` : undefined}
      >
        <option value="">—</option>
        {#each CATEGORY_OPTIONS as cat (cat)}
          <option value={cat}>{cat}</option>
        {/each}
      </select>
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
      <select
        id="item-{item.uid}-scale"
        value={item.scale ?? ''}
        onchange={(e) =>
          onUpdate(item.uid, { scale: (e.currentTarget as HTMLSelectElement).value || null })}
        class={SELECT_CLASS}
      >
        <option value="">—</option>
        {#each SCALE_OPTIONS as scale (scale)}
          <option value={scale}>{scale}</option>
        {/each}
      </select>
    </div>

    <!-- Epoch -->
    <div class="space-y-1">
      <label for="item-{item.uid}-epoch" class={LABEL_CLASS}>
        {m.acquisition_item_epoch_label()}
      </label>
      <input
        id="item-{item.uid}-epoch"
        type="text"
        value={item.epoch ?? ''}
        oninput={(e) =>
          onUpdate(item.uid, { epoch: (e.currentTarget as HTMLInputElement).value || null })}
        class={INPUT_CLASS}
      />
    </div>

    <!-- Power Method -->
    <div class="space-y-1">
      <label for="item-{item.uid}-power" class={LABEL_CLASS}>
        {m.acquisition_item_power_label()}
      </label>
      <select
        id="item-{item.uid}-power"
        value={item.powerMethod ?? ''}
        onchange={(e) =>
          onUpdate(item.uid, { powerMethod: (e.currentTarget as HTMLSelectElement).value || null })}
        class={SELECT_CLASS}
      >
        <option value="">—</option>
        {#each POWER_METHOD_OPTIONS as pm (pm)}
          <option value={pm}>{pm}</option>
        {/each}
      </select>
    </div>

    <!-- Price (full width) -->
    <div class="col-span-2 space-y-1">
      <label for="item-{item.uid}-price" class={LABEL_CLASS}>
        {m.acquisition_item_price_label()}
      </label>
      <CurrencyInput
        id="item-{item.uid}-price"
        value={item.priceAmount}
        symbol={getCurrencySymbol(currency)}
        label={m.acquisition_item_price_label()}
        onchange={(val) => onUpdate(item.uid, { priceAmount: val })}
      />
    </div>
  </div>
</div>
