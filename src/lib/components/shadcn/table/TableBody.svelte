<script lang="ts">
  import { twMerge } from 'tailwind-merge';
  import type { TableBodyProps } from './types.js';
  import type { Snippet } from 'svelte';

  interface Props extends TableBodyProps {
    children?: Snippet;
  }

  let { class: className, children }: Props = $props();

  const bodyClass = $derived(twMerge('[&_tr:last-child]:border-0', className));
</script>

<tbody class={bodyClass}>
  {#if children}
    {@render children()}
  {/if}
</tbody>
