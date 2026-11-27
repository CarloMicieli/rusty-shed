<script lang="ts">
  import type { Snippet } from 'svelte';

  interface Props {
    cancelLabel: string;
    submitLabel: string;
    onCancel: () => void;
    onSubmit: () => void;
    leading?: Snippet;
  }

  let { cancelLabel, submitLabel, onCancel, onSubmit, leading }: Props = $props();
</script>

<div>
  {@render leading?.()}
  <button type="button" onclick={onCancel}>{cancelLabel}</button>
  <button type="button" onclick={onSubmit}>{submitLabel}</button>
</div>
