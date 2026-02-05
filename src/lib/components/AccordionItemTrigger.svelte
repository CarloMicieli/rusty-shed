<script lang="ts">
  /**
   * Accordion Item Trigger Component (shadcn-svelte compatible)
   * Clickable trigger to expand/collapse accordion item
   */
  import { getContext } from 'svelte';

  type Props = {
    class?: string;
    value?: string;
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    children?: any;
  };

  const { class: className = '', value = '', children }: Props = $props();

  // Get context from parent Accordion
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  const context = getContext<any>('accordion-context');
</script>

<button
  type="button"
  data-accordion-trigger
  onclick={() => context?.toggle(value)}
  class="bg-background hover:bg-accent flex w-full items-center justify-between px-4 py-3 text-left transition-colors {className}"
  {value}
>
  {#if children}
    {@render children()}
  {/if}
</button>
