<script lang="ts">
  import { onMount } from 'svelte';
  import {
    DigitalRosterController,
    DigitalRosterState,
    setDigitalRosterContext
  } from '$lib/features/digital-roster';
  import DigitalSummary from '$lib/features/digital-roster/components/DigitalSummary.svelte';
  import DigitalRosterTable from '$lib/features/digital-roster/components/DigitalRosterTable.svelte';
  import DecoderInstallDrawer from '$lib/features/digital-roster/components/DecoderInstallDrawer.svelte';
  import type { DigitalRollingStockView } from '$lib/bindings';
  import * as m from '$lib/paraglide/messages';
  import { Plus } from 'lucide-svelte';
  import { Button, PageHeader } from '$lib/components';

  // Initialize state and controller
  const rosterState = new DigitalRosterState();
  const controller = new DigitalRosterController(rosterState);

  // Set context for child components
  setDigitalRosterContext(controller);

  // Drawer state
  let installDrawerOpen = $state(false);

  onMount(async () => {
    await controller.loadAll();
  });

  function handleFilterChange(text: string) {
    rosterState.setFilterText(text);
  }

  async function handleSaveAddress(id: string, newAddress: number): Promise<boolean> {
    const success = await controller.changeDccAddress(id, newAddress);
    if (success) await controller.loadRollingStocks();
    return success;
  }

  function handleChangeDecoder(_stock: DigitalRollingStockView) {
    installDrawerOpen = true;
  }

  function handleDelete(stock: DigitalRollingStockView) {
    // TODO: implement delete command
    console.warn('Delete not yet implemented for', stock.id);
  }

  function openInstallDrawer() {
    installDrawerOpen = true;
  }

  function closeInstallDrawer() {
    installDrawerOpen = false;
  }

  function handleInstallSuccess() {
    closeInstallDrawer();
  }

  const isRosterEmpty = $derived(rosterState.rollingStocks.length === 0);
</script>

<svelte:head>
  <title>{m.app_digital_roster()} - {m.app_name()}</title>
</svelte:head>

<div class="flex flex-col">
  <div
    class="-mx-4 -mt-4 mb-6 rounded-tl-[24px] border-b border-border bg-card/50 px-6 py-4 lg:-mx-8 lg:-mt-8 lg:mb-8"
  >
    <PageHeader
      title={m.app_digital_roster()}
      subtitle={m.app_digital_dcc()}
      description={m.depot_subtitle()}
    >
      {#snippet actions()}
        {#if !isRosterEmpty}
          <Button size="sm" type="button" onclick={openInstallDrawer}>
            <Plus size={18} class="mr-2" />
            <span>{m.digital_roster_install_decoder()}</span>
          </Button>
        {/if}
      {/snippet}
    </PageHeader>
  </div>

  {#if !isRosterEmpty && !rosterState.isLoading}
    <div class="mb-6 px-4 lg:px-8">
      <DigitalSummary summary={rosterState.summary} loading={rosterState.isLoading} />
    </div>
  {/if}

  <div class="space-y-6">
    <!-- Roster Table -->
    <DigitalRosterTable
      rollingStocks={rosterState.filteredRollingStocks}
      filterText={rosterState.filterText}
      loading={rosterState.isLoading}
      onFilterChange={handleFilterChange}
      onInstallDecoder={openInstallDrawer}
      onSaveAddress={handleSaveAddress}
      onChangeDecoder={handleChangeDecoder}
      onDelete={handleDelete}
    />
  </div>
</div>

<!-- Decoder Install Drawer -->
<DecoderInstallDrawer
  open={installDrawerOpen}
  onClose={closeInstallDrawer}
  onSuccess={handleInstallSuccess}
/>
