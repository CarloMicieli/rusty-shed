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
      'rounded-md border border-border-subtle bg-card text-foreground',
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
