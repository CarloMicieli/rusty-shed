<script lang="ts">
  import * as m from '$lib/paraglide/messages.js';
  import type {
    CollectionItemView,
    SellerView,
    ModelCondition,
    BoxCondition,
    PurchaseCondition
  } from '$lib/bindings';

  interface Props {
    item: CollectionItemView;
    seller: SellerView | null;
  }

  let { item, seller }: Props = $props();

  // ── Date formatter ──────────────────────────────────────────────────────────
  const dateFormatter = new Intl.DateTimeFormat('en-US', {
    year: 'numeric',
    month: 'short',
    day: 'numeric'
  });

  function formatDate(iso: string): string {
    return dateFormatter.format(new Date(iso));
  }

  // ── Price formatter ─────────────────────────────────────────────────────────
  function formatPrice(amount: bigint, currency: string): string {
    return new Intl.NumberFormat('en-US', {
      style: 'currency',
      currency
    }).format(Number(amount) / 100);
  }

  // ── Condition label helpers ─────────────────────────────────────────────────
  function modelConditionLabel(c: ModelCondition): string {
    switch (c) {
      case 'MINT':
        return m.collection_item_condition_mint();
      case 'NEAR_MINT':
        return m.collection_item_condition_near_mint();
      case 'EXCELLENT':
        return m.collection_item_condition_excellent();
      case 'VERY_GOOD':
        return m.collection_item_condition_very_good();
      case 'GOOD':
        return m.collection_item_condition_good();
      case 'FAIR':
        return m.collection_item_condition_fair();
      case 'POOR':
        return m.collection_item_condition_poor();
      case 'FOR_PARTS':
        return m.collection_item_condition_for_parts();
    }
  }

  function boxConditionLabel(c: BoxCondition): string {
    switch (c) {
      case 'ORIGINAL_MINT':
        return m.collection_item_box_original_mint();
      case 'ORIGINAL_GOOD':
        return m.collection_item_box_original_good();
      case 'ORIGINAL_WORN':
        return m.collection_item_box_original_worn();
      case 'REPLACEMENT_BOX':
        return m.collection_item_box_replacement();
      case 'NO_BOX':
        return m.collection_item_box_no_box();
    }
  }

  function purchaseConditionLabel(c: PurchaseCondition): string {
    switch (c) {
      case 'NEW':
        return m.collection_item_purchase_condition_new();
      case 'PRE_OWNED':
        return m.collection_item_purchase_condition_pre_owned();
    }
  }

  // ── Decoder URN parser ──────────────────────────────────────────────────────
  // Format: trn:decoder:{manufacturer}:{code}  →  "{manufacturer} / {code}"
  function parseDecoderUrn(urn: string): string {
    const parts = urn.split(':');
    if (parts.length >= 4) {
      return `${parts[2]} / ${parts[3]}`;
    }
    return urn;
  }

  // ── Derived state ───────────────────────────────────────────────────────────
  const purchasedInfo = $derived(
    item.purchaseInfo?.kind === 'purchased' ? item.purchaseInfo.data : null
  );

  const hasAnyCondition = $derived(
    item.modelCondition !== null || item.boxCondition !== null || item.purchaseCondition !== null
  );

  const digitalEntries = $derived(
    item.rollingStocks.filter((rs) => rs.digital !== null).map((rs) => rs.digital!)
  );
</script>

