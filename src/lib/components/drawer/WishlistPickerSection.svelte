<script lang="ts">
  import * as m from '$lib/paraglide/messages.js';
  import * as Select from '$lib/components/ui/select';
  import { DrawerInput } from '$lib/components/drawer';
  import type { WishlistPreview } from '$lib/bindings';

  interface Props {
    wishlistId: string;
    newListName?: string;
    wishlists: WishlistPreview[];
    disabled?: boolean;
    disableWishlistSelection?: boolean;
    errors?: {
      wishlistId?: string;
    };
  }

  let {
    wishlistId = $bindable(),
    newListName = $bindable(''),
    wishlists,
    disabled = false,
    disableWishlistSelection = false,
    errors = {}
  }: Props = $props();

  const isDropdownDisabled = $derived(
    disabled || disableWishlistSelection || newListName.trim() !== '' || wishlists.length === 0
  );
  const selectedWishlist = $derived(wishlists.find((l) => l.id === wishlistId));
</script>

<div class="space-y-1">
  <span class="text-[10px] font-bold tracking-[0.2em] text-zinc-500 uppercase">
    {m.wishlist_modal_choose_or_create()}
  </span>
  <div class="grid grid-cols-2 gap-3">
    <Select.Root
      type="single"
      value={wishlistId || undefined}
      disabled={isDropdownDisabled}
      onValueChange={(v) => {
        wishlistId = v;
      }}
    >
      <Select.Trigger
        class="w-full border-[#1F1F1F] bg-[#0F0F0F] text-[#E0E0E0]"
        aria-label={m.wishlist_modal_select_list()}
      >
        {#if selectedWishlist}
          {selectedWishlist.name}{#if selectedWishlist.isDefault}
            (default){/if}
        {:else}
          <span class="text-zinc-500">{m.wishlist_modal_select_placeholder()}</span>
        {/if}
      </Select.Trigger>
      <Select.Content>
        {#each wishlists as list (list.id)}
          <Select.Item value={list.id} label={list.name}>
            {list.name}{#if list.isDefault}
              (default){/if}
          </Select.Item>
        {/each}
      </Select.Content>
    </Select.Root>

    {#if !disableWishlistSelection}
      <DrawerInput
        type="text"
        placeholder={m.wishlist_modal_new_list_placeholder()}
        bind:value={newListName}
        {disabled}
      />
    {/if}
  </div>
  {#if errors.wishlistId}
    <p class="text-xs text-destructive">{errors.wishlistId}</p>
  {/if}
</div>
