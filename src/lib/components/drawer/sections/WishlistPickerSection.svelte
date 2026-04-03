<script lang="ts">
  import * as m from '$lib/paraglide/messages.js';
  import * as Popover from '$lib/components/ui/popover';
  import type { WishlistPreview } from '$lib/bindings';
  import { ChevronDown, Check, Search, Plus } from 'lucide-svelte';
  import { tick } from 'svelte';

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

  let open = $state(false);
  let searchQuery = $state('');
  let searchInputEl = $state<HTMLInputElement | null>(null);

  const selectedWishlist = $derived(wishlists.find((l) => l.id === wishlistId));

  const filteredWishlists = $derived(
    searchQuery.trim() === ''
      ? wishlists
      : wishlists.filter((l) => l.name.toLowerCase().includes(searchQuery.trim().toLowerCase()))
  );

  const exactMatch = $derived(
    wishlists.some((l) => l.name.toLowerCase() === searchQuery.trim().toLowerCase())
  );

  const showCreateOption = $derived(
    !disableWishlistSelection && searchQuery.trim() !== '' && !exactMatch
  );

  const displayValue = $derived(
    selectedWishlist
      ? selectedWishlist.name + (selectedWishlist.isDefault ? ' (default)' : '')
      : newListName
        ? `${m.wishlist_picker_new_badge()} ${newListName}`
        : ''
  );

  const isTriggerDisabled = $derived(disabled || disableWishlistSelection);

  async function handleOpenChange(value: boolean) {
    open = value;
    if (value) {
      searchQuery = '';
      await tick();
      searchInputEl?.focus();
    }
  }

  function selectExisting(list: WishlistPreview) {
    wishlistId = list.id;
    newListName = '';
    open = false;
    searchQuery = '';
  }

  function createNew() {
    const name = searchQuery.trim();
    if (!name) return;
    newListName = name;
    wishlistId = '';
    open = false;
    searchQuery = '';
  }
</script>

<div class="space-y-3">
  <h3 class="text-[10px] font-semibold tracking-widest text-zinc-500 uppercase">
    {m.wishlist_modal_choose_or_create()}
  </h3>

  <Popover.Root bind:open onOpenChange={handleOpenChange}>
    <Popover.Trigger
      class="flex w-full items-center justify-between rounded-md border border-border bg-background px-3 py-2 text-sm text-foreground shadow-sm transition-colors hover:bg-accent/50 focus-visible:ring-1 focus-visible:ring-ring focus-visible:outline-none disabled:cursor-not-allowed disabled:opacity-50"
      disabled={isTriggerDisabled}
      aria-label={m.wishlist_modal_select_list()}
    >
      {#if displayValue}
        <span class="truncate font-mono">{displayValue}</span>
      {:else}
        <span class="truncate text-muted-foreground">{m.wishlist_picker_search_placeholder()}</span>
      {/if}
      <ChevronDown size={14} class="ml-2 shrink-0 text-muted-foreground" />
    </Popover.Trigger>

    <Popover.Content class="w-[var(--bits-popover-anchor-width)] p-0" align="start" sideOffset={4}>
      <!-- Search input -->
      <div class="flex items-center gap-2 border-b border-border px-3">
        <Search size={14} class="shrink-0 text-muted-foreground" />
        <input
          bind:this={searchInputEl}
          type="text"
          placeholder={m.wishlist_picker_search_placeholder()}
          bind:value={searchQuery}
          class="h-9 w-full bg-background py-2 text-sm outline-none placeholder:text-muted-foreground"
        />
      </div>

      <!-- List -->
      <div
        class="max-h-52 overflow-y-auto py-1"
        role="listbox"
        aria-label={m.wishlist_modal_select_list()}
      >
        {#each filteredWishlists as list (list.id)}
          <button
            type="button"
            role="option"
            aria-selected={wishlistId === list.id}
            class="flex w-full cursor-pointer items-center gap-2 px-3 py-2 text-left font-mono text-sm hover:bg-accent hover:text-accent-foreground focus-visible:bg-accent focus-visible:outline-none"
            onclick={() => selectExisting(list)}
          >
            <Check size={14} class={wishlistId === list.id ? 'text-primary' : 'invisible'} />
            <span class="truncate">
              {list.name}{#if list.isDefault}&nbsp;(default){/if}
            </span>
          </button>
        {/each}

        {#if filteredWishlists.length === 0 && !showCreateOption}
          <p class="px-3 py-2 text-sm text-muted-foreground">
            {m.wishlist_modal_select_placeholder()}
          </p>
        {/if}

        {#snippet createOption()}
          <button
            type="button"
            class="flex w-full cursor-pointer items-center gap-2 border-t border-primary/30 px-3 py-2 text-left font-mono text-sm text-primary hover:bg-primary/10 focus-visible:bg-primary/10 focus-visible:outline-none"
            onclick={createNew}
          >
            <Plus size={14} class="shrink-0" />
            <span class="truncate">{m.wishlist_picker_create_label()} "{searchQuery.trim()}"</span>
          </button>
        {/snippet}

        {#if showCreateOption}
          {@render createOption()}
        {/if}
      </div>
    </Popover.Content>
  </Popover.Root>

  {#if errors.wishlistId}
    <p class="text-xs text-destructive">{errors.wishlistId}</p>
  {/if}
</div>
