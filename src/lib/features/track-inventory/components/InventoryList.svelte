<script lang="ts">
  import type { TrackInventoryListItem } from '$lib/features/track-inventory';
  import InventoryCard from './InventoryCard.svelte';

  interface Props {
    inventories: TrackInventoryListItem[];
  }

  const { inventories }: Props = $props();
</script>

<div class="grid grid-cols-1 gap-6 md:grid-cols-3">
  {#each inventories as inventory (inventory.id)}
    <InventoryCard {inventory} />
  {/each}
</div>
