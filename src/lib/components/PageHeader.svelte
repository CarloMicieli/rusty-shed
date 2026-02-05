<script lang="ts">
  /**
   * PageHeader Component
   * Standardized 3-tier page header following "My Collection" template
   * Feature: 012-shadcn-migration
   * 
   * Tier 1: Subtitle/Section (uppercase, tracking-widest)
   * Tier 2: Title (h2, bold)
   * Tier 3: Description (text-sm, muted)
   */
  
  interface Props {
    title: string;
    subtitle: string;
    description: string;
    class?: string;
    actions?: import('svelte').Snippet;
  }

  const { title, subtitle, description, class: className = '', actions }: Props = $props();
</script>

<div class="flex flex-col gap-4 md:flex-row md:items-center md:justify-between {className}">
  <div class="flex flex-col gap-2">
    <p class="text-surface-400 text-sm tracking-[0.2em] uppercase">{subtitle}</p>
    <h1 class="h2 font-bold">{title}</h1>
    <p class="text-surface-400 text-sm">{description}</p>
  </div>
  {#if actions}
    <div class="flex flex-col gap-3 md:flex-row md:items-center">
      {@render actions()}
    </div>
  {/if}
</div>
