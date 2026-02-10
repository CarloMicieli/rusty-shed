<script lang="ts">
  import { onMount } from 'svelte';
  import {
    DigitalRosterController,
    DigitalRosterState,
    setDigitalRosterContext
  } from '$lib/features/digital-roster';
  import DigitalSummary from '$lib/features/digital-roster/components/DigitalSummary.svelte';
  import DigitalRosterTable from '$lib/features/digital-roster/components/DigitalRosterTable.svelte';
  import DccAddressEditor from '$lib/features/digital-roster/components/DccAddressEditor.svelte';
  import DecoderInstallDrawer from '$lib/features/digital-roster/components/DecoderInstallDrawer.svelte';
  import type { DigitalRollingStockView } from '$lib/bindings';
  import * as m from '$lib/paraglide/messages';
  import { Plus } from 'lucide-svelte';
  import { Button } from '$lib/components';

  // Initialize state and controller
  const rosterState = new DigitalRosterState();
  const controller = new DigitalRosterController(rosterState);

  // Set context for child components
  setDigitalRosterContext(controller);

  // Modal state
  let editModalOpen = $state(false);
  let installDrawerOpen = $state(false);
  let selectedStock: DigitalRollingStockView | null = $state(null);

  onMount(async () => {
    await controller.loadAll();
  });

  function handleFilterChange(text: string) {
    rosterState.setFilterText(text);
  }

  async function handleSaveAddress(newAddress: number): Promise<boolean> {
    if (!selectedStock) return false;
    const success = await controller.changeDccAddress(selectedStock.id, newAddress);
    if (success) {
      await controller.loadRollingStocks();
    }
    return success;
  }

  async function handleCheckDuplicate(address: number, excludeId: string) {
    return await controller.checkDuplicateAddress(address, excludeId);
  }

  function handleEdit(stock: DigitalRollingStockView) {
    selectedStock = stock;
    editModalOpen = true;
  }

  function handleCloseModal() {
    editModalOpen = false;
    selectedStock = null;
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
</script>

<svelte:head>
  <title>{m.app_digital_roster()} - {m.app_name()}</title>
</svelte:head>

<div class="container mx-auto space-y-6 p-4">
  <!-- Page Header -->
  <div class="flex flex-col gap-4 md:flex-row md:items-center md:justify-between">
    <div>
      <p class="text-sm tracking-[0.2em] text-muted-foreground uppercase">
        {m.app_digital_dcc()}
      </p>
      <h1 class="h2 font-bold">{m.app_digital_roster()}</h1>
      <p class="text-sm text-muted-foreground">
        {m.depot_subtitle()}
      </p>
    </div>
    <div class="flex flex-col gap-3 md:flex-row md:items-center">
      <Button type="button" variant="default" onclick={openInstallDrawer}>
        <Plus size={18} />
        <span>{m.digital_roster_install_decoder()}</span>
      </Button>
    </div>
  </div>

  <!-- Summary Section -->
  <DigitalSummary summary={rosterState.summary} loading={rosterState.isLoading} />

  <!-- Roster Table -->
  <DigitalRosterTable
    rollingStocks={rosterState.filteredRollingStocks}
    filterText={rosterState.filterText}
    loading={rosterState.isLoading}
    onFilterChange={handleFilterChange}
    onEdit={handleEdit}
  />
</div>

<!-- DCC Address Editor Modal -->
{#if selectedStock}
  <DccAddressEditor
    open={editModalOpen}
    stock={selectedStock}
    onSave={handleSaveAddress}
    onCheckDuplicate={handleCheckDuplicate}
    onClose={handleCloseModal}
  />
{/if}

<!-- Decoder Install Drawer -->
<DecoderInstallDrawer
  open={installDrawerOpen}
  onClose={closeInstallDrawer}
  onSuccess={handleInstallSuccess}
/>
