<script lang="ts">
  import type { DigitalRollingStockView } from '$lib/bindings';
  import * as m from '$lib/paraglide/messages';
  import { AlertTriangle } from 'lucide-svelte';
  import { Button, Input } from '$lib/components';

  interface Props {
    open: boolean;
    stock: DigitalRollingStockView;
    onSave: (newAddress: number) => Promise<boolean>;
    onCheckDuplicate: (
      address: number,
      excludeId: string
    ) => Promise<{ isDuplicate: boolean; existingId?: string }>;
    onClose: () => void;
  }

  let { open, stock, onSave, onCheckDuplicate, onClose }: Props = $props();

  let addressOverride = $state<number | null>(null);
  let previousAddress: number | null = null;
  let newAddress = $derived(addressOverride ?? stock.dcc_address);
  let isDuplicateWarning = $state(false);
  let isSaving = $state(false);
  let validationError = $state<string | null>(null);

  $effect(() => {
    if (previousAddress === null) {
      previousAddress = stock.dcc_address;
      return;
    }

    if (stock.dcc_address !== previousAddress) {
      addressOverride = null;
      previousAddress = stock.dcc_address;
    }
  });

  async function handleAddressChange(value: number) {
    addressOverride = value;
    validationError = null;
    isDuplicateWarning = false;

    // Validate range
    if (value < 1 || value > 9999) {
      validationError = m.digital_roster_address_invalid_range();
      return;
    }

    // Check for duplicates if address changed
    if (value !== stock.dcc_address) {
      const result = await onCheckDuplicate(value, stock.id);
      if (result.isDuplicate) {
        isDuplicateWarning = true;
      }
    }
  }

  async function handleSubmit() {
    if (validationError) return;
    if (newAddress === stock.dcc_address) {
      onClose();
      return;
    }

    isSaving = true;
    const success = await onSave(newAddress);
    isSaving = false;

    if (success) {
      onClose();
    }
  }

  function handleClose() {
    onClose();
  }
</script>

{#if open}
  <div
    class="fixed inset-0 z-50 flex items-center justify-center bg-black/60"
    role="presentation"
    tabindex="-1"
    onclick={handleClose}
    onkeydown={(event) => event.key === 'Escape' && handleClose()}
  >
    <div
      class="w-full max-w-md space-y-4 rounded-xl border border-border/70 bg-card p-6"
      role="dialog"
      aria-modal="true"
      tabindex="-1"
      onclick={(event) => event.stopPropagation()}
      onkeydown={(event) => {
        if (event.key === 'Escape') {
          event.stopPropagation();
          handleClose();
        }
      }}
    >
      <h3 class="h3 font-bold">{m.digital_roster_edit_address_title()}</h3>

      <div class="space-y-2">
        <p class="text-sm opacity-75">
          {m.digital_roster_edit_address_subtitle({
            roadNumber: stock.road_number || '-',
            railway: stock.railway_company_name || '-'
          })}
        </p>
      </div>

      <div class="space-y-2">
        <label for="dcc-address" class="label">
          <span>{m.digital_roster_table_address()}</span>
        </label>
        <Input
          id="dcc-address"
          type="number"
          value={String(newAddress)}
          oninput={(e) => handleAddressChange(parseInt((e.target as HTMLInputElement).value))}
          min="1"
          max="9999"
          placeholder="1-9999"
        />

        {#if validationError}
          <p class="text-error-500 text-sm">{validationError}</p>
        {/if}

        {#if isDuplicateWarning}
          <div class="border-warning-500/40 bg-warning-500/10 rounded-md border p-3">
            <div class="flex items-start gap-2">
              <AlertTriangle class="mt-0.5 h-5 w-5 flex-shrink-0" />
              <div>
                <p class="font-semibold">{m.digital_roster_duplicate_address_warning()}</p>
                <p class="text-sm opacity-90">{m.digital_roster_duplicate_address_message()}</p>
              </div>
            </div>
          </div>
        {/if}
      </div>

      <div class="flex justify-end gap-2">
        <Button type="button" variant="ghost" onclick={handleClose} disabled={isSaving}>
          {m.app_cancel()}
        </Button>
        <Button
          type="button"
          variant="default"
          onclick={handleSubmit}
          disabled={isSaving || !!validationError}
        >
          {#if isSaving}
            <span class="animate-pulse">{m.app_loading()}</span>
          {:else}
            {m.digital_roster_edit_address_save()}
          {/if}
        </Button>
      </div>
    </div>
  </div>
{/if}
