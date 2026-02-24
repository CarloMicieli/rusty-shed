<script lang="ts">
  import { page } from '$app/stores';
  import { goto } from '$app/navigation';
  import { getTrackInventoryContext } from '$lib/features/track-inventory';
  import InventoryDetail from '$lib/features/track-inventory/components/InventoryDetail.svelte';
  import RenameInventoryDialog from '$lib/features/track-inventory/components/RenameInventoryDialog.svelte';
  import DeleteInventoryDialog from '$lib/features/track-inventory/components/DeleteInventoryDialog.svelte';
  import AddPurchaseDialog from '$lib/features/track-inventory/components/AddPurchaseDialog.svelte';
  import { onMount } from 'svelte';

  const service = getTrackInventoryContext();

  const inventoryId = $derived($page.params.id);

  let inventory = $state<Awaited<ReturnType<typeof service.fetchInventory>> | null>(null);
  let loading = $state(true);
  let error = $state<string | null>(null);
  let renameDialogOpen = $state(false);
  let deleteDialogOpen = $state(false);
  let addPurchaseDialogOpen = $state(false);

  async function loadInventory() {
    if (!inventoryId) return;

    try {
      loading = true;
      error = null;
      inventory = await service.fetchInventory(inventoryId);
    } catch (err) {
      console.error('Failed to load inventory:', err);
      error = err instanceof Error ? err.message : 'Failed to load inventory';
    } finally {
      loading = false;
    }
  }

  async function handleRename(newName: string) {
    if (!inventoryId) return;
    await service.renameInventory({ id: inventoryId, newName });
    await loadInventory();
  }

  async function handleDelete() {
    if (!inventoryId) return;
    await service.deleteInventory(inventoryId);
    goto('/railway-tracks');
  }

  onMount(() => {
    loadInventory();
  });
</script>

<div class="container mx-auto space-y-6 p-4">
  {#if loading}
    <div class="flex items-center justify-center py-16">
      <div
        class="variant-filled-primary h-12 w-12 animate-spin rounded-full border-4 border-t-transparent"
      ></div>
    </div>
  {:else if error}
    <div class="variant-filled-error rounded-lg p-4">
      <p class="font-medium">{error}</p>
    </div>
  {:else if inventory}
    <InventoryDetail
      {inventory}
      onRename={() => (renameDialogOpen = true)}
      onDelete={() => (deleteDialogOpen = true)}
      onAddPurchase={() => (addPurchaseDialogOpen = true)}
    />
  {:else}
    <div class="variant-ghost-surface rounded-lg p-8 text-center">
      <p class="text-surface-400">Inventory not found</p>
    </div>
  {/if}
</div>

<RenameInventoryDialog
  bind:open={renameDialogOpen}
  currentName={inventory?.name}
  onRename={handleRename}
/>
<DeleteInventoryDialog
  bind:open={deleteDialogOpen}
  inventoryName={inventory?.name}
  onConfirm={handleDelete}
/>
<AddPurchaseDialog
  bind:open={addPurchaseDialogOpen}
  inventoryId={inventoryId || ''}
  onPurchaseAdded={loadInventory}
/>
