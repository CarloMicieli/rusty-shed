<script lang="ts">
  import { onMount } from 'svelte';
  import { Settings2, Plus, TrendingUp, CalendarDays } from 'lucide-svelte';
  import * as m from '$lib/paraglide/messages.js';
  import { toaster } from '$lib/toaster';
  import { createBudgetService } from '$lib/features/budget/services/BudgetService.svelte';
  import { createBudgetState } from '$lib/features/budget/BudgetState.svelte';
  import {
    Card,
    CardHeader,
    CardTitle,
    CardContent,
    Button,
    Alert,
    AlertDescription,
    Badge,
    Table,
    TableHeader,
    TableBody,
    TableRow,
    TableHead,
    TableCell,
    Dialog,
    PageHeader
  } from '$lib/components';
  import BudgetConfigSheet from '$lib/features/budget/components/BudgetConfigSheet.svelte';
  import type { BudgetMode } from '$lib/features/budget/services/BudgetService.svelte';

  // Create service and state in component context
  const service = createBudgetService();
  const budgetState = createBudgetState(service);

  let loading = $state(true);
  let saving = $state(false);
  let error: string | null = $state(null);
  let selectedYear = $state(new Date().getFullYear());
  let loadingRecords = $state(false);
  let configSheetOpen = $state(false);

  // Extra budget modal state
  let extraBudgetModalOpen = $state(false);
  let selectedMonthForExtra: { year: number; month: number } | null = $state(null);

  // Form bindings
  let formMode = $state<BudgetMode>('MONTHLY');
  let formBaseAmount = $state<number>(0);

  // Get current month for highlighting
  const currentMonth = new Date().getMonth() + 1;
  const currentYear = new Date().getFullYear();

  // Month names
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
      configSheetOpen = false;
      toaster.success({ title: m.budget_config_saved_toast(), duration: 2000 });
    } catch (err) {
      error = err instanceof Error ? err.message : String(err);
    } finally {
      saving = false;
    }
  }

  function formatAmount(minorUnits: number, currencyCode: string): string {
    const major = minorUnits / 100;
    return new Intl.NumberFormat(undefined, {
      style: 'currency',
      currency: currencyCode,
      minimumFractionDigits: 2,
      maximumFractionDigits: 2
    }).format(major);
  }

  function getStatusVariant(status: string): 'default' | 'secondary' | 'outline' {
    switch (status) {
      case 'COMPLETED':
        return 'default';
      case 'IN_PROGRESS':
        return 'secondary';
      case 'PROJECTED':
        return 'outline';
      default:
        return 'outline';
    }
  }

  function getRemainingClass(remainingPercentage: number): string {
    if (remainingPercentage >= 50) return 'text-emerald-500';
    if (remainingPercentage >= 25) return 'text-amber-500';
    return 'text-red-500';
  }

  function isCurrentMonth(year: number, month: number): boolean {
    return year === currentYear && month === currentMonth;
  }

  function getRowClasses(year: number, month: number): string {
    const baseClasses = 'transition-colors hover:bg-muted/30';
    const currentMonthClasses = isCurrentMonth(year, month)
      ? 'bg-primary/10 border-l-4 border-l-primary'
      : '';
    return `${baseClasses} ${currentMonthClasses}`;
  }

  function handleAddExtraBudget(year: number, month: number) {
    selectedMonthForExtra = { year, month };
    extraBudgetModalOpen = true;
  }

  function closeExtraBudgetModal() {
    extraBudgetModalOpen = false;
    selectedMonthForExtra = null;
  }

  async function handleExtraBudgetSubmit(amount: number, reason?: string) {
    if (!selectedMonthForExtra) return;

    try {
      await budgetState.addExtraBudget({
        year: selectedMonthForExtra.year,
        month: selectedMonthForExtra.month,
        amount,
        reason
      });
      closeExtraBudgetModal();
      toaster.success({ title: 'Extra budget added successfully', duration: 2000 });
    } catch (error) {
      console.error('Failed to add extra budget:', error);
      // Error toast already shown by service
    }
  }
