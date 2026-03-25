<script lang="ts">
  /**
   * Budget Donut Chart Component
   *
   * Displays remaining budget as a donut chart with color gradient:
   * - Green (>50% remaining)
   * - Yellow (25-50% remaining)
   * - Red (<25% remaining)
   */

  import * as m from '$lib/paraglide/messages.js';
  import { regionalManager } from '$lib/features/settings/RegionalManager.svelte';

  interface Props {
    remainingAmount: number;
    totalAvailable: number;
    remainingPercentage: number;
    currency: string;
  }

  let { remainingAmount, totalAvailable, remainingPercentage, currency }: Props = $props();

  function formatAmount(minorUnits: number, currencyCode: string): string {
    return new Intl.NumberFormat(regionalManager.locale, {
      style: 'currency',
      currency: currencyCode,
      minimumFractionDigits: 0,
      maximumFractionDigits: 0
    }).format(minorUnits / 100);
  }

  function getColorClass(percentage: number): string {
    if (percentage >= 50) return 'text-success-500';
    if (percentage >= 25) return 'text-warning-500';
    return 'text-error-500';
  }

  function getStrokeColor(percentage: number): string {
    if (percentage >= 50) return 'rgb(34 197 94)'; // green-500
    if (percentage >= 25) return 'rgb(234 179 8)'; // yellow-500
    return 'rgb(239 68 68)'; // red-500
  }

  // SVG circle calculations
  const size = 200;
  const strokeWidth = 20;
  const radius = (size - strokeWidth) / 2;
  const circumference = 2 * Math.PI * radius;
  const offset = $derived(circumference - (remainingPercentage / 100) * circumference);
</script>

<div class="flex flex-col items-center gap-4 rounded-lg bg-card p-6">
  <div class="relative">
    <svg width={size} height={size} class="transform transition-transform duration-300">
      <!-- Background circle -->
      <circle
        cx={size / 2}
        cy={size / 2}
        r={radius}
        fill="none"
        stroke="rgb(229 231 235)"
        stroke-width={strokeWidth}
      />
      <!-- Progress circle -->
      <circle
        cx={size / 2}
        cy={size / 2}
        r={radius}
        fill="none"
        stroke={getStrokeColor(remainingPercentage)}
        stroke-width={strokeWidth}
        stroke-dasharray={circumference}
        stroke-dashoffset={offset}
        stroke-linecap="round"
        transform="rotate(-90 {size / 2} {size / 2})"
        class="transition-all duration-500 ease-out"
      />
    </svg>
    <div class="absolute inset-0 flex flex-col items-center justify-center">
      <div class="text-4xl font-bold {getColorClass(remainingPercentage)}">
        {remainingPercentage.toFixed(1)}%
      </div>
      <div class="text-surface-600 mt-1 text-sm capitalize">
        {m.budget_dashboard_donut_remaining()}
      </div>
    </div>
  </div>
  <div class="w-full space-y-2">
    <div class="flex items-center justify-between text-sm">
      <span class="text-surface-600">{m.budget_table_available_header()}:</span>
      <span class="text-surface-900 font-semibold">{formatAmount(totalAvailable, currency)}</span>
    </div>
    <div class="flex items-center justify-between text-sm">
      <span class="text-surface-600">{m.budget_table_remaining_header()}:</span>
      <span class="text-surface-900 font-semibold {getColorClass(remainingPercentage)}">
        {formatAmount(remainingAmount, currency)}
      </span>
    </div>
  </div>
</div>
