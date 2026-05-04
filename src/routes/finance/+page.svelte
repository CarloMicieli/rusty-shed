<script lang="ts">
  import { browser } from '$app/environment';
  import { CalendarDays, Gauge, TrendingUp, Wallet } from 'lucide-svelte';
  import * as Dialog from '$lib/components/ui/dialog';
  import * as m from '$lib/paraglide/messages.js';

  import { Card, CardContent, CardHeader, EmptyState, PageHeader, Skeleton } from '$lib/components';

  import { getBudgetState } from '$lib/features/budget/BudgetState.svelte';
  import BudgetConfigSheet from '$lib/features/budget/components/BudgetConfigSheet.svelte';
  import BudgetMonthRow from '$lib/features/budget/components/BudgetMonthRow.svelte';
  import ExtraBudgetModal from '$lib/features/budget/components/ExtraBudgetModal.svelte';
  import FinanceSettingsButton from '$lib/features/budget/components/FinanceSettingsButton.svelte';
  import { financeState } from '$lib/state/finance.svelte';

  const SELECTED_YEAR_STORAGE_KEY = 'finance:selected-year';
  const gaugeRadius = 48;
  const gaugeCircumference = 2 * Math.PI * gaugeRadius;

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

  function getInitialSelectedYear(): number {
    if (!browser) {
      return currentYear;
    }

    try {
      const storedYear = window.localStorage.getItem(SELECTED_YEAR_STORAGE_KEY);
      const parsedYear = Number(storedYear);
      return yearOptions.includes(parsedYear) ? parsedYear : currentYear;
    } catch (error) {
      console.warn('[finance] Failed to read selected year', error);
      return currentYear;
    }
  }

  const budgetState = getBudgetState();
  const initialSelectedYear = getInitialSelectedYear();

  let selectedYear = $state(initialSelectedYear);
  let configSheetOpen = $state(false);
  let extraBudgetDialogOpen = $state(false);
  let selectedExtraBudget = $state<{ year: number; month: number } | null>(null);
  const initialized = $derived(financeState.hasFetched || financeState.error !== null);

  const financialSummary = $derived.by(() => {
    const dashboardSummary = financeState.data;
    const monthlyRecords = budgetState.monthlyRecords ?? [];

    const totalAvailable =
      dashboardSummary?.totalAvailable ??
      monthlyRecords.reduce((sum, record) => sum + record.available, 0);
    const remainingAmount =
      dashboardSummary?.remainingAmount ??
      monthlyRecords.reduce((sum, record) => sum + record.remaining, 0);
    const rawRemainingPercentage =
      dashboardSummary?.remainingPercentage ??
      (totalAvailable > 0 ? (remainingAmount / totalAvailable) * 100 : 0);
    const remainingPercentage = Math.max(0, Math.min(100, Math.round(rawRemainingPercentage)));

    return {
      monthlyAllocation: budgetState.formattedMonthlyBudget,
      yearlyForecast: budgetState.formattedYearlyBudget,
      remainingAmount: budgetState.formatAmount(remainingAmount),
      remainingPercentage,
      totalAvailable: budgetState.formatAmount(totalAvailable)
    };
  });

  const remainingGaugeOffset = $derived(
    gaugeCircumference - (financialSummary.remainingPercentage / 100) * gaugeCircumference
  );

  $effect(() => {
    try {
      window.localStorage.setItem(SELECTED_YEAR_STORAGE_KEY, String(selectedYear));
    } catch (error) {
      console.warn('[finance] Failed to persist selected year', error);
    }
  });

  async function handleYearChange(year: number) {
    selectedYear = year;
    try {
      await budgetState.loadMonthlyRecords(year);
    } catch (error) {
      console.error('[finance] Failed to load monthly records', error);
    }
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

{#snippet metricDataCard(
  title: string,
  value: string,
  footerLabel: string,
  footerValue: string,
  Icon: typeof CalendarDays
)}
  <Card class="variant-steampunk-riveted rounded-sm border border-border bg-card">
    <CardContent class="flex h-full flex-col gap-6 p-5">
      <div class="flex items-start justify-between gap-4">
        <div class="space-y-2">
          <p class="font-bebas text-lg tracking-widest text-foreground uppercase">{title}</p>
          <p class="font-mono text-3xl text-foreground">{value}</p>
        </div>
        <div
          class="flex h-10 w-10 items-center justify-center rounded-sm border border-border bg-background/50 text-primary"
        >
          <Icon size={18} />
        </div>
      </div>

      <div class="rounded-sm border border-border bg-background/50 p-3">
        <p class="text-[10px] tracking-tighter text-muted-foreground uppercase">{footerLabel}</p>
        <p class="font-mono text-sm text-foreground">{footerValue}</p>
      </div>
    </CardContent>
  </Card>
{/snippet}

