<script lang="ts">
  import { toastStore, type Toast } from '$lib/stores/toast';
  import { AlertCircle, CheckCircle2, X } from 'lucide-svelte';
  import { crossfade } from 'svelte/transition';
  import { quintOut } from 'svelte/easing';

  /**
   * Toast Provider Component
   * Renders all active toast notifications in top-right corner
   * Replaces Skeleton's Toast component with shadcn-svelte styling
   *
   * Features:
   * - Auto-dismiss after configured duration
   * - Dismissible by close button
   * - Supports error and success variants
   * - Smooth animations with Svelte transitions
   * - Top-right fixed positioning (FR-009)
   */

  const [send, receive] = crossfade({
    duration: 200,
    easing: quintOut
  });

  let toasts: Toast[] = [];

  toastStore.subscribe((state) => {
    toasts = state.toasts;
  });

  /**
   * Get icon component for toast variant
   */
  function getIcon(variant?: string) {
    switch (variant) {
      case 'destructive':
        return AlertCircle;
      case 'success':
        return CheckCircle2;
      default:
        return null;
    }
  }

  /**
   * Get CSS classes for toast variant
   */
  function getVariantClasses(variant?: string): string {
    switch (variant) {
      case 'destructive':
        return 'bg-destructive text-destructive-foreground border-destructive';
      case 'success':
        return 'bg-green-600 text-white border-green-600';
      default:
        return 'bg-background text-foreground border-border';
    }
  }

  /**
   * Close a specific toast
   */
  function closeToast(id: string) {
    toastStore.remove(id);
  }
</script>

<!-- Toast Container - Fixed top-right positioning (FR-009) -->
<div class="pointer-events-none fixed top-0 right-0 z-50 flex max-w-md flex-col gap-2 p-4">
  {#each toasts as toast (toast.id)}
    <div
      in:receive={{ key: toast.id }}
      out:send={{ key: toast.id }}
      class="pointer-events-auto flex items-start gap-3 rounded-lg border px-4 py-3 shadow-lg {getVariantClasses(
        toast.variant
      )} animate-in transition-all duration-200 fade-in slide-in-from-right-1/2"
      role="alert"
      aria-live="polite"
    >
      <!-- Icon -->
      {#if getIcon(toast.variant)}
        <svelte:component this={getIcon(toast.variant)} class="mt-0.5 h-5 w-5 flex-shrink-0" />
      {/if}

      <!-- Content -->
      <div class="flex-1">
        <h3 class="text-sm font-semibold">{toast.title}</h3>
        {#if toast.description}
          <p class="mt-0.5 text-sm opacity-90">{toast.description}</p>
        {/if}
      </div>

      <!-- Action Button or Close Button -->
      {#if toast.action}
        <button
          on:click={toast.action.onClick}
          class="flex-shrink-0 text-sm font-medium hover:underline"
        >
          {toast.action.label}
        </button>
      {/if}

      <!-- Close Button -->
      <button
        on:click={() => closeToast(toast.id)}
        class="flex-shrink-0 transition-opacity hover:opacity-75"
        aria-label="Close notification"
      >
        <X class="h-4 w-4" />
      </button>
    </div>
  {/each}
</div>

<style>
  :global(.animate-in) {
    animation: slideIn 0.3s ease-out;
  }

  :global(.fade-in) {
    animation: fadeIn 0.3s ease-out;
  }

  :global(.slide-in-from-right-1\/2) {
    animation: slideInFromRight 0.3s ease-out;
  }

  @keyframes slideIn {
    from {
      opacity: 0;
      transform: translateY(-10px);
    }
    to {
      opacity: 1;
      transform: translateY(0);
    }
  }

  @keyframes fadeIn {
    from {
      opacity: 0;
    }
    to {
      opacity: 1;
    }
  }

  @keyframes slideInFromRight {
    from {
      opacity: 0;
      transform: translateX(50%);
    }
    to {
      opacity: 1;
      transform: translateX(0);
    }
  }
</style>
