<script lang="ts">
	import { twMerge } from 'tailwind-merge';
	import type { CardProps } from './types.js';
	import type { Snippet } from 'svelte';

	interface Props extends CardProps {
		children?: Snippet;
	}

	let { class: className, children }: Props = $props();

	const cardClass = $derived(
		twMerge(
			'rounded-lg border border-[var(--color-surface-300)]',
			'bg-[var(--color-surface-100)] text-[var(--color-surface-900)]',
			'shadow-sm',
			className
		)
	);
</script>

<div class={cardClass}>
	{#if children}
		{@render children()}
	{/if}
</div>
