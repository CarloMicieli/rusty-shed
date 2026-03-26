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
  import {
    DrawerShell,
    DrawerHeader,
    DigitalSection,
    createDrawerForm
  } from '$lib/components/drawer';

  interface Props {
    open: boolean;
    onClose: () => void;
    onSuccess: () => void;
    preselectedStockId?: string | null;
  }

  let { open, onClose, onSuccess, preselectedStockId = null }: Props = $props();

  const controller = getDigitalRosterContext();

  function getTodayStr() {
    const n = new Date();
    return `${n.getFullYear()}-${String(n.getMonth() + 1).padStart(2, '0')}-${String(n.getDate()).padStart(2, '0')}`;
  }

  const f = createDrawerForm({
    initial: () => ({
      selectedRollingStockId: null as string | null,
      selectedDecoderId: null as string | null,
      dccAddress: null as number | null,
      installationDate: getTodayStr()
    }),
    validate: (v) => ({
      rollingStock: !v.selectedRollingStockId
        ? m.digital_roster_validation_rolling_stock()
        : undefined,
      decoder: !v.selectedDecoderId ? m.digital_roster_validation_decoder() : undefined,
      address:
        v.dccAddress === null || v.dccAddress < 1 || v.dccAddress > 9999
          ? m.digital_roster_address_range()
          : undefined
    })
  });

  // Reference data
  let installableRollingStocks = $state<InstallableRollingStockView[]>([]);
  let decoders = $state<Decoder[]>([]);
  let manufacturers = $state<Manufacturer[]>([]);

  // UI state
  let isLoadingData = $state(false);
  let isSubmitting = $state(false);
  let showConfirmDialog = $state(false);
  let duplicateWarning = $state<string | null>(null);

  // Derived state
  let selectedRollingStock = $derived(
    installableRollingStocks.find(
      (rs) => rs.owned_rolling_stock_id === f.values.selectedRollingStockId
    ) ?? null
  );

  let existingDigitalRollingStock = $derived.by((): DigitalRollingStockView | null => {
    if (!selectedRollingStock || !controller.state.rollingStocks) return null;
    return (
      controller.state.rollingStocks.find(
        (drs) => drs.owned_rolling_stock_id === f.values.selectedRollingStockId
      ) ?? null
    );
  });

  let compatibleDecoders = $derived.by(() => {
    const iface = selectedRollingStock?.dcc_interface ?? null;
    if (!iface) return decoders;
    return decoders.filter((d) => d.decoderInterface === iface);
  });

  $effect(() => {
    if (open) handleOpen();
  });

  $effect(() => {
    if (selectedRollingStock?.has_decoder && existingDigitalRollingStock) {
      f.values.dccAddress = existingDigitalRollingStock.dcc_address;
      f.values.selectedDecoderId = existingDigitalRollingStock.decoder.id;
    }
  });

  $effect(() => {
    if (
      f.values.selectedDecoderId &&
      !compatibleDecoders.some((d) => d.id === f.values.selectedDecoderId)
    ) {
      f.values.selectedDecoderId = null;
    }
  });

  async function handleOpen() {
    f.reset();
    duplicateWarning = null;
    showConfirmDialog = false;
    await loadReferenceData();
    if (preselectedStockId) {
      f.values.selectedRollingStockId = preselectedStockId;
      const existing =
        controller.state.rollingStocks?.find(
          (drs) => drs.owned_rolling_stock_id === preselectedStockId
        ) ?? null;
      if (existing) {
        f.values.dccAddress = existing.dcc_address;
        f.values.selectedDecoderId = existing.decoder.id;
      }
      f.syncInitial();
    }
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

  // Expose current validation errors (touched-gated) for child components
  const validationErrors = $derived(f.errors);

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
    f.touch();
    if (!f.isValid || duplicateWarning) return;

    if (selectedRollingStock?.has_decoder && !showConfirmDialog) {
      showConfirmDialog = true;
      return;
    }

    await performInstallation();
  }

  async function performInstallation() {
    if (
      !f.values.selectedRollingStockId ||
      !f.values.selectedDecoderId ||
      f.values.dccAddress === null
    )
      return;

    try {
      isSubmitting = true;
      let success: boolean;

      if (existingDigitalRollingStock) {
        success = await controller.replaceDecoder(
          existingDigitalRollingStock.id,
          f.values.selectedDecoderId
        );
        if (success && f.values.dccAddress !== existingDigitalRollingStock.dcc_address) {
          success = await controller.changeDccAddress(
            existingDigitalRollingStock.id,
            f.values.dccAddress
          );
        }
      } else {
        success = await controller.installDecoder(
          f.values.selectedRollingStockId,
          f.values.selectedDecoderId,
          f.values.dccAddress
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

<DrawerShell
  {open}
  {onClose}
  size="xl"
  hasChanges={f.isDirty}
  labelledby="decoder-install-drawer-title"
>
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
        selectedId={f.values.selectedRollingStockId}
        error={validationErrors.rollingStock}
        touched={f.touched}
        onChange={(id) => (f.values.selectedRollingStockId = id)}
      />

      <DecoderPicker
        decoders={compatibleDecoders}
        {manufacturers}
        selectedId={f.values.selectedDecoderId}
        error={validationErrors.decoder}
        touched={f.touched}
        onChange={(id) => (f.values.selectedDecoderId = id)}
      />

      <DigitalSection
        bind:dccAddress={f.values.dccAddress}
        bind:installationDate={f.values.installationDate}
        onAddressChange={handleAddressChange}
        {duplicateWarning}
        errors={validationErrors}
        touched={f.touched}
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
        class="bg-amber-500 font-bold text-black hover:bg-amber-500/90"
        onclick={handleSubmit}
        disabled={isSubmitting || isLoadingData || (!f.touched && !f.isValid)}
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
