<script lang="ts">
  import { Heart, Star } from 'lucide-svelte';

  type WishlistPreviewLite = {
    id: string;
    name: string;
    is_default: boolean;
  };

  const { wishlist, onRename, onSetDefault } = $props<{
    wishlist: WishlistPreviewLite | null;
    onRename?: (name: string) => void;
    onSetDefault?: () => void;
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
        <input
          class="input-lg variant-ghost-surface input"
          bind:value={nameDraft}
          onblur={handleRenameBlur}
          onkeydown={handleRenameKeydown}
        />
      {:else}
        <button
          type="button"
          class="text-left h4 font-bold"
          onclick={() => (isEditing = true)}
          onkeydown={(e) => e.key === 'Enter' && (isEditing = true)}
        >
          {nameDraft}
        </button>
      {/if}
    </div>
    <div class="flex items-center gap-2">
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
    class="rounded-xl border border-dashed border-surface-700/60 p-8 text-center text-surface-400"
  >
    Wishlist is empty
  </div>
{/if}
