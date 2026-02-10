<script lang="ts">
  import { onMount } from 'svelte';
  import * as m from '$lib/paraglide/messages.js';
  import { PageHeader } from '$lib/components';
  import { toaster } from '$lib/toaster';
  import { createBudgetService } from '$lib/features/budget/services/BudgetService.svelte';
  import { createBudgetState } from '$lib/features/budget/BudgetState.svelte';
  import BudgetConfigForm from '$lib/features/budget/components/BudgetConfigForm.svelte';
  import BudgetTable from '$lib/features/budget/components/BudgetTable.svelte';
  import HistoricalArchive from '$lib/features/budget/components/HistoricalArchive.svelte';
  import type { BudgetMode } from '$lib/features/budget/services/BudgetService.svelte';

  // Create service and state in component context
  const service = createBudgetService();
  const budgetState = createBudgetState(service);

  let loading = $state(true);
  let saving = $state(false);
  let error: string | null = $state(null);
  let selectedYear = $state(new Date().getFullYear());
  let loadingRecords = $state(false);

  // Form bindings
  let formMode = $state<BudgetMode>('MONTHLY');
  let formBaseAmount = $state<number>(0);

  onMount(async () => {
    await loadBudgetConfig();
    if (budgetState.hasConfig) {
      await loadMonthlyRecordsForYear(selectedYear);
    }
  });

  async function loadBudgetConfig() {
    loading = true;
    try {
      await budgetState.load();

      // Sync form with loaded config
      if (budgetState.config) {
        formMode = budgetState.config.mode;
        formBaseAmount = budgetState.config.baseAmount;
      }

      error = null;
    } catch (err) {
      error = err instanceof Error ? err.message : String(err);
      toaster.error({ title: m.budget_error_load_failed(), duration: 5000 });
    } finally {
      loading = false;
    }
  }

  async function loadMonthlyRecordsForYear(year: number) {
    loadingRecords = true;
    try {
      await budgetState.loadMonthlyRecords(year);
      selectedYear = year;
    } catch (err) {
      console.error(`Failed to load records for year ${year}:`, err);
    } finally {
      loadingRecords = false;
    }
  }

  async function handleYearChange(year: number) {
    await loadMonthlyRecordsForYear(year);
  }

  async function handleSubmit(mode: BudgetMode, amount: number) {
    saving = true;
    try {
      await budgetState.save(mode, amount);
      error = null;
    } catch (err) {
      error = err instanceof Error ? err.message : String(err);
      // Error toast is handled by BudgetService
    } finally {
      saving = false;
    }
  }
</script>

<svelte:head>
  <title>{m.app_name()} | {m.budget_title()}</title>
</svelte:head>

<div class="space-y-6">
  <!-- Page Header -->
  <PageHeader
    title={m.budget_config_title()}
    subtitle={m.budget_title()}
    description={m.budget_subtitle()}
  />

  <!-- Loading State -->
  {#if loading}
    <div class="flex items-center justify-center py-12">
      <div class="text-center">
        <div
          class="border-surface-700 border-t-primary-500 mx-auto mb-4 h-12 w-12 animate-spin rounded-full border-4"
        ></div>
        <p class="text-surface-400">{m.budget_loading()}</p>
      </div>
    </div>

    <!-- Error State -->
  {:else if error}
    <div class="border-error-500 bg-error-500/10 rounded-lg border p-6 text-center">
      <p class="text-error-400 mb-4">{error}</p>
      <button
        onclick={loadBudgetConfig}
        class="bg-error-600 hover:bg-error-700 rounded-lg px-4 py-2 font-semibold text-white"
      >
        {m.budget_error_retry()}
      </button>
    </div>

    <!-- Budget Configuration Form -->
  {:else}
    <BudgetConfigForm
      bind:mode={formMode}
      bind:baseAmount={formBaseAmount}
      currency={budgetState.currency}
      {saving}
      onsubmit={handleSubmit}
    />

    <!-- Current Configuration Display (if exists) -->
    {#if budgetState.hasConfig && budgetState.config}
      <section class="card border-surface-700/60 bg-surface-900/50 border p-6 shadow-xl">
        <h3 class="text-surface-100 mb-4 text-lg font-semibold">
          {m.budget_config_summary_title()}
        </h3>

        <div class="grid grid-cols-1 gap-4 md:grid-cols-3">
          <div class="border-surface-700 bg-surface-800/50 rounded-lg border p-4">
            <p class="text-surface-400 text-sm">{m.budget_config_mode_label()}</p>
            <p class="text-primary-400 text-xl font-bold">{budgetState.modeLabel}</p>
          </div>

          <div class="border-surface-700 bg-surface-800/50 rounded-lg border p-4">
            <p class="text-surface-400 text-sm">{m.budget_config_mode_monthly()}</p>
            <p class="text-primary-400 text-xl font-bold">{budgetState.formattedMonthlyBudget}</p>
          </div>

          <div class="border-surface-700 bg-surface-800/50 rounded-lg border p-4">
            <p class="text-surface-400 text-sm">{m.budget_config_mode_yearly()}</p>
            <p class="text-primary-400 text-xl font-bold">{budgetState.formattedYearlyBudget}</p>
          </div>
        </div>

        <div class="text-surface-400 mt-4 text-xs">
          <p>
            {m.budget_table_status_header()}:
            {new Date(budgetState.config.updatedAt).toLocaleString()}
          </p>
          <p>{m.budget_last_reset_year()}: {budgetState.config.lastResetYear}</p>
        </div>
      </section>

      <!-- Year Selector and Budget Table -->
      <section class="card border-surface-700/60 bg-surface-900/50 border p-6 shadow-xl">
        <div class="mb-4 flex items-center justify-between">
          <h3 class="text-surface-100 text-lg font-semibold">
            {m.budget_mode_yearly?.() || 'Monthly Budget Breakdown'}
          </h3>
          <div class="flex items-center gap-2">
            <label for="year-selector" class="text-surface-400 text-sm">Year:</label>
            <select
              id="year-selector"
              bind:value={selectedYear}
              onchange={() => handleYearChange(selectedYear)}
              class="variant-form-material select"
              disabled={loadingRecords}
            >
              {#each Array.from({ length: 6 }, (_, i) => new Date().getFullYear() - i) as year (year)}
                <option value={year}>{year}</option>
              {/each}
            </select>
          </div>
        </div>

        {#if loadingRecords}
          <div class="flex items-center justify-center py-8">
            <div
              class="border-surface-700 border-t-primary-500 h-8 w-8 animate-spin rounded-full border-4"
            ></div>
          </div>
        {:else if budgetState.hasRecords}
          <BudgetTable
            records={budgetState.monthlyRecords}
            {budgetState}
            currency={budgetState.currency}
          />
        {:else}
          <div class="text-surface-400 py-8 text-center">
            {m.dashboard_empty_recent?.() || 'No budget records available for this year.'}
          </div>
        {/if}
      </section>

      <!-- Historical Archive -->
      <section class="card border-surface-700/60 bg-surface-900/50 border p-6 shadow-xl">
        <HistoricalArchive
          {budgetState}
          currentYear={selectedYear}
          onYearSelect={handleYearChange}
        />
      </section>
    {/if}
  {/if}
</div>
