<script lang="ts">
  import * as m from '$lib/paraglide/messages.js';
  import { page } from '$app/stores';
  import { goto } from '$app/navigation';
  import { resolve } from '$app/paths';
  import { getTrackInventoryContext } from '$lib/features/track-inventory';
  import InventoryDetail from '$lib/features/track-inventory/components/InventoryDetail.svelte';
  import RenameInventoryDialog from '$lib/features/track-inventory/components/RenameInventoryDialog.svelte';
  import DeleteInventoryDialog from '$lib/features/track-inventory/components/DeleteInventoryDialog.svelte';
  import AddTracksPurchaseDrawer from '$lib/features/track-inventory/components/AddTracksPurchaseDrawer.svelte';
  import { onMount } from 'svelte';
  import { ChevronLeft, Settings, Plus, Trash2, Edit2 } from 'lucide-svelte';
  import { Button } from '$lib/components';

  const service = getTrackInventoryContext();

  const inventoryId = $derived($page.params.id);

  let inventory = $state<Awaited<ReturnType<typeof service.fetchInventory>> | null>(null);
  let loading = $state(true);
  let error = $state<string | null>(null);
  let renameDialogOpen = $state(false);
  let deleteDialogOpen = $state(false);
  let addPurchaseDrawerOpen = $state(false);
  let showSettings = $state(false);

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

<div class="space-y-6">
  {#if loading}
    <div class="flex items-center justify-center py-16">
      <div
        class="h-12 w-12 animate-spin rounded-full border-4 border-primary border-t-transparent"
      ></div>
    </div>
  {:else if error}
    <div class="rounded-lg border border-destructive/30 bg-destructive/10 p-4">
      <p class="font-medium text-destructive">{error}</p>
    </div>
  {:else if inventory}
    <!-- Page header: back + title + actions -->
    <div class="flex flex-col gap-6 md:flex-row md:items-start md:justify-between">
      <div class="flex items-start gap-4">
        <a
          href={resolve('/railway-tracks')}
          class="mt-1 flex h-10 w-10 items-center justify-center rounded-xl border border-white/5 bg-zinc-900/50 text-zinc-400 transition-colors hover:bg-zinc-800 hover:text-white"
          aria-label={m.track_inventory_back_label()}
        >
          <ChevronLeft size={22} />
        </a>
        <div>
          <div class="flex items-center gap-3">
            <p class="text-[10px] font-bold tracking-[0.3em] text-zinc-500 uppercase">
              {m.track_inventory_section_label()}
            </p>
            <div class="h-px w-8 bg-zinc-800"></div>
          </div>
          <h1 class="mt-1 text-4xl font-bold tracking-tight text-zinc-100">{inventory.name}</h1>
          {#if inventory.description}
            <p class="mt-2 max-w-xl text-sm leading-relaxed text-zinc-400">
              {inventory.description}
            </p>
          {/if}
        </div>
      </div>

      <div class="flex flex-wrap items-center gap-3">
        <div class="relative">
          <Button
            variant="outline"
            class="h-11 border-white/10 bg-zinc-900/50 text-zinc-300 hover:bg-zinc-800"
            onclick={() => (showSettings = !showSettings)}
          >
            <Settings size={18} class={showSettings ? 'rotate-90 transition-transform' : ''} />
            <span>{m.track_inventory_management_button()}</span>
          </Button>

          {#if showSettings}
            <div
              class="absolute top-full right-0 z-20 mt-2 w-48 overflow-hidden rounded-xl border border-white/10 bg-layout-surface shadow-2xl"
            >
              <button
                onclick={() => {
                  showSettings = false;
                  renameDialogOpen = true;
                }}
                class="flex w-full items-center gap-3 px-4 py-3 text-sm text-zinc-300 transition-colors hover:bg-zinc-800 hover:text-white"
              >
                <Edit2 size={16} />
                <span>{m.track_inventory_rename_button()}</span>
              </button>
              <button
                onclick={() => {
                  showSettings = false;
                  deleteDialogOpen = true;
                }}
                class="flex w-full items-center gap-3 border-t border-white/5 px-4 py-3 text-sm text-zinc-500 transition-colors hover:bg-red-950/30 hover:text-red-500"
              >
                <Trash2 size={16} />
                <span>{m.inventory_delete_action()}</span>
              </button>
            </div>
          {/if}
        </div>

        <Button
          variant="rusty"
          class="h-11 px-6 shadow-lg shadow-amber-500/10"
          onclick={() => (addPurchaseDrawerOpen = true)}
        >
          <Plus size={18} />
          <span>{m.track_inventory_detail_add_purchase()}</span>
        </Button>
      </div>
    </div>

    <InventoryDetail {inventory} onAddPurchase={() => (addPurchaseDrawerOpen = true)} />
  {:else}
    <div class="rounded-xl border border-dashed border-border p-8 text-center">
      <p class="text-muted-foreground">{m.track_inventories_not_found()}</p>
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
<AddTracksPurchaseDrawer
  bind:open={addPurchaseDrawerOpen}
  inventoryId={inventoryId || ''}
  onPurchaseAdded={loadInventory}
/>
