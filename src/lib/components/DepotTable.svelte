<script lang="ts">
	import type { DepotEntry } from '$lib/data/mock';
	import { _ } from 'svelte-i18n';

	let { data } = $props<{ data: DepotEntry[] }>();
</script>

<div class="table-container">
	<table class="table-hover table">
		<thead>
			<tr>
				<th>{$_('depot.manufacturer')}</th>
				<th>{$_('depot.product_code')}</th>
				<th>{$_('depot.category')}</th>
				<th>{$_('depot.scale')}</th>
				<th>{$_('depot.company')}</th>
				<th>{$_('depot.description')}</th>
			</tr>
		</thead>
		<tbody>
			{#each data as row}
				<tr>
					<td class="font-bold">{row.manufacturer}</td>
					<td class="font-mono text-primary-400">{row.productCode}</td>
					<td>
						<span class="variant-soft-surface badge">
							{row.category}
						</span>
					</td>
					<td>
						<span class="variant-soft-secondary badge font-bold">
							{row.scale}
						</span>
					</td>
					<td>
						<span class="variant-filled-surface badge font-bold tracking-wider">
							{row.railwayCompany}
						</span>
					</td>
					<td class="text-surface-300">{row.description}</td>
				</tr>
			{/each}
		</tbody>
	</table>
</div>
