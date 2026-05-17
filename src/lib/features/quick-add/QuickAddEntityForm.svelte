<script lang="ts">
  import * as m from '$lib/paraglide/messages.js';
  import { Button } from '$lib/components';
  import { DrawerInput } from '$lib/components/drawer';
  import { commands, type Manufacturer, type Seller } from '$lib/bindings';
  import { quickAddFormSchema, type QuickAddFormValues } from '$lib/schemas/quick-add-form';
  import type { QuickAddMode, QuickAddTarget } from './types';

  interface Props {
    target: QuickAddTarget;
    mode?: QuickAddMode;
    existingNames: string[];
    initialValues?: Partial<QuickAddFormState>;
    submitLabel?: string;
    onSubmit?: (values: QuickAddFormState) => Promise<Manufacturer | Seller>;
    onSuccess: (entity: Manufacturer | Seller) => void;
    onCancel: () => void;
    onError?: (message: string) => void;
    onDirtyChange?: (dirty: boolean) => void;
  }

  interface QuickAddFormState {
    name: string;
    websiteUrl: string;
    countryCode: string;
    notes: string;
  }

  let {
    target,
    mode = 'QUICK',
    existingNames,
    initialValues = {},
    submitLabel,
    onSubmit,
    onSuccess,
    onCancel,
    onError,
    onDirtyChange
  }: Props = $props();

  let values = $state<QuickAddFormState>({ name: '', websiteUrl: '', countryCode: '', notes: '' });
  let isSaving = $state(false);
  let saveError = $state<string | null>(null);
  let fieldError = $state<string | null>(null);

  $effect(() => {
    values = {
      name: initialValues.name ?? '',
      websiteUrl: initialValues.websiteUrl ?? '',
      countryCode: initialValues.countryCode ?? '',
      notes: initialValues.notes ?? ''
    };
  });

  const entityLabel = $derived.by(() => {
    switch (target) {
      case 'manufacturer':
        return m.quick_add_entity_manufacturer();
      case 'buyer':
        return m.quick_add_entity_buyer();
      default:
        return m.quick_add_entity_seller();
    }
  });

  const normalizedName = $derived(values.name.trim().toLowerCase());
  const isDuplicate = $derived(
    normalizedName.length > 0 &&
      existingNames.some((entry) => entry.trim().toLowerCase() === normalizedName)
  );
  const isDirty = $derived(
    values.name.trim().length > 0 ||
      values.websiteUrl.trim().length > 0 ||
      values.countryCode.trim().length > 0 ||
      values.notes.trim().length > 0
  );

  $effect(() => {
    onDirtyChange?.(isDirty);
  });

  const canSave = $derived(values.name.trim().length > 0 && !isDuplicate && !isSaving);

  async function handleSave() {
    fieldError = null;
    saveError = null;

    const payload: QuickAddFormValues = {
      name: values.name,
      websiteUrl: values.websiteUrl,
      countryCode: values.countryCode
    };

    const validation = quickAddFormSchema.safeParse(payload);
    if (!validation.success) {
      fieldError = validation.error.issues[0]?.message ?? m.quick_add_name_required();
      return;
    }

    isSaving = true;
    try {
      if (onSubmit) {
        const entity = await onSubmit(values);
        onSuccess(entity);
        return;
      }

      if (target === 'manufacturer') {
        const result = await commands.createManufacturer({
          name: values.name.trim(),
          websiteUrl: values.websiteUrl.trim() || null,
          countryCode: values.countryCode.trim().toUpperCase() || null
        });

        if (result.status === 'ok') {
          onSuccess(result.data);
          return;
        }
      } else if (target === 'seller') {
        const result = await commands.createSeller({
          name: values.name.trim(),
          sellerType: 'SHOP',
          email: null,
          phone: null,
          websiteUrl: values.websiteUrl.trim() || null,
          streetAddress: null,
          extendedAddress: null,
          city: null,
          stateRegion: null,
          postalCode: null,
          countryCode: values.countryCode.trim().toUpperCase() || null
        });

        if (result.status === 'ok') {
          onSuccess(result.data);
          return;
        }
      } else {
        const result = await commands.createBuyer({
          name: values.name.trim(),
          sellerType: 'SHOP',
          email: null,
          phone: null,
          websiteUrl: values.websiteUrl.trim() || null,
          streetAddress: null,
          extendedAddress: null,
          city: null,
          stateRegion: null,
          postalCode: null,
          countryCode: values.countryCode.trim().toUpperCase() || null
        });

        if (result.status === 'ok') {
          onSuccess(result.data);
          return;
        }
      }

      saveError = m.quick_add_save_failed();
      onError?.(saveError);
    } catch {
      saveError = m.quick_add_save_failed();
      onError?.(saveError);
    } finally {
      isSaving = false;
    }
  }

  function handleCancel() {
    onCancel();
  }
