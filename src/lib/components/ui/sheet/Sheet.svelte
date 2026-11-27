<script lang="ts">
  import { twMerge } from 'tailwind-merge';
  import type { SheetProps, SheetSide } from './types.js';
  import type { Snippet } from 'svelte';

  interface Props extends SheetProps {
    children?: Snippet;
  }

  let {
    open = $bindable(false),
    onOpenChange,
    side = 'right',
    class: className,
    children
  }: Props = $props();

  const sideVariants: Record<SheetSide, string> = {
    right:
      'right-0 top-0 h-full w-3/4 border-l sm:w-96 translate-x-full data-[state=open]:translate-x-0',
    left: 'left-0 top-0 h-full w-3/4 border-r sm:w-96 -translate-x-full data-[state=open]:translate-x-0',
    top: 'top-0 left-0 h-3/4 w-full border-b sm:h-96 -translate-y-full data-[state=open]:translate-y-0',
    bottom:
      'bottom-0 left-0 h-3/4 w-full border-t rounded-t-lg sm:h-96 translate-y-full data-[state=open]:translate-y-0'
  };

  const sheetClass = $derived(
    twMerge(
      'fixed z-50 border-border-subtle bg-card shadow-lg transition-transform duration-300 ease-in-out',
      sideVariants[side],
      className
    )
  );

  const overlayClass = $derived(
    twMerge(
      'fixed inset-0 z-40 bg-black/50 transition-opacity duration-300',
      open ? 'opacity-100' : 'opacity-0 pointer-events-none'
    )
  );

  function handleBackdropClick() {
    if (onOpenChange) {
      onOpenChange(false);
    } else {
      open = false;
    }
  }

  function handleEscapeKey(event: KeyboardEvent) {
    if (event.key === 'Escape' && open) {
      handleBackdropClick();
    }
  }
</script>

<svelte:window onkeydown={handleEscapeKey} />

{#if open}
  <!-- Backdrop -->
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div class={overlayClass} onclick={handleBackdropClick}></div>

  <!-- Sheet content -->
  <div class={sheetClass} data-state={open ? 'open' : 'closed'} role="dialog" aria-modal="true">
    {#if children}
      {@render children()}
    {/if}
  </div>
{/if}
