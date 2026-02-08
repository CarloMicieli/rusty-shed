<script lang="ts">
  import { twMerge } from 'tailwind-merge';
  import type { CardContentProps } from './types.js';
  import type { Snippet } from 'svelte';

  interface Props extends CardContentProps {
    children?: Snippet;
  }

  let { class: className, children }: Props = $props();

  const contentClass = $derived(twMerge('p-6 pt-0', className));
</script>

<div class={contentClass}>
  {#if children}
    {@render children()}
  {/if}
</div>
