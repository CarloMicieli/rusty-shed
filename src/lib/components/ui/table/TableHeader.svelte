<script lang="ts">
  import { twMerge } from 'tailwind-merge';
  import type { TableHeaderProps } from './types.js';
  import type { Snippet } from 'svelte';

  interface Props extends TableHeaderProps {
    children?: Snippet;
  }

  let { class: className, children }: Props = $props();

  const headerClass = $derived(twMerge('[&_tr]:border-b', className));
</script>

<thead class={headerClass}>
  {#if children}
    {@render children()}
  {/if}
</thead>
