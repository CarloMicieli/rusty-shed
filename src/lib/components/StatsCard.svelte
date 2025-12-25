<script lang="ts">
	import type { Stat } from '$lib/data/mock';
	import { TrendingUp, TrendingDown, Minus } from 'lucide-svelte';
	import { _ } from 'svelte-i18n';

	let { stat } = $props<{ stat: Stat }>();
</script>

<div
	class="variant-filled-surface hover:variant-filled-secondary space-y-2 card p-4 transition-colors duration-200"
>
	<div class="flex items-center justify-between text-surface-400">
		<span class="text-sm font-bold tracking-widest uppercase">{$_(`stats.${stat.label}`)}</span>
		{#if stat.trend === 'up'}
			<TrendingUp size={16} class="text-success-500" />
		{:else if stat.trend === 'down'}
			<TrendingDown size={16} class="text-error-500" />
		{:else}
			<Minus size={16} class="text-surface-400" />
		{/if}
	</div>
	<div class="flex items-end gap-2">
		<h3 class="h3 font-bold text-primary-500">{stat.value}</h3>
		<span
			class="{stat.trend === 'up'
				? 'text-success-500'
				: stat.trend === 'down'
					? 'text-error-500'
					: 'text-surface-400'} text-sm font-medium"
		>
			{stat.trendValue}
		</span>
	</div>
</div>
