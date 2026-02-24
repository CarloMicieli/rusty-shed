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

<svelte:head>
  <title>{m.track_inventories_title()}</title>
</svelte:head>

<div class="space-y-6">
  <div class="flex items-start justify-between">
    <div>
      <p class="text-[10px] font-bold tracking-[0.3em] text-zinc-500 uppercase">{m.app_tracks()}</p>
      <h1 class="mt-1 text-4xl font-bold text-zinc-100">{m.track_inventories_title()}</h1>
      <p class="mt-1 text-sm text-zinc-400">{m.track_inventories_subtitle()}</p>
    </div>
    <div class="flex items-center gap-3">
      <Button variant="rusty" onclick={() => (createDialogOpen = true)} disabled={loading}>
        <Plus size={18} />
        <span>{m.track_inventories_create_button()}</span>
      </Button>
    </div>
  </div>

  {#if loading}
    <div class="flex items-center justify-center py-16">
      <div
        class="h-12 w-12 animate-spin rounded-full border-4 border-primary border-t-transparent"
      ></div>
    </div>
  {:else if error}
    <div class="rounded-2xl border border-border bg-destructive/10 p-4">
      <p class="font-medium">{error}</p>
    </div>
  {:else if inventories.length === 0}
    <EmptyState onCreateClick={() => (createDialogOpen = true)} />
  {:else}
    <InventoryList {inventories} />
  {/if}
</div>

<CreateInventoryDialog bind:open={createDialogOpen} onCreate={handleCreate} />
