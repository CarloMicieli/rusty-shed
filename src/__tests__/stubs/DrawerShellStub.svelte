<script lang="ts">
  import type { Snippet } from 'svelte';

  type CloseArgs = { requestClose: () => void };

  interface Props {
    open?: boolean;
    onClose?: () => void;
    header?: Snippet<[CloseArgs]>;
    footer?: Snippet<[CloseArgs]>;
    children?: Snippet;
  }

  let { open = false, onClose, header, footer, children }: Props = $props();

  const requestClose = () => {
    onClose?.();
  };
</script>

{#if open}
  <div role="dialog" data-testid="drawer-shell-stub">
    {#if header}
      {@render header({ requestClose })}
    {/if}
    {@render children?.()}
    {#if footer}
      {@render footer({ requestClose })}
    {/if}
  </div>
{/if}
