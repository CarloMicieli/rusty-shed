<script lang="ts">
  import * as m from '$lib/paraglide/messages';
  import { getTrackInventoryContext } from '$lib/features/track-inventory';
  import type { TrackInventoryListItem, TrackInventoryView } from '$lib/features/track-inventory';
  import InventoryDetail from '$lib/features/track-inventory/components/InventoryDetail.svelte';
  import InventorySidebar from '$lib/features/track-inventory/components/InventorySidebar.svelte';
  import CreateInventoryDialog from '$lib/features/track-inventory/components/CreateInventoryDialog.svelte';
  import RenameInventoryDialog from '$lib/features/track-inventory/components/RenameInventoryDialog.svelte';
  import DeleteInventoryDialog from '$lib/features/track-inventory/components/DeleteInventoryDialog.svelte';
  import AddTracksPurchaseDrawer from '$lib/features/track-inventory/components/AddTracksPurchaseDrawer.svelte';
  import { onMount } from 'svelte';
  import { Plus, TrainTrack } from 'lucide-svelte';
  import { Button, PageHeader } from '$lib/components';

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
        {#if !loading}
          <Button variant="default" onclick={() => (createDialogOpen = true)}>
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
      <div
        class="flex flex-col items-center justify-center gap-8 rounded-3xl border border-white/5 bg-[#0c0c0c]/50 px-4 py-24 text-center"
      >
        <div class="relative">
          <div class="absolute inset-0 rounded-full bg-zinc-500/10 blur-3xl"></div>
          <div
            class="relative flex h-32 w-32 items-center justify-center rounded-full border border-white/10 bg-zinc-900/50"
          >
            <TrainTrack size={56} class="text-zinc-600 opacity-50" />
          </div>
        </div>

        <div class="flex max-w-sm flex-col items-center gap-3 text-center">
          <h3 class="text-2xl font-bold text-zinc-200">{m.track_inventories_empty_title()}</h3>
          <p class="text-sm leading-relaxed text-zinc-500">{m.track_inventories_empty_message()}</p>
        </div>

        <button
          type="button"
          class="group relative mt-2 inline-flex cursor-pointer items-center gap-3 overflow-hidden rounded-full bg-amber-500 px-8 py-4 font-bold tracking-wide text-black transition-all hover:scale-105 hover:bg-amber-400 hover:shadow-[0_0_20px_rgba(245,158,11,0.4)] active:scale-95"
          onclick={() => (createDialogOpen = true)}
        >
          <div
            class="absolute inset-0 translate-y-full bg-white/20 transition-transform duration-300 group-hover:translate-y-0"
          ></div>
          <TrainTrack class="h-5 w-5" />
          <span>{m.track_inventories_create_button()}</span>
        </button>
      </div>
    </div>
  {:else}
    <!-- Sidebar + Content split -->
    <div class="-mx-4 flex flex-1 flex-col md:flex-row lg:-mx-8">
      <!-- Left Sidebar: Inventory list -->
      <aside class="flex-shrink-0 border-r border-border bg-card md:w-80">
        <div class="sticky top-4 p-4">
          <InventorySidebar {inventories} activeId={activeInventoryId} onSelect={handleSelect} />
        </div>
      </aside>

      <!-- Main Content: Selected inventory detail -->
      <div class="flex-1 bg-background">
        <div class="p-6">
          {#if detailLoading}
            <div class="flex items-center justify-center py-16">
              <div
                class="h-12 w-12 animate-spin rounded-full border-4 border-primary border-t-transparent"
              ></div>
            </div>
          {:else if activeInventory}
            <InventoryDetail
              inventory={activeInventory}
              onRename={() => (renameDialogOpen = true)}
              onDelete={() => (deleteDialogOpen = true)}
              onAddPurchase={() => (addPurchaseDrawerOpen = true)}
            />
          {:else}
            <div
              class="flex flex-col items-center justify-center rounded-3xl border border-dashed border-white/5 bg-zinc-900/10 py-20 text-center"
            >
              <TrainTrack size={48} class="mb-4 text-zinc-700 opacity-20" />
              <p class="text-sm text-zinc-500">{m.track_inventories_empty_title()}</p>
            </div>
          {/if}
        </div>
      </div>
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
