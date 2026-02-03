<script lang="ts">
  import type { DigitalRollingStockView } from '$lib/bindings';
  import * as m from '$lib/paraglide/messages';
  import { AlertTriangle } from 'lucide-svelte';
  import { untrack } from 'svelte';

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

  // eslint-disable-next-line svelte/prefer-writable-derived
  let localAddress = $state(untrack(() => stock.dcc_address));
  let newAddress = $derived(localAddress);
  let isDuplicateWarning = $state(false);
  let isSaving = $state(false);
  let validationError = $state<string | null>(null);

  $effect(() => {
    localAddress = stock.dcc_address;
  });

  async function handleAddressChange(value: number) {
    localAddress = value;
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
      class="w-full max-w-md space-y-4 rounded-xl border border-surface-700/70 bg-surface-900 p-6"
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
        <input
          id="dcc-address"
          type="number"
          class="input"
          class:input-error={validationError}
          class:input-warning={isDuplicateWarning}
          bind:value={newAddress}
          oninput={(e) => handleAddressChange(parseInt((e.target as HTMLInputElement).value))}
          min="1"
          max="9999"
          placeholder="1-9999"
        />

        {#if validationError}
          <p class="text-sm text-error-500">{validationError}</p>
        {/if}

        {#if isDuplicateWarning}
          <div class="alert variant-ghost-warning">
            <div class="alert-message">
              <div class="flex items-start gap-2">
                <AlertTriangle class="mt-0.5 h-5 w-5 flex-shrink-0" />
                <div>
                  <p class="font-semibold">{m.digital_roster_duplicate_address_warning()}</p>
                  <p class="text-sm opacity-90">{m.digital_roster_duplicate_address_message()}</p>
                </div>
              </div>
            </div>
          </div>
        {/if}
      </div>

      <div class="flex justify-end gap-2">
        <button type="button" class="variant-ghost btn" onclick={handleClose} disabled={isSaving}>
          {m.app_cancel()}
        </button>
        <button
          type="button"
          class="variant-filled-primary btn"
          onclick={handleSubmit}
          disabled={isSaving || !!validationError}
        >
          {#if isSaving}
            <span class="animate-pulse">{m.app_loading()}</span>
          {:else}
            {m.digital_roster_edit_address_save()}
          {/if}
        </button>
      </div>
    </div>
  </div>
{/if}
