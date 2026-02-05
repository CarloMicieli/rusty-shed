<script lang="ts">
	import { twMerge } from 'tailwind-merge';
	import type { TableHeadProps } from './types.js';
	import type { Snippet } from 'svelte';

	interface Props extends TableHeadProps {
		children?: Snippet;
	}

	let { class: className, children }: Props = $props();

	const headClass = $derived(
		twMerge(
			'h-12 px-4 text-left align-middle font-medium text-[var(--color-surface-700)]',
			'[&:has([role=checkbox])]:pr-0',
			className
		)
	);
</script>

<th class={headClass}>
	{#if children}
		{@render children()}
	{/if}
</th>
