<script lang="ts">
  import { twMerge } from 'tailwind-merge';
  import type { TableCellProps } from './types.js';
  import type { Snippet } from 'svelte';

  interface Props extends TableCellProps {
    children?: Snippet;
  }

  let { class: className, children }: Props = $props();

  const cellClass = $derived(twMerge('p-4 align-middle [&:has([role=checkbox])]:pr-0', className));
</script>

<td class={cellClass}>
  {#if children}
    {@render children()}
  {/if}
</td>
