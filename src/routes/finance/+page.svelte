<script lang="ts">
  import { onMount } from 'svelte';
  import { Settings2, CalendarDays, TrendingUp, Wallet } from 'lucide-svelte';
  import GaugeStatCard from '$lib/components/GaugeStatCard.svelte';
  import * as m from '$lib/paraglide/messages.js';
  import * as Dialog from '$lib/components/ui/dialog';

  // State & Services
  import { createBudgetService } from '$lib/features/budget/services/BudgetService.svelte';
  import { createBudgetState } from '$lib/features/budget/BudgetState.svelte';

  // Shared Components
  import {
    Card,
    CardHeader,
    CardTitle,
    CardContent,
    Button,
    PageHeader,
    EmptyState
  } from '$lib/components';

  // Feature Components
  import BudgetConfigSheet from '$lib/features/budget/components/BudgetConfigSheet.svelte';
  import BudgetMonthRow from '$lib/features/budget/components/BudgetMonthRow.svelte';
  import ExtraBudgetModal from '$lib/features/budget/components/ExtraBudgetModal.svelte';

  const service = createBudgetService();
  const budgetState = createBudgetState(service);
  // UI Local State
  let selectedYear = $state(new Date().getFullYear());
  let configSheetOpen = $state(false);
  let extraBudgetDialogOpen = $state(false);
  let selectedExtraBudget = $state<{ year: number; month: number } | null>(null);

  const monthNames = [
    'January',
    'February',
    'March',
    'April',
    'May',
    'June',
    'July',
    'August',
    'September',
    'October',
    'November',
    'December'
  ];

  const now = new Date();
  const currentYear = now.getFullYear();
  const currentMonth = now.getMonth() + 1;
  const yearOptions = Array.from({ length: 5 }, (_, i) => currentYear - i);

  onMount(async () => {
    await budgetState.load();
    if (budgetState.hasConfig) {
      await budgetState.loadMonthlyRecords(selectedYear);
    }
  });

  async function handleYearChange(year: number) {
    selectedYear = year;
    await budgetState.loadMonthlyRecords(year);
  }

  function openExtraBudget(year: number, month: number) {
    selectedExtraBudget = { year, month };
    extraBudgetDialogOpen = true;
  }

  function closeExtraBudgetDialog() {
    extraBudgetDialogOpen = false;
    selectedExtraBudget = null;
  }
</script>

<svelte:head>
  <title>{m.app_name()} | {m.budget_title()}</title>
</svelte:head>

<div class="flex flex-col">
  <div
    class="-mx-4 -mt-4 mb-6 border-b border-border bg-card/50 px-6 py-4 lg:-mx-8 lg:-mt-8 lg:mb-8"
  >
    <PageHeader
      title={m.budget_title()}
      subtitle={m.app_finance()}
      description={m.budget_subtitle()}
    />
  </div>

  <div class="space-y-6">
    {#if budgetState.isLoading}
      <div class="flex flex-col items-center justify-center gap-4 py-24">
        <div
          class="h-10 w-10 animate-spin rounded-full border-4 border-muted border-t-primary"
        ></div>
        <p class="font-mono text-xs tracking-widest text-muted-foreground uppercase">
          {m.budget_loading()}
        </p>
      </div>
    {:else if budgetState.hasConfig}
      <div
        class={[
          'relative transition-opacity duration-150',
          budgetState.isTransitioning && 'pointer-events-none opacity-50'
        ]}
      >
        {#if budgetState.isTransitioning}
          <div class="absolute inset-0 z-10 flex items-center justify-center">
            <div
              class="h-5 w-5 animate-spin rounded-full border-2 border-muted border-t-primary"
            ></div>
          </div>
        {/if}

        <div class="grid gap-4 md:grid-cols-3">
          <GaugeStatCard
            label="Monthly Allocation"
            value={budgetState.formattedMonthlyBudget}
            icon={CalendarDays}
          />
          <GaugeStatCard
            label="Yearly Forecast"
            value={budgetState.formattedYearlyBudget}
            icon={TrendingUp}
          />
          <Card class="border-border bg-card">
            <CardContent class="pt-6">
              <Button variant="outline" class="w-full" onclick={() => (configSheetOpen = true)}>
                <Settings2 size={16} class="mr-2" />
                System Config
              </Button>
            </CardContent>
          </Card>
        </div>

        <div class="mt-6">
          <Card class="border-border bg-card">
            <CardHeader class="border-b border-border/50 pb-4">
              <div class="flex items-center justify-between">
                <CardTitle class="text-sm font-bold tracking-tight uppercase"
                  >Ledger_{selectedYear}</CardTitle
                >
                <select
                  bind:value={selectedYear}
                  onchange={() => handleYearChange(selectedYear)}
                  class="h-8 rounded border border-border bg-card px-2 font-mono text-xs text-foreground outline-none focus:border-primary/50"
                >
                  {#each yearOptions as year (year)}
                    <option value={year}>{year}</option>
                  {/each}
                </select>
              </div>
            </CardHeader>

            <CardContent class="p-0">
              <div class="overflow-x-auto">
                <table class="w-full border-collapse text-left">
                  <thead>
                    <tr
                      class="border-b border-border bg-card/30 text-[9px] font-bold tracking-widest text-muted-foreground uppercase"
                    >
                      <th class="px-4 py-3">Month</th>
                      <th class="px-4 py-3 text-right">Base</th>
                      <th class="px-4 py-3 text-right">Extra</th>
                      <th class="px-4 py-3 text-right">Available</th>
                      <th class="px-4 py-3 text-right">Spent</th>
                      <th class="px-4 py-3 text-right">Remaining</th>
                      <th class="px-4 py-3">Status</th>
                      <th class="px-4 py-3 text-right">Actions</th>
                    </tr>
                  </thead>
                  <tbody>
                    {#each budgetState.enhancedMonthlyRecords as record (record.month)}
                      <BudgetMonthRow
                        {record}
                        monthName={monthNames[record.month - 1]}
                        isCurrent={record.year === currentYear && record.month === currentMonth}
                        onExtra={openExtraBudget}
                      />
                    {/each}
                  </tbody>
                </table>
              </div>
            </CardContent>
          </Card>
        </div>
      </div>
    {:else}
      <EmptyState
        icon={Wallet}
        title={m.budget_empty_state_title()}
        description={m.budget_empty_state_message()}
        ctaLabel={m.dashboard_chart_budget_set_cta()}
        onCta={() => (configSheetOpen = true)}
      />
    {/if}
  </div>
</div>

<BudgetConfigSheet
  bind:open={configSheetOpen}
  onsubmit={async (mode, amount) => {
    configSheetOpen = false;
    await budgetState.save(mode, amount, budgetState.currency);
  }}
/>

{#if selectedExtraBudget}
  <Dialog.Root bind:open={extraBudgetDialogOpen}>
    <Dialog.Content class="max-w-md">
      <ExtraBudgetModal
        {budgetState}
        year={selectedExtraBudget.year}
        month={selectedExtraBudget.month}
        onClose={closeExtraBudgetDialog}
      />
    </Dialog.Content>
  </Dialog.Root>
{/if}
