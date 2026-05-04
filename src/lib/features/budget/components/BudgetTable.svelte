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

  interface EnrichedRecord extends MonthlyBudgetRecordDto {
    monthName: string;
    displayBaseBudget: string;
    displayExtraBudget: string;
    displayRolloverIn: string;
    displayAvailable: string;
    displayActualSpend: string;
    displayRemaining: string;
    displayRemainingPercentage: string;
    displayRolloverOut: string;
    remainingClass: string;
    statusLabel: string;
    statusVariant: 'default' | 'secondary' | 'outline';
  }

  // Get localized month names using Intl.DateTimeFormat
  const monthNames = $derived.by(() => {
    const formatter = new Intl.DateTimeFormat(regionalManager.locale, { month: 'long' });
    return Array.from({ length: 12 }, (_, i) => formatter.format(new Date(2000, i, 1)));
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

  const enrichedRecords = $derived.by<EnrichedRecord[]>(() =>
    records.map((record) => ({
      ...record,
      monthName: monthNames[record.month - 1],
      displayBaseBudget: formatAmount(record.baseBudget, record.currency),
      displayExtraBudget:
        record.extraBudget > 0 ? formatAmount(record.extraBudget, record.currency) : '—',
      displayRolloverIn:
        record.rolloverIn > 0 ? formatAmount(record.rolloverIn, record.currency) : '—',
      displayAvailable: formatAmount(record.available, record.currency),
      displayActualSpend: formatAmount(record.actualSpend, record.currency),
      displayRemaining: formatAmount(record.remaining, record.currency),
      displayRemainingPercentage: `${record.remainingPercentage.toFixed(1)}%`,
      displayRolloverOut:
        record.rolloverOut > 0 ? formatAmount(record.rolloverOut, record.currency) : '—',
      remainingClass: getRemainingClass(record.remainingPercentage),
      statusLabel: getStatusLabel(record.status),
      statusVariant: getStatusBadgeVariant(record.status)
    }))
  );
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
        {#each enrichedRecords as row (row.month)}
          <TableRow>
            <td class="p-4 align-middle font-semibold">{row.monthName}</td>
            <td class="p-4 text-right align-middle">{row.displayBaseBudget}</td>
            <td class="p-4 text-right align-middle">{row.displayExtraBudget}</td>
            <td class="p-4 text-right align-middle">{row.displayRolloverIn}</td>
            <td class="p-4 text-right align-middle font-semibold">{row.displayAvailable}</td>
            <td class="p-4 text-right align-middle">{row.displayActualSpend}</td>
            <td class="p-4 text-right align-middle {row.remainingClass}">{row.displayRemaining}</td>
            <td class="p-4 text-right align-middle {row.remainingClass}">
              {row.displayRemainingPercentage}
            </td>
            <td class="p-4 text-right align-middle">{row.displayRolloverOut}</td>
            <td class="p-4 align-middle">
              <Badge variant={row.statusVariant}>
                {row.statusLabel}
              </Badge>
            </td>
            <td class="p-4 align-middle">
              <Button
                type="button"
                size="sm"
                variant="ghost"
                onclick={() => handleAddExtra(row.year, row.month)}
              >
                <Plus class="h-4 w-4" />
                <span class="hidden sm:inline">Extra</span>
              </Button>
            </td>
          </TableRow>
        {/each}
      </TableBody>
    </Table>
  </div>

  {#if enrichedRecords.length === 0}
    <div class="py-8 text-center text-muted-foreground">No budget records available</div>
  {/if}
</div>
