<script lang="ts">
  import * as m from '$lib/paraglide/messages.js';
  import {
    DrawerSectionBar,
    WishlistPickerSection,
    WishlistPreferencesSection
  } from '$lib/components/drawer';
  import type { WishlistPreview, WishlistPriority } from '$lib/bindings';

  interface Props {
    wishlistId: string;
    newListName?: string;
    wishlists: WishlistPreview[];
    priority: WishlistPriority;
    desiredPrice: number | null;
    currency: string;
    notes?: string;
    errors?: {
      wishlistId?: string;
      desiredPrice?: string;
    };
    disabled?: boolean;
    disableWishlistSelection?: boolean;
    expanded?: boolean;
  }

  let {
    wishlistId = $bindable(),
    newListName = $bindable(''),
    wishlists,
    priority = $bindable(),
    desiredPrice = $bindable(),
    currency,
    notes = $bindable(undefined),
    errors = {},
    disabled = false,
    disableWishlistSelection = false,
    expanded = $bindable(true)
  }: Props = $props();
</script>

<div class="space-y-3">
  <DrawerSectionBar
    label={m.drawer_section_wishlist()}
    {expanded}
    onToggle={() => (expanded = !expanded)}
  />

  {#if expanded}
    <WishlistPickerSection
      bind:wishlistId
      bind:newListName
      {wishlists}
      {disabled}
      {disableWishlistSelection}
      errors={{ wishlistId: errors.wishlistId }}
    />

    <WishlistPreferencesSection
      bind:priority
      bind:desiredPrice
      {currency}
      bind:notes
      {disabled}
      showCard={false}
      errors={{ desiredPrice: errors.desiredPrice }}
    />
  {/if}
</div>
