<script lang="ts">
	import { twMerge } from 'tailwind-merge';
	import type { AlertProps, AlertVariant } from './types.js';
	import type { Snippet } from 'svelte';

	interface Props extends AlertProps {
		children?: Snippet;
	}

	let { variant = 'default', class: className, children }: Props = $props();

	const variants: Record<AlertVariant, string> = {
		default: 'bg-[var(--color-surface-100)] text-[var(--color-surface-900)] border-[var(--color-surface-300)]',
		destructive:
			'bg-[var(--color-error-100)] text-[var(--color-error-900)] border-[var(--color-error-500)]',
		success:
			'bg-[var(--color-success-100)] text-[var(--color-success-900)] border-[var(--color-success-500)]',
		warning:
			'bg-yellow-50 text-yellow-900 border-yellow-500 dark:bg-yellow-950 dark:text-yellow-100'
	};

	const alertClass = $derived(
		twMerge(
			'relative w-full rounded-lg border p-4',
			'[&>svg~*]:pl-7 [&>svg+div]:translate-y-[-3px]',
			'[&>svg]:absolute [&>svg]:left-4 [&>svg]:top-4 [&>svg]:text-current',
			variants[variant],
			className
		)
	);
</script>

<div class={alertClass} role="alert">
	{#if children}
		{@render children()}
	{/if}
</div>
