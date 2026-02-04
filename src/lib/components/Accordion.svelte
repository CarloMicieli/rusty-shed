<script lang="ts">
  /**
   * Accordion Component (shadcn-svelte compatible)
   * Replaces Skeleton's Accordion component with a lightweight Svelte 5 implementation
   * Supports collapsible mode and multiple/single selection
   */

  type Props = {
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    children?: any;
    collapsible?: boolean;
    defaultValue?: string;
  };

  const { children, collapsible = false, defaultValue = '' }: Props = $props();

  let expandedValues: Record<string, boolean> = $state(
    defaultValue ? { [defaultValue]: true } : {}
  );

  function toggle(value: string) {
    if (collapsible && expandedValues[value]) {
      // Collapse if already expanded and collapsible
      expandedValues[value] = false;
    } else {
      if (!collapsible) {
        // Single selection mode: collapse others
        Object.keys(expandedValues).forEach((key) => {
          if (key !== value) {
            expandedValues[key] = false;
          }
        });
      }
      expandedValues[value] = true;
    }
  }

  // Create context for item components
  const context = {
    toggle,
    isExpanded: (value: string) => expandedValues[value] ?? false,
    expandedValues
  };
</script>

<div class="divide-y divide-border rounded-lg border bg-background">
  {#if children}
    {@render children(context)}
  {/if}
</div>

