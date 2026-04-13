<script lang="ts">
  import type { Snippet } from 'svelte';
  import type { RollingStock } from '$lib/types/railway-model';
  import { Settings, Trash2 } from 'lucide-svelte';
  import * as m from '$lib/paraglide/messages';
  import * as AlertDialog from '$lib/components/ui/alert-dialog';
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
    onDelete?: (unitId: string) => Promise<void> | void;
    deletePending?: boolean;
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
    onDelete,
    deletePending = false,
    extraActions
  }: Props = $props();

  let showDeleteDialog = $state(false);
  let confirmDeletePending = $state(false);

  const editSpecsClass =
    'variant-steampunk-lever flex items-center gap-2 rounded-sm border border-border bg-background px-3 py-1 text-[10px] font-bold tracking-widest text-muted-foreground uppercase transition-all hover:border-primary/50 hover:bg-primary/5 hover:text-primary active:scale-95';
  const deleteClass =
    'flex items-center justify-center rounded-sm border border-border p-1 text-muted-foreground transition-colors hover:border-destructive/50 hover:bg-destructive/5 hover:text-destructive disabled:cursor-not-allowed disabled:opacity-50';

  async function handleConfirmDelete() {
    if (!onDelete || confirmDeletePending || deletePending) return;
    confirmDeletePending = true;
    try {
      await onDelete(unit.id);
      showDeleteDialog = false;
    } finally {
      confirmDeletePending = false;
    }
  }
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
      <button
        type="button"
        class={deleteClass}
        aria-label={m.rolling_stock_delete_button()}
        disabled={deletePending}
        onclick={(event) => {
          event.stopPropagation();
          showDeleteDialog = true;
        }}
      >
        <Trash2 size={12} />
      </button>
      <button type="button" class={editSpecsClass} onclick={onEditSpecs}>
        <Settings size={12} />
        {m.rolling_stock_edit_specs_button()}
      </button>
    {/if}
    {@render extraActions?.()}
  </div>
</div>

<AlertDialog.Root bind:open={showDeleteDialog}>
  <AlertDialog.Content>
    <AlertDialog.Header>
      <AlertDialog.Title>{m.rolling_stock_delete_confirm_title()}</AlertDialog.Title>
      <AlertDialog.Description>
        {m.rolling_stock_delete_confirm_message()}
      </AlertDialog.Description>
    </AlertDialog.Header>
    <AlertDialog.Footer>
      <AlertDialog.Cancel>{m.common_cancel()}</AlertDialog.Cancel>
      <AlertDialog.Action
        onclick={handleConfirmDelete}
        disabled={confirmDeletePending || deletePending}
      >
        {#if confirmDeletePending || deletePending}
          {m.rolling_stock_delete_loading()}
        {:else}
          {m.common_delete()}
        {/if}
      </AlertDialog.Action>
    </AlertDialog.Footer>
  </AlertDialog.Content>
</AlertDialog.Root>
