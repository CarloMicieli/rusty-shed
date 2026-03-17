<script lang="ts">
  import * as m from '$lib/paraglide/messages.js';
  import { Cpu } from 'lucide-svelte';
  import { commands } from '$lib/bindings';
  import type {
    Decoder,
    InstallableRollingStockView,
    Manufacturer,
    DigitalRollingStockView
  } from '$lib/bindings';
  import { getDigitalRosterContext } from '../DigitalRosterState.svelte';
  import { Button } from '$lib/components';
  import DecoderInstallConfirmDialog from './DecoderInstallConfirmDialog.svelte';
  import DecoderRollingStockPicker from './DecoderRollingStockPicker.svelte';
  import DecoderPicker from './DecoderPicker.svelte';
  import { DrawerShell, DrawerHeader, DigitalSection } from '$lib/components/drawer';

  interface Props {
    open: boolean;
    onClose: () => void;
    onSuccess: () => void;
  }

  let { open, onClose, onSuccess }: Props = $props();

  const controller = getDigitalRosterContext();

  // Form state
  let selectedRollingStockId = $state<string | null>(null);
  let selectedDecoderId = $state<string | null>(null);
  let dccAddress = $state<number | null>(null);
  let installationDate = $state<string | null>(new Date().toISOString().split('T')[0]);

  // Reference data
  let installableRollingStocks = $state<InstallableRollingStockView[]>([]);
  let decoders = $state<Decoder[]>([]);
  let manufacturers = $state<Manufacturer[]>([]);

  // UI state
  let isLoadingData = $state(false);
  let isSubmitting = $state(false);
  let showConfirmDialog = $state(false);
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
    if (!selectedRollingStock || !controller.state.rollingStocks) return null;
    return (
      controller.state.rollingStocks.find(
        (drs) => drs.owned_rolling_stock_id === selectedRollingStockId
      ) ?? null
    );
  });

  $effect(() => {
    if (open) handleOpen();
  });

  $effect(() => {
    if (selectedRollingStock?.has_decoder && existingDigitalRollingStock) {
      dccAddress = existingDigitalRollingStock.dcc_address;
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
    const n = new Date();
    installationDate = `${n.getFullYear()}-${String(n.getMonth() + 1).padStart(2, '0')}-${String(n.getDate()).padStart(2, '0')}`;
    touched = false;
    validationErrors = {};
    duplicateWarning = null;
    showConfirmDialog = false;
  }

  async function loadReferenceData() {
    try {
      isLoadingData = true;
      const [installableResult, decodersResult, manufacturersResult] = await Promise.all([
        commands.getInstallableRollingStocks(),
        commands.getDecoders(),
        commands.getManufacturers()
      ]);

      if (installableResult.status === 'ok') installableRollingStocks = installableResult.data;
      if (decodersResult.status === 'ok') {
        decoders = decodersResult.data.filter((d) => d.decoderType !== 'FUNCTION');
      }
      if (manufacturersResult.status === 'ok') manufacturers = manufacturersResult.data;
    } catch (error) {
      console.error('Error loading reference data:', error);
    } finally {
      isLoadingData = false;
    }
  }

  function validateForm() {
    const errors: typeof validationErrors = {};
    if (!selectedRollingStockId) errors.rollingStock = m.digital_roster_validation_rolling_stock();
    if (!selectedDecoderId) errors.decoder = m.digital_roster_validation_decoder();
    if (dccAddress === null || dccAddress < 1 || dccAddress > 9999) {
      errors.address = m.digital_roster_address_range();
    }
    return errors;
  }

  async function handleAddressChange(addr: number | null) {
    if (addr !== null && addr >= 1 && addr <= 9999) {
      const excludeId = existingDigitalRollingStock?.id ?? null;
      const result = await controller.checkDuplicateAddress(addr, excludeId);
      duplicateWarning = result.isDuplicate
        ? m.digital_roster_duplicate_warning({ address: addr.toString() })
        : null;
    } else {
      duplicateWarning = null;
    }
  }

  async function handleSubmit() {
    touched = true;
    if (!isFormValid) return;

    if (selectedRollingStock?.has_decoder && !showConfirmDialog) {
      showConfirmDialog = true;
      return;
    }

    await performInstallation();
  }

  async function performInstallation() {
    if (!selectedRollingStockId || !selectedDecoderId || dccAddress === null) return;

    try {
      isSubmitting = true;
      let success: boolean;

      if (existingDigitalRollingStock) {
        success = await controller.replaceDecoder(
          existingDigitalRollingStock.id,
          selectedDecoderId
        );
        if (success && dccAddress !== existingDigitalRollingStock.dcc_address) {
          success = await controller.changeDccAddress(existingDigitalRollingStock.id, dccAddress);
        }
      } else {
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
</script>

<DrawerShell {open} {onClose} size="lg" {hasChanges} labelledby="decoder-install-drawer-title">
  {#snippet header({ requestClose })}
    <DrawerHeader
      id="decoder-install-drawer-title"
      title={m.digital_roster_install_decoder()}
      subtitle={m.app_digital_roster()}
      icon={Cpu}
      onClose={requestClose}
    />
  {/snippet}

  {#if isLoadingData}
    <div class="flex items-center justify-center py-8">
      <div class="border-primary-500 h-12 w-12 animate-spin rounded-full border-b-2"></div>
    </div>
  {:else}
    <form id="install-decoder-form" class="space-y-6" onsubmit={(e) => e.preventDefault()}>
      <DecoderRollingStockPicker
        rollingStocks={installableRollingStocks}
        selectedId={selectedRollingStockId}
        error={validationErrors.rollingStock}
        {touched}
        onChange={(id) => (selectedRollingStockId = id)}
      />

      <DecoderPicker
        {decoders}
        {manufacturers}
        selectedId={selectedDecoderId}
        error={validationErrors.decoder}
        {touched}
        onChange={(id) => (selectedDecoderId = id)}
      />

      <DigitalSection
        bind:dccAddress
        bind:installationDate
        onAddressChange={handleAddressChange}
        {duplicateWarning}
        errors={validationErrors}
        {touched}
        disabled={isSubmitting}
      />
    </form>
  {/if}

  {#snippet footer({ requestClose })}
    <div class="flex justify-end gap-3 p-4">
      <Button type="button" variant="ghost" onclick={requestClose} disabled={isSubmitting}>
        {m.add_model_cancel()}
      </Button>
      <Button
        type="submit"
        variant="default"
        class="bg-[#D48A42] font-bold text-black hover:bg-[#D48A42]/90"
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
  {/snippet}
</DrawerShell>

<DecoderInstallConfirmDialog
  open={showConfirmDialog}
  {isSubmitting}
  onConfirm={handleConfirmReplace}
  onCancel={() => (showConfirmDialog = false)}
/>
