<script lang="ts">
  import type { TrackInventoryListItem } from '$lib/features/track-inventory';
  import InventoryCard from './InventoryCard.svelte';

  interface Props {
    inventories: TrackInventoryListItem[];
  }

  const { inventories }: Props = $props();
</script>

<div class="grid gap-4 sm:grid-cols-2 lg:grid-cols-3">
  {#each inventories as inventory (inventory.id)}
    <InventoryCard {inventory} />
  {/each}
</div>