<aside class="w-full shrink-0 space-y-4 lg:w-80">
  <!-- ── Acquisition Section ──────────────────────────────────────────────── -->
  <section class="rounded-lg border border-white/10 bg-black/20 p-4">
    <h2 class="mb-3 text-xs font-semibold tracking-widest text-muted-foreground uppercase">
      {m.collection_item_section_acquisition()}
    </h2>

    {#if purchasedInfo}
      <dl class="space-y-2 text-sm">
        <!-- Seller -->
        <div class="flex justify-between gap-2">
          <dt class="shrink-0 text-muted-foreground">{m.collection_item_seller()}</dt>
          <dd class="text-right font-medium">
            {#if seller}
              {#if seller.websiteUrl}
                <a
                  href={seller.websiteUrl}
                  target="_blank"
                  rel="noopener noreferrer"
                  class="text-primary hover:underline"
                >
                  {seller.name}
                </a>
              {:else}
                {seller.name}
              {/if}
            {:else}
              <span class="text-muted-foreground italic">{m.collection_item_not_recorded()}</span>
            {/if}
          </dd>
        </div>

        <!-- Purchase date -->
        {#if purchasedInfo.purchaseDate}
          <div class="flex justify-between gap-2">
            <dt class="shrink-0 text-muted-foreground">{m.collection_item_purchase_date()}</dt>
            <dd class="text-right font-medium">{formatDate(purchasedInfo.purchaseDate)}</dd>
          </div>
        {/if}

        <!-- Price -->
        {#if purchasedInfo.price}
          <div class="flex justify-between gap-2">
            <dt class="shrink-0 text-muted-foreground">{m.collection_item_price()}</dt>
            <dd class="text-right font-medium">
              {formatPrice(purchasedInfo.price.amount, purchasedInfo.price.currency)}
            </dd>
          </div>
        {/if}
      </dl>
    {:else}
      <p class="text-sm text-muted-foreground italic">{m.collection_item_not_recorded()}</p>
    {/if}
  </section>

  <!-- ── Condition Section ────────────────────────────────────────────────── -->
  {#if hasAnyCondition}
    <section class="rounded-lg border border-white/10 bg-black/20 p-4">
      <h2 class="mb-3 text-xs font-semibold tracking-widest text-muted-foreground uppercase">
        {m.collection_item_section_condition()}
      </h2>

      <dl class="space-y-2 text-sm">
        {#if item.purchaseCondition}
          <div class="flex justify-between gap-2">
            <dt class="shrink-0 text-muted-foreground">{m.collection_item_purchase_condition()}</dt>
            <dd>
              <span class="rounded-full bg-muted px-2 py-0.5 text-xs font-medium">
                {purchaseConditionLabel(item.purchaseCondition)}
              </span>
            </dd>
          </div>
        {/if}

        {#if item.modelCondition}
          <div class="flex justify-between gap-2">
            <dt class="shrink-0 text-muted-foreground">{m.collection_item_model_condition()}</dt>
            <dd>
              <span class="rounded-full bg-muted px-2 py-0.5 text-xs font-medium">
                {modelConditionLabel(item.modelCondition)}
              </span>
            </dd>
          </div>
        {/if}

        {#if item.boxCondition}
          <div class="flex justify-between gap-2">
            <dt class="shrink-0 text-muted-foreground">{m.collection_item_box_condition()}</dt>
            <dd>
              <span class="rounded-full bg-muted px-2 py-0.5 text-xs font-medium">
                {boxConditionLabel(item.boxCondition)}
              </span>
            </dd>
          </div>
        {/if}
      </dl>
    </section>
  {/if}

  <!-- ── Operational Section ──────────────────────────────────────────────── -->
  <section class="rounded-lg border border-white/10 bg-black/20 p-4">
    <h2 class="mb-3 text-xs font-semibold tracking-widest text-muted-foreground uppercase">
      {m.collection_item_section_operational()}
    </h2>

    {#if digitalEntries.length > 0}
      <ul class="space-y-3">
        {#each digitalEntries as digital, i (i)}
          <li class="space-y-1 text-sm">
            <div class="flex items-center justify-between gap-2">
              <span class="text-muted-foreground">{m.collection_item_dcc_address()}</span>
              <span class="text-2xl font-bold text-primary tabular-nums">{digital.dcc_address}</span
              >
            </div>
            <div class="flex items-center justify-between gap-2">
              <span class="text-muted-foreground">{m.collection_item_decoder()}</span>
              <span class="font-medium">{parseDecoderUrn(digital.installed_decoder_id)}</span>
            </div>
          </li>
        {/each}
      </ul>
    {:else}
      <p class="text-sm text-muted-foreground italic">{m.collection_item_dcc_not_configured()}</p>
    {/if}
  </section>

  <!-- ── Personal Context Section ─────────────────────────────────────────── -->
  <section class="rounded-lg border border-white/10 bg-black/20 p-4">
    <h2 class="mb-3 text-xs font-semibold tracking-widest text-muted-foreground uppercase">
      {m.collection_item_section_personal_context()}
    </h2>

    <dl class="space-y-2 text-sm">
      <!-- Added date (always present) -->
      <div class="flex justify-between gap-2">
        <dt class="shrink-0 text-muted-foreground">{m.collection_item_added_date()}</dt>
        <dd class="font-medium">{formatDate(item.addedDate)}</dd>
      </div>

      <!-- Notes (only if present) -->
      {#if item.notes}
        <div class="mt-2">
          <dt class="mb-1 text-muted-foreground">{m.collection_item_notes()}</dt>
          <dd>
            <p class="line-clamp-3 text-sm leading-relaxed">{item.notes}</p>
          </dd>
        </div>
      {/if}
    </dl>
  </section>
</aside>
