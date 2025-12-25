<script lang="ts">
	import { stats, quickActions, recentItems, depotData } from '$lib/data/mock';
	import StatsCard from '$lib/components/StatsCard.svelte';
	import QuickActionButton from '$lib/components/QuickActionButton.svelte';
	import RecentItemCard from '$lib/components/RecentItemCard.svelte';
	import DepotView from '$lib/components/DepotView.svelte';
	import { _ } from 'svelte-i18n';
</script>

<svelte:head>
	<title>{$_('app.name')} | {$_('app.dashboard')}</title>
</svelte:head>

<!-- Stats Grid -->
<section>
	<h3 class="mb-4 h3 text-sm font-bold tracking-wider text-surface-300 uppercase">
		{$_('dashboard.yard_statistics')}
	</h3>
	<div class="grid grid-cols-2 gap-4 lg:grid-cols-4">
		{#each stats as stat}
			<StatsCard {stat} />
		{/each}
	</div>
</section>

<div class="grid grid-cols-1 gap-8 lg:grid-cols-3">
	<!-- Main Content Area: Recent Items & Depot -->
	<div class="space-y-8 lg:col-span-2">
		<!-- Recent Items -->
		<section>
			<div class="mb-4 flex items-center justify-between">
				<h3 class="h3 text-sm font-bold tracking-wider text-surface-300 uppercase">
					{$_('dashboard.recently_added')}
				</h3>
				<a href="/activity" class="text-accent-500 text-sm font-bold hover:underline"
					>{$_('dashboard.view_all')}</a
				>
			</div>

			<!-- Mobile: Swipe Scroll -->
			<div
				class="hide-scrollbar flex snap-x snap-mandatory gap-4 overflow-x-auto pb-4 lg:grid lg:grid-cols-2 lg:pb-0"
			>
				{#each recentItems as item}
					<div class="min-w-[80%] snap-center lg:min-w-0">
						<RecentItemCard {item} />
					</div>
				{/each}
			</div>
		</section>

		<!-- The Depot -->
		<section>
			<div class="mb-4 flex items-center justify-between">
				<h3 class="h3 text-sm font-bold tracking-wider text-surface-300 uppercase">
					{$_('dashboard.the_depot')}
				</h3>
				<button class="variant-ghost-surface btn btn-sm">{$_('dashboard.filter')}</button>
			</div>
			<DepotView data={depotData} />
		</section>
	</div>

	<!-- Sidebar Area: Quick Actions -->
	<div class="lg:col-start-3">
		<section class="sticky top-24">
			<h3
				class="mb-4 hidden h3 text-sm font-bold tracking-wider text-surface-300 uppercase lg:block"
			>
				{$_('dashboard.quick_actions')}
			</h3>

			<!-- Desktop: Vertical List -->
			<div class="hidden flex-col gap-3 lg:flex">
				{#each quickActions as action}
					<QuickActionButton {action} />
				{/each}
			</div>

			<!-- Mobile: FAB / Horizontal Row (Floating above content on mobile, usually? Or just inline?) -->
			<!-- User requested: "Mobile: FAB or horizontal icon row" -->
			<!-- I'll implement a horizontal row for now as FAB blocks content often -->
			<div class="mb-8 grid grid-cols-2 gap-2 lg:hidden">
				{#each quickActions as action}
					<QuickActionButton {action} />
				{/each}
			</div>
		</section>
	</div>
</div>
