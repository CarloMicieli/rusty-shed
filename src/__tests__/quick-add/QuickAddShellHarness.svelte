<script lang="ts">
  import QuickAddShell from '$lib/components/drawer/QuickAddShell.svelte';

  interface Props {
    open: boolean;
    title: string;
    onDismiss: () => void;
  }

  let { open, title, onDismiss }: Props = $props();
</script>

<QuickAddShell {open} {title} {onDismiss}>
  Body
  {#snippet footer()}
    Footer
  {/snippet}
</QuickAddShell>
