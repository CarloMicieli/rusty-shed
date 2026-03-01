<script lang="ts">
  /**
   * Activity Heatmap Component
   *
   * Displays 5-year quarterly spending activity as a heatmap grid.
   * Color intensity represents spending level (None/Low/Medium/High).
   */

  import { SvelteMap } from 'svelte/reactivity';
  import * as Dialog from '$lib/components/ui/dialog';
  import * as m from '$lib/paraglide/messages.js';
  import type { QuarterlyActivityPoint, QuarterlySummary } from '../services/BudgetService.svelte';
  import type { BudgetState } from '../BudgetState.svelte';
  import QuarterlySummaryModal from './QuarterlySummaryModal.svelte';

  interface Props {
    quarterlyActivity: QuarterlyActivityPoint[];
    currency: string;
    budgetState: BudgetState;
  }

  let { quarterlyActivity, currency, budgetState }: Props = $props();

  let dialogOpen = $state(false);
  let selectedSummary = $state<QuarterlySummary | null>(null);

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

  const levelColorMap: Record<string, string> = {
    NONE: 'bg-muted',
    LOW: 'bg-emerald-300',
    MEDIUM: 'bg-amber-300',
    HIGH: 'bg-rose-400'
  };

  function getLevelColor(level: string): string {
    return levelColorMap[level] ?? 'bg-muted';
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

  function formatAmount(minorUnits: number, currencyCode: string = currency): string {
    const major = minorUnits / 100;
    return new Intl.NumberFormat(undefined, {
      style: 'currency',
      currency: currencyCode,
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
      selectedSummary = summary;
      dialogOpen = true;
    } else {
      selectedSummary = null;
      dialogOpen = false;
    }
  }
</script>

<div class="rounded-lg bg-card p-6">
  <h3 class="mb-4 text-lg font-semibold">{m.budget_dashboard_heatmap_title()}</h3>
  <div class="space-y-1">
    <!-- Header row with quarter labels -->
    <div class="grid grid-cols-[80px_repeat(4,1fr)] gap-1">
      <div
        class="flex items-center justify-end pr-2 text-sm font-medium text-muted-foreground"
      ></div>
      {#each quarters as quarter (quarter)}
        <div class="pb-1 text-center text-xs font-medium text-muted-foreground">{quarter}</div>
      {/each}
    </div>

    <!-- Data rows -->
    {#each years as year (year)}
      <div class="grid grid-cols-[80px_repeat(4,1fr)] gap-1">
        <div class="flex items-center justify-end pr-2 text-sm font-medium text-muted-foreground">
          {year}
        </div>
        {#each quarters as quarter (quarter)}
          {@const activity = getActivityLevel(year, quarter)}
          <button
            type="button"
            class="hover:ring-primary-500 relative flex aspect-square cursor-pointer items-center justify-center rounded transition-all duration-200 hover:z-10 hover:ring-2 {activity
              ? getLevelColor(activity.spendingLevel)
              : 'bg-muted'}"
            title={activity
              ? `${year} ${quarter}: ${formatAmount(activity.amount)} (${getLevelLabel(activity.spendingLevel)})`
              : `${year} ${quarter}: No data`}
            onclick={() => handleCellClick(year, quarter)}
          >
            {#if activity && activity.amount > 0}
              <span class="text-xs font-medium text-foreground">
                {formatAmount(activity.amount)}
              </span>
            {/if}
          </button>
        {/each}
      </div>
    {/each}
  </div>

  <!-- Legend -->
  <div class="mt-4 flex items-center gap-4 border-t border-border/20 pt-4">
    <span class="text-sm font-medium text-muted-foreground">Spending Level:</span>
    <div class="flex gap-3">
      <div class="flex items-center gap-1.5">
        <div class="h-4 w-4 rounded bg-muted"></div>
        <span class="text-xs text-muted-foreground">None</span>
      </div>
      <div class="flex items-center gap-1.5">
        <div class="h-4 w-4 rounded bg-emerald-300"></div>
        <span class="text-xs text-muted-foreground">Low</span>
      </div>
      <div class="flex items-center gap-1.5">
        <div class="h-4 w-4 rounded bg-amber-300"></div>
        <span class="text-xs text-muted-foreground">Medium</span>
      </div>
      <div class="flex items-center gap-1.5">
        <div class="h-4 w-4 rounded bg-rose-400"></div>
        <span class="text-xs text-muted-foreground">High</span>
      </div>
    </div>
  </div>
</div>

{#if selectedSummary}
  <Dialog.Root bind:open={dialogOpen}>
    <Dialog.Content class="max-w-2xl">
      <QuarterlySummaryModal
        summary={selectedSummary}
        onClose={() => {
          dialogOpen = false;
          selectedSummary = null;
        }}
      />
    </Dialog.Content>
  </Dialog.Root>
{/if}
