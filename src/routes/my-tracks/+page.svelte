<script lang="ts">
  import * as m from '$lib/paraglide/messages';
  import { getTrackInventoryContext } from '$lib/features/track-inventory';
  import InventoryList from '$lib/features/track-inventory/components/InventoryList.svelte';
  import EmptyState from '$lib/features/track-inventory/components/EmptyState.svelte';
  import CreateInventoryDialog from '$lib/features/track-inventory/components/CreateInventoryDialog.svelte';
  import { onMount } from 'svelte';
  import { Plus } from 'lucide-svelte';
  import { Button } from '$lib/components';

  const service = getTrackInventoryContext();

  let inventories = $state<Awaited<ReturnType<typeof service.fetchInventories>>>([]);
  let loading = $state(true);
  let error = $state<string | null>(null);
  let createDialogOpen = $state(false);

  async function loadInventories() {
    try {
      loading = true;
      error = null;
      inventories = await service.fetchInventories();
    } catch (err) {
      console.error('Failed to load inventories:', err);
      error = err instanceof Error ? err.message : 'Failed to load inventories';
    } finally {
      loading = false;
    }
  }

  async function handleCreate(name: string, description: string) {
    await service.createInventory({ name, description });
    await loadInventories();
  }

  onMount(() => {
    loadInventories();
  });
</script>

<div class="container mx-auto space-y-6 p-4">
  <div class="flex items-center justify-between">
    <div>
      <h1 class="h1 font-bold">{m.track_inventories_title()}</h1>
      <p class="text-surface-300">{m.track_inventories_subtitle()}</p>
    </div>
    <Button
      onclick={() => (createDialogOpen = true)}
      variant="default"
      class="gap-2"
      disabled={loading}
    >
      <Plus size={20} />
      <span class="hidden sm:inline">{m.track_inventories_create_button()}</span>
    </Button>
  </div>

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
  {:else if inventories.length === 0}
    <EmptyState onCreateClick={() => (createDialogOpen = true)} />
  {:else}
    <InventoryList {inventories} />
  {/if}
</div>

<CreateInventoryDialog bind:open={createDialogOpen} onCreate={handleCreate} />
