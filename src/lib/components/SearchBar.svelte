<script lang="ts">
	import { Search, X } from 'lucide-svelte';
	import { _ } from 'svelte-i18n';

	let isExpanded = $state(false);

	function toggleSearch() {
		isExpanded = !isExpanded;
	}
</script>

<!-- Desktop: Inline Input -->
<div class="relative hidden w-64 items-center lg:flex xl:w-96">
	<Search class="pointer-events-none absolute left-3 text-surface-400" size={18} />
	<input
		type="text"
		placeholder={$_('app.search_placeholder')}
		class="input rounded-full border-surface-600 bg-surface-800 py-2 pl-10 text-sm transition-colors focus:border-primary-500"
	/>
</div>

<!-- Mobile: Icon Trigger + Overlay -->
<div class="lg:hidden">
	<button class="variant-ghost-surface btn-icon" onclick={toggleSearch}>
		<Search size={20} />
	</button>

	{#if isExpanded}
		<div
			class="bg-background/95 animate-fade-in fixed inset-0 z-50 flex flex-col p-4 pt-20 backdrop-blur-sm"
		>
			<button class="absolute top-4 right-4 btn-icon" onclick={toggleSearch}>
				<X size={24} />
			</button>
			<div class="relative w-full">
				<Search class="absolute top-1/2 left-4 -translate-y-1/2 text-surface-400" size={20} />
				<!-- svelte-ignore a11y_autofocus -->
				<input
					type="text"
					placeholder={$_('app.search_mobile_placeholder')}
					class="input rounded-xl border-primary-500 bg-surface-900 py-4 pl-12 text-lg shadow-xl"
					autoFocus
				/>
			</div>
			<div class="mt-8 text-center text-sm tracking-widest text-surface-400 uppercase">
				{$_('app.search_instruction')}
			</div>
		</div>
	{/if}
</div>
