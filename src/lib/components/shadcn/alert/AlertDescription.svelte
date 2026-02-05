<script lang="ts">
  import { twMerge } from 'tailwind-merge';
  import type { AlertDescriptionProps } from './types.js';
  import type { Snippet } from 'svelte';

  interface Props extends AlertDescriptionProps {
    children?: Snippet;
  }

  let { class: className, children }: Props = $props();

  const descriptionClass = $derived(twMerge('text-sm [&_p]:leading-relaxed', className));
</script>

<div class={descriptionClass}>
  {#if children}
    {@render children()}
  {/if}
</div>
