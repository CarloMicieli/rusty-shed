<script lang="ts">
  import type { Snippet } from 'svelte';

  interface Props {
    label: string;
    children: Snippet;
  }

  const { label, children }: Props = $props();
</script>

<div>
  <p class="mb-1 text-xs font-medium text-muted-foreground">{label}</p>
  {@render children()}
</div>
