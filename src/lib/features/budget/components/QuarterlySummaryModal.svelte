<script lang="ts">
  /**
   * Quarterly Summary Modal Component
   *
   * Shows spending breakdown by category for a selected quarter.
   * Displays total spending and category-by-category breakdown with percentages.
   */

  import type { QuarterlySummary } from '../services/BudgetService.svelte';
  import * as m from '$lib/paraglide/messages.js';

  interface Props {
    summary: QuarterlySummary;
    onClose?: () => void;
  }

  let { summary, onClose }: Props = $props();

  // @ts-expect-error - paraglide message may not exist yet
  const noSpendingText = m.budget_no_spending?.() || 'No spending data for this quarter';
  // @ts-expect-error - paraglide message may not exist yet
  const closeText = m.common_close?.() || 'Close';

  function formatAmount(minorUnits: number, currencyCode: string): string {
    const major = minorUnits / 100;
    return new Intl.NumberFormat(undefined, {
      style: 'currency',
      currency: currencyCode,
      minimumFractionDigits: 2,
      maximumFractionDigits: 2
    }).format(major);
  }

  function getCategoryLabel(category: string): string {
    // Map category enum values to display labels
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

  function closeModal() {
    onClose?.();
  }
</script>

<div class="modal-content max-w-2xl rounded-lg bg-card p-6 shadow-xl">
  <!-- Header -->
  <header class="mb-6">
    <h2 class="text-surface-50 text-2xl font-bold">
      {summary.year} - {summary.quarter}
    </h2>
    <p class="text-surface-400 mt-1">Category Spending Breakdown</p>
  </header>

  <!-- Total Spending -->
  <div class="mb-6 rounded-lg border border-border bg-muted p-4">
    <p class="text-surface-400 text-sm">Total Spending</p>
    <p class="text-primary-400 text-3xl font-bold">
      {formatAmount(summary.totalSpending.amount, summary.totalSpending.currency)}
    </p>
  </div>

  <!-- Category Breakdown -->
  {#if summary.categoryBreakdown.length > 0}
    <div class="space-y-3">
      <h3 class="text-surface-100 mb-3 text-lg font-semibold">By Category</h3>

      {#each summary.categoryBreakdown as category, index (category.category)}
        <div class="category-item rounded-lg border border-border bg-muted p-3">
          <div class="mb-2 flex items-center justify-between">
            <div class="flex items-center gap-2">
              <div class="h-3 w-3 rounded-full {getCategoryColor(index)}"></div>
              <span class="text-surface-100 font-medium">{getCategoryLabel(category.category)}</span
              >
            </div>
            <span class="text-surface-50 font-bold">
              {formatAmount(category.amount.amount, category.amount.currency)}
            </span>
          </div>

          <!-- Progress Bar -->
          <div class="h-2 w-full overflow-hidden rounded-full bg-muted">
            <div
              class="h-full {getCategoryColor(index)} transition-all duration-300"
              style="width: {category.percentage}%"
            ></div>
          </div>

          <!-- Percentage -->
          <div class="mt-1 text-right">
            <span class="text-surface-400 text-xs">{category.percentage.toFixed(1)}%</span>
          </div>
        </div>
      {/each}
    </div>
  {:else}
    <div class="text-surface-500 py-8 text-center">
      {noSpendingText}
    </div>
  {/if}

  <!-- Footer -->
  <footer class="mt-6 flex justify-end">
    <button type="button" class="variant-filled-primary btn" onclick={closeModal}>
      {closeText}
    </button>
  </footer>
</div>

<style>
  .modal-content {
    min-width: 500px;
    max-height: 80vh;
    overflow-y: auto;
  }

  .category-item {
    transition: all 0.2s ease;
  }

  .category-item:hover {
    background-color: rgb(63 63 70);
  }
</style>
