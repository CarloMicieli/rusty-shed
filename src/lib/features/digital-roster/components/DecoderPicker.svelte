<script lang="ts">
  import * as m from '$lib/paraglide/messages.js';
  import * as Select from '$lib/components/ui/select';
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

  const selectedDecoder = $derived(decoders.find((d) => d.id === selectedId) ?? null);

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
  <Select.Root
    type="single"
    value={selectedId ?? undefined}
    onValueChange={(v) => onChange(v || null)}
  >
    <Select.Trigger id="decoder" class="w-full" data-error={touched && !!error}>
      {#if selectedDecoder}
        {formatLabel(selectedDecoder)}
      {:else}
        <span class="text-muted-foreground">{m.form_new_model_select_placeholder()}</span>
      {/if}
    </Select.Trigger>
    <Select.Content>
      {#each decoders as decoder (decoder.id)}
        <Select.Item value={decoder.id} label={formatLabel(decoder)} />
      {/each}
    </Select.Content>
  </Select.Root>
  {#if touched && error}
    <p class="text-error-500 mt-1 text-xs">{error}</p>
  {/if}
</div>
