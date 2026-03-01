<script lang="ts">
  import * as m from '$lib/paraglide/messages.js';
  import { X } from 'lucide-svelte';
  import { commands } from '$lib/bindings';
  import type {
    Decoder,
    InstallableRollingStockView,
    Manufacturer,
    DigitalRollingStockView
  } from '$lib/bindings';
  import { getDigitalRosterContext } from '../DigitalRosterState.svelte';
  import { Button, Input } from '$lib/components';

  interface Props {
    /** Controls drawer visibility */
    open: boolean;
    /** Callback when drawer requests close */
    onClose: () => void;
    /** Callback when decoder is successfully installed */
    onSuccess: () => void;
  }

  let { open, onClose, onSuccess }: Props = $props();

  const controller = getDigitalRosterContext();

  // Form state
  let selectedRollingStockId = $state<string | null>(null);
  let selectedDecoderId = $state<string | null>(null);
  let dccAddress = $state<number | null>(null);
  let installationDate = $state<string>(new Date().toISOString().split('T')[0]);

  // Reference data
  let installableRollingStocks = $state<InstallableRollingStockView[]>([]);
  let decoders = $state<Decoder[]>([]);
  let manufacturers = $state<Manufacturer[]>([]);

  // UI state
  let isLoadingData = $state(false);
  let isSubmitting = $state(false);
  let showConfirmDialog = $state(false);
  let showDiscardDialog = $state(false);
  let duplicateWarning = $state<string | null>(null);

  // Validation
  let validationErrors = $state<{
    rollingStock?: string;
    decoder?: string;
    address?: string;
  }>({});
  let touched = $state(false);

  // Derived state
  let selectedRollingStock = $derived(
    installableRollingStocks.find((rs) => rs.owned_rolling_stock_id === selectedRollingStockId) ??
      null
  );

  let hasChanges = $derived(
    selectedRollingStockId !== null ||
      selectedDecoderId !== null ||
      dccAddress !== null ||
      installationDate !== new Date().toISOString().split('T')[0]
  );

  let isFormValid = $derived.by(() => {
    if (!touched) return false;

    const errors = validateForm();
    validationErrors = errors;
    return Object.keys(errors).length === 0 && !duplicateWarning;
  });

  let existingDigitalRollingStock = $derived.by((): DigitalRollingStockView | null => {
    if (!selectedRollingStock || !controller.state.rollingStocks) {
      return null;
    }

    return (
      controller.state.rollingStocks.find(
        (drs) => drs.owned_rolling_stock_id === selectedRollingStockId
      ) ?? null
    );
  });

  // Watch for drawer open
  $effect(() => {
    if (open) {
      handleOpen();
    }
  });

  // Watch for rolling stock changes to check for existing decoder
  $effect(() => {
    if (selectedRollingStock?.has_decoder && existingDigitalRollingStock) {
      // Pre-fill the address from existing decoder
      dccAddress = existingDigitalRollingStock.dcc_address;
    }
  });

  // Watch for address changes to check for duplicates
  $effect(() => {
    if (dccAddress !== null && dccAddress >= 1 && dccAddress <= 9999) {
      checkDuplicateAddress();
    } else {
      duplicateWarning = null;
    }
  });

  async function handleOpen() {
    resetForm();
    await loadReferenceData();
  }

  function resetForm() {
    selectedRollingStockId = null;
    selectedDecoderId = null;
    dccAddress = null;
    installationDate = new Date().toISOString().split('T')[0];
    touched = false;
    validationErrors = {};
    duplicateWarning = null;
    showConfirmDialog = false;
    showDiscardDialog = false;
  }

  async function loadReferenceData() {
    try {
      isLoadingData = true;

      const [installableResult, decodersResult, manufacturersResult] = await Promise.all([
        commands.getInstallableRollingStocks(),
        commands.getDecoders(),
        commands.getManufacturers()
      ]);

      if (installableResult.status === 'ok') {
        installableRollingStocks = installableResult.data;
      }

      if (decodersResult.status === 'ok') {
        // Filter out Function decoders
        decoders = decodersResult.data.filter((d) => d.decoderType !== 'FUNCTION');
      }

      if (manufacturersResult.status === 'ok') {
        manufacturers = manufacturersResult.data;
      }
    } catch (error) {
      console.error('Error loading reference data:', error);
    } finally {
      isLoadingData = false;
    }
  }

  function validateForm() {
    const errors: typeof validationErrors = {};

    if (!selectedRollingStockId) {
      errors.rollingStock = m.digital_roster_validation_rolling_stock();
    }

    if (!selectedDecoderId) {
      errors.decoder = m.digital_roster_validation_decoder();
    }

    if (dccAddress === null || dccAddress < 1 || dccAddress > 9999) {
      errors.address = m.digital_roster_address_range();
    }

    return errors;
  }

  async function checkDuplicateAddress() {
    if (!dccAddress) {
      duplicateWarning = null;
      return;
    }

    const excludeId = existingDigitalRollingStock?.id ?? null;
    const result = await controller.checkDuplicateAddress(dccAddress, excludeId);

    if (result.isDuplicate) {
      duplicateWarning = m.digital_roster_duplicate_warning({ address: dccAddress.toString() });
    } else {
      duplicateWarning = null;
    }
  }

  function handleCloseRequest() {
    if (hasChanges) {
      showDiscardDialog = true;
    } else {
      onClose();
    }
  }

  async function handleSubmit() {
    touched = true;

    if (!isFormValid) {
      return;
    }

    // Check if we need to show replacement confirmation
    if (selectedRollingStock?.has_decoder && !showConfirmDialog) {
      showConfirmDialog = true;
      return;
    }

    await performInstallation();
  }

  async function performInstallation() {
    if (!selectedRollingStockId || !selectedDecoderId || dccAddress === null) {
      return;
    }

    try {
      isSubmitting = true;

      let success: boolean;

      if (existingDigitalRollingStock) {
        // Replace existing decoder
        success = await controller.replaceDecoder(
          existingDigitalRollingStock.id,
          selectedDecoderId
        );

        // Also update address if it changed
        if (success && dccAddress !== existingDigitalRollingStock.dcc_address) {
          success = await controller.changeDccAddress(existingDigitalRollingStock.id, dccAddress);
        }
      } else {
        // Install new decoder
        success = await controller.installDecoder(
          selectedRollingStockId,
          selectedDecoderId,
          dccAddress
        );
      }

      if (success) {
        onSuccess();
        onClose();
      }
    } catch (error) {
      console.error('Error installing decoder:', error);
    } finally {
      isSubmitting = false;
      showConfirmDialog = false;
    }
  }

  function handleConfirmReplace() {
    showConfirmDialog = false;
    performInstallation();
  }

  function handleCancelReplace() {
    showConfirmDialog = false;
  }

  function handleDiscardConfirm() {
    showDiscardDialog = false;
    onClose();
  }

  function handleDiscardCancel() {
    showDiscardDialog = false;
  }

  function getManufacturerName(manufacturerId: string): string {
    return manufacturers.find((m) => m.id === manufacturerId)?.name ?? manufacturerId;
  }

  function formatRollingStockLabel(rs: InstallableRollingStockView): string {
    const parts = [];
    if (rs.series_code) parts.push(rs.series_code);
    if (rs.road_number) parts.push(rs.road_number);
    if (rs.railway_company_name) parts.push(`(${rs.railway_company_name})`);
    return parts.join(' ') || rs.owned_rolling_stock_id;
  }

  function formatDecoderLabel(decoder: Decoder): string {
    const manufacturer = getManufacturerName(decoder.manufacturerId);
    return `${manufacturer} ${decoder.productCode} (${decoder.decoderType})`;
  }
