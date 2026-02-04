<script lang="ts">
  /**
   * Accordion Item Indicator Component (shadcn-svelte compatible)
   * Visual indicator (chevron) that rotates when accordion expands
   */

  import { ChevronDown } from 'lucide-svelte';

  type Props = {
    class?: string;
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    context?: any;
    value?: string;
    isExpanded?: boolean;
  };

  const { class: className = '', context = null, value = '', isExpanded = false }: Props = $props();

  let expanded = $derived.by(() => {
    if (!context || !value) return isExpanded;
    return context.isExpanded?.(value) ?? false;
  });
</script>

<ChevronDown
  class="h-4 w-4 transition-transform duration-200 {expanded ? 'rotate-180' : ''} {className}"
/>
