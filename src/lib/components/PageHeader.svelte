<script lang="ts">
  interface Props {
    title: string;
    subtitle: string;
    description?: string;
  }

  const { title, subtitle, description }: Props = $props();
</script>

<div class="flex flex-col gap-2">
  <p class="text-sm tracking-[0.2em] text-surface-400 uppercase">{subtitle}</p>
  <h1 class="h2 font-bold">{title}</h1>
  {#if description}
    <p class="text-sm text-surface-400">{description}</p>
  {/if}
</div>
