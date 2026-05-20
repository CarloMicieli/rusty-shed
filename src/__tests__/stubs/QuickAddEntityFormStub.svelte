<script lang="ts">
  interface FormValues {
    name: string;
    websiteUrl: string;
    countryCode: string;
    notes: string;
  }

  interface Props {
    target: 'manufacturer' | 'seller' | 'buyer';
    initialValues?: Partial<FormValues>;
    submitLabel?: string;
    onSubmit?: (values: FormValues) => Promise<unknown>;
    onSuccess: (entity: unknown) => void;
    onCancel: () => void;
  }

  let { target, initialValues = {}, submitLabel, onSubmit, onSuccess, onCancel }: Props = $props();

  async function triggerSuccess() {
    if (onSubmit) {
      const entity = await onSubmit({
        name: `${initialValues.name ?? 'entity'} updated`,
        websiteUrl: initialValues.websiteUrl ?? '',
        countryCode: initialValues.countryCode ?? 'IT',
        notes: initialValues.notes ?? ''
      });
      onSuccess(entity);
      return;
    }

    if (target === 'manufacturer') {
      onSuccess({
        id: 'trn:manufacturer:new-maker',
        name: 'New Maker',
        registeredCompanyName: null,
        countryCode: 'IT',
        status: 'ACTIVE',
        websiteUrl: null,
        isSystemSeeded: false,
        usageCount: 0
      });
      return;
    }

    onSuccess({
      id: 'trn:seller:new-party',
      name: 'New Party',
      sellerType: 'SHOP',
      email: null,
      phone: null,
      websiteUrl: null,
      address: null
    });
  }
</script>

<button type="button" onclick={triggerSuccess}>quick-form-success</button>
<button type="button" onclick={onCancel}>quick-form-cancel</button>
{#if submitLabel}
  <span>{submitLabel}</span>
{/if}
