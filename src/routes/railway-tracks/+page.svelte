<script lang="ts">
  import * as m from '$lib/paraglide/messages';
  import { getTrackInventoryContext } from '$lib/features/track-inventory';
  import type { TrackInventoryListItem, TrackInventoryView } from '$lib/features/track-inventory';
  import InventoryDetail from '$lib/features/track-inventory/components/InventoryDetail.svelte';
  import TrackCommandBar from '$lib/features/track-inventory/components/TrackCommandBar.svelte';
  import CreateInventoryDialog from '$lib/features/track-inventory/components/CreateInventoryDialog.svelte';
  import RenameInventoryDialog from '$lib/features/track-inventory/components/RenameInventoryDialog.svelte';
  import DeleteInventoryDialog from '$lib/features/track-inventory/components/DeleteInventoryDialog.svelte';
  import AddTracksPurchaseDrawer from '$lib/features/track-inventory/components/AddTracksPurchaseDrawer.svelte';
  import { onMount } from 'svelte';
  import { Plus, TrainTrack } from 'lucide-svelte';
  import { Button, PageHeader, EmptyState } from '$lib/components';

  const service = getTrackInventoryContext();

  let inventories = $state<TrackInventoryListItem[]>([]);
  let activeInventoryId = $state<string | null>(null);
  let activeInventory = $state<TrackInventoryView | null>(null);
  let loading = $state(true);
  let detailLoading = $state(false);
  let error = $state<string | null>(null);

  let createDialogOpen = $state(false);
  let renameDialogOpen = $state(false);
  let deleteDialogOpen = $state(false);
  let addPurchaseDrawerOpen = $state(false);

  async function loadInventories() {
    try {
      loading = true;
      error = null;
      inventories = await service.fetchInventories();
      if (inventories.length > 0 && activeInventoryId === null) {
        await handleSelect(inventories[0].id);
      }
    } catch (err) {
      console.error('Failed to load inventories:', err);
      error = err instanceof Error ? err.message : 'Failed to load inventories';
    } finally {
      loading = false;
    }
  }

  async function loadActiveInventory() {
    if (!activeInventoryId) return;
    try {
      detailLoading = true;
      activeInventory = await service.fetchInventory(activeInventoryId);
    } catch (err) {
      console.error('Failed to load inventory detail:', err);
    } finally {
      detailLoading = false;
    }
  }

  async function handleSelect(id: string) {
    activeInventoryId = id;
    await loadActiveInventory();
  }

  async function handleCreate(name: string, description: string) {
    const newId = await service.createInventory({ name, description });
    inventories = await service.fetchInventories();
    await handleSelect(newId);
  }

  async function handleRename(newName: string) {
    if (!activeInventoryId) return;
    await service.renameInventory({ id: activeInventoryId, newName });
    inventories = await service.fetchInventories();
    await loadActiveInventory();
  }

  async function handleDelete() {
    if (!activeInventoryId) return;
    await service.deleteInventory(activeInventoryId);
    activeInventoryId = null;
    activeInventory = null;
    inventories = await service.fetchInventories();
    if (inventories.length > 0) {
      await handleSelect(inventories[0].id);
    }
  }

  onMount(() => {
    void loadInventories();
  });
</script>

<svelte:head>
  <title>{m.track_inventories_title()}</title>
</svelte:head>

<div class="mb-10 flex flex-col">
  <!-- Page Header -->
  <div class="-mx-4 -mt-4 border-b border-border bg-card/50 px-6 py-4 lg:-mx-8 lg:-mt-8">
    <PageHeader
      title={m.track_inventories_title()}
      subtitle={m.app_tracks()}
      description={m.track_inventories_subtitle()}
    >
      {#snippet actions()}
        {#if !loading && inventories.length > 0}
          <Button
            variant="default"
            class="rounded-sm bg-primary text-primary-foreground transition-all duration-150 ease-out hover:brightness-110 active:scale-[0.99]"
            onclick={() => (createDialogOpen = true)}
          >
            <Plus size={18} />
            <span>{m.track_inventories_create_button()}</span>
          </Button>
        {/if}
      {/snippet}
    </PageHeader>
  </div>

  {#if loading}
    <div class="flex items-center justify-center py-16">
      <div
        class="h-12 w-12 animate-spin rounded-full border-4 border-primary border-t-transparent"
      ></div>
    </div>
  {:else if error}
    <div class="mx-4 mt-6 rounded-2xl border border-border bg-destructive/10 p-4">
      <p class="font-medium text-destructive">{error}</p>
    </div>
  {:else if inventories.length === 0}
    <!-- Full empty state when no inventories exist -->
    <div class="p-6">
      <EmptyState
        icon={TrainTrack}
        title={m.track_inventories_empty_title()}
        description={m.track_inventories_empty_message()}
        ctaLabel={m.track_inventories_create_button()}
        onCta={() => (createDialogOpen = true)}
      />
    </div>
  {:else}
    <!-- Command Bar + full-width content -->
    <div class="mt-6 space-y-6 px-4 lg:px-0">
      <TrackCommandBar
        {inventories}
        {activeInventoryId}
        {activeInventory}
        onSelect={handleSelect}
        onRename={() => (renameDialogOpen = true)}
        onDelete={() => (deleteDialogOpen = true)}
        onAddPurchase={() => (addPurchaseDrawerOpen = true)}
      />

      {#if detailLoading}
        <div class="flex items-center justify-center py-16">
          <div
            class="h-12 w-12 animate-spin rounded-full border-4 border-primary border-t-transparent"
          ></div>
        </div>
      {:else if activeInventory}
        <InventoryDetail
          inventory={activeInventory}
          onAddPurchase={() => (addPurchaseDrawerOpen = true)}
        />
      {:else}
        <div class="rounded-sm border border-border bg-card p-4">
          <EmptyState
            icon={TrainTrack}
            title={m.track_inventories_empty_title()}
            description={m.track_inventories_empty_message()}
            ctaLabel={m.track_inventories_create_button()}
            onCta={() => (createDialogOpen = true)}
          />
        </div>
      {/if}
    </div>
  {/if}
</div>

<CreateInventoryDialog bind:open={createDialogOpen} onCreate={handleCreate} />
<RenameInventoryDialog
  bind:open={renameDialogOpen}
  currentName={activeInventory?.name}
  onRename={handleRename}
/>
<DeleteInventoryDialog
  bind:open={deleteDialogOpen}
  inventoryName={activeInventory?.name}
  onConfirm={handleDelete}
/>
<AddTracksPurchaseDrawer
  bind:open={addPurchaseDrawerOpen}
  inventoryId={activeInventoryId ?? ''}
  onPurchaseAdded={loadActiveInventory}
/>
