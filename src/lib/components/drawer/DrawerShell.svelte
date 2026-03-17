<script lang="ts">
  import type { Snippet } from 'svelte';
  import * as m from '$lib/paraglide/messages.js';
  import { Button } from '$lib/components/ui/button';

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
    error = null,
    discardTitle,
    discardDescription,
    discardConfirm,
    discardCancel
  }: Props = $props();

  let showDiscardDialog = $state(false);

  const sizeClass = $derived(
    size === 'md' ? 'max-w-lg' : size === 'xl' ? 'max-w-3xl' : 'max-w-2xl'
  );

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
    class="fixed inset-y-0 right-0 z-50 flex w-full {sizeClass} translate-x-0 flex-col bg-zinc-950 shadow-2xl transition-transform duration-300"
    role="dialog"
    aria-modal="true"
    aria-labelledby={labelledby}
  >
    <!-- Sticky header -->
    <div class="border-b border-white/10">
      {@render header({ requestClose: handleCloseRequest })}
    </div>

    <!-- Optional sticky band (e.g. batch fields) -->
    {#if stickyBand}
      <div class="border-b border-white/10">
        {@render stickyBand()}
      </div>
    {/if}

    <!-- Scrollable body -->
    <div class="flex-1 overflow-y-auto p-4">
      {@render children()}

      {#if error}
        <div
          class="mt-4 rounded-lg border border-destructive/40 bg-destructive/10 p-3 text-sm text-destructive"
        >
          {error}
        </div>
      {/if}
    </div>

    <!-- Sticky footer -->
    <div class="border-t border-white/10">
      {@render footer({ requestClose: handleCloseRequest })}
    </div>
  </div>

  <!-- Discard dialog -->
  {#if showDiscardDialog}
    <div
      class="fixed inset-0 z-[60] flex items-center justify-center bg-background/80 backdrop-blur-sm"
    >
      <div class="w-full max-w-md rounded-lg border border-border bg-background p-6 shadow-xl">
        <h3 class="mb-2 text-lg font-bold text-foreground">
          {discardTitle ?? m.drawer_discard_title()}
        </h3>
        <p class="mb-4 text-muted-foreground">
          {discardDescription ?? m.drawer_discard_description()}
        </p>
        <div class="flex justify-end gap-3">
          <Button type="button" variant="ghost" onclick={handleDiscardCancel}>
            {discardCancel ?? m.drawer_discard_cancel()}
          </Button>
          <Button type="button" variant="destructive" onclick={handleDiscardConfirm}>
            {discardConfirm ?? m.drawer_discard_confirm()}
          </Button>
        </div>
      </div>
    </div>
  {/if}
{/if}
