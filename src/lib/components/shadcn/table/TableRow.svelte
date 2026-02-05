<script lang="ts">
	import { twMerge } from 'tailwind-merge';
	import type { TableRowProps } from './types.js';
	import type { Snippet } from 'svelte';

	interface Props extends TableRowProps {
		children?: Snippet;
	}

	let { class: className, children }: Props = $props();

	const rowClass = $derived(
		twMerge(
			'border-b border-[var(--color-surface-300)] transition-colors',
			'hover:bg-[var(--color-surface-200)]/50',
			'data-[state=selected]:bg-[var(--color-surface-200)]',
			className
		)
	);
</script>

<tr class={rowClass}>
	{#if children}
		{@render children()}
	{/if}
</tr>
