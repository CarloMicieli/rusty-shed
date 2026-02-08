<script lang="ts">
  import { untrack } from 'svelte';
  import * as m from '$lib/paraglide/messages.js';
  import { superForm } from 'sveltekit-superforms';
  import { zodClient } from 'sveltekit-superforms/adapters';
  import { sellerSchema } from '$lib/schemas/seller';
  import * as Form from '$lib/components/ui/form';
  import type { ControlAttrs } from 'formsnap';
  import { Button, Input } from '$lib/components';
  import * as sellerService from '$lib/services/sellerService';
  import type { FormSeller } from '$lib/services/sellerAdapter';
  import { toaster } from '$lib/toaster';

  interface Props {
    onClose?: () => void;
    onSaved?: (seller: FormSeller) => void;
    initial?: FormSeller | null;
  }

  let { onClose = () => {}, onSaved = () => {}, initial = null }: Props = $props();

  // Capture initial values as snapshot to avoid Svelte 5 reactivity warnings
  const initialSnapshot = untrack(() => $state.snapshot(initial));

  const formObj = superForm(
    {
      id: initialSnapshot?.id,
      name: initialSnapshot?.name ?? '',
      sellerType: initialSnapshot?.sellerType ?? 'SHOP',
      email: initialSnapshot?.email ?? '',
      phone: initialSnapshot?.phone ?? '',
      websiteUrl: initialSnapshot?.websiteUrl ?? '',
      streetAddress: initialSnapshot?.streetAddress ?? '',
      extendedAddress: initialSnapshot?.extendedAddress ?? '',
      city: initialSnapshot?.city ?? '',
      stateRegion: initialSnapshot?.stateRegion ?? '',
      postalCode: initialSnapshot?.postalCode ?? '',
      countryCode: initialSnapshot?.countryCode ?? ''
    },
    {
      SPA: true,
      dataType: 'json',
      // eslint-disable-next-line @typescript-eslint/no-explicit-any
      validators: zodClient(sellerSchema as any),
      onUpdate: async ({ form: formData }) => {
        if (formData.valid) {
          try {
            const payload: FormSeller = {
              ...formData.data,
              email: formData.data.email || null,
              phone: formData.data.phone || null,
              websiteUrl: formData.data.websiteUrl || null,
              streetAddress: formData.data.streetAddress || null,
              extendedAddress: formData.data.extendedAddress || null,
              city: formData.data.city || null,
              stateRegion: formData.data.stateRegion || null,
              postalCode: formData.data.postalCode || null,
              countryCode: formData.data.countryCode || null
            };

            const res = payload.id
              ? await sellerService.updateSeller(payload)
              : await sellerService.createSeller(payload);

            if (res.status === 'ok') {
              toaster.success('Seller saved successfully');
              onSaved(res.data);
              onClose();
            } else {
              toaster.error('Failed to save seller');
            }
          } catch {
            toaster.error('An unexpected error occurred');
          }
        }
      }
    }
  );

  const { form, enhance, submitting, tainted } = formObj;

  const hasUnsavedChanges = $derived(
    typeof $tainted === 'boolean' ? $tainted : Object.keys($tainted ?? {}).length > 0
  );

  function close() {
    onClose();
  }
</script>

