<script lang="ts">
  import * as m from '$lib/paraglide/messages.js';
  import type { WishlistItem, WishlistPriority, WishlistStatus } from '$lib/bindings';

  interface Props {
    item: WishlistItem;
    wishlistName: string;
  }

  let { item, wishlistName }: Props = $props();

  // ── Date formatter ──────────────────────────────────────────────────────────
  const dateFormatter = new Intl.DateTimeFormat(undefined, { dateStyle: 'medium' });

  function formatDate(iso: string): string {
    return dateFormatter.format(new Date(iso));
  }

  // ── Price formatter ─────────────────────────────────────────────────────────
  function formatPrice(amount: bigint, currency: string): string {
    return new Intl.NumberFormat(undefined, {
      style: 'currency',
      currency
    }).format(Number(amount) / 100);
  }

  // ── Priority label ───────────────────────────────────────────────────────────
  function priorityLabel(p: WishlistPriority): string {
    switch (p) {
      case 'LOW':
        return m.wishlist_priority_low();
      case 'NORMAL':
        return m.wishlist_priority_normal();
      case 'HIGH':
        return m.wishlist_priority_high();
    }
  }

  // ── Status label ─────────────────────────────────────────────────────────────
  function statusLabel(s: WishlistStatus): string {
    switch (s) {
      case 'WANTED':
        return m.wishlist_item_status_wanted();
      case 'ON_ORDER':
        return m.wishlist_item_status_on_order();
      case 'PURCHASED':
        return m.wishlist_item_status_purchased();
      case 'IGNORED':
        return m.wishlist_item_status_ignored();
    }
  }
</script>

<aside class="w-full shrink-0 space-y-4 lg:w-80">
  <!-- ── Wish List Details Section ─────────────────────────────────────────── -->
  <section class="rounded-lg border border-white/10 bg-black/20 p-4">
    <h2 class="mb-3 text-xs font-semibold tracking-widest text-muted-foreground uppercase">
      {m.wishlist_item_section_details()}
    </h2>

    <dl class="space-y-2 text-sm">
      <!-- List name -->
      <div class="flex justify-between gap-2">
        <dt class="shrink-0 text-muted-foreground">{m.wishlist_item_wishlist_name()}</dt>
        <dd class="text-right font-medium">{wishlistName}</dd>
      </div>

      <!-- Priority -->
      <div class="flex justify-between gap-2">
        <dt class="shrink-0 text-muted-foreground">{m.wishlist_field_priority()}</dt>
        <dd>
          <span class="rounded-full bg-muted px-2 py-0.5 text-xs font-medium">
            {priorityLabel(item.priority)}
          </span>
        </dd>
      </div>

      <!-- Status -->
      <div class="flex justify-between gap-2">
        <dt class="shrink-0 text-muted-foreground">{m.wishlist_item_status()}</dt>
        <dd>
          <span class="rounded-full bg-muted px-2 py-0.5 text-xs font-medium">
            {statusLabel(item.status)}
          </span>
        </dd>
      </div>

      <!-- Desired Price -->
      <div class="flex justify-between gap-2">
        <dt class="shrink-0 text-muted-foreground">{m.wishlist_field_desired_price()}</dt>
        <dd class="text-right font-medium">
          {#if item.desiredPrice}
            {formatPrice(item.desiredPrice.amount, item.desiredPrice.currency)}
          {:else}
            <span class="text-muted-foreground italic">{m.wishlist_item_price_not_set()}</span>
          {/if}
        </dd>
      </div>

      <!-- Purchased Price (only shown when non-null) -->
      {#if item.purchasedPrice}
        <div class="flex justify-between gap-2">
          <dt class="shrink-0 text-muted-foreground">{m.wishlist_item_purchased_price()}</dt>
          <dd class="text-right font-medium">
            {formatPrice(item.purchasedPrice.amount, item.purchasedPrice.currency)}
          </dd>
        </div>
      {/if}
    </dl>
  </section>

  <!-- ── Personal Context Section ──────────────────────────────────────────── -->
  {#if item.notes || item.addedDate}
    <section class="rounded-lg border border-white/10 bg-black/20 p-4">
      <h2 class="mb-3 text-xs font-semibold tracking-widest text-muted-foreground uppercase">
        {m.wishlist_item_section_personal_context()}
      </h2>

      <dl class="space-y-2 text-sm">
        <!-- Added date -->
        <div class="flex justify-between gap-2">
          <dt class="shrink-0 text-muted-foreground">{m.wishlist_item_added_date()}</dt>
          <dd class="font-medium">{formatDate(item.addedDate)}</dd>
        </div>

        <!-- Notes (only if present) -->
        {#if item.notes}
          <div class="mt-2">
            <dt class="mb-1 text-muted-foreground">{m.wishlist_item_notes()}</dt>
            <dd>
              <p class="line-clamp-3 text-sm leading-relaxed">{item.notes}</p>
            </dd>
          </div>
        {/if}
      </dl>
    </section>
  {/if}
</aside>
