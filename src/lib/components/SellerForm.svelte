<script lang="ts">
  import { untrack } from 'svelte';
  import * as m from '$lib/paraglide/messages.js';
  import { superForm } from 'sveltekit-superforms';
  import { zodClient } from '$lib/vendor/superforms-adapters';
  import { sellerSchema } from '$lib/schemas/seller';
  import * as Form from '$lib/components/ui/form';
  import type { ControlAttrs } from 'formsnap';
  import { Button } from '$lib/components/ui/button';
  import { Input } from '$lib/components/ui/input';
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
              toaster.success(m.seller_form_save_success());
              onSaved(res.data);
              onClose();
            } else {
              toaster.error(m.seller_form_save_failed());
            }
          } catch {
            toaster.error(m.seller_form_error_unexpected());
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
    <h3 class="text-base font-semibold">
      {$form.id ? m.seller_form_edit_title() : m.seller_form_new_title()}
    </h3>
    <Button variant="ghost" onclick={close}>×</Button>
  </div>

  <form method="POST" use:enhance>
    <div class="card-body space-y-3">
      <Form.Field form={formObj} name="name">
        <Form.Control>
          {#snippet children({ props }: { props: ControlAttrs })}
            <Form.Label required>{m.seller_form_field_name()}</Form.Label>
            <Input
              {...props}
              bind:value={$form.name}
              placeholder={m.seller_form_placeholder_name()}
            />
          {/snippet}
        </Form.Control>
        <Form.FieldErrors />
      </Form.Field>

      <Form.Field form={formObj} name="sellerType">
        <Form.Control>
          {#snippet children({ props }: { props: ControlAttrs })}
            <Form.Label required>{m.seller_form_field_seller_type()}</Form.Label>
            <select
              {...props}
              bind:value={$form.sellerType}
              class="h-9 w-full rounded-md border border-input bg-background px-3 py-2 text-sm transition-colors outline-none focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/40"
            >
              <option value="SHOP">{m.seller_form_type_shop()}</option>
              <option value="PRIVATE">{m.seller_form_type_private()}</option>
              <option value="MANUFACTURER">{m.seller_form_type_manufacturer()}</option>
            </select>
          {/snippet}
        </Form.Control>
        <Form.FieldErrors />
      </Form.Field>

      <Form.Field form={formObj} name="email">
        <Form.Control>
          {#snippet children({ props }: { props: ControlAttrs })}
            <Form.Label>{m.seller_form_field_email()}</Form.Label>
            <Input
              {...props}
              bind:value={$form.email}
              placeholder={m.seller_form_placeholder_email()}
              type="email"
            />
          {/snippet}
        </Form.Control>
        <Form.FieldErrors />
      </Form.Field>

      <Form.Field form={formObj} name="phone">
        <Form.Control>
          {#snippet children({ props }: { props: ControlAttrs })}
            <Form.Label>{m.seller_form_field_phone()}</Form.Label>
            <Input
              {...props}
              bind:value={$form.phone}
              placeholder={m.seller_form_placeholder_phone()}
            />
          {/snippet}
        </Form.Control>
        <Form.FieldErrors />
      </Form.Field>

      <Form.Field form={formObj} name="websiteUrl">
        <Form.Control>
          {#snippet children({ props }: { props: ControlAttrs })}
            <Form.Label>{m.seller_form_field_website()}</Form.Label>
            <Input
              {...props}
              bind:value={$form.websiteUrl}
              placeholder={m.seller_form_placeholder_website()}
              type="url"
            />
          {/snippet}
        </Form.Control>
        <Form.FieldErrors />
      </Form.Field>

      <Form.Field form={formObj} name="streetAddress">
        <Form.Control>
          {#snippet children({ props }: { props: ControlAttrs })}
            <Form.Label>{m.seller_form_field_street_address()}</Form.Label>
            <Input
              {...props}
              bind:value={$form.streetAddress}
              placeholder={m.seller_form_placeholder_street_address()}
            />
          {/snippet}
        </Form.Control>
        <Form.FieldErrors />
      </Form.Field>

      <Form.Field form={formObj} name="extendedAddress">
        <Form.Control>
          {#snippet children({ props }: { props: ControlAttrs })}
            <Form.Label>{m.seller_form_field_extended_address()}</Form.Label>
            <Input
              {...props}
              bind:value={$form.extendedAddress}
              placeholder={m.seller_form_placeholder_extended_address()}
            />
          {/snippet}
        </Form.Control>
        <Form.FieldErrors />
      </Form.Field>

      <div class="grid grid-cols-3 gap-2">
        <Form.Field form={formObj} name="city">
          <Form.Control>
            {#snippet children({ props }: { props: ControlAttrs })}
              <Form.Label>{m.seller_form_field_city()}</Form.Label>
              <Input
                {...props}
                bind:value={$form.city}
                placeholder={m.seller_form_placeholder_city()}
              />
            {/snippet}
          </Form.Control>
          <Form.FieldErrors />
        </Form.Field>

        <Form.Field form={formObj} name="stateRegion">
          <Form.Control>
            {#snippet children({ props }: { props: ControlAttrs })}
              <Form.Label>{m.seller_form_field_region()}</Form.Label>
              <Input
                {...props}
                bind:value={$form.stateRegion}
                placeholder={m.seller_form_placeholder_region()}
              />
            {/snippet}
          </Form.Control>
          <Form.FieldErrors />
        </Form.Field>

        <Form.Field form={formObj} name="postalCode">
          <Form.Control>
            {#snippet children({ props }: { props: ControlAttrs })}
              <Form.Label>{m.seller_form_field_postal_code()}</Form.Label>
              <Input
                {...props}
                bind:value={$form.postalCode}
                placeholder={m.seller_form_placeholder_postal_code()}
              />
            {/snippet}
          </Form.Control>
          <Form.FieldErrors />
        </Form.Field>
      </div>

      <Form.Field form={formObj} name="countryCode">
        <Form.Control>
          {#snippet children({ props }: { props: ControlAttrs })}
            <Form.Label>{m.seller_form_field_country_code()}</Form.Label>
            <Input
              {...props}
              bind:value={$form.countryCode}
              placeholder={m.seller_form_placeholder_country_code()}
            />
          {/snippet}
        </Form.Control>
        <Form.FieldErrors />
      </Form.Field>

      {#if hasUnsavedChanges}
        <p class="text-warning text-sm">{m.seller_form_unsaved_changes()}</p>
      {/if}
    </div>

    <div class="card-footer flex justify-end gap-2">
      <Button variant="ghost" type="button" onclick={close} disabled={$submitting}
        >{m.form_new_model_cancel()}</Button
      >
      <Button variant="default" type="submit" disabled={$submitting}>
        {$submitting
          ? m.wishlist_modal_saving()
          : $form.id
            ? m.seller_form_submit_update()
            : m.seller_form_submit_create()}
      </Button>
    </div>
  </form>
</div>
