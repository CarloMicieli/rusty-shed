<script lang="ts">
	import { twMerge } from 'tailwind-merge';
	import type { CardFooterProps } from './types.js';
	import type { Snippet } from 'svelte';

	interface Props extends CardFooterProps {
		children?: Snippet;
	}

	let { class: className, children }: Props = $props();

	const footerClass = $derived(twMerge('flex items-center p-6 pt-0', className));
</script>

<div class={footerClass}>
	{#if children}
		{@render children()}
	{/if}
</div>
