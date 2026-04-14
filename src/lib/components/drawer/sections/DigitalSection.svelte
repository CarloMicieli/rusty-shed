<script lang="ts">
  import type { Snippet } from 'svelte';
  import * as m from '$lib/paraglide/messages.js';
  import { FormInput } from '$lib/components/drawer';
  import { DatePickerField } from '$lib/components';
  import { today, getLocalTimeZone } from '@internationalized/date';

  interface Props {
    dccAddress: number | null;
    installationDate: string | null;
    onAddressChange?: (addr: number | null) => Promise<void>;
    duplicateWarning?: string | null;
    errors?: { address?: string };
    touched?: boolean;
    disabled?: boolean;
    children?: Snippet;
  }

  let {
    dccAddress = $bindable(),
    installationDate = $bindable(),
    onAddressChange,
    duplicateWarning = null,
    errors = {},
    touched = false,
    disabled = false,
    children
  }: Props = $props();
</script>

<div class="overflow-hidden rounded-lg border border-layout-border bg-layout-surface p-4">
  <section>
    <div class="mb-4 flex items-center justify-between">
      <p class="text-[10px] font-bold tracking-[0.2em] text-muted-foreground uppercase">
        {m.drawer_section_digital()}
      </p>
    </div>

    <div class="space-y-4">
      {#if children}
        {@render children()}
      {/if}

      <!-- DCC Address -->
      <FormInput
        label={m.digital_roster_address_label()}
        id="digital-section-dcc-address"
        type="number"
        min="1"
        max="9999"
        value={dccAddress ? String(dccAddress) : ''}
        oninput={async (e) => {
          const val = parseInt((e.currentTarget as HTMLInputElement).value) || null;
          dccAddress = val;
          await onAddressChange?.(val);
        }}
        placeholder={m.digital_roster_address_range_hint()}
        {disabled}
        error={touched && errors.address ? errors.address : undefined}
      />
      {#if duplicateWarning}
        <p class="text-warning-500 mt-1 text-xs">{duplicateWarning}</p>
      {/if}

      <!-- Installation Date -->
      <div class="space-y-1">
        <label for="digital-section-install-date" class="block">
          <span class="text-sm text-muted-foreground">{m.digital_roster_date_label()}</span>
        </label>
        <DatePickerField
          id="digital-section-install-date"
          bind:value={installationDate}
          maxValue={today(getLocalTimeZone())}
          {disabled}
        />
      </div>
    </div>
  </section>
</div>
