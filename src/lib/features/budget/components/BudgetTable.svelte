<script lang="ts">
  /**
   * Budget Table Component
   *
   * Displays a 12-month breakdown of budget with rollover calculations.
   * Shows base budget, extra budget, spending, rollover, and status for each month.
   */

  import { Plus } from 'lucide-svelte';
  import * as m from '$lib/paraglide/messages.js';
  import { regionalManager } from '$lib/features/settings/RegionalManager.svelte';
  import type { BudgetState } from '../BudgetState.svelte';
  import type { MonthlyBudgetRecordDto } from '../services/BudgetService.svelte';
  import {
    Badge,
    Button,
    Table,
    TableBody,
    TableCell,
    TableHead,
    TableHeader,
    TableRow
  } from '$lib/components';

  interface Props {
    records: MonthlyBudgetRecordDto[];
    budgetState?: BudgetState;
    currency?: string;
    onAddExtra?: (year: number, month: number) => void;
  }

  let {
    records,
    budgetState: _budgetState,
    currency: _currency = 'EUR',
    onAddExtra
  }: Props = $props();

  // Get localized month names using Intl.DateTimeFormat
  const monthNames = $derived.by(() => {
    return Array.from({ length: 12 }, (_, i) =>
      new Intl.DateTimeFormat(regionalManager.locale, { month: 'long' }).format(
        new Date(2000, i, 1)
      )
    );
  });

  function formatAmount(minorUnits: number, currencyCode: string): string {
    return regionalManager.formatCurrencyWith(minorUnits, currencyCode);
  }

  function getStatusBadgeVariant(status: string): 'default' | 'secondary' | 'outline' {
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

  function getStatusLabel(status: string): string {
    switch (status) {
      case 'COMPLETED':
        return m.budget_status_completed();
      case 'IN_PROGRESS':
        return m.budget_status_in_progress();
      case 'PROJECTED':
        return m.budget_status_projected();
      default:
        return status;
    }
  }

  function getRemainingClass(remainingPercentage: number): string {
    if (remainingPercentage >= 50) return 'text-success-500';
    if (remainingPercentage >= 25) return 'text-warning-500';
    return 'text-error-500';
  }

  function handleAddExtra(year: number, month: number) {
    onAddExtra?.(year, month);
  }
</script>

<div class="budget-table-container">
  <div class="overflow-x-auto">
    <Table>
      <TableHeader>
        <TableRow>
          <TableHead>{m.budget_table_month_header()}</TableHead>
          <TableHead class="text-right">{m.budget_base_budget()}</TableHead>
          <TableHead class="text-right">{m.budget_extra_budget()}</TableHead>
          <TableHead class="text-right">{m.budget_rollover_in()}</TableHead>
          <TableHead class="text-right">{m.budget_table_available_header()}</TableHead>
          <TableHead class="text-right">{m.budget_table_spent_header()}</TableHead>
          <TableHead class="text-right">{m.budget_table_remaining_header()}</TableHead>
          <TableHead class="text-right">{m.budget_remaining_percent()}</TableHead>
          <TableHead class="text-right">{m.budget_rollover_out()}</TableHead>
          <TableHead>{m.budget_table_status_header()}</TableHead>
          <TableHead>{m.budget_table_actions_header()}</TableHead>
        </TableRow>
      </TableHeader>
      <TableBody>
        {#each records as record (record.month)}
          <TableRow>
            <TableCell class="font-semibold">{monthNames[record.month - 1]}</TableCell>
            <TableCell class="text-right"
              >{formatAmount(record.baseBudget, record.currency)}</TableCell
            >
            <TableCell class="text-right">
              {record.extraBudget > 0 ? formatAmount(record.extraBudget, record.currency) : '—'}
            </TableCell>
            <TableCell class="text-right">
              {record.rolloverIn > 0 ? formatAmount(record.rolloverIn, record.currency) : '—'}
            </TableCell>
            <TableCell class="text-right font-semibold"
              >{formatAmount(record.available, record.currency)}</TableCell
            >
            <TableCell class="text-right"
              >{formatAmount(record.actualSpend, record.currency)}</TableCell
            >
            <TableCell class="text-right {getRemainingClass(record.remainingPercentage)}">
              {formatAmount(record.remaining, record.currency)}
            </TableCell>
            <TableCell class="text-right {getRemainingClass(record.remainingPercentage)}">
              {record.remainingPercentage.toFixed(1)}%
            </TableCell>
            <TableCell class="text-right">
              {record.rolloverOut > 0 ? formatAmount(record.rolloverOut, record.currency) : '—'}
            </TableCell>
            <TableCell>
              <Badge variant={getStatusBadgeVariant(record.status)}>
                {getStatusLabel(record.status)}
              </Badge>
            </TableCell>
            <TableCell>
              <Button
                type="button"
                size="sm"
                variant="ghost"
                onclick={() => handleAddExtra(record.year, record.month)}
              >
                <Plus class="h-4 w-4" />
                <span class="hidden sm:inline">Extra</span>
              </Button>
            </TableCell>
          </TableRow>
        {/each}
      </TableBody>
    </Table>
  </div>

  {#if records.length === 0}
    <div class="py-8 text-center text-muted-foreground">No budget records available</div>
  {/if}
</div>
