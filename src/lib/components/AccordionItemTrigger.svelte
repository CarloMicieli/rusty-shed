<script lang="ts">
  /**
   * Accordion Item Trigger Component (shadcn-svelte compatible)
   * Clickable trigger to expand/collapse accordion item
   */

  type Props = {
    class?: string;
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    context?: any;
    value?: string;
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    children?: any;
  };

  const { class: className = '', context = null, value = '', children }: Props = $props();
</script>

<button
  data-accordion-trigger
  on:click={() => context?.toggle(value)}
  class="flex w-full items-center justify-between bg-background px-4 py-3 text-left hover:bg-accent transition-colors {className}"
  {value}
>
  {#if children}
    {@render children()}
  {/if}
</button>
