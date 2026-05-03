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
  import { superForm } from 'sveltekit-superforms';
  import { zod4 as zod } from 'sveltekit-superforms/adapters';
  import { decoderInstallSchema } from '$lib/schemas/decoder-install-form';

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

  function getInitialData() {
    return {
      selectedRollingStockId: null as string | null,
      selectedDecoderId: null as string | null,
      dccAddress: null as number | null,
      installationDate: getTodayStr()
    };
  }

  // Reference data
  let installableRollingStocks = $state<InstallableRollingStockView[]>([]);
  let decoders = $state<Decoder[]>([]);
  let manufacturers = $state<Manufacturer[]>([]);

  // UI state
  let isLoadingData = $state(false);
  let isSubmitting = $state(false);
  let showConfirmDialog = $state(false);
  let duplicateWarning = $state<string | null>(null);
  let hasSubmitted = $state(false);
  let formEl: HTMLFormElement | undefined = $state();

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  const { form, errors, tainted, enhance, reset, isTainted } = superForm(getInitialData() as any, {
    SPA: true,
    dataType: 'json',
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    validators: zod(decoderInstallSchema as any),
    onUpdate: async ({ form: fd }) => {
      if (!fd.valid) return;
      if (selectedRollingStock?.has_decoder && !showConfirmDialog) {
        showConfirmDialog = true;
        return;
      }
      await performInstallation();
    }
  });

  const hasChanges = $derived(isTainted($tainted));

  // Map schema field names → child component prop names
  const mappedErrors = $derived({
    rollingStock: $errors.selectedRollingStockId?.[0] as string | undefined,
    decoder: $errors.selectedDecoderId?.[0] as string | undefined,
    address: $errors.dccAddress?.[0] as string | undefined
  });

  // Derived state
  let selectedRollingStock = $derived(
    installableRollingStocks.find(
      (rs) => rs.owned_rolling_stock_id === $form.selectedRollingStockId
    ) ?? null
  );

  let existingDigitalRollingStock = $derived.by((): DigitalRollingStockView | null => {
    if (!selectedRollingStock || !controller.state.rollingStocks) return null;
    return (
      controller.state.rollingStocks.find(
        (drs) => drs.owned_rolling_stock_id === $form.selectedRollingStockId
      ) ?? null
    );
  });

  let compatibleDecoders = $derived.by(() => {
    const iface = selectedRollingStock?.dcc_interface ?? null;
    if (!iface) return decoders;
    return decoders.filter((d) => d.decoderInterface === iface);
  });

  $effect(() => {
    if (open) void handleOpen();
  });

  $effect(() => {
    if (selectedRollingStock?.has_decoder && existingDigitalRollingStock) {
      $form.dccAddress = existingDigitalRollingStock.dcc_address;
      $form.selectedDecoderId = existingDigitalRollingStock.decoder.id;
    }
  });

  $effect(() => {
    if (
      $form.selectedDecoderId &&
      !compatibleDecoders.some((d) => d.id === $form.selectedDecoderId)
    ) {
      $form.selectedDecoderId = null;
    }
  });

  async function handleOpen() {
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    reset({ data: getInitialData() as any });
    hasSubmitted = false;
    duplicateWarning = null;
    showConfirmDialog = false;
    await loadReferenceData();
    if (preselectedStockId) {
      const existing =
        controller.state.rollingStocks?.find(
          (drs) => drs.owned_rolling_stock_id === preselectedStockId
        ) ?? null;
      const preselectedData = {
        ...getInitialData(),
        selectedRollingStockId: preselectedStockId,
        dccAddress: existing?.dcc_address ?? null,
        selectedDecoderId: existing?.decoder.id ?? null
      };
      // Reset with preselected data as baseline so dirty check is accurate
      // eslint-disable-next-line @typescript-eslint/no-explicit-any
      reset({ data: preselectedData as any });
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

  function handleSubmit() {
    hasSubmitted = true;
    formEl?.requestSubmit();
  }

  async function performInstallation() {
    if (!$form.selectedRollingStockId || !$form.selectedDecoderId || $form.dccAddress === null)
      return;

    try {
      isSubmitting = true;
      let success: boolean;

      if (existingDigitalRollingStock) {
        success = await controller.replaceDecoder(
          existingDigitalRollingStock.id,
          $form.selectedDecoderId as string
        );
        if (success && $form.dccAddress !== existingDigitalRollingStock.dcc_address) {
          success = await controller.changeDccAddress(
            existingDigitalRollingStock.id,
            $form.dccAddress as number
          );
        }
      } else {
        success = await controller.installDecoder(
          $form.selectedRollingStockId as string,
          $form.selectedDecoderId as string,
          $form.dccAddress as number
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
    void performInstallation();
  }
</script>

<DrawerShell {open} {onClose} size="xl" {hasChanges} labelledby="decoder-install-drawer-title">
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
    <form id="install-decoder-form" bind:this={formEl} use:enhance class="space-y-6">
      <!-- Hidden submit button to enable Enter-to-submit keyboard navigability -->
      <button type="submit" class="hidden" aria-hidden="true" tabindex="-1"></button>
      <DecoderRollingStockPicker
        rollingStocks={installableRollingStocks}
        selectedId={$form.selectedRollingStockId}
        error={mappedErrors.rollingStock}
        touched={hasSubmitted}
        onChange={(id) => ($form.selectedRollingStockId = id)}
      />

      <DigitalSection
        bind:dccAddress={$form.dccAddress}
        bind:installationDate={$form.installationDate}
        onAddressChange={handleAddressChange}
        {duplicateWarning}
        errors={mappedErrors}
        touched={hasSubmitted}
        disabled={isSubmitting}
      >
        <DecoderPicker
          decoders={compatibleDecoders}
          {manufacturers}
          selectedId={$form.selectedDecoderId}
          error={mappedErrors.decoder}
          touched={hasSubmitted}
          onChange={(id) => ($form.selectedDecoderId = id)}
        />
      </DigitalSection>
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
        disabled={isSubmitting || isLoadingData}
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
