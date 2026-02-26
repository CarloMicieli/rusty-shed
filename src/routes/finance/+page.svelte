<script lang="ts">
  import { onMount } from 'svelte';
  import { Settings2, CalendarDays, TrendingUp } from 'lucide-svelte';
  import * as m from '$lib/paraglide/messages.js';
  import { getModalStore } from '$lib/stores/modal';

  // State & Services
  import { createBudgetService } from '$lib/features/budget/services/BudgetService.svelte';
  import { createBudgetState } from '$lib/features/budget/BudgetState.svelte';

  // Shared Components
  import { Card, CardHeader, CardTitle, CardContent, Button, PageHeader } from '$lib/components';

  // Feature Components
  import BudgetConfigSheet from '$lib/features/budget/components/BudgetConfigSheet.svelte';
  import BudgetMonthRow from '$lib/features/budget/components/BudgetMonthRow.svelte';
  import ExtraBudgetModal from '$lib/features/budget/components/ExtraBudgetModal.svelte';

  const service = createBudgetService();
  const budgetState = createBudgetState(service);
  const modalStore = getModalStore();

  // UI Local State
  let selectedYear = $state(new Date().getFullYear());
  let configSheetOpen = $state(false);

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
    modalStore.trigger({
      type: 'component',
      component: {
        ref: ExtraBudgetModal,
        props: {
          budgetState,
          year,
          month
        }
      }
    });
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
      <div class="grid gap-6 md:grid-cols-3">
        <Card class="border-border bg-card">
          <CardHeader class="pb-2">
            <CardTitle
              class="flex items-center gap-2 text-[10px] font-bold tracking-widest text-muted-foreground uppercase"
            >
              <CalendarDays class="h-3 w-3 text-primary" />
              Monthly Allocation
            </CardTitle>
          </CardHeader>
          <CardContent>
            <div class="font-mono text-2xl font-bold">
              {budgetState.formattedMonthlyBudget}
            </div>
          </CardContent>
        </Card>

        <Card class="border-border bg-card">
          <CardHeader class="pb-2">
            <CardTitle
              class="flex items-center gap-2 text-[10px] font-bold tracking-widest text-muted-foreground uppercase"
            >
              <TrendingUp class="h-3 w-3 text-primary" />
              Yearly Forecast
            </CardTitle>
          </CardHeader>
          <CardContent>
            <div class="font-mono text-2xl font-bold">
              {budgetState.formattedYearlyBudget}
            </div>
          </CardContent>
        </Card>

        <Card class="border-border bg-card">
          <CardContent class="pt-6">
            <Button variant="outline" class="w-full" onclick={() => (configSheetOpen = true)}>
              <Settings2 size={16} class="mr-2" />
              System Config
            </Button>
          </CardContent>
        </Card>
      </div>

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
              {#each Array.from({ length: 5 }, (_, i) => new Date().getFullYear() - i) as year (year)}
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
                    isCurrent={record.year === new Date().getFullYear() &&
                      record.month === new Date().getMonth() + 1}
                    onExtra={openExtraBudget}
                  />
                {/each}
              </tbody>
            </table>
          </div>
        </CardContent>
      </Card>
    {:else}
      <div
        class="flex flex-col items-center justify-center rounded-xl border border-dashed border-border py-32"
      >
        <p class="mb-6 font-mono text-sm text-muted-foreground">NO_BUDGET_CONFIG_FOUND</p>
        <Button onclick={() => (configSheetOpen = true)}>Initialize System Budget</Button>
      </div>
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
