<script lang="ts">
  /**
   * Accordion Item Component (shadcn-svelte compatible)
   * Individual accordion item that works with Accordion parent
   */

  type Props = {
    value: string;
    _disabled?: boolean;
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    context?: any;
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    children?: any;
  };

  const { value, _disabled = false, context = null, children }: Props = $props();

  const isExpanded = context?.isExpanded(value) ?? false;
</script>

<div data-accordion-item class="overflow-hidden">
  {#if children}
    {@render children({ isExpanded, value })}
  {/if}
</div>
