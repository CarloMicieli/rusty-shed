<script lang="ts">
	import type { QuickAction } from '$lib/data/mock';
	import * as Lucide from 'lucide-svelte';
	import { _ } from 'svelte-i18n';

	let { action } = $props<{ action: QuickAction }>();

	// Dynamically resolve icon
	let Icon = $derived((Lucide as any)[action.icon] || Lucide.HelpCircle);
</script>

<button
	class="variant-ghost-surface hover:variant-filled-primary group btn w-full justify-start border border-surface-700/50 p-4 transition-all duration-200"
>
	<Icon
		class="text-accent-500 group-hover:text-on-primary mr-3 transition-transform group-hover:scale-110"
		size={20}
	/>
	<span class="group-hover:text-on-primary font-semibold tracking-wide uppercase"
		>{$_(`actions.${action.label}`)}</span
	>
</button>
