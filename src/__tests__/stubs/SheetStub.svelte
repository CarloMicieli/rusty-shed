<script lang="ts">
  import type { Snippet } from 'svelte';

  let {
    open = false,
    side,
    class: className,
    onOpenChange,
    children
  }: {
    open?: boolean;
    side?: string;
    class?: string;
    onOpenChange?: (value: boolean) => void;
    children?: Snippet;
  } = $props();
</script>

<div
  data-testid="sheet-stub"
  data-open={open ? 'true' : 'false'}
  data-side={side}
  class={className}
>
  <button type="button" data-testid="sheet-close" onclick={() => onOpenChange?.(false)}
    >close</button
  >
  <button type="button" data-testid="sheet-open" onclick={() => onOpenChange?.(true)}>open</button>
  {@render children?.()}
</div>
