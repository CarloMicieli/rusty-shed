<script lang="ts">
  import CurrencyInput from '$lib/components/CurrencyInput.svelte';

  interface Props {
    label: string;
    value?: number | null;
    symbol: string;
    id?: string;
    placeholder?: string;
    disabled?: boolean;
    error?: string;
    required?: boolean;
    inputClass?: string;
    onchange?: (value: number | null) => void;
  }

  let { label, value = $bindable<number | null>(null), error, required, ...rest }: Props = $props();
</script>

<div class="space-y-1">
  <span class="text-xs text-zinc-400">{label}{required ? ' *' : ''}</span>
  <CurrencyInput bind:value {label} {...rest} />
  {#if error}
    <p class="text-xs text-destructive">{error}</p>
  {/if}
</div>
