<script lang="ts">
  import type { Snippet } from 'svelte';

  interface Props {
    open: boolean;
    title: string;
    children?: Snippet;
    footer?: Snippet;
  }

  let { open, title, children, footer }: Props = $props();
</script>

{#if open}
  <section aria-label={title}>
    {@render children?.()}
    {@render footer?.()}
  </section>
{/if}
