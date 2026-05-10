<script lang="ts">
  import * as m from '$lib/paraglide/messages.js';
  import { DrawerSectionCard, FormPrice, PriorityToggle } from '$lib/components/drawer';
  import { regionalManager } from '$lib/features/settings/RegionalManager.svelte';
  import type { WishlistPriority } from '$lib/bindings';

  interface Props {
    priority: WishlistPriority;
    desiredPrice: number | null;
    currency: string;
    notes?: string;
    disabled?: boolean;
    showCard?: boolean;
    errors?: {
      desiredPrice?: string;
    };
  }

  let {
    priority = $bindable(),
    desiredPrice = $bindable(),
    currency,
    notes = $bindable(undefined),
    disabled = false,
    showCard = true,
    errors = {}
  }: Props = $props();

  const currencySymbol = $derived(regionalManager.getCurrencySymbol(currency));
</script>

{#snippet sectionContent()}
  <div class="space-y-3">
    <PriorityToggle bind:priority {disabled} />

    <!-- Desired price -->
    <FormPrice
      label={m.wishlist_modal_desired_price()}
      id="wishlist-section-desired-price"
      bind:value={desiredPrice}
      symbol={currencySymbol}
      inputClass="bg-background border-border rounded-sm text-foreground placeholder:text-muted-foreground"
      {disabled}
      error={errors.desiredPrice}
    />

    {#if notes !== undefined}
      <div class="space-y-1">
        <label
          for="wishlist-preferences-notes"
          class="text-[10px] font-bold tracking-[0.2em] text-muted-foreground uppercase"
        >
          {m.wishlist_field_notes()}
        </label>
        <textarea
          id="wishlist-preferences-notes"
          bind:value={notes}
          {disabled}
          rows={3}
          placeholder={m.wishlist_modal_notes_placeholder()}
          class="flex w-full rounded-sm border border-border bg-background px-3 py-2 text-sm text-foreground placeholder:text-muted-foreground focus:border-primary/60 focus:ring-1 focus:ring-primary focus:outline-none disabled:cursor-not-allowed disabled:opacity-50"
        ></textarea>
      </div>
    {/if}
  </div>
{/snippet}

{#if showCard}
  <DrawerSectionCard label={m.drawer_section_wishlist()}>
    {@render sectionContent()}
  </DrawerSectionCard>
{:else}
  {@render sectionContent()}
{/if}
