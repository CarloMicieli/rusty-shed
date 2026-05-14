<script lang="ts">
  import type { Snippet } from 'svelte';

  let {
    children,
    ...restProps
  }: {
    children?: Snippet;
  } & Record<string, unknown> = $props();
</script>

<button data-testid="select-trigger-primitive" {...restProps}>
  {@render children?.()}
</button>
