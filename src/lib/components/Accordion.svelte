<script lang="ts">
  /**
   * Accordion Component (shadcn-svelte compatible)
   * Replaces Skeleton's Accordion component with a lightweight Svelte 5 implementation
   * Supports collapsible mode and multiple/single selection
   */
  import { setContext } from 'svelte';

  type Props = {
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    children?: any;
    collapsible?: boolean;
    defaultValue?: string;
    value?: string[];
    onValueChange?: (details: { value: string[] }) => void;
    multiple?: boolean;
    class?: string;
  };

  const {
    children,
    collapsible = false,
    defaultValue = '',
    value = [],
    onValueChange,
    multiple = false,
    class: className = ''
  }: Props = $props();

  // Initialize expanded values from defaultValue
  let expandedValues: Record<string, boolean> = $state({});

  $effect(() => {
    if (defaultValue && Object.keys(expandedValues).length === 0) {
      expandedValues = { [defaultValue]: true };
    }
  });

  function toggle(itemValue: string) {
    if (collapsible && expandedValues[itemValue]) {
      // Collapse if already expanded and collapsible
      expandedValues[itemValue] = false;
    } else {
      if (!multiple && !collapsible) {
        // Single selection mode: collapse others
        Object.keys(expandedValues).forEach((key) => {
          if (key !== itemValue) {
            expandedValues[key] = false;
          }
        });
      }
      expandedValues[itemValue] = true;
    }

    // Call onValueChange callback if provided
    if (onValueChange) {
      const activeValues = Object.keys(expandedValues).filter((k) => expandedValues[k]);
      onValueChange({ value: activeValues });
    }
  }

  // Create context for item components
  const context = {
    toggle,
    isExpanded: (itemValue: string) => {
      return expandedValues[itemValue] ?? false;
    },
    get expandedValues() {
      return expandedValues;
    }
  };

  setContext('accordion-context', context);
</script>

<div class="divide-border bg-background divide-y rounded-lg border {className}">
  {#if children}
    {@render children(context)}
  {/if}
</div>
