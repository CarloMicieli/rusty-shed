<script lang="ts">
  import * as m from '$lib/paraglide/messages';
  import { X, Loader2 } from 'lucide-svelte';
  import { Input, Textarea, Button } from '$lib/components';

  interface Props {
    open?: boolean;
    onClose?: () => void;
    onCreate?: (name: string, description: string) => void;
  }

  let { open = $bindable(false), onClose, onCreate }: Props = $props();

  let name = $state('');
  let description = $state('');
  let submitting = $state(false);
  let error = $state<string | null>(null);

  // Prevent background scrolling when modal is open
  $effect(() => {
    if (open) {
      const originalStyle = window.getComputedStyle(document.body).overflow;
      document.body.style.overflow = 'hidden';
      return () => {
        document.body.style.overflow = originalStyle;
      };
    }
  });

  function handleClose() {
    if (submitting) return;
    open = false;
    resetForm();
  }

  function resetForm() {
    name = '';
    description = '';
    error = null;
    onClose?.();
  }

  async function handleSubmit(e: Event) {
    e.preventDefault();

    if (!name.trim()) {
      error = m.track_inventory_create_validation_name();
      return;
    }

    try {
      submitting = true;
      error = null;
      await onCreate?.(name.trim(), description.trim());
      open = false;
      resetForm();
    } catch (err) {
      console.error('Failed to create inventory:', err);
      error = err instanceof Error ? err.message : 'Failed to create inventory';
    } finally {
      submitting = false;
    }
  }

  // Handle escape key
  function handleKeydown(e: KeyboardEvent) {
    if (e.key === 'Escape' && open) {
      handleClose();
    }
  }
</script>

<svelte:window onkeydown={handleKeydown} />

{#if open}
  <!-- 1. Overlay: inset-0, heavy blur, semi-transparent black -->
  <div
    class="fixed inset-0 z-[100] bg-black/80 backdrop-blur-md transition-all duration-300"
    onclick={handleClose}
    aria-hidden="true"
  ></div>

  <!-- 2. Centering: absolute positioning with translate -->
  <div
    class="fixed top-1/2 left-1/2 z-[101] w-full max-w-lg -translate-x-1/2 -translate-y-1/2 p-4"
    role="dialog"
    aria-modal="true"
    aria-labelledby="modal-title"
  >
    <!-- 3. Container: Deep charcoal background, amber-tinted border, rounded-2xl -->
    <div
      class="flex flex-col overflow-hidden rounded-2xl border border-amber-500/20 bg-layout-surface shadow-2xl"
    >
      <!-- Header -->
      <div class="flex items-center justify-between border-b border-white/5 p-6">
        <h2 id="modal-title" class="text-xl font-bold tracking-tight text-zinc-100 uppercase">
          {m.track_inventory_create_dialog_title()}
        </h2>
        <!-- Close icon in top right corner -->
        <button
          onclick={handleClose}
          class="rounded-lg p-2 text-zinc-500 transition-colors hover:bg-white/5 hover:text-white"
          disabled={submitting}
          aria-label={m.dialog_close_button()}
        >
          <X size={20} />
        </button>
      </div>

      <!-- Form Body -->
      <form onsubmit={handleSubmit} class="space-y-6 p-6">
        <!-- Labels: small, uppercase, tracking-widest in zinc-500 -->
        <div class="space-y-2">
          <label
            for="inventory-name"
            class="ml-1 text-[10px] font-bold tracking-[0.2em] text-zinc-500 uppercase"
          >
            {m.track_inventory_create_field_name()}
          </label>
          <!-- Input: dark background, subtle border, amber focus ring -->
          <Input
            id="inventory-name"
            bind:value={name}
            placeholder={m.create_inventory_name_placeholder()}
            class="h-12 rounded-xl border-white/10 bg-zinc-950 text-zinc-100 placeholder:text-zinc-600 focus:border-white/20 focus-visible:ring-0 focus-visible:ring-offset-0"
            disabled={submitting}
            required
            autofocus
          />
        </div>

        <div class="space-y-2">
          <label
            for="inventory-desc"
            class="ml-1 text-[10px] font-bold tracking-[0.2em] text-zinc-500 uppercase"
          >
            {m.track_inventory_create_field_description()}
          </label>
          <!-- Textarea: dark background, subtle border, amber focus ring -->
          <Textarea
            id="inventory-desc"
            bind:value={description}
            placeholder={m.create_inventory_desc_placeholder()}
            class="min-h-[120px] resize-none rounded-xl border-white/10 bg-zinc-950 text-zinc-100 placeholder:text-zinc-600 focus:border-white/20 focus-visible:ring-0 focus-visible:ring-offset-0"
            disabled={submitting}
          />
        </div>

        {#if error}
          <div class="rounded-lg border border-red-500/20 bg-red-500/10 p-4 text-sm text-red-400">
            {error}
          </div>
        {/if}

        <!-- Actions -->
        <div class="flex items-center justify-end gap-3 pt-4">
          <!-- Cancel Button: ghost style -->
          <Button
            type="button"
            variant="ghost"
            onclick={handleClose}
            class="px-6 text-zinc-500 transition-colors hover:bg-transparent hover:text-white"
            disabled={submitting}
          >
            {m.track_inventory_create_cancel()}
          </Button>

          <!-- Create Button: Primary amber button (rusty variant) -->
          <Button
            type="submit"
            variant="rusty"
            class="h-12 min-w-[140px] px-8 text-base"
            disabled={submitting || !name.trim()}
          >
            {#if submitting}
              <Loader2 size={18} class="mr-2 animate-spin" />
              <span>{m.create_inventory_saving()}</span>
            {:else}
              {m.track_inventory_create_submit()}
            {/if}
          </Button>
        </div>
      </form>
    </div>
  </div>
{/if}
