<script lang="ts">
  import * as m from '$lib/paraglide/messages.js';
  import { ShoppingBag } from 'lucide-svelte';
  import {
    commands,
    type Manufacturer,
    type PowerMethod,
    type Scale,
    type SellerView
  } from '$lib/bindings';
  import { toaster } from '$lib/toaster';
  import { settingsState } from '$lib/features/settings/SettingsState.svelte';
  import { Button } from '$lib/components';
  import type { AcquisitionFormState, AcquisitionItemEntry, BatchDefaults } from './types.js';
  import { createDefaultFormState, createDefaultItem, toRecordAcquisitionArgs } from './types.js';
  import AcquisitionBatchFields from './components/AcquisitionBatchFields.svelte';
  import AcquisitionItemCard from './components/AcquisitionItemCard.svelte';
  import { DrawerShell, DrawerHeader, DrawerFooter } from '$lib/components/drawer';
  import { superForm } from 'sveltekit-superforms';
  import { zod4 as zod } from 'sveltekit-superforms/adapters';
  import { acquisitionSchema } from '$lib/schemas/acquisition-form';

  interface Props {
    open: boolean;
    onClose: () => void;
    onSuccess: () => void;
  }

  let { open, onClose, onSuccess }: Props = $props();

  // Reference data
  let sellers = $state<SellerView[]>([]);
  let manufacturers = $state<Manufacturer[]>([]);

  // UI state
  let isSubmitting = $state(false);
  let isLoadingData = $state(false);
  let formEl: HTMLFormElement | undefined = $state();

  function getDefaults() {
    return {
      scale: (settingsState.settings?.favouriteScale as Scale) || null,
      powerMethod: (settingsState.settings?.powerMethod as PowerMethod) || null
    };
  }

  const { form, errors, tainted, enhance, reset, isTainted } = superForm(
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    createDefaultFormState(getDefaults()) as any,
    {
      SPA: true,
      dataType: 'json',
      // eslint-disable-next-line @typescript-eslint/no-explicit-any
      validators: zod(acquisitionSchema as any),
      onUpdate: async ({ form: fd }) => {
        if (!fd.valid) return;
        isSubmitting = true;
        try {
          const args = toRecordAcquisitionArgs($form as AcquisitionFormState, currency);
          const result = await commands.recordAcquisition(args);
          if (result.status === 'ok') {
            toaster.success(m.acquisition_toast_success());
            onSuccess();
            onClose();
          } else {
            toaster.error(m.acquisition_error_finalize());
          }
        } catch (e) {
          console.error('Error saving acquisition:', e);
          toaster.error(m.acquisition_error_finalize());
        } finally {
          isSubmitting = false;
        }
      }
    }
  );

  let currency = $derived(settingsState.settings.currency ?? 'EUR');
  let hasChanges = $derived(isTainted($tainted));

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  const itemsErrors = $derived($errors.items as any);
  const formItems = $derived($form.items as AcquisitionItemEntry[]);

  $effect.pre(() => {
    if (open) {
      // eslint-disable-next-line @typescript-eslint/no-explicit-any
      reset({ data: createDefaultFormState(getDefaults()) as any });
    }
  });

  $effect(() => {
    if (open) {
      void loadReferenceData();
    }
  });

  async function loadReferenceData() {
    isLoadingData = true;
    try {
      const [mfgResult, sellerResult] = await Promise.all([
        commands.getManufacturers(),
        commands.getSellers()
      ]);
      manufacturers = mfgResult.status === 'ok' ? mfgResult.data : [];
      sellers = sellerResult.status === 'ok' ? sellerResult.data : [];
    } catch (e) {
      console.error('Error loading reference data:', e);
    } finally {
      isLoadingData = false;
    }
  }

  function handleAddItem() {
    $form.items = [...formItems, createDefaultItem()];
  }

  function handleDuplicate(uid: string) {
    const source = formItems.find((i) => i.uid === uid);
    if (!source) return;
    const clone: AcquisitionItemEntry = { ...source, uid: crypto.randomUUID(), productCode: '' };
    const idx = formItems.findIndex((i) => i.uid === uid);
    const next = [...formItems];
    next.splice(idx + 1, 0, clone);
    $form.items = next;
  }

  function handleRemove(uid: string) {
    if (formItems.length <= 1) return;
    $form.items = formItems.filter((i) => i.uid !== uid);
  }

  function handleUpdateItem(uid: string, patch: Partial<AcquisitionItemEntry>) {
    $form.items = formItems.map((i) => (i.uid === uid ? { ...i, ...patch } : i));
  }

  function handleBatchDefaultChange(field: 'scale' | 'powerMethod', value: string | null) {
    $form.batchDefaults = { ...($form.batchDefaults as BatchDefaults), [field]: value };
  }

  function handleSubmit() {
    formEl?.requestSubmit();
  }
