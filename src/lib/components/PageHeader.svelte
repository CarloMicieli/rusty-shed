<script lang="ts">
  interface Props {
    title: string;
    subtitle: string;
    description?: string;
  }

  const { title, subtitle, description }: Props = $props();
</script>

<div class="flex flex-col gap-2">
  <p class="text-surface-400 text-sm tracking-[0.2em] uppercase">{subtitle}</p>
  <h1 class="h2 font-bold">{title}</h1>
  {#if description}
    <p class="text-surface-400 text-sm">{description}</p>
  {/if}
</div>
