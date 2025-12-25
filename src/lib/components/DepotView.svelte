<script lang="ts">
	import type { DepotEntry } from '$lib/data/mock';
	import DepotTable from './DepotTable.svelte';
	import DepotListCard from './DepotListCard.svelte';

	let { data } = $props<{ data: DepotEntry[] }>();
</script>

<!-- Mobile View: Cards -->
<div class="space-y-4 lg:hidden">
	{#each data as depot}
		<DepotListCard {depot} />
	{/each}
</div>

<!-- Desktop View: Table -->
<div
	class="hidden overflow-hidden rounded-container border border-surface-700/50 bg-surface-800 lg:block"
>
	<DepotTable {data} />
</div>