{#snippet remainingGaugeCard()}
  <Card class="variant-steampunk-riveted rounded-sm border border-border bg-card">
    <CardContent class="flex h-full flex-col gap-6 p-5">
      <div class="flex items-start justify-between gap-4">
        <div class="space-y-2">
          <p class="font-bebas text-lg tracking-widest text-foreground uppercase">
            {m.dashboard_chart_budget_title()}
          </p>
          <p class="text-[10px] tracking-tighter text-muted-foreground uppercase">
            {m.dashboard_chart_budget_label()}
          </p>
        </div>
        <div
          class="flex h-10 w-10 items-center justify-center rounded-sm border border-border bg-background/50 text-primary"
        >
          <Gauge size={18} />
        </div>
      </div>

      <div class="flex items-center gap-4">
        <div
          class="variant-steampunk-gauge relative flex h-32 w-32 items-center justify-center rounded-full border border-border bg-background/60"
        >
          <svg class="h-28 w-28 -rotate-90" viewBox="0 0 120 120" aria-hidden="true">
            <circle
              class="stroke-muted/20"
              cx="60"
              cy="60"
              r={gaugeRadius}
              fill="none"
              stroke-width="8"
            />
            <circle
              class="stroke-primary transition-all duration-300 ease-out"
              cx="60"
              cy="60"
              r={gaugeRadius}
              fill="none"
              stroke-width="8"
              stroke-linecap="round"
              stroke-dasharray={gaugeCircumference}
              stroke-dashoffset={remainingGaugeOffset}
            />
          </svg>

          <div
            class="pointer-events-none absolute inset-0 flex flex-col items-center justify-center"
          >
            <span class="font-mono text-2xl text-foreground">
              {financialSummary.remainingPercentage}%
            </span>
            <span class="text-[10px] tracking-tighter text-muted-foreground uppercase">
              {m.dashboard_chart_budget_remaining()}
            </span>
          </div>
        </div>

        <div class="min-w-0 flex-1 rounded-sm border border-border bg-background/50 p-3">
          <p class="text-[10px] tracking-tighter text-muted-foreground uppercase">
            {m.budget_summary_total_available()}
          </p>
          <p class="font-mono text-sm text-foreground">{financialSummary.totalAvailable}</p>
          <p class="mt-3 text-[10px] tracking-tighter text-muted-foreground uppercase">
            {m.dashboard_chart_budget_title()}
          </p>
          <p class="font-mono text-sm text-foreground">{financialSummary.remainingAmount}</p>
        </div>
      </div>
    </CardContent>
  </Card>
{/snippet}

{#snippet metricDataCardSkeleton()}
  <Card class="variant-steampunk-riveted rounded-sm border border-border bg-card">
    <CardContent class="flex h-full flex-col gap-6 p-5">
      <div class="flex items-start justify-between gap-4">
        <div class="flex-1 space-y-2">
          <Skeleton class="h-4 w-28 rounded-sm" />
          <Skeleton class="h-8 w-32 rounded-sm" />
        </div>
        <Skeleton class="h-10 w-10 rounded-sm" />
      </div>

      <div class="rounded-sm border border-border bg-background/50 p-3">
        <Skeleton class="h-3 w-24 rounded-sm" />
        <Skeleton class="mt-2 h-4 w-32 rounded-sm" />
      </div>
    </CardContent>
  </Card>
{/snippet}

{#snippet remainingGaugeCardSkeleton()}
  <Card class="variant-steampunk-riveted rounded-sm border border-border bg-card">
    <CardContent class="flex h-full flex-col gap-6 p-5">
      <div class="flex items-start justify-between gap-4">
        <div class="flex-1 space-y-2">
          <Skeleton class="h-4 w-32 rounded-sm" />
          <Skeleton class="h-3 w-24 rounded-sm" />
        </div>
        <Skeleton class="h-10 w-10 rounded-sm" />
      </div>

      <div class="flex items-center gap-4">
        <Skeleton class="h-32 w-32 rounded-full" />

        <div class="min-w-0 flex-1 rounded-sm border border-border bg-background/50 p-3">
          <Skeleton class="h-3 w-28 rounded-sm" />
          <Skeleton class="mt-2 h-4 w-32 rounded-sm" />
          <Skeleton class="mt-4 h-3 w-24 rounded-sm" />
          <Skeleton class="mt-2 h-4 w-28 rounded-sm" />
        </div>
      </div>
    </CardContent>
  </Card>
{/snippet}

{#snippet financeTableSkeleton()}
  <Card class="variant-steampunk-riveted rounded-sm border border-border bg-card">
    <CardHeader class="border-b border-border/50 pb-4">
      <div class="flex items-center justify-between gap-4">
        <div class="space-y-2">
          <Skeleton class="h-4 w-32 rounded-sm" />
          <Skeleton class="h-3 w-48 rounded-sm" />
        </div>
        <Skeleton class="h-10 w-28 rounded-sm" />
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
            {#each monthNames as monthName (monthName)}
              <tr class="border-b border-border/40">
                <td class="px-4 py-3">
                  <Skeleton class="h-4 w-20 rounded-sm" />
                </td>
                <td class="px-4 py-3">
                  <Skeleton class="ml-auto h-4 w-16 rounded-sm" />
                </td>
                <td class="px-4 py-3">
                  <Skeleton class="ml-auto h-4 w-14 rounded-sm" />
                </td>
                <td class="px-4 py-3">
                  <Skeleton class="ml-auto h-4 w-[4.5rem] rounded-sm" />
                </td>
                <td class="px-4 py-3">
                  <Skeleton class="ml-auto h-4 w-16 rounded-sm" />
                </td>
                <td class="px-4 py-3">
                  <Skeleton class="ml-auto h-4 w-[4.5rem] rounded-sm" />
                </td>
                <td class="px-4 py-3">
                  <Skeleton class="h-6 w-20 rounded-sm" />
                </td>
                <td class="px-4 py-3">
                  <Skeleton class="ml-auto h-8 w-8 rounded-sm" />
                </td>
              </tr>
            {/each}
          </tbody>
        </table>
      </div>
    </CardContent>
  </Card>
{/snippet}

<div class="flex w-full max-w-none flex-col">
  <div
    class="-mx-4 -mt-4 mb-6 border-b border-border bg-card/50 px-6 py-4 lg:-mx-8 lg:-mt-8 lg:mb-8"
  >
    <PageHeader
      title={m.budget_title()}
      subtitle={m.app_finance()}
      description={m.budget_subtitle()}
    >
      {#snippet actions()}
        <FinanceSettingsButton onclick={() => (configSheetOpen = true)} />
      {/snippet}
    </PageHeader>
  </div>

  <div class="w-full max-w-none space-y-6">
    {#if !initialized}
      <div class="space-y-6" aria-busy="true" aria-label={m.budget_loading()}>
        <div class="grid gap-4 lg:grid-cols-3">
          {@render metricDataCardSkeleton()}
          {@render metricDataCardSkeleton()}
          {@render remainingGaugeCardSkeleton()}
        </div>

        {@render financeTableSkeleton()}
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

        <div class="space-y-6">
          <div class="grid gap-4 lg:grid-cols-3">
            {@render metricDataCard(
              m.budget_summary_monthly_allocation(),
              financialSummary.monthlyAllocation,
              m.budget_summary_budget_mode(),
              budgetState.modeLabel,
              CalendarDays
            )}
            {@render metricDataCard(
              m.budget_summary_yearly_forecast(),
              financialSummary.yearlyForecast,
              m.budget_summary_tracking_year(),
              String(selectedYear),
              TrendingUp
            )}
            {@render remainingGaugeCard()}
          </div>

          <Card class="variant-steampunk-riveted rounded-sm border border-border bg-card">
            <CardHeader class="border-b border-border/50 pb-4">
              <div class="flex items-center justify-between gap-4">
                <div class="space-y-1">
                  <p class="font-bebas text-lg tracking-widest text-foreground uppercase">
                    {m.budget_summary_tracking_year()}
                  </p>
                  <p class="text-[10px] tracking-tighter text-muted-foreground uppercase">
                    {m.budget_subtitle()}
                  </p>
                </div>
                <select
                  bind:value={selectedYear}
                  onchange={() => handleYearChange(selectedYear)}
                  class="h-10 rounded-sm border border-border bg-card px-3 font-mono text-sm text-foreground outline-none focus:border-primary focus:ring-1 focus:ring-primary"
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
    await financeState.refresh();
    await budgetState.loadMonthlyRecords(selectedYear);
  }}
  onSave={async () => {
    await financeState.refresh();
    await budgetState.loadMonthlyRecords(selectedYear);
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
