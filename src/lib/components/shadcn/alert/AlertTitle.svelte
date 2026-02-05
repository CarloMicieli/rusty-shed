<script lang="ts">
	import { twMerge } from 'tailwind-merge';
	import type { AlertTitleProps } from './types.js';
	import type { Snippet } from 'svelte';

	interface Props extends AlertTitleProps {
		children?: Snippet;
	}

	let { class: className, children }: Props = $props();

	const titleClass = $derived(twMerge('mb-1 font-medium leading-none tracking-tight', className));
</script>

<h5 class={titleClass}>
	{#if children}
		{@render children()}
	{/if}
</h5>