</script>

<div class="space-y-4" data-form-mode={mode}>
  <div class="space-y-1">
    <label for="quick-add-name" class="text-xs tracking-wider text-muted-foreground uppercase">
      {m.quick_add_field_name()}
    </label>
    <DrawerInput
      id="quick-add-name"
      type="text"
      placeholder={m.quick_add_name_placeholder()}
      value={values.name}
      oninput={(event) => (values.name = (event.currentTarget as HTMLInputElement).value)}
      aria-invalid={fieldError ? 'true' : undefined}
    />
  </div>

  <div class="space-y-1">
    <label for="quick-add-website" class="text-xs tracking-wider text-muted-foreground uppercase">
      {m.quick_add_field_website()}
    </label>
    <DrawerInput
      id="quick-add-website"
      type="text"
      placeholder={m.quick_add_website_placeholder()}
      value={values.websiteUrl}
      oninput={(event) => (values.websiteUrl = (event.currentTarget as HTMLInputElement).value)}
    />
  </div>

  <div class="space-y-1">
    <label for="quick-add-country" class="text-xs tracking-wider text-muted-foreground uppercase">
      {m.quick_add_field_country()}
    </label>
    <DrawerInput
      id="quick-add-country"
      type="text"
      maxLength={2}
      placeholder={m.quick_add_country_placeholder()}
      class="uppercase"
      value={values.countryCode}
      oninput={(event) => (values.countryCode = (event.currentTarget as HTMLInputElement).value)}
    />
  </div>

  {#if mode === 'FULL'}
    <div class="space-y-1">
      <label for="quick-add-notes" class="text-xs tracking-wider text-muted-foreground uppercase">
        {m.settings_library_field_notes()}
      </label>
      <textarea
        id="quick-add-notes"
        class="min-h-20 w-full rounded-sm border border-border bg-background px-3 py-2 text-sm"
        placeholder={m.settings_library_notes_placeholder()}
        value={values.notes}
        oninput={(event) => (values.notes = (event.currentTarget as HTMLTextAreaElement).value)}
      ></textarea>
    </div>
  {/if}

  {#if isDuplicate}
    <p class="text-warning text-xs">
      {m.quick_add_duplicate_warning({ entity: entityLabel })}
    </p>
  {/if}

  {#if fieldError}
    <p class="text-xs text-destructive">{fieldError}</p>
  {/if}

  {#if saveError}
    <p class="text-xs text-destructive">{saveError}</p>
  {/if}

  <div class="flex items-center justify-end gap-2 pt-2">
    <Button
      type="button"
      variant="outline"
      class="rounded-sm border-border bg-background tracking-widest text-foreground uppercase"
      onclick={handleCancel}
    >
      {m.quick_add_cancel()}
    </Button>
    <Button
      type="button"
      class="variant-steampunk-lever rounded-sm bg-primary font-bebas tracking-widest text-primary-foreground uppercase shadow-[2px_2px_0px_0px_rgba(0,0,0,0.2)] hover:bg-primary/90"
      onclick={handleSave}
      disabled={!canSave}
    >
      {isSaving ? m.settings_saving_button() : (submitLabel ?? m.quick_add_save())}
    </Button>
  </div>
</div>
