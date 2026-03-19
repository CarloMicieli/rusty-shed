<script lang="ts">
  import { onMount } from 'svelte';
  import { Settings2, CalendarDays, TrendingUp, Wallet } from 'lucide-svelte';
  import * as m from '$lib/paraglide/messages.js';
  import * as Dialog from '$lib/components/ui/dialog';

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
    class="-mx-4 -mt-4 mb-6 rounded-tl-[24px] border-b border-border bg-card/50 px-6 py-4 lg:-mx-8 lg:-mt-8 lg:mb-8"
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
      <div class="space-y-4 rounded-lg border border-white/10 bg-black/20 p-4">
        <div
          class="flex flex-col items-center justify-center gap-8 rounded-3xl border border-white/5 bg-[#0c0c0c]/50 py-24"
        >
          <div class="relative">
            <div class="absolute inset-0 rounded-full bg-zinc-500/10 blur-3xl"></div>
            <div
              class="relative flex h-32 w-32 items-center justify-center rounded-full border border-white/10 bg-zinc-900/50"
            >
              <Wallet size={56} class="text-zinc-600 opacity-50" />
            </div>
          </div>

          <div class="flex max-w-sm flex-col items-center gap-3 text-center">
            <h3 class="text-2xl font-bold text-zinc-200">{m.budget_empty_state_title()}</h3>
            <p class="text-sm leading-relaxed text-zinc-500">{m.budget_empty_state_message()}</p>
          </div>

          <button
            type="button"
            class="group relative mt-2 inline-flex cursor-pointer items-center gap-3 overflow-hidden rounded-full bg-amber-500 px-8 py-4 font-bold tracking-wide text-black transition-all hover:scale-105 hover:bg-amber-400 hover:shadow-[0_0_20px_rgba(245,158,11,0.4)] active:scale-95"
            onclick={() => (configSheetOpen = true)}
          >
            <div
              class="absolute inset-0 translate-y-full bg-white/20 transition-transform duration-300 group-hover:translate-y-0"
            ></div>
            <Wallet class="h-5 w-5" />
            <span>{m.dashboard_chart_budget_set_cta()}</span>
          </button>
        </div>
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
