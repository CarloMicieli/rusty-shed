<script lang="ts">
  import * as m from '$lib/paraglide/messages.js';
  import { commands, type Manufacturer, type Seller } from '$lib/bindings';
  import { quickAddFormSchema, type QuickAddFormValues } from '$lib/schemas/quick-add-form';
  import type { QuickAddTarget } from './types';

  interface Props {
    target: QuickAddTarget;
    existingNames: string[];
    onSuccess: (entity: Manufacturer | Seller) => void;
    onCancel: () => void;
    onDirtyChange?: (dirty: boolean) => void;
  }

  let { target, existingNames, onSuccess, onCancel, onDirtyChange }: Props = $props();

  let values = $state<QuickAddFormValues>({ name: '', websiteUrl: '', countryCode: '' });
  let isSaving = $state(false);
  let saveError = $state<string | null>(null);
  let fieldError = $state<string | null>(null);

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
    values.name.trim().length > 0 || values.websiteUrl.trim().length > 0 || values.countryCode.trim().length > 0
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
      } else {
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
      }

      saveError = m.quick_add_save_failed();
    } catch {
      saveError = m.quick_add_save_failed();
    } finally {
      isSaving = false;
    }
  }

  function handleCancel() {
    onCancel();
  }
</script>

<div class="space-y-4">
  <div class="space-y-1">
    <label for="quick-add-name" class="text-xs font-semibold tracking-wide uppercase">
      {m.quick_add_field_name()}
    </label>
    <input
      id="quick-add-name"
      class="h-10 w-full rounded-sm border border-border bg-background px-3 text-sm"
      value={values.name}
      oninput={(event) => (values.name = (event.currentTarget as HTMLInputElement).value)}
      aria-invalid={fieldError ? 'true' : undefined}
    />
  </div>

  <div class="space-y-1">
    <label for="quick-add-website" class="text-xs font-semibold tracking-wide uppercase">
      {m.quick_add_field_website()}
    </label>
    <input
      id="quick-add-website"
      class="h-10 w-full rounded-sm border border-border bg-background px-3 text-sm"
      value={values.websiteUrl}
      oninput={(event) => (values.websiteUrl = (event.currentTarget as HTMLInputElement).value)}
    />
  </div>

  <div class="space-y-1">
    <label for="quick-add-country" class="text-xs font-semibold tracking-wide uppercase">
      {m.quick_add_field_country()}
    </label>
    <input
      id="quick-add-country"
      maxlength="2"
      class="h-10 w-full rounded-sm border border-border bg-background px-3 text-sm uppercase"
      value={values.countryCode}
      oninput={(event) => (values.countryCode = (event.currentTarget as HTMLInputElement).value)}
    />
  </div>

  {#if isDuplicate}
    <p class="text-xs text-amber-400">
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
    <button type="button" class="rounded-sm border border-border px-3 py-2 text-sm" onclick={handleCancel}>
      {m.quick_add_cancel()}
    </button>
    <button
      type="button"
      class="rounded-sm bg-primary px-3 py-2 text-sm text-primary-foreground disabled:opacity-50"
      onclick={handleSave}
      disabled={!canSave}
    >
      {isSaving ? m.settings_saving_button() : m.quick_add_save()}
    </button>
  </div>
</div>
