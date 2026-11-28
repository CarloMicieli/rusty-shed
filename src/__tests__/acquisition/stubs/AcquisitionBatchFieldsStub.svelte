<script lang="ts">
  interface Props {
    sellers: Array<{ id: string; name: string }>;
    onQuickAddSeller?: () => void;
  }

  let { sellers, onQuickAddSeller }: Props = $props();
</script>

<div data-testid="batch-fields">sellers:{sellers.length}</div>
<button type="button" aria-label="quick-add-seller" onclick={() => onQuickAddSeller?.()}></button>
