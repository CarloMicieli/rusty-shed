<script lang="ts">
  /**
   * Accordion Item Content Component (shadcn-svelte compatible)
   * Content that shows/hides based on accordion item expansion state
   */
  import { getContext } from 'svelte';

  type Props = {
    class?: string;
    value?: string;
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    children?: any;
    isExpanded?: boolean;
  };

  const { class: className = '', value = '', children, isExpanded = false }: Props = $props();

  // Get context from parent Accordion
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  const context = getContext<any>('accordion-context');

  let expanded = $derived.by(() => {
    if (!context || !value) return isExpanded;
    return context.isExpanded?.(value) ?? false;
  });
</script>

{#if expanded}
  <div data-accordion-content class="bg-muted/50 overflow-hidden px-4 py-3 {className}">
    {#if children}
      {@render children()}
    {/if}
  </div>
{/if}

<style>
  :global([data-accordion-content]) {
    animation: slideDown 0.2s ease-out;
  }

  @keyframes slideDown {
    from {
      opacity: 0;
      transform: translateY(-10px);
    }
    to {
      opacity: 1;
      transform: translateY(0);
    }
  }
</style>
