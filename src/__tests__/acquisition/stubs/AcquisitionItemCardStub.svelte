<script lang="ts">
  interface ItemEntry {
    uid: string;
  }

  interface Props {
    item: ItemEntry;
    canRemove: boolean;
    onDuplicate: (uid: string) => void;
    onRemove: (uid: string) => void;
    onQuickAddManufacturer?: (uid: string) => void;
  }

  let { item, canRemove, onDuplicate, onRemove, onQuickAddManufacturer }: Props = $props();
</script>

<div data-testid="acquisition-item-card">
  <span>{item.uid}</span>
  <button type="button" onclick={() => onQuickAddManufacturer?.(item.uid)}
    >quick-add-{item.uid}</button
  >
  <button type="button" onclick={() => onDuplicate(item.uid)}>duplicate-{item.uid}</button>
  {#if canRemove}
    <button type="button" onclick={() => onRemove(item.uid)}>remove-{item.uid}</button>
  {/if}
</div>
