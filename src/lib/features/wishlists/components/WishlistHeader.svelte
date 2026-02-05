<script lang="ts">
  import { Heart, Star, Plus } from 'lucide-svelte';
  import type { WishlistPreview } from '$lib/bindings';
  import * as m from '$lib/paraglide/messages.js';
  import { Input } from '$lib/components';

  const { wishlist, onRename, onSetDefault, onAddModel } = $props<{
    wishlist: WishlistPreview | null;
    onRename?: (name: string) => void;
    onSetDefault?: () => void;
    onAddModel?: () => void;
  }>();

  let isEditing = $state(false);
  let nameDraft = $state('');

  $effect(() => {
    if (!wishlist) return;
    if (!isEditing) nameDraft = wishlist.name;
  });

  async function handleRenameBlur() {
    if (!wishlist) return;
    if (nameDraft.trim() && nameDraft !== wishlist.name) {
      console.log('WishlistHeader: rename requested', nameDraft.trim());
      onRename?.(nameDraft.trim());
    } else {
      nameDraft = wishlist.name;
    }
    isEditing = false;
  }

  function handleRenameKeydown(event: KeyboardEvent) {
    if (event.key === 'Enter') {
      void handleRenameBlur();
    }
    if (event.key === 'Escape') {
      isEditing = false;
      if (wishlist) nameDraft = wishlist.name;
    }
  }
</script>

{#if wishlist}
  <div class="flex flex-wrap items-center justify-between gap-3">
    <div class="flex items-center gap-2">
      <Heart class="text-accent-500" size={20} />
      {#if isEditing}
        <Input
          class="input-lg variant-ghost-surface"
          value={nameDraft}
          oninput={(e) => (nameDraft = e.currentTarget.value)}
          onblur={handleRenameBlur}
          onkeydown={handleRenameKeydown}
        />
      {:else}
        <button
          type="button"
          class="h4 text-left font-bold"
          onclick={() => (isEditing = true)}
          onkeydown={(e) => e.key === 'Enter' && (isEditing = true)}
        >
          {nameDraft}
        </button>
      {/if}
    </div>
    <div class="flex items-center gap-2">
      <button class="variant-soft-primary btn" onclick={() => onAddModel?.()}>
        <Plus size={16} />
        <span>{m.wishlist_add_model_button()}</span>
      </button>
      <button
        class="variant-soft-primary btn"
        class:variant-filled-primary={wishlist.is_default}
        onclick={() => onSetDefault?.()}
      >
        <Star size={16} />
        <span>Set as Default</span>
      </button>
    </div>
  </div>
{:else}
  <div
    class="border-surface-700/60 text-surface-400 rounded-xl border border-dashed p-8 text-center"
  >
    Wishlist is empty
  </div>
{/if}
