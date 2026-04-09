<script lang="ts">
  import type { Snippet } from 'svelte';
  import type { RollingStock } from '$lib/types/railway-model';
  import { Settings } from 'lucide-svelte';
  import * as m from '$lib/paraglide/messages';
  import RollingStockIdentityCluster from './RollingStockIdentityCluster.svelte';
  import RollingStockClassificationCluster from './RollingStockClassificationCluster.svelte';
  import type { RollingStockUnitSpecsFormState } from './rolling-stock-unit-specs-form-state';

  interface Props {
    unit: RollingStock;
    editable: boolean;
    formState: RollingStockUnitSpecsFormState | undefined;
    specLoaded: boolean;
    onSaveRoadNumber: (value: string) => Promise<void>;
    onSaveCategory: (category: string) => Promise<void>;
    onSaveSubcategory: (subcategory: string) => Promise<void>;
    onEditSpecs: () => void;
    extraActions?: Snippet;
  }

  const {
    unit,
    editable,
    formState,
    specLoaded,
    onSaveRoadNumber,
    onSaveCategory,
    onSaveSubcategory,
    onEditSpecs,
    extraActions
  }: Props = $props();

  const editSpecsClass =
    'variant-steampunk-lever flex items-center gap-2 rounded-sm border border-border bg-background px-3 py-1 text-[10px] font-bold tracking-widest text-muted-foreground uppercase transition-all hover:border-primary/50 hover:bg-primary/5 hover:text-primary active:scale-95';
</script>

<div class="grid grid-cols-3 items-center gap-2">
  <div class="flex min-w-0 items-center">
    <RollingStockIdentityCluster {unit} {editable} {onSaveRoadNumber} />
  </div>

  <div class="flex justify-center">
    <RollingStockClassificationCluster
      {unit}
      {editable}
      {specLoaded}
      {formState}
      {onSaveCategory}
      {onSaveSubcategory}
    />
  </div>

  <div class="flex items-center justify-end gap-2">
    {#if editable}
      <button type="button" class={editSpecsClass} onclick={onEditSpecs}>
        <Settings size={12} />
        {m.rolling_stock_edit_specs_button()}
      </button>
    {/if}
    {@render extraActions?.()}
  </div>
</div>
