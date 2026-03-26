<script lang="ts">
  import type { Snippet } from 'svelte';

  interface Props {
    label: string;
    children: Snippet;
  }

  const { label, children }: Props = $props();
</script>

<div>
  <p class="mb-1 text-[10px] font-medium tracking-wider text-muted-foreground uppercase">{label}</p>
  {@render children()}
</div>
