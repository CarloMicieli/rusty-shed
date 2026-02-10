<script lang="ts">
  import { Check } from 'lucide-svelte';
  import * as m from '$lib/paraglide/messages.js';
  import { Button } from '$lib/components';
  import type { BudgetMode } from '../services/BudgetService.svelte';

  let {
    mode = $bindable('MONTHLY'),
    baseAmount = $bindable(0),
    currency,
    saving = false,
    onsubmit
  } = $props<{
    mode?: BudgetMode;
    baseAmount?: number;
    currency?: string;
    saving?: boolean;
    onsubmit: (mode: BudgetMode, amount: number) => void;
  }>();

  // Local form state for amount input (in major units)
  // eslint-disable-next-line svelte/prefer-writable-derived
  let amountInputValue = $state((baseAmount / 100).toFixed(2));

  // Keep amountInputValue in sync when baseAmount changes from outside
  $effect(() => {
    amountInputValue = (baseAmount / 100).toFixed(2);
  });

  // Mode options
  const modeOptions: { label: string; value: BudgetMode }[] = [
    { label: m.budget_config_mode_monthly(), value: 'MONTHLY' },
    { label: m.budget_config_mode_yearly(), value: 'YEARLY' }
  ];

  // Derived: Calculate equivalent display amounts
  const monthlyDisplay = $derived.by(() => {
    const amount = parseFloat(amountInputValue) || 0;
    if (mode === 'MONTHLY') return amount;
    return amount / 12;
  });

  const yearlyDisplay = $derived.by(() => {
    const amount = parseFloat(amountInputValue) || 0;
    if (mode === 'YEARLY') return amount;
    return amount * 12;
  });

  function handleSubmit(event: Event) {
    event.preventDefault();

    // Convert major units to minor units (cents)
    const amount = parseFloat(amountInputValue) || 0;
    const minorUnits = Math.round(amount * 100);

    // Update the bindable baseAmount
    baseAmount = minorUnits;

    onsubmit(mode, minorUnits);
  }

  function formatCurrency(value: number): string {
    return new Intl.NumberFormat(undefined, {
      style: 'currency',
      currency: currency ?? 'EUR',
      minimumFractionDigits: 2,
      maximumFractionDigits: 2
    }).format(value);
  }
</script>

<section class="card border-surface-700/60 bg-surface-900/50 border shadow-xl">
  <header class="border-surface-700/60 flex items-center justify-between gap-4 border-b p-6">
    <div>
      <p class="text-surface-400 text-sm font-semibold tracking-widest uppercase">
        {m.budget_config_title()}
      </p>
      <h2 class="text-surface-100 text-xl font-bold">{m.budget_subtitle()}</h2>
    </div>
  </header>

  <form class="space-y-6 p-6" onsubmit={handleSubmit}>
    <!-- Budget Mode Toggle -->
    <div class="space-y-2">
      <label for="budget-mode" class="text-surface-300 block text-sm font-medium">
        {m.budget_config_mode_label()}
      </label>
      <div id="budget-mode" class="flex gap-3">
        {#each modeOptions as option (option.value)}
          <label
            class="border-surface-700 bg-surface-800/50 hover:border-primary-500 hover:bg-surface-800 flex flex-1 cursor-pointer items-center gap-3 rounded-lg border p-4 transition-all"
            class:border-primary-500={mode === option.value}
            class:bg-surface-800={mode === option.value}
          >
            <input
              type="radio"
              name="mode"
              value={option.value}
              bind:group={mode}
              class="sr-only"
            />
            <div
              class="flex h-5 w-5 items-center justify-center rounded-full border-2 transition-colors"
              class:border-primary-500={mode === option.value}
              class:bg-primary-500={mode === option.value}
              class:border-surface-500={mode !== option.value}
            >
              {#if mode === option.value}
                <div class="h-2 w-2 rounded-full bg-white"></div>
              {/if}
            </div>
            <span class="text-surface-100 font-medium">{option.label}</span>
          </label>
        {/each}
      </div>
    </div>

    <!-- Budget Amount Input -->
    <div class="space-y-2">
      <label for="amount" class="text-surface-300 block text-sm font-medium">
        {m.budget_config_amount_label()}
        {mode === 'MONTHLY' ? m.budget_config_mode_monthly() : m.budget_config_mode_yearly()}
      </label>
      <input
        id="amount"
        type="number"
        step="0.01"
        min="0"
        bind:value={amountInputValue}
        required
        disabled={saving}
        class="border-surface-700 bg-surface-800 placeholder-surface-500 focus:border-primary-500 focus:ring-primary-500/50 text-surface-100 w-full rounded-lg border px-4 py-3 focus:ring-2 focus:outline-none disabled:cursor-not-allowed disabled:opacity-50"
        placeholder="0.00"
      />
      <p class="text-surface-400 text-xs">
        {m.budget_config_amount_helper()}
      </p>
    </div>

    <!-- Derived Calculations Display -->
    <div class="border-surface-700 bg-surface-800/30 space-y-2 rounded-lg border p-4">
      <h3 class="text-surface-300 text-sm font-semibold">{m.budget_config_summary_title()}</h3>
      <div class="grid grid-cols-2 gap-4 text-sm">
        <div>
          <p class="text-surface-400">{m.budget_config_mode_monthly()}</p>
          <p class="text-primary-400 text-lg font-bold">{formatCurrency(monthlyDisplay)}</p>
        </div>
        <div>
          <p class="text-surface-400">{m.budget_config_mode_yearly()}</p>
          <p class="text-primary-400 text-lg font-bold">{formatCurrency(yearlyDisplay)}</p>
        </div>
      </div>
    </div>

    <!-- Submit Button -->
    <div class="flex items-center gap-3">
      <Button type="submit" variant="secondary" disabled={saving}>
        <Check size={16} />
        <span>{saving ? m.budget_config_saving_button() : m.budget_config_save_button()}</span>
      </Button>

      {#if saving}
        <div class="text-surface-400 flex items-center gap-2 text-sm">
          <div
            class="border-surface-600 border-t-primary-500 h-4 w-4 animate-spin rounded-full border-2"
          ></div>
          <span>{m.budget_config_saving_status()}</span>
        </div>
      {/if}
    </div>
  </form>
</section>
