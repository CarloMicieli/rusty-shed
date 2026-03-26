<script lang="ts">
  import * as m from '$lib/paraglide/messages.js';
  import * as Popover from '$lib/components/ui/popover';
  import { Calendar } from '$lib/components/ui/calendar';
  import { commands } from '$lib/bindings';
  import type { Currency, WishlistItem, WishlistPriority, WishlistStatus } from '$lib/bindings';
  import { toaster } from '$lib/toaster';
  import { CalendarDate } from '@internationalized/date';
  import { CurrencyInput } from '$lib/components';
  import { regionalManager } from '$lib/features/settings/RegionalManager.svelte';
  import InPlaceSelectEdit from '$lib/components/InPlaceSelectEdit.svelte';

  interface Props {
    item: WishlistItem;
    wishlistId: string;
    wishlistName: string;
    defaultCurrency?: Currency;
    onUpdate?: () => void;
  }

  let {
    item = $bindable(),
    wishlistId,
    wishlistName,
    defaultCurrency = 'EUR' as Currency,
    onUpdate
  }: Props = $props();

  // ── Active inline-edit field ─────────────────────────────────────────────
  // Used only for desiredPrice and addedDate (text input / calendar).
  // Priority and Status use Select.Root with their own open state.
  type EditableField = 'desiredPrice';
  let activeField = $state<EditableField | null>(null);

  // ── Price edit state ─────────────────────────────────────────────────────
  let priceInputCents = $state<number | null>(null);
  let priceError = $state<string | null>(null);
  let priceDisplayValue = $state<string>('');
  let priceHasInvalidInput = $state<boolean>(false);

  // ── Date popover state ───────────────────────────────────────────────────
  let datePopoverOpen = $state(false);

  // ── Date formatter ──────────────────────────────────────────────────────
  // Delegate to RegionalManager to ensure OS locale (via Tauri) is used.
  const formatDate = (iso: string) => regionalManager.formatDate(iso);

  // ── Price formatter ─────────────────────────────────────────────────────
  const formatPrice = (amount: bigint, currency: string) =>
    regionalManager.formatCurrencyWith(amount, currency);

  // ── Priority label ───────────────────────────────────────────────────────
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

  // ── Status label ─────────────────────────────────────────────────────────
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

  // ── Activate / deactivate field ──────────────────────────────────────────
  function activateDesiredPrice() {
    activeField = 'desiredPrice';
    priceError = null;
    priceInputCents = item.desiredPrice ? Number(item.desiredPrice.amount) : null;
  }

  function cancelField() {
    activeField = null;
    priceError = null;
    datePopoverOpen = false;
  }

  // ── Generic save helper with optimistic update + rollback ────────────────
  async function saveField(
    partialArgs: Partial<{
      priority: WishlistPriority | null;
      status: WishlistStatus | null;
      desiredPriceAmount: bigint | null | undefined;
      desiredPriceCurrency: string | null;
      addedDate: string | null;
    }>,
    optimisticUpdate: () => void,
    rollback: () => void
  ): Promise<void> {
    optimisticUpdate();
    activeField = null;
    datePopoverOpen = false;

    const result = await commands.updateWishlistItem({
      wishlistId,
      itemId: item.id,
      priority: partialArgs.priority ?? null,
      status: partialArgs.status ?? null,
      desiredPriceAmount: partialArgs.desiredPriceAmount,
      desiredPriceCurrency: partialArgs.desiredPriceCurrency ?? null,
      addedDate: partialArgs.addedDate ?? null
    });

    if (result.status !== 'ok') {
      rollback();
      toaster.error(typeof result.error === 'string' ? result.error : m.wishlist_item_error());
      return;
    }

    onUpdate?.();
  }

  // ── Priority save ────────────────────────────────────────────────────────
  function handlePriorityChange(value: string) {
    const newPriority = value as WishlistPriority;
    const prev = item.priority;
    void saveField(
      { priority: newPriority },
      () => {
        item = { ...item, priority: newPriority };
      },
      () => {
        item = { ...item, priority: prev };
      }
    );
  }

  // ── Status save ──────────────────────────────────────────────────────────
  function handleStatusChange(value: string) {
    const newStatus = value as WishlistStatus;
    const prev = item.status;
    void saveField(
      { status: newStatus },
      () => {
        item = { ...item, status: newStatus };
      },
      () => {
        item = { ...item, status: prev };
      }
    );
  }

  // ── Price save ───────────────────────────────────────────────────────────
  function commitPrice() {
    priceError = null;

    const trimmedDisplay = priceDisplayValue.trim();

    // Check if invalid characters were filtered out
    if (priceHasInvalidInput) {
      priceError = m.wishlist_item_price_invalid_format();
      return;
    }

    // If empty, clear the price
    if (trimmedDisplay === '') {
      const prev = item.desiredPrice;
      void saveField(
        { desiredPriceAmount: null, desiredPriceCurrency: null },
        () => {
          item = { ...item, desiredPrice: null };
        },
        () => {
          item = { ...item, desiredPrice: prev };
        }
      );
      return;
    }

    // If parsed value is negative, show error
    if (priceInputCents !== null && priceInputCents < 0) {
      priceError = m.wishlist_item_price_negative();
      return;
    }

    // If value is null but we have content, it's an edge case - treat as clearing
    if (priceInputCents === null) {
      const prev = item.desiredPrice;
      void saveField(
        { desiredPriceAmount: null, desiredPriceCurrency: null },
        () => {
          item = { ...item, desiredPrice: null };
        },
        () => {
          item = { ...item, desiredPrice: prev };
        }
      );
      return;
    }

    const amountCents = priceInputCents;
    const currency = item.desiredPrice?.currency ?? defaultCurrency;
    const prev = item.desiredPrice;

    void saveField(
      { desiredPriceAmount: amountCents as unknown as bigint, desiredPriceCurrency: currency },
      () => {
        item = { ...item, desiredPrice: { amount: amountCents as unknown as bigint, currency } };
      },
      () => {
        item = { ...item, desiredPrice: prev };
      }
    );
  }

  function handlePriceKeydown(e: KeyboardEvent) {
    if (e.key === 'Enter') commitPrice();
    if (e.key === 'Escape') cancelField();
  }

  // ── Date save ────────────────────────────────────────────────────────────
  function handleDateSelect(date: CalendarDate | undefined) {
    if (!date) return;
    const iso = `${date.year}-${String(date.month).padStart(2, '0')}-${String(date.day).padStart(2, '0')}`;
    const prev = item.addedDate;
    void saveField(
      { addedDate: iso },
      () => {
        item = { ...item, addedDate: iso };
      },
      () => {
        item = { ...item, addedDate: prev };
      }
    );
  }

  // ── Derived Calendar DateValue for current added date ───────────────────
  const calendarValue = $derived.by(() => {
    const [y, mo, d] = item.addedDate.split('-').map(Number);
    return new CalendarDate(y, mo, d);
  });

  // ── Today as CalendarDate (max selectable) ───────────────────────────────
  const today = $derived.by(() => {
    const n = new Date();
    return new CalendarDate(n.getFullYear(), n.getMonth() + 1, n.getDate());
  });

  // ── Keyboard: Escape closes active field ─────────────────────────────────
  // Note: Calendar and Popover handle their own escape; this is for Price input.
  // Using CurrencyInput's onkeydown already handles Enter/Escape in some contexts,
  // but we can keep logic here if needed. Actually, simple cancelField is good.
