<script lang="ts">
  interface Props {
    label?: string;
    value?: string | null;
    onchange?: (value: string | null) => void;
    disabled?: boolean;
    error?: string;
  }

  let {
    label = 'Epoch',
    value = null,
    onchange,
    disabled = false,
    error = undefined
  }: Props = $props();
</script>

<div aria-disabled={disabled} data-error={error ? 'true' : 'false'}>
  <span>{label}</span>
  <button type="button" onclick={() => onchange?.(value)} aria-label="epoch stub">Epoch</button>
</div>
