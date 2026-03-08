<script lang="ts">
  /**
   * Yearly Spending Chart Component
   *
   * Displays monthly spending as a bar chart with a horizontal budget goal line.
   * Shows 12 bars (one per month) with the current month highlighted.
   */

  import * as m from '$lib/paraglide/messages.js';
  import { getLocale } from '$lib/paraglide/runtime';
  import type { MonthlySpendingPoint } from '../services/BudgetService.svelte';

  interface Props {
    monthlySpending: MonthlySpendingPoint[];
    monthlyGoal: number;
    currency: string;
  }

  let { monthlySpending, monthlyGoal, currency }: Props = $props();

  const monthNames = $derived.by(() => {
    const locale = getLocale();
    return Array.from({ length: 12 }, (_, i) =>
      new Intl.DateTimeFormat(locale, { month: 'short' }).format(new Date(2000, i, 1))
    );
  });
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
    return 'bg-muted';
  }
</script>

<div class="rounded-lg bg-card p-6">
  <h3 class="text-surface-900 mb-4 text-lg font-semibold">{m.budget_dashboard_bar_title()}</h3>
  <div class="relative h-64">
    <!-- Goal line -->
    <div
      class="border-success-500 absolute right-0 left-0 z-10 border-t-2 border-dashed"
      style="bottom: {getBarHeight(monthlyGoal)}%"
    >
      <span class="text-success-700 absolute -top-5 right-0 bg-card px-2 text-xs">
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
                  class="text-surface-700 absolute -top-6 left-1/2 -translate-x-1/2 text-xs font-medium whitespace-nowrap"
                >
                  {formatAmount(point.amount)}
                </span>
              {/if}
            </div>
          </div>
          <div
            class="text-surface-600 mt-2 text-xs"
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
