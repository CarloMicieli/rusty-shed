<script lang="ts">
  import type { Snippet } from 'svelte';
  import * as m from '$lib/paraglide/messages.js';
  import { Button } from '$lib/components/ui/button';
  import { cn } from '$lib/utils';
  import { createMobileMatchMediaState } from '$lib/state/match-media.svelte';
  import { onMount } from 'svelte';

  interface Props {
    open: boolean;
    onClose: () => void;
    size?: 'md' | 'lg' | 'xl';
    hasChanges?: boolean;
    labelledby?: string;
    header: Snippet<[{ requestClose: () => void }]>;
    children: Snippet;
    footer: Snippet<[{ requestClose: () => void }]>;
    stickyBand?: Snippet;
    dimmed?: boolean;
    error?: string | null;
    discardTitle?: string;
    discardDescription?: string;
    discardConfirm?: string;
    discardCancel?: string;
  }

  let {
    open,
    onClose,
    size = 'lg',
    hasChanges = false,
    labelledby,
    header,
    children,
    footer,
    stickyBand,
    dimmed = false,
    error = null,
    discardTitle,
    discardDescription,
    discardConfirm,
    discardCancel
  }: Props = $props();

  let showDiscardDialog = $state(false);
  let isMobileViewport = $state(false);

  const sizeClass = $derived(
    size === 'md' ? 'max-w-lg' : size === 'xl' ? 'max-w-3xl' : 'max-w-2xl'
  );

  const mobileMedia = createMobileMatchMediaState();

  $effect(() => {
    const unsubscribe = mobileMedia.subscribe((matches) => {
      isMobileViewport = matches;
    });

    return () => {
      unsubscribe();
    };
  });

  $effect(() => {
    if (open) {
      const mainElement = document.querySelector('main');
      document.body.style.overflow = 'hidden';
      if (mainElement) mainElement.style.overflow = 'hidden';
    } else {
      const mainElement = document.querySelector('main');
      document.body.style.overflow = '';
      if (mainElement) mainElement.style.overflow = '';
    }
    return () => {
      const mainElement = document.querySelector('main');
      document.body.style.overflow = '';
      if (mainElement) mainElement.style.overflow = '';
    };
  });

  onMount(() => {
    return () => {
      mobileMedia.destroy();
    };
  });

  function handleCloseRequest() {
    if (hasChanges) {
      showDiscardDialog = true;
    } else {
      onClose();
    }
  }

  function handleDiscardConfirm() {
    showDiscardDialog = false;
    onClose();
  }

  function handleDiscardCancel() {
    showDiscardDialog = false;
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === 'Escape') {
      handleCloseRequest();
    }
  }
</script>

<svelte:window onkeydown={handleKeydown} />

{#if open}
  <!-- Overlay -->
  <div
    class="fixed inset-0 z-40 bg-black/60 backdrop-blur-sm"
    onclick={handleCloseRequest}
    role="presentation"
  ></div>

  <!-- Drawer panel -->
  <div
    class={cn(
      'fixed z-50 flex w-full flex-col border-primary bg-card shadow-2xl transition-transform motion-reduce:transition-none',
      'transform-gpu [will-change:transform] motion-reduce:[will-change:auto]',
      isMobileViewport
        ? 'right-0 bottom-0 left-0 max-h-[85vh] translate-y-0 rounded-t-2xl border-t-2'
        : `inset-y-0 right-0 ${sizeClass} translate-x-0 border-l-2`
    )}
    role="dialog"
    aria-modal="true"
    aria-labelledby={labelledby}
  >
    <!-- Sticky header -->
    <div class="border-b border-border">
      {@render header({ requestClose: handleCloseRequest })}
    </div>

    <!-- Optional sticky band (e.g. batch fields) -->
    {#if stickyBand}
      <div class="border-b border-border">
        {@render stickyBand()}
      </div>
    {/if}

    <!-- Scrollable body -->
    <div
      class={cn(
        'flex-1 overflow-y-auto p-4 transition-opacity',
        dimmed && 'pointer-events-none opacity-70'
      )}
    >
      {@render children()}

      {#if error}
        <div
          class="mt-4 rounded-sm border border-destructive/40 bg-destructive/10 p-3 text-sm text-destructive"
        >
          {error}
        </div>
      {/if}
    </div>

    <!-- Sticky footer -->
    <div class="border-t border-border">
      {@render footer({ requestClose: handleCloseRequest })}
    </div>
  </div>

  <!-- Discard dialog -->
  {#if showDiscardDialog}
    <div class="fixed inset-0 z-[60] flex items-center justify-center bg-black/60 backdrop-blur-sm">
      <div class="w-full max-w-md rounded-sm border-2 border-destructive bg-card p-6 shadow-xl">
        <h3 class="mb-2 font-bebas text-xl tracking-widest text-destructive uppercase">
          {discardTitle ?? m.drawer_discard_title()}
        </h3>
        <p class="mb-6 text-sm text-muted-foreground">
          {discardDescription ?? m.drawer_discard_description()}
        </p>
        <div class="flex justify-end gap-3">
          <Button type="button" variant="ghost" onclick={handleDiscardConfirm}>
            {discardConfirm ?? m.drawer_discard_confirm()}
          </Button>
          <Button type="button" variant="default" onclick={handleDiscardCancel}>
            {discardCancel ?? m.drawer_discard_cancel()}
          </Button>
        </div>
      </div>
    </div>
  {/if}
{/if}
