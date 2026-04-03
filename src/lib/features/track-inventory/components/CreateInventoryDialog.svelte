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
  <!-- 1. Overlay: softer veil for light theme -->
  <div
    class="fixed inset-0 z-[100] bg-black/50 backdrop-blur-sm transition-all duration-300"
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
    <!-- 3. Container: Parchment card, industrial hard-offset shadow, sharp corners -->
    <div
      class="flex flex-col overflow-hidden rounded-sm border-2 border-border bg-card shadow-[4px_4px_0px_0px_rgba(0,0,0,0.2)]"
    >
      <!-- Header -->
      <div class="flex items-center justify-between border-b border-border p-6">
        <h2 id="modal-title" class="font-bebas text-2xl tracking-wider text-primary uppercase">
          {m.track_inventory_create_dialog_title()}
        </h2>
        <!-- Close icon in top right corner -->
        <button
          onclick={handleClose}
          class="rounded-sm p-2 text-muted-foreground transition-colors hover:bg-muted/30 hover:text-foreground"
          disabled={submitting}
          aria-label={m.dialog_close_button()}
        >
          <X size={20} />
        </button>
      </div>

      <!-- Form Body -->
      <form onsubmit={handleSubmit} class="space-y-6 p-6">
        <!-- Name field: stamped label + parchment input -->
        <div class="space-y-2">
          <label
            for="inventory-name"
            class="text-[10px] font-bold tracking-[0.2em] text-muted-foreground uppercase"
          >
            {m.track_inventory_create_field_name()}
          </label>
          <Input
            id="inventory-name"
            bind:value={name}
            placeholder={m.create_inventory_name_placeholder()}
            class="h-12 rounded-sm border-border bg-background text-foreground placeholder:text-muted-foreground/60 focus-visible:border-primary focus-visible:ring-1 focus-visible:ring-primary"
            disabled={submitting}
            required
            autofocus
          />
        </div>

        <!-- Description field: stamped label + parchment textarea -->
        <div class="space-y-2">
          <label
            for="inventory-desc"
            class="text-[10px] font-bold tracking-[0.2em] text-muted-foreground uppercase"
          >
            {m.track_inventory_create_field_description()}
          </label>
          <Textarea
            id="inventory-desc"
            bind:value={description}
            placeholder={m.create_inventory_desc_placeholder()}
            class="min-h-[120px] resize-none rounded-sm border-border bg-background text-foreground placeholder:text-muted-foreground/60 focus-visible:border-primary focus-visible:ring-1 focus-visible:ring-primary"
            disabled={submitting}
          />
        </div>

        {#if error}
          <div
            class="rounded-sm border border-destructive/30 bg-destructive/10 p-4 text-sm text-destructive"
          >
            {error}
          </div>
        {/if}

        <!-- Actions -->
        <div class="flex items-center justify-end gap-3 pt-4">
          <!-- Cancel Button: secondary/outline style -->
          <Button
            type="button"
            variant="outline"
            onclick={handleClose}
            class="rounded-sm border-border px-6 text-muted-foreground hover:bg-muted/30 hover:text-foreground"
            disabled={submitting}
          >
            {m.track_inventory_create_cancel()}
          </Button>

          <!-- Create Button: Brass mechanical lever -->
          <Button
            type="submit"
            variant="default"
            class="variant-steampunk-lever h-12 min-w-[140px] rounded-sm px-8 font-bebas text-lg uppercase shadow-inner"
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
