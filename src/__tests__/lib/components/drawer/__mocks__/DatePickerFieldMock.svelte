<script lang="ts">
  interface Props {
    id?: string;
    value?: string | null;
    maxValue?: unknown;
    disabled?: boolean;
  }

  let { id, value = $bindable(''), disabled = false, maxValue: _maxValue }: Props = $props();
</script>

<input {id} type="date" bind:value {disabled} data-testid="date-picker-mock" />
