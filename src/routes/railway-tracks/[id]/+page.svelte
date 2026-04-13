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
  import DetailBackLink from '$lib/components/DetailBackLink.svelte';
  import { onMount } from 'svelte';
  import { Settings, Plus, Trash2, Edit2 } from 'lucide-svelte';
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
    goto(resolve('/railway-tracks'));
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
    <div class="rounded-sm border border-destructive/30 bg-destructive/10 p-4">
      <p class="font-medium text-destructive">{error}</p>
    </div>
  {:else if inventory}
    {#snippet settingsAction(
      label: string,
      Icon: typeof Edit2,
      onclick: () => void,
      danger: boolean = false
    )}
      <button
        {onclick}
        class={[
          'flex w-full items-center gap-3 px-4 py-3 text-sm transition-colors',
          danger
            ? 'border-t border-border text-destructive hover:bg-destructive/10 hover:text-destructive'
            : 'text-foreground hover:bg-background hover:text-foreground'
        ]}
      >
        <Icon size={16} />
        <span>{label}</span>
      </button>
    {/snippet}

    <!-- Page header: back + title + actions -->
    <div class="flex flex-col gap-6 md:flex-row md:items-start md:justify-between">
      <div class="flex items-start gap-4">
        <DetailBackLink
          path="/railway-tracks"
          ariaLabel={m.track_inventory_back_label()}
          class="mt-1"
        />
        <div>
          <div class="flex items-center gap-3">
            <p class="text-[10px] font-bold tracking-[0.3em] text-muted-foreground uppercase">
              {m.track_inventory_section_label()}
            </p>
            <div class="h-px w-8 bg-border"></div>
          </div>
          <h1 class="mt-1 font-bebas text-4xl tracking-widest text-foreground uppercase">
            {inventory.name}
          </h1>
          {#if inventory.description}
            <p class="mt-2 max-w-xl text-sm leading-relaxed text-muted-foreground">
              {inventory.description}
            </p>
          {/if}
        </div>
      </div>

      <div class="flex flex-wrap items-center gap-3">
        <div class="relative">
          <Button
            variant="outline"
            class="h-11 rounded-sm border-border bg-card text-foreground hover:bg-background"
            onclick={() => (showSettings = !showSettings)}
          >
            <Settings size={18} class={showSettings ? 'rotate-90 transition-transform' : ''} />
            <span>{m.track_inventory_management_button()}</span>
          </Button>

          {#if showSettings}
            <div
              class="absolute top-full right-0 z-20 mt-2 w-48 overflow-hidden rounded-sm border border-border bg-card shadow-lg"
            >
              {@render settingsAction(m.track_inventory_rename_button(), Edit2, () => {
                showSettings = false;
                renameDialogOpen = true;
              })}
              {@render settingsAction(
                m.inventory_delete_action(),
                Trash2,
                () => {
                  showSettings = false;
                  deleteDialogOpen = true;
                },
                true
              )}
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
    <div class="rounded-sm border border-dashed border-border p-8 text-center">
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
