<script lang="ts">
	import { twMerge } from 'tailwind-merge';
	import type { TableProps } from './types.js';
	import type { Snippet } from 'svelte';

	interface Props extends TableProps {
		children?: Snippet;
	}

	let { class: className, children }: Props = $props();

	const tableClass = $derived(twMerge('w-full caption-bottom text-sm', className));
</script>

<div class="relative w-full overflow-auto">
	<table class={tableClass}>
		{#if children}
			{@render children()}
		{/if}
	</table>
</div>
