<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import * as m from '$lib/paraglide/messages.js';
  import { wishlistService } from '$lib/stores/WishlistService.svelte';

  const dispatch = createEventDispatcher<{ close: void; saved: void }>();

  const wishlists = $derived(wishlistService.wishlists);
  const defaultWishlist = $derived(wishlistService.defaultWishlist);

  let selectedId: string | null = $derived.by(() => defaultWishlist?.id ?? null);
  let newListName = $state('');
  let modelId = $state('');
  let notes = $state('');
  let isSubmitting = $state(false);
  let formError = $state<string | null>(null);

  $effect(() => {
    if (!selectedId && defaultWishlist) {
      selectedId = defaultWishlist.id;
    }
  });

  async function handleSubmit() {
    formError = null;
    if (!modelId.trim()) {
      formError = m.wishlist_modal_missing_model();
      return;
    }

    isSubmitting = true;
    try {
      let targetId = selectedId;

      if (newListName.trim()) {
        const created = await wishlistService.createWishlist(newListName.trim(), false);
        if (!created) {
          formError = m.wishlist_modal_create_failed();
          return;
        }
        targetId = created.id;
      }

      if (!targetId) {
        formError = m.wishlist_modal_select_list_error();
        return;
      }

      const added = await wishlistService.addItem(targetId, modelId.trim());
      if (!added) {
        formError = m.wishlist_modal_add_failed();
        return;
      }

      dispatch('saved');
      close();
    } finally {
      isSubmitting = false;
    }
  }

  function close() {
    dispatch('close');
    newListName = '';
    modelId = '';
    notes = '';
    formError = null;
  }
</script>

<div class="fixed inset-0 z-50 flex items-center justify-center bg-black/60 p-4">
  <div
    class="w-full max-w-lg rounded-container border border-surface-700/70 bg-surface-900 shadow-xl"
  >
    <div class="flex items-center justify-between border-b border-surface-800 px-4 py-3">
      <h3 class="text-base font-semibold tracking-wide uppercase">{m.wishlist_modal_title()}</h3>
      <button class="variant-ghost-surface btn btn-sm" onclick={close} aria-label="close">
        {m.wishlist_modal_close()}
      </button>
    </div>

    <div class="space-y-4 p-4">
      <div class="space-y-2">
        <label
          for="wishlist-select"
          class="text-xs font-semibold tracking-wide text-surface-300 uppercase"
        >
          {m.wishlist_modal_choose_or_create()}
        </label>
        <div class="grid grid-cols-1 gap-2 md:grid-cols-2">
          <select
            id="wishlist-select"
            class="select"
            bind:value={selectedId}
            aria-label={m.wishlist_modal_select_list()}
          >
            <option value="" disabled selected={!selectedId}>
              {m.wishlist_modal_select_placeholder()}
            </option>
            {#each wishlists as list (list.id)}
              <option value={list.id}>
                {list.name}
                {#if list.is_default}
                  (default)
                {/if}
              </option>
            {/each}
          </select>
          <input
            class="input"
            type="text"
            placeholder={m.wishlist_modal_new_list_placeholder()}
            bind:value={newListName}
          />
        </div>
      </div>

      <div class="space-y-2">
        <label
          for="model-id"
          class="text-xs font-semibold tracking-wide text-surface-300 uppercase"
        >
          {m.wishlist_modal_item_id_label()}
        </label>
        <input
          id="model-id"
          class="input"
          type="text"
          placeholder={m.wishlist_modal_item_id_placeholder()}
          bind:value={modelId}
        />
      </div>

      <div class="space-y-2">
        <label
          for="wishlist-notes"
          class="text-xs font-semibold tracking-wide text-surface-300 uppercase"
        >
          {m.wishlist_modal_notes_label()}
        </label>
        <textarea
          id="wishlist-notes"
          class="textarea"
          rows="3"
          placeholder={m.wishlist_modal_notes_placeholder()}
          bind:value={notes}
        ></textarea>
      </div>

      {#if formError}
        <div
          class="variant-soft-error rounded-container border border-error-700/40 p-3 text-sm text-error-100"
        >
          {formError}
        </div>
      {/if}
    </div>

    <div class="flex items-center justify-end gap-2 border-t border-surface-800 px-4 py-3">
      <button class="variant-ghost-surface btn btn-sm" onclick={close} disabled={isSubmitting}>
        {m.wishlist_modal_cancel()}
      </button>
      <button
        class="variant-filled-primary btn btn-sm"
        onclick={handleSubmit}
        disabled={isSubmitting}
      >
        {isSubmitting ? m.wishlist_modal_saving() : m.wishlist_modal_save()}
      </button>
    </div>
  </div>
</div>
