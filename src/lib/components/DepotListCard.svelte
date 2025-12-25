<script lang="ts">
	import type { DepotEntry } from '$lib/data/mock';
	import { Tag, Building2, TrainFront } from 'lucide-svelte';

	let { depot } = $props<{ depot: DepotEntry }>();
</script>

<div
	class="variant-filled-surface space-y-3 card border-l-4 border-surface-600 p-4 transition-colors hover:border-primary-500/50"
>
	<div class="flex items-start justify-between">
		<div>
			<h4 class="h4 font-bold">
				{depot.manufacturer}
				<span class="ml-1 font-mono text-primary-400">#{depot.productCode}</span>
			</h4>
			<p class="line-clamp-2 pt-1 text-sm text-surface-300">
				{depot.description}
			</p>
		</div>
		<span class="variant-filled-secondary badge font-bold">
			{depot.scale}
		</span>
	</div>

	<div class="flex flex-wrap gap-2 border-t border-surface-700/50 pt-3">
		<span class="variant-soft-surface badge flex items-center gap-1">
			<TrainFront size={12} />
			{depot.category}
		</span>
		<span class="variant-soft-surface badge flex items-center gap-1">
			<Building2 size={12} />
			{depot.railwayCompany}
		</span>
	</div>
</div>
