<script lang="ts">
  import * as m from '$lib/paraglide/messages.js';
  import { Button, Input } from '$lib/components';
  import * as sellerService from '$lib/services/sellerService';
  import type { FormSeller } from '$lib/services/sellerAdapter';
  import type { SellerType } from '$lib/bindings';

  interface Props {
    onClose?: () => void;
    onSaved?: (seller: FormSeller) => void;
    initial?: FormSeller | null;
  }

  let { onClose = () => {}, onSaved = () => {}, initial = null }: Props = $props();

  let id = $state<string | null>(null);
  let name = $state('');
  let sellerType = $state<SellerType>('SHOP' as SellerType);
  let email = $state('');
  let phone = $state('');
  let websiteUrl = $state('');

  let streetAddress = $state('');
  let extendedAddress = $state('');
  let city = $state('');
  let stateRegion = $state('');
  let postalCode = $state('');
  let countryCode = $state('');

  $effect(() => {
    if (initial) {
      id = initial.id ?? null;
      name = initial.name ?? '';
      sellerType = initial.sellerType ?? ('SHOP' as SellerType);
      email = initial.email ?? '';
      phone = initial.phone ?? '';
      websiteUrl = initial.websiteUrl ?? '';

      streetAddress = initial.streetAddress ?? '';
      extendedAddress = initial.extendedAddress ?? '';
      city = initial.city ?? '';
      stateRegion = initial.stateRegion ?? '';
      postalCode = initial.postalCode ?? '';
      countryCode = initial.countryCode ?? '';
    }
  });

  let isSubmitting = $state(false);
  let formError = $state<string | null>(null);

  function toForm(): FormSeller {
    return {
      id: id ?? undefined,
      name,
      sellerType,
      email: email || null,
      phone: phone || null,
      websiteUrl: websiteUrl || null,
      streetAddress: streetAddress || null,
      extendedAddress: extendedAddress || null,
      city: city || null,
      stateRegion: stateRegion || null,
      postalCode: postalCode || null,
      countryCode: countryCode || null
    } as FormSeller;
  }

  async function handleSubmit() {
    formError = null;
    if (!name.trim()) {
      formError = m.form_new_model_basic_info();
      return;
    }

    isSubmitting = true;
    try {
      const form = toForm();
      let res;
      if (form.id) {
        res = await sellerService.updateSeller(form);
      } else {
        res = await sellerService.createSeller(form);
      }

      if (res.status === 'ok') {
        onSaved(res.data);
        onClose();
      } else {
        formError = 'Failed to save seller';
      }
    } finally {
      isSubmitting = false;
    }
  }

  function close() {
    onClose();
  }
</script>

<div class="card">
  <div class="card-header">
    <h3 class="text-base font-semibold">{id ? 'Edit Seller' : 'New Seller'}</h3>
    <Button variant="ghost" onclick={close}>×</Button>
  </div>

  <div class="card-body space-y-3">
    <Input placeholder="Name" bind:value={name} />

    <select class="select" bind:value={sellerType}>
      <option value="SHOP">Shop</option>
      <option value="INDIVIDUAL">Individual</option>
      <option value="OTHER">Other</option>
    </select>

    <Input placeholder="Email" bind:value={email} />
    <Input placeholder="Phone" bind:value={phone} />
    <Input placeholder="Website" bind:value={websiteUrl} />

    <Input placeholder="Street address" bind:value={streetAddress} />
    <Input placeholder="Extended address" bind:value={extendedAddress} />
    <div class="grid grid-cols-3 gap-2">
      <Input placeholder="City" bind:value={city} />
      <Input placeholder="Region" bind:value={stateRegion} />
      <Input placeholder="Postal code" bind:value={postalCode} />
    </div>
    <Input placeholder="Country code" bind:value={countryCode} />

    {#if formError}
      <div class="text-error">{formError}</div>
    {/if}
  </div>

  <div class="card-footer flex justify-end gap-2">
    <Button variant="ghost" onclick={close} disabled={isSubmitting}
      >{m.form_new_model_cancel()}</Button
    >
    <Button variant="default" onclick={handleSubmit} disabled={isSubmitting}>
      {isSubmitting ? m.wishlist_modal_saving() : m.form_new_model_create()}
    </Button>
  </div>
</div>
