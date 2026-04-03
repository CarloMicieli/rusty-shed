<script lang="ts">
  import { onMount } from 'svelte';
  import * as Select from '$lib/components/ui/select';
  import { commands } from '$lib/bindings';
  import type { CouplerType } from '$lib/bindings';

  interface Props {
    /** CouplingSocket value used to filter compatible couplers (e.g. "NEM_362"). */
    compatibleSocket: string | null;
    /** The TRN of the currently installed coupler, if any. */
    currentCouplerTypeId: string | null;
    /** ID of the owned rolling stock to update when the selection changes. */
    ownedRollingStockId: string;
    /** Called after a successful save so the parent can refresh its local state. */
    onChange?: (id: string | null) => void;
  }

  const { compatibleSocket, currentCouplerTypeId, ownedRollingStockId, onChange }: Props = $props();

  let couplers = $state<CouplerType[]>([]);
  let saving = $state(false);

  const selectedCoupler = $derived(couplers.find((c) => c.id === currentCouplerTypeId) ?? null);

  onMount(async () => {
    const result = await commands.getCouplerTypes(compatibleSocket);
    if (result.status === 'ok') {
      couplers = result.data;
    }
  });

  async function handleChange(value: string | undefined) {
    const newId = value || null;
    saving = true;
    try {
      const result = await commands.setRollingStockCoupler({
        ownedRollingStockId,
        couplerTypeId: newId
      });
      if (result.status === 'ok') {
        onChange?.(newId);
      }
    } finally {
      saving = false;
    }
  }
</script>

<Select.Root
  type="single"
  value={currentCouplerTypeId ?? undefined}
  onValueChange={handleChange}
  disabled={saving}
>
  <Select.Trigger class="w-full">
    {#if selectedCoupler}
      <span>{selectedCoupler.manufacturer} {selectedCoupler.name}</span>
    {:else}
      <span class="text-muted-foreground">—</span>
    {/if}
  </Select.Trigger>
  <Select.Content>
    <Select.Item value="" label="—" />
    {#each couplers as coupler (coupler.id)}
      <Select.Item value={coupler.id} label="{coupler.manufacturer} {coupler.name}" />
    {/each}
  </Select.Content>
</Select.Root>
