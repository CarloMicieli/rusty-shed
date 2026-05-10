<script lang="ts">
  import type { Snippet } from 'svelte';

  interface Props {
    label: string;
    className?: string;
    children?: Snippet;
  }

  let { label, className = '', children }: Props = $props();
</script>

<div class={`overflow-hidden rounded-sm border border-border bg-card p-4 ${className}`.trim()}>
  <section>
    <p class="mb-4 text-[10px] font-bold tracking-[0.2em] text-muted-foreground uppercase">
      {label}
    </p>
    {#if children}
      {@render children()}
    {/if}
  </section>
</div>
