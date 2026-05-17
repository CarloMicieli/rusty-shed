<script lang="ts">
  import type { Snippet } from 'svelte';

  interface Props {
    open: boolean;
    title: string;
    onDismiss: () => void;
    children: Snippet;
    footer: Snippet;
  }

  let { open, title, onDismiss, children, footer }: Props = $props();

  let keyboardInset = $state(0);
  let touchStartY = $state<number | null>(null);
  let touchDeltaY = $state(0);

  function updateKeyboardInset() {
    if (!window.visualViewport) {
      keyboardInset = 0;
      return;
    }

    const viewport = window.visualViewport;
    const inset = Math.max(0, window.innerHeight - viewport.height - viewport.offsetTop);
    keyboardInset = inset;
  }

  function handleTouchStart(event: TouchEvent) {
    if (!window.matchMedia('(max-width: 767px)').matches) return;
    touchStartY = event.touches[0]?.clientY ?? null;
    touchDeltaY = 0;
  }

  function handleTouchMove(event: TouchEvent) {
    if (touchStartY === null) return;
    const currentY = event.touches[0]?.clientY ?? touchStartY;
    touchDeltaY = Math.max(0, currentY - touchStartY);
  }

  function handleTouchEnd() {
    if (touchDeltaY > 90) {
      onDismiss();
    }
    touchStartY = null;
    touchDeltaY = 0;
  }

  $effect(() => {
    if (!open) return;
    updateKeyboardInset();
    window.visualViewport?.addEventListener('resize', updateKeyboardInset);
    window.visualViewport?.addEventListener('scroll', updateKeyboardInset);

    return () => {
      window.visualViewport?.removeEventListener('resize', updateKeyboardInset);
      window.visualViewport?.removeEventListener('scroll', updateKeyboardInset);
    };
  });
</script>

{#if open}
  <div
    class="fixed inset-0 z-[105] bg-black/40"
    onclick={onDismiss}
    role="presentation"
    aria-hidden="true"
  ></div>

  <div
    class="fixed inset-x-0 bottom-0 z-[110] flex h-[82vh] max-h-[82vh] w-full flex-col rounded-sm border border-border bg-card shadow-2xl transition-transform md:inset-y-0 md:right-0 md:left-auto md:h-full md:max-h-none md:w-full md:max-w-md md:rounded-sm"
    style={`transform: translateY(${touchDeltaY}px); padding-bottom: ${keyboardInset}px;`}
    role="dialog"
    tabindex="-1"
    aria-modal="true"
    aria-labelledby="quick-add-title"
    ontouchstart={handleTouchStart}
    ontouchmove={handleTouchMove}
    ontouchend={handleTouchEnd}
  >
    <div class="mx-auto mt-2 mb-1 h-1.5 w-10 rounded-full bg-muted md:hidden"></div>

    <header class="border-b border-border px-4 py-3">
      <h2 id="quick-add-title" class="font-bebas text-lg tracking-widest uppercase">
        {title}
      </h2>
    </header>

    <div class="flex-1 overflow-y-auto p-4">
      {@render children()}
    </div>

    <div class="border-t border-border p-4">
      {@render footer()}
    </div>
  </div>
{/if}
