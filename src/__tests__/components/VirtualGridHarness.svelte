<script lang="ts">
  import VirtualGrid from '$lib/components/VirtualGrid.svelte';

  interface Props {
    items: string[];
  }

  let { items }: Props = $props();
</script>

<VirtualGrid {items} itemHeight={100} itemMinWidth={200} gap={10} overscan={1}>
  {#snippet children(item, index)}
    <div data-testid={`grid-item-${index}`}>{item}</div>
  {/snippet}
</VirtualGrid>