<div class="card">
  <div class="card-header">
    <h3 class="text-base font-semibold">{$form.id ? 'Edit Seller' : 'New Seller'}</h3>
    <Button variant="ghost" onclick={close}>×</Button>
  </div>

  <form method="POST" use:enhance>
    <div class="card-body space-y-3">
      <Form.Field form={formObj} name="name">
        <Form.Control>
          {#snippet children({ props }: { props: ControlAttrs })}
            <Form.Label required>Name</Form.Label>
            <Input {...props} bind:value={$form.name} placeholder="Seller name" />
          {/snippet}
        </Form.Control>
        <Form.FieldErrors />
      </Form.Field>

      <Form.Field form={formObj} name="sellerType">
        <Form.Control>
          {#snippet children({ props }: { props: ControlAttrs })}
            <Form.Label required>Seller Type</Form.Label>
            <select {...props} bind:value={$form.sellerType} class="select">
              <option value="SHOP">Shop</option>
              <option value="PRIVATE">Private</option>
              <option value="MANUFACTURER">Manufacturer</option>
            </select>
          {/snippet}
        </Form.Control>
        <Form.FieldErrors />
      </Form.Field>

      <Form.Field form={formObj} name="email">
        <Form.Control>
          {#snippet children({ props }: { props: ControlAttrs })}
            <Form.Label>Email</Form.Label>
            <Input
              {...props}
              bind:value={$form.email}
              placeholder="email@example.com"
              type="email"
            />
          {/snippet}
        </Form.Control>
        <Form.FieldErrors />
      </Form.Field>

      <Form.Field form={formObj} name="phone">
        <Form.Control>
          {#snippet children({ props }: { props: ControlAttrs })}
            <Form.Label>Phone</Form.Label>
            <Input {...props} bind:value={$form.phone} placeholder="Phone number" />
          {/snippet}
        </Form.Control>
        <Form.FieldErrors />
      </Form.Field>

      <Form.Field form={formObj} name="websiteUrl">
        <Form.Control>
          {#snippet children({ props }: { props: ControlAttrs })}
            <Form.Label>Website</Form.Label>
            <Input
              {...props}
              bind:value={$form.websiteUrl}
              placeholder="https://example.com"
              type="url"
            />
          {/snippet}
        </Form.Control>
        <Form.FieldErrors />
      </Form.Field>

      <Form.Field form={formObj} name="streetAddress">
        <Form.Control>
          {#snippet children({ props }: { props: ControlAttrs })}
            <Form.Label>Street Address</Form.Label>
            <Input {...props} bind:value={$form.streetAddress} placeholder="123 Main St" />
          {/snippet}
        </Form.Control>
        <Form.FieldErrors />
      </Form.Field>

      <Form.Field form={formObj} name="extendedAddress">
        <Form.Control>
          {#snippet children({ props }: { props: ControlAttrs })}
            <Form.Label>Extended Address</Form.Label>
            <Input {...props} bind:value={$form.extendedAddress} placeholder="Apt 4B" />
          {/snippet}
        </Form.Control>
        <Form.FieldErrors />
      </Form.Field>

      <div class="grid grid-cols-3 gap-2">
        <Form.Field form={formObj} name="city">
          <Form.Control>
            {#snippet children({ props }: { props: ControlAttrs })}
              <Form.Label>City</Form.Label>
              <Input {...props} bind:value={$form.city} placeholder="City" />
            {/snippet}
          </Form.Control>
          <Form.FieldErrors />
        </Form.Field>

        <Form.Field form={formObj} name="stateRegion">
          <Form.Control>
            {#snippet children({ props }: { props: ControlAttrs })}
              <Form.Label>Region</Form.Label>
              <Input {...props} bind:value={$form.stateRegion} placeholder="Region" />
            {/snippet}
          </Form.Control>
          <Form.FieldErrors />
        </Form.Field>

        <Form.Field form={formObj} name="postalCode">
          <Form.Control>
            {#snippet children({ props }: { props: ControlAttrs })}
              <Form.Label>Postal Code</Form.Label>
              <Input {...props} bind:value={$form.postalCode} placeholder="12345" />
            {/snippet}
          </Form.Control>
          <Form.FieldErrors />
        </Form.Field>
      </div>

      <Form.Field form={formObj} name="countryCode">
        <Form.Control>
          {#snippet children({ props }: { props: ControlAttrs })}
            <Form.Label>Country Code</Form.Label>
            <Input {...props} bind:value={$form.countryCode} placeholder="US" />
          {/snippet}
        </Form.Control>
        <Form.FieldErrors />
      </Form.Field>

      {#if hasUnsavedChanges}
        <p class="text-warning text-sm">You have unsaved changes</p>
      {/if}
    </div>

    <div class="card-footer flex justify-end gap-2">
      <Button variant="ghost" type="button" onclick={close} disabled={$submitting}
        >{m.form_new_model_cancel()}</Button
      >
      <Button variant="default" type="submit" disabled={$submitting}>
        {$submitting ? m.wishlist_modal_saving() : m.form_new_model_create()}
      </Button>
    </div>
  </form>
</div>
