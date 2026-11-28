<script lang="ts">
  import type { Snippet } from 'svelte';

  interface Props {
    dccAddress: number | null;
    installationDate: string;
    duplicateWarning?: string | null;
    onAddressChange?: (address: number | null) => Promise<void> | void;
    children?: Snippet;
  }

  let {
    dccAddress = $bindable<number | null>(null),
    installationDate = $bindable<string>(''),
    duplicateWarning = null,
    onAddressChange,
    children
  }: Props = $props();

  async function handleAddressInput(event: Event) {
    const value = (event.currentTarget as HTMLInputElement).value;
    dccAddress = value === '' ? null : Number(value);
    await onAddressChange?.(dccAddress);
  }
</script>

<div data-testid="digital-section-stub">
  <input
    data-testid="dcc-address-input"
    type="number"
    value={dccAddress ?? ''}
    oninput={handleAddressInput}
  />
  <input
    data-testid="installation-date-input"
    value={installationDate}
    oninput={(event) => (installationDate = (event.currentTarget as HTMLInputElement).value)}
  />
  {#if duplicateWarning}
    <p data-testid="duplicate-warning">{duplicateWarning}</p>
  {/if}
  {@render children?.()}
</div>
