<script lang="ts">
  /**
   * Budget Table Component
   *
   * Displays a 12-month breakdown of budget with rollover calculations.
   * Shows base budget, extra budget, spending, rollover, and status for each month.
   */

  import { getModalStore } from '$lib/stores/modal';
  import type { BudgetState } from '../BudgetState.svelte';
  import type { MonthlyBudgetRecordDto } from '../services/BudgetService.svelte';
  import ExtraBudgetModal from './ExtraBudgetModal.svelte';

  interface Props {
    records: MonthlyBudgetRecordDto[];
    budgetState: BudgetState;
    currency?: string;
  }

  let { records, budgetState, currency: _currency = 'EUR' }: Props = $props();

  const modalStore = getModalStore();

  // Month names for display
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

  function formatAmount(minorUnits: number, currencyCode: string): string {
    const major = minorUnits / 100;
    return new Intl.NumberFormat(undefined, {
      style: 'currency',
      currency: currencyCode,
      minimumFractionDigits: 2,
      maximumFractionDigits: 2
    }).format(major);
  }

  function getStatusBadgeClass(status: string): string {
    switch (status) {
      case 'COMPLETED':
        return 'badge-success';
      case 'IN_PROGRESS':
        return 'badge-primary';
      case 'PROJECTED':
        return 'badge-ghost';
      default:
        return 'badge-ghost';
    }
  }

  function getStatusLabel(status: string): string {
    switch (status) {
      case 'COMPLETED':
        return 'Completed';
      case 'IN_PROGRESS':
        return 'In Progress';
      case 'PROJECTED':
        return 'Projected';
      default:
        return status;
    }
  }

  function getRemainingClass(remainingPercentage: number): string {
    if (remainingPercentage >= 50) return 'text-success-500';
    if (remainingPercentage >= 25) return 'text-warning-500';
    return 'text-error-500';
  }

  /**
   * Open modal to add extra budget for a specific month.
   */
  function openExtraBudgetModal(year: number, month: number) {
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

<div class="budget-table-container">
  <div class="overflow-x-auto">
    <table class="table-hover table-compact table w-full">
      <thead>
        <tr>
          <th>Month</th>
          <th class="text-right">Base Budget</th>
          <th class="text-right">Extra Budget</th>
          <th class="text-right">Rollover In</th>
          <th class="text-right">Available</th>
          <th class="text-right">Spent</th>
          <th class="text-right">Remaining</th>
          <th class="text-right">Remaining %</th>
          <th class="text-right">Rollover Out</th>
          <th>Status</th>
          <th>Actions</th>
        </tr>
      </thead>
      <tbody>
        {#each records as record (record.month)}
          <tr>
            <td class="font-semibold">{monthNames[record.month - 1]}</td>
            <td class="text-right">{formatAmount(record.baseBudget, record.currency)}</td>
            <td class="text-right">
              {record.extraBudget > 0 ? formatAmount(record.extraBudget, record.currency) : '—'}
            </td>
            <td class="text-right">
              {record.rolloverIn > 0 ? formatAmount(record.rolloverIn, record.currency) : '—'}
            </td>
            <td class="text-right font-semibold"
              >{formatAmount(record.available, record.currency)}</td
            >
            <td class="text-right">{formatAmount(record.actualSpend, record.currency)}</td>
            <td class="text-right {getRemainingClass(record.remainingPercentage)}">
              {formatAmount(record.remaining, record.currency)}
            </td>
            <td class="text-right {getRemainingClass(record.remainingPercentage)}">
              {record.remainingPercentage.toFixed(1)}%
            </td>
            <td class="text-right">
              {record.rolloverOut > 0 ? formatAmount(record.rolloverOut, record.currency) : '—'}
            </td>
            <td>
              <span class="badge {getStatusBadgeClass(record.status)}">
                {getStatusLabel(record.status)}
              </span>
            </td>
            <td>
              <button
                type="button"
                class="variant-ghost-primary btn btn-sm"
                onclick={() => openExtraBudgetModal(record.year, record.month)}
              >
                <svg
                  xmlns="http://www.w3.org/2000/svg"
                  fill="none"
                  viewBox="0 0 24 24"
                  stroke-width="1.5"
                  stroke="currentColor"
                  class="h-4 w-4"
                >
                  <path stroke-linecap="round" stroke-linejoin="round" d="M12 4.5v15m7.5-7.5h-15" />
                </svg>
                <span class="hidden sm:inline">Extra</span>
              </button>
            </td>
          </tr>
        {/each}
      </tbody>
    </table>
  </div>

  {#if records.length === 0}
    <div class="text-surface-500 py-8 text-center">No budget records available</div>
  {/if}
</div>

<style>
  .table {
    min-width: 100%;
    border-collapse: collapse;
  }

  .table > :not([hidden]) ~ :not([hidden]) {
    border-top-width: 1px;
    border-color: rgb(228 228 231);
  }

  thead {
    background-color: rgb(63 63 70);
  }

  th {
    padding: 0.75rem 1rem;
    font-size: 0.75rem;
    font-weight: 500;
    letter-spacing: 0.05em;
    color: rgb(161 161 170);
    text-transform: uppercase;
  }

  td {
    padding: 0.75rem 1rem;
    font-size: 0.875rem;
    color: rgb(250 250 250);
  }

  tr:hover {
    background-color: rgb(82 82 91);
  }

  .badge {
    display: inline-flex;
    align-items: center;
    border-radius: 9999px;
    padding: 0.125rem 0.625rem;
    font-size: 0.75rem;
    font-weight: 500;
  }

  .badge-success {
    background-color: rgb(220 252 231);
    color: rgb(22 101 52);
  }

  .badge-primary {
    background-color: rgb(219 234 254);
    color: rgb(30 64 175);
  }

  .badge-ghost {
    background-color: rgb(63 63 70);
    color: rgb(161 161 170);
  }
</style>
