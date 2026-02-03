<script lang="ts">
  import type { MaintenanceCardView } from '$lib/bindings';
  import * as m from '$lib/paraglide/messages.js';
  import { getMaintenanceState } from '../MaintenanceState.svelte';

  let { selectedId = $bindable(), onSelect } = $props<{
    selectedId: string | null;
    onSelect?: (id: string | null) => void;
  }>();

  const state = getMaintenanceState();
  const cards = $derived(state.cards);

  function handleChange(event: Event) {
    const target = event.target as HTMLSelectElement;
    const value = target.value || null;
    selectedId = value;
    onSelect?.(value);
  }

  // Format display text for maintenance card
  function formatCard(card: MaintenanceCardView): string {
    const parts = [
      card.ownedRollingStockId,
      card.nextMaintenanceDate ? `Due: ${card.nextMaintenanceDate}` : 'No due date'
    ];
    return parts.join(' - ');
  }
</script>

<div class="space-y-2">
  <label for="card-select" class="label">
    {m.maintenance_add_event_select_card()}
  </label>
  <select id="card-select" class="select" value={selectedId ?? ''} onchange={handleChange}>
    <option value="">{m.maintenance_add_event_card_placeholder()}</option>
    {#each cards as card (card.id)}
      <option value={card.id}>
        {formatCard(card)}
      </option>
    {/each}
  </select>
</div>