</script>

<svelte:head>
  <title>{m.app_name()} | {m.budget_title()}</title>
</svelte:head>

<div class="space-y-6 px-4 py-6 sm:px-6 lg:px-8">
  <!-- Page Header -->
  <PageHeader
    title={m.budget_title()}
    subtitle="Budget Management"
    description={m.budget_subtitle()}
  />

  <!-- Error Alert -->
  {#if error && !loading}
    <Alert variant="destructive">
      <AlertDescription>{error}</AlertDescription>
    </Alert>
  {/if}

  <!-- Loading State -->
  {#if loading}
    <div class="flex items-center justify-center py-24">
      <div class="text-center">
        <div
          class="mx-auto mb-4 h-12 w-12 animate-spin rounded-full border-4 border-muted border-t-primary"
        ></div>
        <p class="text-muted-foreground">{m.budget_loading()}</p>
      </div>
    </div>
  {:else if budgetState.hasConfig && budgetState.config}
    <!-- Top Summary Cards -->
    <div class="grid gap-6 md:grid-cols-3">
      <!-- Card 1: Monthly Allocation -->
      <Card>
        <CardHeader class="pb-3">
          <CardTitle class="flex items-center gap-2 text-sm font-medium text-muted-foreground">
            <CalendarDays class="h-4 w-4 text-primary" />
            {m.budget_config_mode_monthly()}
          </CardTitle>
        </CardHeader>
        <CardContent>
          <div class="text-3xl font-bold">
            {budgetState.formattedMonthlyBudget}
          </div>
        </CardContent>
      </Card>

      <!-- Card 2: Yearly Total -->
      <Card>
        <CardHeader class="pb-3">
          <CardTitle class="flex items-center gap-2 text-sm font-medium text-muted-foreground">
            <TrendingUp class="h-4 w-4 text-primary" />
            {m.budget_config_mode_yearly()}
          </CardTitle>
        </CardHeader>
        <CardContent>
          <div class="text-3xl font-bold">
            {budgetState.formattedYearlyBudget}
          </div>
        </CardContent>
      </Card>

      <!-- Card 3: Budget Control -->
      <Card>
        <CardHeader class="pb-3">
          <CardTitle class="text-sm font-medium text-muted-foreground">
            {m.budget_config_title()}
          </CardTitle>
        </CardHeader>
        <CardContent>
          <Button class="w-full" onclick={() => (configSheetOpen = true)}>
            <Settings2 size={16} />
            Configure Budget
          </Button>
        </CardContent>
      </Card>
    </div>

    <!-- Budget Configuration Sheet -->
    <BudgetConfigSheet
      bind:open={configSheetOpen}
      bind:mode={formMode}
      bind:baseAmount={formBaseAmount}
      currency={budgetState.currency}
      {saving}
      onsubmit={handleSubmit}
    />

    <!-- Main Data Table -->
    <Card class="border-zinc-800 bg-zinc-950">
      <CardHeader class="pb-4">
        <div class="flex items-center justify-between">
          <CardTitle class="text-lg font-semibold">
            {selectedYear} Budget Breakdown
          </CardTitle>
          <div class="flex items-center gap-2">
            <label for="year-selector" class="text-sm text-muted-foreground">Year:</label>
            <select
              id="year-selector"
              bind:value={selectedYear}
              onchange={() => handleYearChange(selectedYear)}
              class="h-9 rounded-md border border-input bg-background px-3 text-sm focus:border-ring focus:ring-1 focus:ring-ring focus:outline-none"
              disabled={loadingRecords}
            >
              {#each Array.from({ length: 6 }, (_, i) => new Date().getFullYear() - i) as year (year)}
                <option value={year}>{year}</option>
              {/each}
            </select>
          </div>
        </div>
      </CardHeader>
      <CardContent class="p-0">
        {#if loadingRecords}
          <div class="flex items-center justify-center py-12">
            <div
              class="h-8 w-8 animate-spin rounded-full border-4 border-muted border-t-primary"
            ></div>
          </div>
        {:else if budgetState.hasRecords}
          <div class="overflow-x-auto rounded-lg">
            <Table>
              <TableHeader>
                <TableRow class="hover:bg-transparent">
                  <TableHead>Month</TableHead>
                  <TableHead class="text-right">Base</TableHead>
                  <TableHead class="text-right">Extra</TableHead>
                  <TableHead class="text-right">Rollover In</TableHead>
                  <TableHead class="text-right">Available</TableHead>
                  <TableHead class="text-right">Spent</TableHead>
                  <TableHead class="text-right">Remaining</TableHead>
                  <TableHead class="text-right">Remaining %</TableHead>
                  <TableHead class="text-right">Rollover Out</TableHead>
                  <TableHead>Status</TableHead>
                  <TableHead>Actions</TableHead>
                </TableRow>
              </TableHeader>
              <TableBody>
                {#each budgetState.monthlyRecords as record (record.month)}
                  <TableRow class={getRowClasses(record.year, record.month)}>
                    <TableCell class="py-2 font-medium">
                      {monthNames[record.month - 1]}
                      {#if isCurrentMonth(record.year, record.month)}
                        <Badge variant="outline" class="ml-2">Current</Badge>
                      {/if}
                    </TableCell>
                    <TableCell class="py-2 text-right">
                      {formatAmount(record.baseBudget, record.currency)}
                    </TableCell>
                    <TableCell class="py-2 text-right">
                      {record.extraBudget > 0
                        ? formatAmount(record.extraBudget, record.currency)
                        : '—'}
                    </TableCell>
                    <TableCell class="py-2 text-right">
                      {record.rolloverIn > 0
                        ? formatAmount(record.rolloverIn, record.currency)
                        : '—'}
                    </TableCell>
                    <TableCell class="py-2 text-right font-semibold">
                      {formatAmount(record.available, record.currency)}
                    </TableCell>
                    <TableCell class="py-2 text-right">
                      {formatAmount(record.actualSpend, record.currency)}
                    </TableCell>
                    <TableCell
                      class="py-2 text-right font-medium {getRemainingClass(
                        record.remainingPercentage
                      )}"
                    >
                      {formatAmount(record.remaining, record.currency)}
                    </TableCell>
                    <TableCell
                      class="py-2 text-right font-medium {getRemainingClass(
                        record.remainingPercentage
                      )}"
                    >
                      {record.remainingPercentage.toFixed(1)}%
                    </TableCell>
                    <TableCell class="py-2 text-right">
                      {record.rolloverOut > 0
                        ? formatAmount(record.rolloverOut, record.currency)
                        : '—'}
                    </TableCell>
                    <TableCell class="py-2">
                      <Badge variant={getStatusVariant(record.status)}>
                        {record.status === 'COMPLETED'
                          ? 'Completed'
                          : record.status === 'IN_PROGRESS'
                            ? 'In Progress'
                            : 'Projected'}
                      </Badge>
                    </TableCell>
                    <TableCell class="py-2">
                      <Button
                        variant="ghost"
                        size="sm"
                        class="h-8"
                        onclick={() => handleAddExtraBudget(record.year, record.month)}
                      >
                        <Plus size={14} />
                        <span class="hidden sm:inline">Extra</span>
                      </Button>
                    </TableCell>
                  </TableRow>
                {/each}
              </TableBody>
            </Table>
          </div>
        {:else}
          <div class="py-12 text-center text-muted-foreground">
            No budget records available for this year.
          </div>
        {/if}
      </CardContent>
    </Card>

    <!-- Budget Info -->
    <Card>
      <CardContent class="pt-6">
        <div class="flex items-center justify-between text-xs text-muted-foreground">
          <div class="flex items-center gap-4">
            <span>
              {m.budget_table_status_header()}:
              {new Date(budgetState.config.updatedAt).toLocaleString()}
            </span>
            <span>
              {m.budget_last_reset_year()}: {budgetState.config.lastResetYear}
            </span>
          </div>
          <span class="font-mono">v{budgetState.config.version}</span>
        </div>
      </CardContent>
    </Card>
  {:else}
    <!-- No Configuration State -->
    <div class="flex items-center justify-center py-24">
      <Card class="max-w-md">
        <CardHeader>
          <CardTitle class="text-center">No Budget Configuration</CardTitle>
        </CardHeader>
        <CardContent class="text-center">
          <p class="mb-6 text-muted-foreground">Set up your budget to start tracking.</p>
          <Button onclick={() => (configSheetOpen = true)}>
            <Settings2 size={16} />
            Configure Budget
          </Button>
        </CardContent>
      </Card>
    </div>

    <!-- Budget Configuration Sheet (for initial setup) -->
    <BudgetConfigSheet
      bind:open={configSheetOpen}
      bind:mode={formMode}
      bind:baseAmount={formBaseAmount}
      currency={budgetState.currency || 'EUR'}
      {saving}
      onsubmit={handleSubmit}
    />
  {/if}

  <!-- Extra Budget Modal -->
  {#if selectedMonthForExtra}
    <Dialog.Root bind:open={extraBudgetModalOpen}>
      <Dialog.Content class="sm:max-w-md">
        <Dialog.Header>
          <Dialog.Title>
            {m.budget_extra_modal_title()} - {monthNames[selectedMonthForExtra.month - 1]}
            {selectedMonthForExtra.year}
          </Dialog.Title>
        </Dialog.Header>
        <div class="space-y-4 py-4">
          <form
            onsubmit={(e) => {
              e.preventDefault();
              const formData = new FormData(e.currentTarget);
              const amount = parseFloat(formData.get('amount') as string) || 0;
              const reason = formData.get('reason') as string;
              handleExtraBudgetSubmit(Math.round(amount * 100), reason || undefined);
            }}
            class="space-y-4"
          >
            <div>
              <label for="extra-amount" class="mb-2 block text-sm font-medium">
                {m.budget_extra_amount_label()}
              </label>
              <div class="relative">
                <span class="absolute top-1/2 left-3 -translate-y-1/2 text-muted-foreground">
                  {budgetState.currency === 'EUR' ? '€' : '$'}
                </span>
                <input
                  id="extra-amount"
                  name="amount"
                  type="number"
                  step="0.01"
                  min="0"
                  class="flex h-10 w-full rounded-md border border-input bg-background py-2 pr-4 pl-8 text-sm ring-offset-background file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 focus-visible:outline-none disabled:cursor-not-allowed disabled:opacity-50"
                  placeholder="0.00"
                  required
                />
              </div>
            </div>
            <div>
              <label for="extra-reason" class="mb-2 block text-sm font-medium">
                {m.budget_extra_reason_label()}
              </label>
              <textarea
                id="extra-reason"
                name="reason"
                rows="3"
                maxlength="500"
                class="flex min-h-[80px] w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background placeholder:text-muted-foreground focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 focus-visible:outline-none disabled:cursor-not-allowed disabled:opacity-50"
                placeholder={m.budget_extra_reason_placeholder()}
              ></textarea>
            </div>
            <div class="flex justify-end gap-3">
              <Button type="button" variant="outline" onclick={closeExtraBudgetModal}>
                {m.budget_extra_cancel_button()}
              </Button>
              <Button type="submit">
                {m.budget_extra_save_button()}
              </Button>
            </div>
          </form>
        </div>
      </Dialog.Content>
    </Dialog.Root>
  {/if}
</div>
