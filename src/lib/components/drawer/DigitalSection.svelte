<script lang="ts">
  import * as m from '$lib/paraglide/messages.js';
  import { DrawerSectionBar, DrawerInput } from '$lib/components/drawer';
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
    expanded?: boolean;
  }

  let {
    dccAddress = $bindable(),
    installationDate = $bindable(),
    onAddressChange,
    duplicateWarning = null,
    errors = {},
    touched = false,
    disabled = false,
    expanded = $bindable(true)
  }: Props = $props();
</script>

<div class="space-y-4">
  <DrawerSectionBar
    label={m.drawer_section_digital()}
    {expanded}
    onToggle={() => (expanded = !expanded)}
  />

  {#if expanded}
    <!-- DCC Address -->
    <div class="space-y-1">
      <label for="digital-section-dcc-address" class="block">
        <span class="text-sm text-muted-foreground">{m.digital_roster_address_label()}</span>
      </label>
      <DrawerInput
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
        placeholder="1-9999"
        {disabled}
      />
      {#if touched && errors.address}
        <p class="text-error-500 mt-1 text-xs">{errors.address}</p>
      {/if}
      {#if duplicateWarning}
        <p class="text-warning-500 mt-1 text-xs">{duplicateWarning}</p>
      {/if}
    </div>

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
  {/if}
</div>
