<script lang="ts">
	import { twMerge } from 'tailwind-merge';
	import type { CardTitleProps } from './types.js';
	import type { Snippet } from 'svelte';

	interface Props extends CardTitleProps {
		children?: Snippet;
	}

	let { class: className, children }: Props = $props();

	const titleClass = $derived(
		twMerge('text-2xl font-semibold leading-none tracking-tight', className)
	);
</script>

<h3 class={titleClass}>
	{#if children}
		{@render children()}
	{/if}
</h3>
