<script lang="ts">
  import type { Snippet } from 'svelte';

  interface Props {
    open: boolean;
    onClose: () => void;
    header?: Snippet<[payload: { requestClose: () => void }]>;
    stickyBand?: Snippet;
    footer?: Snippet<[payload: { requestClose: () => void }]>;
    children?: Snippet;
  }

  let { open, onClose, header, stickyBand, footer, children }: Props = $props();
</script>

{#if open}
  <div role="dialog" aria-label="acquisition-drawer">
    {@render header?.({ requestClose: onClose })}
    {@render stickyBand?.()}
    {@render children?.()}
    {@render footer?.({ requestClose: onClose })}
  </div>
{/if}
