<script lang="ts">
  import { createViewport } from './viewport.svelte';

  let {
    onViewport
  }: {
    onViewport: (v: ReturnType<typeof createViewport>) => void;
  } = $props();

  const viewport = createViewport();

  $effect(() => {
    onViewport(viewport);
  });
</script>