</script>

<!-- Drawer Overlay -->
{#if open}
  <div
    class="fixed inset-0 z-50 bg-black/50"
    onclick={handleCloseRequest}
    role="presentation"
  ></div>
{/if}

<!-- Drawer Container -->
<div
  class="fixed top-0 right-0 z-50 h-full w-full max-w-2xl transform transition-transform duration-300 ease-in-out"
  class:translate-x-0={open}
  class:translate-x-full={!open}
  role="dialog"
  aria-modal="true"
  aria-labelledby="drawer-title"
>
  <div class="flex h-full flex-col overflow-y-auto border-l border-border/60 bg-card shadow-2xl">
    <!-- Header -->
    <div class="flex items-center justify-between border-b border-border/60 p-6">
      <div>
        <p class="text-xs tracking-[0.2em] text-muted-foreground uppercase">
          {m.app_digital_roster()}
        </p>
        <h2 id="drawer-title" class="text-xl font-semibold">
          {m.digital_roster_install_decoder()}
        </h2>
      </div>
      <Button
        type="button"
        variant="ghost"
        size="icon"
        class="h-8 w-8"
        onclick={handleCloseRequest}
        aria-label={m.add_model_cancel()}
      >
        <X size={16} />
      </Button>
    </div>

    <!-- Content (scrollable) -->
    <div class="flex-1 overflow-y-auto p-6">
      {#if isLoadingData}
        <div class="flex items-center justify-center py-8">
          <div class="border-primary-500 h-12 w-12 animate-spin rounded-full border-b-2"></div>
        </div>
      {:else}
        <form id="install-decoder-form" class="space-y-6" onsubmit={(e) => e.preventDefault()}>
          <!-- Rolling Stock Selection -->
          <div>
            <label for="rolling-stock" class="block space-y-1">
              <span class="text-sm text-muted-foreground"
                >{m.digital_roster_rolling_stock_label()}</span
              >
            </label>
            <select
              id="rolling-stock"
              bind:value={selectedRollingStockId}
              class="h-9 w-full rounded-md border border-input bg-background px-3 py-2 text-sm transition-colors outline-none focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/40"
              class:input-error={touched && validationErrors.rollingStock}
            >
              <option value={null}>{m.form_new_model_select_placeholder()}</option>
              {#each installableRollingStocks as rs (rs.owned_rolling_stock_id)}
                <option value={rs.owned_rolling_stock_id}>
                  {formatRollingStockLabel(rs)}
                  {#if rs.has_decoder}
                    ({m.digital_roster_has_decoder()})
                  {/if}
                </option>
              {/each}
            </select>
            {#if touched && validationErrors.rollingStock}
              <p class="text-error-500 mt-1 text-xs">{validationErrors.rollingStock}</p>
            {/if}
          </div>

          <!-- Decoder Selection -->
          <div>
            <label for="decoder" class="block space-y-1">
              <span class="text-sm text-muted-foreground">{m.digital_roster_decoder_label()}</span>
            </label>
            <select
              id="decoder"
              bind:value={selectedDecoderId}
              class="h-9 w-full rounded-md border border-input bg-background px-3 py-2 text-sm transition-colors outline-none focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/40"
              class:input-error={touched && validationErrors.decoder}
            >
              <option value={null}>{m.form_new_model_select_placeholder()}</option>
              {#each decoders as decoder (decoder.id)}
                <option value={decoder.id}>
                  {formatDecoderLabel(decoder)}
                </option>
              {/each}
            </select>
            {#if touched && validationErrors.decoder}
              <p class="text-error-500 mt-1 text-xs">{validationErrors.decoder}</p>
            {/if}
          </div>

          <!-- DCC Address -->
          <div>
            <label for="dcc-address" class="block space-y-1">
              <span class="text-sm text-muted-foreground">{m.digital_roster_address_label()}</span>
            </label>
            <Input
              id="dcc-address"
              type="number"
              min="1"
              max="9999"
              value={dccAddress ? String(dccAddress) : ''}
              oninput={(e) => (dccAddress = parseInt(e.currentTarget.value) || null)}
              placeholder="1-9999"
            />
            {#if touched && validationErrors.address}
              <p class="text-error-500 mt-1 text-xs">{validationErrors.address}</p>
            {/if}
            {#if duplicateWarning}
              <p class="text-warning-500 mt-1 text-xs">{duplicateWarning}</p>
            {/if}
          </div>

          <!-- Installation Date (for future use, not currently stored) -->
          <div>
            <label for="installation-date" class="block space-y-1">
              <span class="text-sm text-muted-foreground">{m.digital_roster_date_label()}</span>
            </label>
            <Input
              id="installation-date"
              type="date"
              value={installationDate}
              oninput={(e) => (installationDate = e.currentTarget.value)}
            />
          </div>
        </form>
      {/if}
    </div>

    <!-- Footer -->
    <div class="border-t border-border/60 p-6">
      <div class="flex justify-end gap-3">
        <Button type="button" variant="ghost" onclick={handleCloseRequest} disabled={isSubmitting}>
          {m.add_model_cancel()}
        </Button>
        <Button
          type="submit"
          variant="default"
          onclick={handleSubmit}
          disabled={isSubmitting || isLoadingData || (!touched && !isFormValid)}
        >
          {#if isSubmitting}
            {m.app_loading()}
          {:else}
            {m.digital_roster_save()}
          {/if}
        </Button>
      </div>
    </div>
  </div>
</div>

<!-- Confirmation Dialog for Replacing Decoder -->
{#if showConfirmDialog}
  <div class="fixed inset-0 z-[60] flex items-center justify-center bg-black/50">
    <div class="mx-4 max-w-md space-y-4 rounded-lg border border-border bg-card p-6">
      <h3 class="text-lg font-semibold">{m.digital_roster_confirm_replace_title()}</h3>
      <p class="text-sm text-muted-foreground">
        {m.digital_roster_confirm_replace()}
      </p>
      <div class="flex justify-end gap-3">
        <Button type="button" variant="ghost" onclick={handleCancelReplace} disabled={isSubmitting}>
          {m.add_model_cancel()}
        </Button>
        <Button
          type="button"
          variant="default"
          onclick={handleConfirmReplace}
          disabled={isSubmitting}
        >
          {m.digital_roster_replace()}
        </Button>
      </div>
    </div>
  </div>
{/if}

<!-- Discard Changes Dialog -->
{#if showDiscardDialog}
  <div class="fixed inset-0 z-[60] flex items-center justify-center bg-black/50">
    <div class="mx-4 max-w-md space-y-4 rounded-lg border border-border bg-card p-6">
      <h3 class="text-lg font-semibold">{m.add_model_discard_title()}</h3>
      <p class="text-sm text-muted-foreground">
        {m.add_model_discard_message()}
      </p>
      <div class="flex justify-end gap-3">
        <Button type="button" variant="ghost" onclick={handleDiscardCancel}>
          {m.add_model_cancel()}
        </Button>
        <Button type="button" variant="destructive" onclick={handleDiscardConfirm}>
          {m.digital_roster_cancel()}
        </Button>
      </div>
    </div>
  </div>
{/if}
