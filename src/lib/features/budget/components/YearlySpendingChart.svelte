<script lang="ts">
  /**
   * Yearly Spending Chart Component
   *
   * Displays monthly spending as a bar chart with a horizontal budget goal line.
   * Shows 12 bars (one per month) with the current month highlighted.
   */

  import type { MonthlySpendingPoint } from '../services/BudgetService.svelte';

  interface Props {
    monthlySpending: MonthlySpendingPoint[];
    monthlyGoal: number;
    currency: string;
  }

  let { monthlySpending, monthlyGoal, currency }: Props = $props();

  const monthNames = [
    'Jan',
    'Feb',
    'Mar',
    'Apr',
    'May',
    'Jun',
    'Jul',
    'Aug',
    'Sep',
    'Oct',
    'Nov',
    'Dec'
  ];
  const currentMonth = new Date().getMonth() + 1;

  // Calculate max value for scaling
  const maxValue = $derived(Math.max(monthlyGoal, ...monthlySpending.map((p) => p.amount)));

  function getBarHeight(amount: number): number {
    if (maxValue === 0) return 0;
    return (amount / maxValue) * 100;
  }

  function formatAmount(minorUnits: number): string {
    const major = minorUnits / 100;
    return new Intl.NumberFormat(undefined, {
      style: 'currency',
      currency: currency,
      minimumFractionDigits: 0,
      maximumFractionDigits: 0
    }).format(major);
  }

  function getBarColor(month: number, amount: number): string {
    if (month === currentMonth) return 'bg-primary-500';
    if (amount > monthlyGoal) return 'bg-error-400';
    return 'bg-surface-400';
  }
</script>

<div class="rounded-lg bg-surface-50 p-6">
  <h3 class="mb-4 text-lg font-semibold text-surface-900">Monthly Spending</h3>
  <div class="relative h-64">
    <!-- Goal line -->
    <div
      class="absolute right-0 left-0 z-10 border-t-2 border-dashed border-success-500"
      style="bottom: {getBarHeight(monthlyGoal)}%"
    >
      <span class="absolute -top-5 right-0 bg-surface-50 px-2 text-xs text-success-700">
        {formatAmount(monthlyGoal)} goal
      </span>
    </div>

    <!-- Bars -->
    <div class="flex h-full items-end justify-between gap-1">
      {#each monthlySpending as point, index (point.month)}
        <div class="flex flex-1 flex-col items-center">
          <div class="flex h-full w-full items-end justify-center">
            <div
              class="bar relative w-full cursor-pointer rounded-t transition-all duration-300 hover:opacity-80 {getBarColor(
                point.month,
                point.amount
              )}"
              style="height: {getBarHeight(point.amount)}%"
              title="{monthNames[index]}: {formatAmount(point.amount)}"
            >
              {#if point.amount > 0}
                <span
                  class="absolute -top-6 left-1/2 -translate-x-1/2 text-xs font-medium whitespace-nowrap text-surface-700"
                >
                  {formatAmount(point.amount)}
                </span>
              {/if}
            </div>
          </div>
          <div
            class="mt-2 text-xs text-surface-600"
            class:font-bold={point.month === currentMonth}
            class:text-primary-600={point.month === currentMonth}
          >
            {monthNames[index]}
          </div>
        </div>
      {/each}
    </div>
  </div>
</div>

<style>
  .bar {
    min-height: 4px;
  }
</style>
