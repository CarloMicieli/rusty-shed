<script lang="ts">
  import { Settings, Plus, Star, Trash2, Edit2, X } from 'lucide-svelte';
  import type { WishlistPreview } from '$lib/bindings';
  import * as m from '$lib/paraglide/messages.js';
  import { Button, Badge, Input } from '$lib/components';

  const { wishlist, onRename, onSetDefault, onAddModel, onDelete } = $props<{
    wishlist: WishlistPreview | null;
    onRename?: (name: string) => void;
    onSetDefault?: () => void;
    onAddModel?: () => void;
    onDelete?: (id: string) => void;
  }>();

  let isEditing = $state(false);
  let nameDraft = $state('');
  let showSettings = $state(false);

  $effect(() => {
    if (!wishlist) return;
    if (!isEditing) nameDraft = wishlist.name;
  });

  async function handleRenameSubmit() {
    if (!wishlist) return;
    if (nameDraft.trim() && nameDraft !== wishlist.name) {
      onRename?.(nameDraft.trim());
    }
    isEditing = false;
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === 'Enter') void handleRenameSubmit();
    if (e.key === 'Escape') {
      isEditing = false;
      if (wishlist) nameDraft = wishlist.name;
    }
  }

  function toggleSettings() {
    showSettings = !showSettings;
  }

  function formatDate(value: string): string {
    const d = new Date(value);
    return isNaN(d.getTime()) ? '-' : d.toLocaleDateString();
  }
</script>

{#if wishlist}
  <div class="flex flex-col gap-6 border-b border-white/5 pb-6">
    <div class="flex items-start justify-between">
      <div class="space-y-1">
        <div class="flex items-center gap-3">
          {#if isEditing}
            <div class="flex items-center gap-2">
              <Input
                bind:value={nameDraft}
                onkeydown={handleKeydown}
                class="h-8 min-w-[300px] bg-zinc-900 font-bold text-white shadow-inner focus:ring-amber-500/50"
                autofocus
              />
              <Button size="sm" class="h-8" onclick={handleRenameSubmit}>Save</Button>
              <Button size="sm" variant="ghost" class="h-8" onclick={() => (isEditing = false)}
                >Cancel</Button
              >
            </div>
          {:else}
            <h2 class="text-3xl font-bold tracking-tight text-white">{wishlist.name}</h2>
            {#if wishlist.isDefault}
              <Badge
                class="border-amber-500/20 bg-amber-500/10 text-[10px] font-bold text-amber-500 ring-1 ring-amber-500/20"
              >
                Default
              </Badge>
            {/if}
          {/if}
        </div>
        <p class="font-mono text-xs tracking-wider text-zinc-500 uppercase">
          {wishlist.count}
          {m.stats_rolling_stocks()} · Last updated {formatDate(wishlist.updatedAt)}
        </p>
      </div>

      <div class="flex items-center gap-3">
        <Button onclick={onAddModel} variant="rusty" class="shadow-lg shadow-amber-500/10">
          <Plus size={18} class="mr-2" />
          {m.wishlist_add_model_button()}
        </Button>

        <div class="relative">
          <Button
            variant="outline"
            size="icon"
            onclick={toggleSettings}
            class="border-zinc-800 text-zinc-400 hover:bg-zinc-800 hover:text-white"
          >
            {#if showSettings}
              <X size={18} />
            {:else}
              <Settings size={18} />
            {/if}
          </Button>

          {#if showSettings}
            <div
              class="absolute top-12 right-0 z-50 w-48 animate-in rounded-xl border border-zinc-800 bg-[#0c0c0c] p-1 shadow-2xl duration-200 fade-in zoom-in"
              onmouseleave={() => (showSettings = false)}
              role="menu"
              tabindex="-1"
            >
              <button
                onclick={() => {
                  isEditing = true;
                  showSettings = false;
                }}
                class="flex w-full items-center rounded-lg px-3 py-2 text-sm text-zinc-300 transition-colors hover:bg-zinc-800 hover:text-white"
              >
                <Edit2 size={14} class="mr-2" />
                Rename
              </button>
              <button
                onclick={() => {
                  onSetDefault?.();
                  showSettings = false;
                }}
                class="flex w-full items-center rounded-lg px-3 py-2 text-sm text-zinc-300 transition-colors hover:bg-zinc-800 hover:text-white"
              >
                <Star size={14} class="mr-2" />
                Set as Default
              </button>
              <div class="my-1 h-px bg-zinc-800"></div>
              <button
                onclick={() => {
                  onDelete?.(wishlist.id);
                  showSettings = false;
                }}
                class="flex w-full items-center rounded-lg px-3 py-2 text-sm text-red-400 transition-colors hover:bg-red-400/10"
              >
                <Trash2 size={14} class="mr-2" />
                Delete List
              </button>
            </div>
          {/if}
        </div>
      </div>
    </div>
  </div>
{:else}
  <!-- Empty selection handled by Dashboard -->
{/if}
