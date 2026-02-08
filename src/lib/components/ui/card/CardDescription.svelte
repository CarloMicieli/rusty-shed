<script lang="ts">
  import { twMerge } from 'tailwind-merge';
  import type { CardDescriptionProps } from './types.js';
  import type { Snippet } from 'svelte';

  interface Props extends CardDescriptionProps {
    children?: Snippet;
  }

  let { class: className, children }: Props = $props();

  const descriptionClass = $derived(twMerge('text-sm text-[var(--color-surface-600)]', className));
</script>

<p class={descriptionClass}>
  {#if children}
    {@render children()}
  {/if}
</p>