</script>

{#snippet detailsSection()}
  <div class="overflow-hidden rounded-lg border border-border bg-card">
    <div class="border-b border-border px-4 py-3">
      <h3 class="text-xs font-semibold tracking-widest text-muted-foreground uppercase">
        {m.wishlist_item_section_details()}
      </h3>
    </div>

    <div class="space-y-3 p-4">
      <!-- List name -->
      <div class="space-y-1">
        <span class="text-[9px] font-medium tracking-wider text-muted-foreground uppercase">
          {m.wishlist_item_wishlist_name()}
        </span>
        <div class="text-xs font-semibold text-foreground">
          {wishlistName}
        </div>
      </div>

      <!-- Price & Added Row -->
      <div class="grid grid-cols-3 gap-x-3 border-t border-border pt-3">
        <!-- Desired Price -->
        <div class="flex flex-col gap-0.5">
          <span class="text-[9px] font-medium tracking-wider text-muted-foreground uppercase">
            {m.wishlist_field_desired_price()}
          </span>
          {#if activeField === 'desiredPrice'}
            <div class="mt-1">
              <CurrencyInput
                bind:value={priceInputCents}
                bind:displayValue={priceDisplayValue}
                bind:hasInvalidInput={priceHasInvalidInput}
                symbol={regionalManager.getCurrencySymbol(
                  item.desiredPrice?.currency ?? defaultCurrency
                )}
                onblur={commitPrice}
                onkeydown={handlePriceKeydown}
                placeholder="0.00"
                class="w-full"
                inputClass="border-primary ring-primary/30"
                label={m.wishlist_field_desired_price()}
                autofocus
              />
              {#if priceError}
                <p class="mt-1 text-[10px] text-red-400" role="alert">{priceError}</p>
              {/if}
            </div>
          {:else}
            <button
              type="button"
              class="-mx-1 flex h-6 w-full cursor-pointer items-center rounded border-0 bg-transparent px-1 text-left transition-colors duration-150 hover:border hover:border-dashed hover:border-primary/40 hover:bg-primary/15"
              onclick={activateDesiredPrice}
              aria-label={m.wishlist_item_edit_field_label({
                field: m.wishlist_field_desired_price()
              })}
            >
              {#if item.desiredPrice}
                <span class="text-xs font-semibold text-foreground">
                  {formatPrice(item.desiredPrice.amount, item.desiredPrice.currency)}
                </span>
              {:else}
                <span class="text-xs text-muted-foreground italic"
                  >{m.wishlist_item_price_not_set()}</span
                >
              {/if}
            </button>
          {/if}
        </div>

        <!-- Empty slot -->
        <div></div>

        <!-- Added Date -->
        <div class="flex flex-col gap-0.5">
          <span class="text-[9px] font-medium tracking-wider text-muted-foreground uppercase">
            {m.wishlist_item_added_date()}
          </span>
          <Popover.Root
            bind:open={datePopoverOpen}
            onOpenChange={(open) => {
              if (!open) cancelField();
            }}
          >
            <Popover.Trigger
              class="group -mx-1 flex h-6 cursor-pointer items-center justify-start rounded px-1 text-left transition-colors duration-150 outline-none hover:border hover:border-dashed hover:border-primary/40 hover:bg-primary/15"
              aria-label={m.wishlist_item_edit_field_label({
                field: m.wishlist_item_added_date()
              })}
            >
              <span class="text-xs font-medium text-foreground">
                {formatDate(item.addedDate)}
              </span>
            </Popover.Trigger>
            <Popover.Content class="w-auto border-border bg-card p-0" align="end">
              <Calendar
                type="single"
                value={calendarValue}
                maxValue={today}
                onValueChange={(v) => handleDateSelect(v as CalendarDate | undefined)}
              />
            </Popover.Content>
          </Popover.Root>
        </div>
      </div>

      <!-- Purchased Price (read-only, only shown when non-null) -->
      {#if item.purchasedPrice}
        <div class="flex flex-col gap-0.5 border-t border-border pt-3">
          <span class="text-[9px] font-medium tracking-wider text-muted-foreground uppercase">
            {m.wishlist_item_purchased_price()}
          </span>
          <span class="text-xs font-semibold text-foreground">
            {formatPrice(item.purchasedPrice.amount, item.purchasedPrice.currency)}
          </span>
        </div>
      {/if}
    </div>
  </div>
{/snippet}

{#snippet personalContextSection()}
  <div class="overflow-hidden rounded-lg border border-border bg-card">
    <div class="border-b border-border px-4 py-3">
      <h3 class="text-xs font-semibold tracking-widest text-muted-foreground uppercase">
        {m.wishlist_item_section_personal_context()}
      </h3>
    </div>

    <div class="space-y-3 p-4">
      <!-- Priority & Status Row -->
      <div class="grid grid-cols-3 gap-x-3">
        <!-- Priority -->
        <div class="flex flex-col gap-0.5">
          <span class="text-[9px] font-medium tracking-wider text-muted-foreground uppercase">
            {m.wishlist_field_priority()}
          </span>
          <InPlaceSelectEdit
            value={item.priority}
            displayLabel={priorityLabel(item.priority)}
            ariaLabel={m.wishlist_item_edit_field_label({ field: m.wishlist_field_priority() })}
            options={[
              { value: 'LOW', label: m.wishlist_priority_low() },
              { value: 'NORMAL', label: m.wishlist_priority_normal() },
              { value: 'HIGH', label: m.wishlist_priority_high() }
            ]}
            onSave={async (v: string) => handlePriorityChange(v)}
          />
        </div>

        <!-- Status -->
        <div class="flex flex-col gap-0.5">
          <span class="text-[9px] font-medium tracking-wider text-muted-foreground uppercase">
            {m.wishlist_item_status()}
          </span>
          <InPlaceSelectEdit
            value={item.status}
            displayLabel={statusLabel(item.status)}
            ariaLabel={m.wishlist_item_edit_field_label({ field: m.wishlist_item_status() })}
            options={[
              { value: 'WANTED', label: m.wishlist_item_status_wanted() },
              { value: 'ON_ORDER', label: m.wishlist_item_status_on_order() },
              { value: 'PURCHASED', label: m.wishlist_item_status_purchased() },
              { value: 'IGNORED', label: m.wishlist_item_status_ignored() }
            ]}
            onSave={async (v: string) => handleStatusChange(v)}
          />
        </div>
      </div>

      <!-- Notes -->
      {#if item.notes}
        <div class="space-y-1 border-t border-border pt-3">
          <span class="text-[9px] font-medium tracking-wider text-muted-foreground uppercase">
            {m.wishlist_item_notes()}
          </span>
          <p class="line-clamp-6 text-xs leading-relaxed text-foreground">
            {item.notes}
          </p>
        </div>
      {/if}
    </div>
  </div>
{/snippet}

<aside class="w-full shrink-0 space-y-4 lg:w-80">
  {@render detailsSection()}
  {@render personalContextSection()}
</aside>
