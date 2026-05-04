<script lang="ts">
  import * as Dialog from '$lib/components/ui/dialog';
  import { Button, CurrencyInput, Input, DatePickerField } from '$lib/components';
  import { commands } from '$lib/bindings';
  import * as m from '$lib/paraglide/messages.js';
  import { regionalManager } from '$lib/features/settings/RegionalManager.svelte';

  interface Props {
    open: boolean;
    itemId: string;
    defaultCurrency?: string;
    onClose: () => void;
    onSold?: () => Promise<void>;
  }

  let { open, itemId, defaultCurrency = 'EUR', onClose, onSold }: Props = $props();

  let saleDate = $state<string | null>(new Date().toISOString().split('T')[0]);
  let amountCents = $state<number | null>(null);
  let currency = $state('EUR');
  let buyerId = $state('');
  let isSubmitting = $state(false);
  let errorMessage = $state<string | null>(null);

  $effect(() => {
    if (!open) {
      currency = defaultCurrency;
    }
  });

  function handleOpenChange(nextOpen: boolean) {
    if (!nextOpen) {
      resetForm();
      onClose();
    }
  }

  function resetForm() {
    saleDate = new Date().toISOString().split('T')[0];
    amountCents = null;
    currency = defaultCurrency;
    buyerId = '';
    errorMessage = null;
    isSubmitting = false;
  }

  async function submitSale() {
    if (isSubmitting) return;

    if (!saleDate) {
      errorMessage = m.collection_item_sell_error_date_required();
      return;
    }

    if (amountCents === null) {
      errorMessage = m.collection_item_sell_error_amount_required();
      return;
    }

    errorMessage = null;
    isSubmitting = true;

    try {
      const result = await commands.sellCollectionItem({
        itemId,
        saleDate,
        amount: amountCents,
        currency,
        buyerId: buyerId.trim().length > 0 ? buyerId.trim() : null
      });

      if (result.status === 'error') {
        throw new Error('sell_failed');
      }

      if (onSold) {
        await onSold();
      }

      resetForm();
      onClose();
    } catch {
      errorMessage = m.collection_item_sell_error_generic();
    } finally {
      isSubmitting = false;
    }
  }
</script>

<Dialog.Root {open} onOpenChange={handleOpenChange}>
  <Dialog.Content aria-labelledby="sell-dialog-title" aria-describedby="sell-dialog-description">
    <div class="space-y-4 p-6">
      <div class="space-y-1">
        <h2 id="sell-dialog-title" class="text-lg font-semibold">
          {m.collection_item_sell_dialog_title()}
        </h2>
        <p id="sell-dialog-description" class="text-sm text-muted-foreground">
          {m.collection_item_sell_dialog_description()}
        </p>
      </div>

      <div class="space-y-3">
        <div class="space-y-1">
          <p class="text-xs font-medium tracking-wide text-muted-foreground uppercase">
            {m.collection_item_sale_date()}
          </p>
          <DatePickerField value={saleDate} onSelect={(iso) => (saleDate = iso)} placeholder="—" />
        </div>

        <div class="space-y-1">
          <p class="text-xs font-medium tracking-wide text-muted-foreground uppercase">
            {m.collection_item_sale_price()}
          </p>
          <CurrencyInput
            bind:value={amountCents}
            symbol={regionalManager.getCurrencySymbol(currency)}
            disabled={isSubmitting}
            class="w-full"
            label={m.collection_item_sale_price()}
          />
        </div>

        <div class="space-y-1">
          <p class="text-xs font-medium tracking-wide text-muted-foreground uppercase">
            {m.collection_item_sale_buyer_id()}
          </p>
          <Input
            bind:value={buyerId}
            disabled={isSubmitting}
            placeholder={m.collection_item_sale_buyer_placeholder()}
          />
        </div>
      </div>

      {#if errorMessage}
        <p class="text-xs text-destructive" role="alert">{errorMessage}</p>
      {/if}

      <div class="flex justify-end gap-2">
        <Button variant="ghost" onclick={onClose} disabled={isSubmitting}>
          {m.collection_item_sell_dialog_cancel()}
        </Button>
        <Button variant="rusty" onclick={submitSale} disabled={isSubmitting}>
          {m.collection_item_sell_dialog_submit()}
        </Button>
      </div>
    </div>
  </Dialog.Content>
</Dialog.Root>
