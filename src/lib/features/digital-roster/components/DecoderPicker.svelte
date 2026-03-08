<script lang="ts">
  import * as m from '$lib/paraglide/messages.js';
  import type { Decoder, Manufacturer } from '$lib/bindings';

  interface Props {
    decoders: Decoder[];
    manufacturers: Manufacturer[];
    selectedId: string | null;
    error?: string;
    touched?: boolean;
    onChange: (id: string | null) => void;
  }

  const { decoders, manufacturers, selectedId, error, touched = false, onChange }: Props = $props();

  function getManufacturerName(manufacturerId: string): string {
    return manufacturers.find((m) => m.id === manufacturerId)?.name ?? manufacturerId;
  }

  function formatLabel(decoder: Decoder): string {
    const manufacturer = getManufacturerName(decoder.manufacturerId);
    return `${manufacturer} ${decoder.productCode} (${decoder.decoderType})`;
  }
</script>

<div>
  <label for="decoder" class="block space-y-1">
    <span class="text-sm text-muted-foreground">{m.digital_roster_decoder_label()}</span>
  </label>
  <select
    id="decoder"
    value={selectedId ?? ''}
    onchange={(e) => onChange(e.currentTarget.value || null)}
    class="h-9 w-full rounded-md border border-input bg-background px-3 py-2 text-sm transition-colors outline-none focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/40"
    class:input-error={touched && error}
  >
    <option value="">{m.form_new_model_select_placeholder()}</option>
    {#each decoders as decoder (decoder.id)}
      <option value={decoder.id}>
        {formatLabel(decoder)}
      </option>
    {/each}
  </select>
  {#if touched && error}
    <p class="text-error-500 mt-1 text-xs">{error}</p>
  {/if}
</div>

<style>
  .input-error {
    border-color: rgb(var(--color-error-500));
  }
</style>
