<script lang="ts">
  import { twMerge } from 'tailwind-merge';
  import type { CardHeaderProps } from './types.js';
  import type { Snippet } from 'svelte';

  interface Props extends CardHeaderProps {
    children?: Snippet;
  }

  let { class: className, children }: Props = $props();

  const headerClass = $derived(twMerge('flex flex-col space-y-1.5 p-6', className));
</script>

<div class={headerClass}>
  {#if children}
    {@render children()}
  {/if}
</div>
