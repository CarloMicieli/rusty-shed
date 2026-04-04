<script lang="ts">
  import type { RollingStock } from '$lib/types/railway-model';
  import type { RollingStockCategory } from '$lib/bindings';
  import BadgePicker from '$lib/components/BadgePicker.svelte';
  import { CATEGORY_OPTIONS, getSubcategoryOptions } from './constants';
  import type { RollingStockUnitSpecsFormState } from './rolling-stock-unit-specs-form-state';

  interface Props {
    unit: RollingStock;
    editable: boolean;
    specLoaded: boolean;
    formState: RollingStockUnitSpecsFormState | undefined;
    onSaveCategory: (category: string) => Promise<void>;
    onSaveSubcategory: (subcategory: string) => Promise<void>;
  }

  const { unit, editable, specLoaded, formState, onSaveCategory, onSaveSubcategory }: Props =
    $props();
</script>

{#if editable && specLoaded}
  {@const currentCategory = (formState?.category ?? unit.category) as RollingStockCategory | null}
  {@const subcategoryOpts = getSubcategoryOptions(currentCategory)}
  <div class="flex items-center justify-center gap-1 px-2">
    <BadgePicker
      value={formState?.category ?? unit.category ?? ''}
      options={CATEGORY_OPTIONS}
      onSelect={onSaveCategory}
    />
    {#if subcategoryOpts.length > 0}
      <span class="font-sans text-[10px] leading-none text-muted-foreground/40" aria-hidden="true"
        >•</span
      >
      <div class="inline-flex items-center" class:animate-pulse={formState?.subcategoryFlashed}>
        <BadgePicker
          value={formState?.subcategory ?? ''}
          options={subcategoryOpts}
          onSelect={onSaveSubcategory}
        />
      </div>
    {/if}
  </div>
{:else}
  {@const catLabel =
    CATEGORY_OPTIONS.find((o) => o.id === unit.category)?.label ?? unit.rolling_stock_type ?? ''}
  {@const subcatOpts = getSubcategoryOptions(unit.category as RollingStockCategory | null)}
  {@const subcatLabel = subcatOpts.find((o) => o.id === unit.subcategory)?.label ?? null}
  <div class="flex items-center justify-center px-4">
    <div
      class="font-sans text-[10px] font-semibold tracking-wider text-muted-foreground/80 uppercase"
    >
      {catLabel || '—'}{#if subcatLabel}
        <span aria-hidden="true" class="text-muted-foreground/40"> • </span>{subcatLabel}
      {/if}
    </div>
  </div>
{/if}