</script>

<DrawerShell
  {open}
  {onClose}
  size="xl"
  {hasChanges}
  labelledby="acquisition-drawer-title"
  discardTitle={m.acquisition_discard_title()}
  discardDescription={m.acquisition_discard_description()}
  discardConfirm={m.acquisition_discard_confirm()}
  discardCancel={m.acquisition_discard_cancel()}
>
  {#snippet header({ requestClose })}
    <DrawerHeader
      id="acquisition-drawer-title"
      title={m.acquisition_drawer_title()}
      subtitle={m.acquisition_drawer_subtitle()}
      icon={ShoppingBag}
      onClose={requestClose}
    />
  {/snippet}

  {#snippet stickyBand()}
    <div class="px-4 py-3">
      <AcquisitionBatchFields
        sellerId={$form.sellerId as string | null}
        onSellerChange={(id) => ($form.sellerId = id)}
        purchaseDate={$form.purchaseDate as string}
        onDateChange={(date) => ($form.purchaseDate = date)}
        batchDefaults={$form.batchDefaults as BatchDefaults}
        onBatchDefaultChange={handleBatchDefaultChange}
        {sellers}
      />
    </div>
  {/snippet}

  <form bind:this={formEl} use:enhance class="space-y-3">
    {#if itemsErrors?._errors?.[0]}
      <div
        class="rounded-lg border border-destructive/40 bg-destructive/10 p-3 text-sm text-destructive"
      >
        {itemsErrors._errors[0]}
      </div>
    {/if}

    {#if isLoadingData}
      <div class="flex items-center justify-center py-12">
        <p class="text-sm text-zinc-500">Loading…</p>
      </div>
    {:else}
      {#each formItems as item, idx (item.uid)}
        <AcquisitionItemCard
          {item}
          index={idx}
          {manufacturers}
          {currency}
          errors={{
            manufacturerId: itemsErrors?.[idx]?.manufacturerId?.[0],
            productCode: itemsErrors?.[idx]?.productCode?.[0],
            category: itemsErrors?.[idx]?.category?.[0]
          }}
          canRemove={formItems.length > 1}
          onUpdate={handleUpdateItem}
          onDuplicate={handleDuplicate}
          onRemove={handleRemove}
        />
      {/each}
    {/if}
  </form>

  {#snippet footer({ requestClose })}
    <DrawerFooter
      cancelLabel={m.acquisition_cancel_button()}
      submitLabel={isSubmitting
        ? m.acquisition_finalizing_button()
        : m.acquisition_finalize_button()}
      onCancel={requestClose}
      onSubmit={handleSubmit}
      submitting={isSubmitting}
      isLoading={isLoadingData}
    >
      {#snippet leading()}
        <Button
          type="button"
          variant="rusty"
          onclick={handleAddItem}
          disabled={isSubmitting || isLoadingData}
          class="shadow-lg shadow-amber-500/10"
        >
          {m.acquisition_add_item_button()}
        </Button>
      {/snippet}
    </DrawerFooter>
  {/snippet}
</DrawerShell>
