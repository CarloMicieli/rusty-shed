<script lang="ts">
  /**
   * Accordion Item Component (shadcn-svelte compatible)
   * Individual accordion item that works with Accordion parent
   */
  import { getContext } from 'svelte';

  type Props = {
    value: string;
    _disabled?: boolean;
    class?: string;
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    children?: any;
  };

  const { value, _disabled = false, class: className = '', children }: Props = $props();
  
  // Get context from parent Accordion
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  const context = getContext<any>('accordion-context');
  
  const isExpanded = $derived(context?.isExpanded(value) ?? false);
</script>

<div data-accordion-item class="overflow-hidden {className}">
  {#if children}
    {@render children({ isExpanded, value, context })}
  {/if}
</div>
