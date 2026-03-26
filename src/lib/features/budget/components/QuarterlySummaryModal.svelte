<script lang="ts">
  /**
   * Quarterly Summary Modal Component
   *
   * Shows spending breakdown by category for a selected quarter.
   * Displays total spending and category-by-category breakdown with percentages.
   */

  import type { QuarterlySummary } from '../services/BudgetService.svelte';
  import * as m from '$lib/paraglide/messages.js';
  import * as Dialog from '$lib/components/ui/dialog';
  import { Button } from '$lib/components';
  import { regionalManager } from '$lib/features/settings/RegionalManager.svelte';

  interface Props {
    summary: QuarterlySummary;
    onClose?: () => void;
  }

  let { summary, onClose }: Props = $props();

  function formatAmount(minorUnits: number, currencyCode: string): string {
    return regionalManager.formatCurrencyWith(minorUnits, currencyCode);
  }

  function getCategoryLabel(category: string): string {
    const categoryMap: Record<string, string> = {
      LOCOMOTIVES: 'Locomotives',
      TRAIN_SETS: 'Train Sets',
      PASSENGER_CARS: 'Passenger Cars',
      FREIGHT_CARS: 'Freight Cars',
      TRACK: 'Track',
      BUILDINGS: 'Buildings',
      ACCESSORIES: 'Accessories',
      DIGITAL: 'Digital',
      BOOKS: 'Books',
      OTHER: 'Other'
    };
    return categoryMap[category] || category;
  }

  function getCategoryColor(index: number): string {
    const colors = [
      'bg-primary-500',
      'bg-secondary-500',
      'bg-tertiary-500',
      'bg-success-500',
      'bg-warning-500',
      'bg-error-500',
      'bg-muted'
    ];
    return colors[index % colors.length];
  }
</script>

<Dialog.Header>
  <Dialog.Title>{summary.year} - {summary.quarter}</Dialog.Title>
  <Dialog.Description>{m.budget_quarterly_breakdown()}</Dialog.Description>
</Dialog.Header>

<!-- Total Spending -->
<div class="rounded-lg border border-border bg-muted p-4">
  <p class="text-sm text-muted-foreground">{m.budget_quarterly_total_label()}</p>
  <p class="text-primary-400 text-3xl font-bold">
    {formatAmount(summary.totalSpending.amount, summary.totalSpending.currency)}
  </p>
</div>

<!-- Category Breakdown -->
{#if summary.categoryBreakdown.length > 0}
  <div class="space-y-3">
    <h3 class="text-lg font-semibold text-foreground">{m.budget_quarterly_category_label()}</h3>

    {#each summary.categoryBreakdown as category, index (category.category)}
      <div
        class="rounded-lg border border-border bg-muted p-3 transition-all duration-200 hover:bg-muted/75"
      >
        <div class="mb-2 flex items-center justify-between">
          <div class="flex items-center gap-2">
            <div class="h-3 w-3 rounded-full {getCategoryColor(index)}"></div>
            <span class="font-medium text-foreground">{getCategoryLabel(category.category)}</span>
          </div>
          <span class="font-bold text-foreground">
            {formatAmount(category.amount.amount, category.amount.currency)}
          </span>
        </div>

        <!-- Progress Bar -->
        <div class="h-2 w-full overflow-hidden rounded-full bg-background">
          <div
            class="h-full {getCategoryColor(index)} transition-all duration-300"
            style="width: {category.percentage}%"
          ></div>
        </div>

        <!-- Percentage -->
        <div class="mt-1 text-right">
          <span class="text-xs text-muted-foreground">{category.percentage.toFixed(1)}%</span>
        </div>
      </div>
    {/each}
  </div>
{:else}
  <div class="py-8 text-center text-muted-foreground">
    {m.budget_no_spending()}
  </div>
{/if}

<Dialog.Footer>
  <Dialog.Close>
    <Button type="button" onclick={onClose}>{m.common_close()}</Button>
  </Dialog.Close>
</Dialog.Footer>
