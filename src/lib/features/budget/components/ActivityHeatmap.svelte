<script lang="ts">
  /**
   * Activity Heatmap Component
   *
   * Displays 5-year quarterly spending activity as a heatmap grid.
   * Color intensity represents spending level (None/Low/Medium/High).
   */

  import { SvelteMap } from 'svelte/reactivity';
  // @ts-expect-error - Skeleton types not available but package works at runtime
  import { getModalStore } from '@skeletonlabs/skeleton';
  import type { QuarterlyActivityPoint } from '../services/BudgetService.svelte';
  import type { BudgetState } from '../BudgetState.svelte';
  import QuarterlySummaryModal from './QuarterlySummaryModal.svelte';

  interface Props {
    quarterlyActivity: QuarterlyActivityPoint[];
    currency: string;
    budgetState: BudgetState;
  }

  let { quarterlyActivity, currency, budgetState }: Props = $props();

  const modalStore = getModalStore();

  // Group activities by year and quarter (reactive)
  const activityMap = $derived.by(() => {
    const map = new SvelteMap<string, QuarterlyActivityPoint>();
    quarterlyActivity.forEach((point) => {
      const key = `${point.year}-${point.quarter}`;
      map.set(key, point);
    });
    return map;
  });

  // Get unique years sorted (reactive)
  const years = $derived(Array.from(new Set(quarterlyActivity.map((p) => p.year))).sort());
  const quarters = ['Q1', 'Q2', 'Q3', 'Q4'] as const;

  function getActivityLevel(year: number, quarter: string): QuarterlyActivityPoint | undefined {
    return activityMap.get(`${year}-${quarter}`);
  }

  function getLevelColor(level: string): string {
    switch (level) {
      case 'NONE':
        return 'bg-surface-200';
      case 'LOW':
        return 'bg-success-200';
      case 'MEDIUM':
        return 'bg-warning-300';
      case 'HIGH':
        return 'bg-error-400';
      default:
        return 'bg-surface-200';
    }
  }

  function getLevelLabel(level: string): string {
    switch (level) {
      case 'NONE':
        return 'No spending';
      case 'LOW':
        return 'Low spending';
      case 'MEDIUM':
        return 'Medium spending';
      case 'HIGH':
        return 'High spending';
      default:
        return 'Unknown';
    }
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

  async function handleCellClick(year: number, quarter: string) {
    // Load quarterly summaries for the selected year
    await budgetState.loadQuarterlySummaries(year, currency);

    // Find the summary for the clicked quarter
    const summary = budgetState.quarterlySummaries.find(
      (s) => s.year === year && s.quarter === quarter
    );

    if (summary) {
      modalStore.trigger({
        type: 'component',
        component: {
          ref: QuarterlySummaryModal,
          props: {
            summary
          }
        }
      });
    }
  }
</script>

<div class="rounded-lg bg-surface-50 p-6">
  <h3 class="mb-4 text-lg font-semibold text-surface-900">5-Year Quarterly Activity</h3>
  <div class="space-y-1">
    <!-- Header row with quarter labels -->
    <div class="grid grid-cols-[80px_repeat(4,1fr)] gap-1">
      <div class="flex items-center justify-end pr-2 text-sm font-medium text-surface-700"></div>
      {#each quarters as quarter (quarter)}
        <div class="pb-1 text-center text-xs font-medium text-surface-600">{quarter}</div>
      {/each}
    </div>

    <!-- Data rows -->
    {#each years as year (year)}
      <div class="grid grid-cols-[80px_repeat(4,1fr)] gap-1">
        <div class="flex items-center justify-end pr-2 text-sm font-medium text-surface-700">
          {year}
        </div>
        {#each quarters as quarter (quarter)}
          {@const activity = getActivityLevel(year, quarter)}
          <button
            type="button"
            class="relative flex aspect-square cursor-pointer items-center justify-center rounded transition-all duration-200 hover:z-10 hover:ring-2 hover:ring-primary-500 {activity
              ? getLevelColor(activity.spendingLevel)
              : 'bg-surface-200'}"
            title={activity
              ? `${year} ${quarter}: ${formatAmount(activity.amount)} (${getLevelLabel(activity.spendingLevel)})`
              : `${year} ${quarter}: No data`}
            onclick={() => handleCellClick(year, quarter)}
          >
            {#if activity && activity.amount > 0}
              <span class="text-xs font-medium text-surface-900">
                {formatAmount(activity.amount)}
              </span>
            {/if}
          </button>
        {/each}
      </div>
    {/each}
  </div>

  <!-- Legend -->
  <div class="mt-4 flex items-center gap-4 border-t border-surface-200 pt-4">
    <span class="text-sm font-medium text-surface-700">Spending Level:</span>
    <div class="flex gap-3">
      <div class="flex items-center gap-1.5">
        <div class="h-4 w-4 rounded bg-surface-200"></div>
        <span class="text-xs text-surface-600">None</span>
      </div>
      <div class="flex items-center gap-1.5">
        <div class="h-4 w-4 rounded bg-success-200"></div>
        <span class="text-xs text-surface-600">Low</span>
      </div>
      <div class="flex items-center gap-1.5">
        <div class="h-4 w-4 rounded bg-warning-300"></div>
        <span class="text-xs text-surface-600">Medium</span>
      </div>
      <div class="flex items-center gap-1.5">
        <div class="h-4 w-4 rounded bg-error-400"></div>
        <span class="text-xs text-surface-600">High</span>
      </div>
    </div>
  </div>
</div>
