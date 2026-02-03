<script lang="ts">
  import MaintenanceCardItem from './MaintenanceCardItem.svelte';
  import type { MaintenanceCardView } from '$lib/bindings';

  let { cards } = $props<{ cards: MaintenanceCardView[] }>();

  // Cards are already sorted by due date from backend (top 10)
  const displayCards = $derived(cards.slice(0, 10));
</script>

<div class="space-y-4">
  {#each displayCards as card (card.id)}
    <MaintenanceCardItem {card} />
  {/each}
</div>
