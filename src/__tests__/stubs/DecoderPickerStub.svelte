<script lang="ts">
  interface DecoderLike {
    id: string;
  }

  interface Props {
    decoders?: DecoderLike[];
    selectedId: string | null;
    onChange: (id: string | null) => void;
  }

  let { decoders = [], selectedId, onChange }: Props = $props();
</script>

<div data-testid="decoder-picker-stub">
  <p data-testid="selected-decoder-id">{selectedId ?? 'none'}</p>
  {#each decoders as decoder (decoder.id)}
    <button
      type="button"
      data-testid={`decoder-${decoder.id}`}
      onclick={() => onChange(decoder.id)}
    >
      {decoder.id}
    </button>
  {/each}
</div>
