<script lang="ts">
  /**
   * Dialog Component (shadcn-svelte compatible)
   * Modal dialog overlay component
   * Supports Steampunk theme
   *
   * Feature: 012-shadcn-migration
   */
  import { twMerge } from 'tailwind-merge';

  type Props = {
    open?: boolean;
    onOpenChange?: (open: boolean) => void;
    class?: string;
    'aria-labelledby'?: string;
    'aria-describedby'?: string;
    children?: import('svelte').Snippet;
  };

  let {
    open = $bindable(false),
    onOpenChange,
    class: className = '',
    'aria-labelledby': ariaLabelledby,
    'aria-describedby': ariaDescribedby,
    children
  }: Props = $props();

  function handleBackdropClick(e: MouseEvent) {
    if (e.target === e.currentTarget && onOpenChange) {
      onOpenChange(false);
    }
  }

  function handleKeyDown(e: KeyboardEvent) {
    if (e.key === 'Escape' && onOpenChange) {
      e.preventDefault();
      onOpenChange(false);
    }
  }

  const backdropClass = 'fixed inset-0 z-50 bg-black/60 flex items-center justify-center p-4';
  const contentClass = $derived(
    twMerge(
      'bg-surface-900 rounded-lg border border-surface-700/70 shadow-xl max-w-lg w-full',
      className
    )
  );
</script>

{#if open}
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div class={backdropClass} onclick={handleBackdropClick} onkeydown={handleKeyDown}>
    <div
      class={contentClass}
      role="dialog"
      aria-modal="true"
      aria-labelledby={ariaLabelledby}
      aria-describedby={ariaDescribedby}
      tabindex="-1"
    >
      {#if children}
        {@render children()}
      {/if}
    </div>
  </div>
{/if}
