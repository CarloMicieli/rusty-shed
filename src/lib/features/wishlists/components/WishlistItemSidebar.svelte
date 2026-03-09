<script lang="ts">
  import * as m from '$lib/paraglide/messages.js';
  import * as Select from '$lib/components/ui/select';
  import * as Popover from '$lib/components/ui/popover';
  import { Calendar } from '$lib/components/ui/calendar';
  import { commands } from '$lib/bindings';
  import type { Currency, WishlistItem, WishlistPriority, WishlistStatus } from '$lib/bindings';
  import { toaster } from '$lib/toaster';
  import { CalendarDate } from '@internationalized/date';
  import { Pencil } from 'lucide-svelte';

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
  type EditableField = 'desiredPrice' | 'addedDate';
  let activeField = $state<EditableField | null>(null);

  // ── Select open states ───────────────────────────────────────────────────
  let prioritySelectOpen = $state(false);
  let statusSelectOpen = $state(false);

  // ── Price edit state ─────────────────────────────────────────────────────
  let priceInputValue = $state('');
  let priceError = $state<string | null>(null);

  // ── Date popover state ───────────────────────────────────────────────────
  let datePopoverOpen = $state(false);

  // ── Date formatter ──────────────────────────────────────────────────────
  const dateFormatter = new Intl.DateTimeFormat(undefined, { dateStyle: 'medium' });

  function formatDate(iso: string): string {
    // Parse as local date to avoid UTC midnight offset
    const [y, mo, d] = iso.split('-').map(Number);
    return dateFormatter.format(new Date(y, mo - 1, d));
  }

  // ── Price formatter ─────────────────────────────────────────────────────
  function formatPrice(amount: bigint, currency: string): string {
    return new Intl.NumberFormat(undefined, {
      style: 'currency',
      currency
    }).format(Number(amount) / 100);
  }

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
    priceInputValue = item.desiredPrice ? String(Number(item.desiredPrice.amount) / 100) : '';
  }

  function activateAddedDate() {
    activeField = 'addedDate';
    datePopoverOpen = true;
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
    const raw = priceInputValue.trim();

    if (raw === '') {
      // Clear desired price
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

    const parsed = parseFloat(raw);
    if (isNaN(parsed)) {
      priceError = m.wishlist_item_price_invalid_format();
      return;
    }
    if (parsed < 0) {
      priceError = m.wishlist_item_price_negative();
      return;
    }

    const amountCents = BigInt(Math.round(parsed * 100));
    const currency = item.desiredPrice?.currency ?? defaultCurrency;
    const prev = item.desiredPrice;

    void saveField(
      { desiredPriceAmount: amountCents, desiredPriceCurrency: currency },
      () => {
        item = { ...item, desiredPrice: { amount: amountCents, currency } };
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
  function handleGlobalKeydown(e: KeyboardEvent) {
    if (e.key === 'Escape' && activeField) {
      cancelField();
    }
  }
</script>

<svelte:document onkeydown={handleGlobalKeydown} />

<aside class="w-full shrink-0 space-y-4 lg:w-80">
  <!-- ── Wish List Details Section ─────────────────────────────────────────── -->
  <section class="rounded-lg border border-white/10 bg-black/20 p-4">
    <h2 class="mb-3 text-xs font-semibold tracking-widest text-muted-foreground uppercase">
      {m.wishlist_item_section_details()}
    </h2>

    <dl class="space-y-2 text-sm">
      <!-- List name (read-only — no hover affordance, no edit handler) -->
      <div class="flex justify-between gap-2">
        <dt class="shrink-0 text-muted-foreground">{m.wishlist_item_wishlist_name()}</dt>
        <dd class="text-right font-medium">{wishlistName}</dd>
      </div>

      <!-- Priority (inline select — badge IS the trigger) -->
      <div class="flex items-center justify-between gap-2">
        <dt class="shrink-0 text-muted-foreground">{m.wishlist_field_priority()}</dt>
        <dd>
          <Select.Root
            type="single"
            value={item.priority}
            bind:open={prioritySelectOpen}
            onValueChange={handlePriorityChange}
          >
            <Select.Trigger
              class="group flex h-6 cursor-pointer items-center gap-1 rounded-full border-none bg-muted px-2 py-0.5 text-xs font-medium shadow-none hover:bg-muted/70"
              aria-label={m.wishlist_item_edit_field_label({ field: m.wishlist_field_priority() })}
            >
              {priorityLabel(item.priority)}
              <Pencil class="h-3 w-3 opacity-0 transition-opacity group-hover:opacity-60" />
            </Select.Trigger>
            <Select.Content>
              <Select.Item value="LOW" label={m.wishlist_priority_low()} />
              <Select.Item value="NORMAL" label={m.wishlist_priority_normal()} />
              <Select.Item value="HIGH" label={m.wishlist_priority_high()} />
            </Select.Content>
          </Select.Root>
        </dd>
      </div>

      <!-- Status (inline select — badge IS the trigger) -->
      <div class="flex items-center justify-between gap-2">
        <dt class="shrink-0 text-muted-foreground">{m.wishlist_item_status()}</dt>
        <dd>
          <Select.Root
            type="single"
            value={item.status}
            bind:open={statusSelectOpen}
            onValueChange={handleStatusChange}
          >
            <Select.Trigger
              class="group flex h-6 cursor-pointer items-center gap-1 rounded-full border-none bg-muted px-2 py-0.5 text-xs font-medium shadow-none hover:bg-muted/70"
              aria-label={m.wishlist_item_edit_field_label({ field: m.wishlist_item_status() })}
            >
              {statusLabel(item.status)}
              <Pencil class="h-3 w-3 opacity-0 transition-opacity group-hover:opacity-60" />
            </Select.Trigger>
            <Select.Content>
              <Select.Item value="WANTED" label={m.wishlist_item_status_wanted()} />
              <Select.Item value="ON_ORDER" label={m.wishlist_item_status_on_order()} />
              <Select.Item value="PURCHASED" label={m.wishlist_item_status_purchased()} />
              <Select.Item value="IGNORED" label={m.wishlist_item_status_ignored()} />
            </Select.Content>
          </Select.Root>
        </dd>
      </div>

      <!-- Desired Price (inline input) -->
      <div class="flex items-start justify-between gap-2">
        <dt class="shrink-0 text-muted-foreground">{m.wishlist_field_desired_price()}</dt>
        <dd class="text-right">
          {#if activeField === 'desiredPrice'}
            <div class="flex flex-col items-end gap-1">
              <div class="flex items-center gap-1">
                <input
                  type="text"
                  inputmode="decimal"
                  class="w-24 rounded border border-border bg-background px-2 py-0.5 text-right text-xs font-medium focus:ring-1 focus:ring-primary focus:outline-none"
                  bind:value={priceInputValue}
                  onkeydown={handlePriceKeydown}
                  onblur={commitPrice}
                  placeholder="0.00"
                  aria-label={m.wishlist_item_edit_field_label({
                    field: m.wishlist_field_desired_price()
                  })}
                />
                <span class="text-xs text-muted-foreground">
                  {item.desiredPrice?.currency ?? defaultCurrency}
                </span>
              </div>
              {#if priceError}
                <p class="text-xs text-destructive">{priceError}</p>
              {/if}
            </div>
          {:else}
            <button
              class="group flex cursor-pointer items-center gap-1 font-medium hover:opacity-70"
              onclick={activateDesiredPrice}
              aria-label={m.wishlist_item_edit_field_label({
                field: m.wishlist_field_desired_price()
              })}
            >
              {#if item.desiredPrice}
                {formatPrice(item.desiredPrice.amount, item.desiredPrice.currency)}
              {:else}
                <span class="text-muted-foreground italic">{m.wishlist_item_price_not_set()}</span>
              {/if}
              <Pencil class="h-3 w-3 opacity-0 transition-opacity group-hover:opacity-60" />
            </button>
          {/if}
        </dd>
      </div>

      <!-- Purchased Price (read-only, only shown when non-null) -->
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
  <section class="rounded-lg border border-white/10 bg-black/20 p-4">
    <h2 class="mb-3 text-xs font-semibold tracking-widest text-muted-foreground uppercase">
      {m.wishlist_item_section_personal_context()}
    </h2>

    <dl class="space-y-2 text-sm">
      <!-- Added date (inline calendar popover) -->
      <div class="flex items-center justify-between gap-2">
        <dt class="shrink-0 text-muted-foreground">{m.wishlist_item_added_date()}</dt>
        <dd>
          <Popover.Root
            bind:open={datePopoverOpen}
            onOpenChange={(open) => {
              if (!open) cancelField();
            }}
          >
            <Popover.Trigger>
              <button
                class="group flex cursor-pointer items-center gap-1 font-medium hover:opacity-70"
                onclick={activateAddedDate}
                aria-label={m.wishlist_item_edit_field_label({
                  field: m.wishlist_item_added_date()
                })}
              >
                {formatDate(item.addedDate)}
                <Pencil class="h-3 w-3 opacity-0 transition-opacity group-hover:opacity-60" />
              </button>
            </Popover.Trigger>
            <Popover.Content class="w-auto p-0" align="end">
              <Calendar
                type="single"
                value={calendarValue}
                maxValue={today}
                onValueChange={(v) => handleDateSelect(v as CalendarDate | undefined)}
              />
            </Popover.Content>
          </Popover.Root>
        </dd>
      </div>

      <!-- Notes (read-only if present) -->
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
</aside>
