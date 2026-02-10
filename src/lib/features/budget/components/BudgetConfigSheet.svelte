<script lang="ts">
  import { Check, X } from 'lucide-svelte';
  import * as m from '$lib/paraglide/messages.js';
  import { Button, Input, Sheet } from '$lib/components';
  import type { BudgetMode } from '../services/BudgetService.svelte';

  let {
    open = $bindable(false),
    mode = $bindable('MONTHLY' as BudgetMode),
    baseAmount = $bindable(0),
    currency,
    saving = false,
    onsubmit
  } = $props<{
    open?: boolean;
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

  function getModeButtonClasses(isActive: boolean): string {
    const baseClasses =
      'flex flex-1 items-center justify-center gap-2 rounded-lg border px-4 py-3 text-sm font-medium transition-all';
    if (isActive) {
      return `${baseClasses} border-amber-600 bg-amber-950/30 text-amber-500`;
    }
    return `${baseClasses} border-zinc-800 bg-zinc-900 text-zinc-400`;
  }

  function getRadioClasses(isActive: boolean): string {
    const baseClasses =
      'flex h-5 w-5 items-center justify-center rounded-full border-2 transition-colors';
    if (isActive) {
      return `${baseClasses} border-amber-600 bg-amber-600`;
    }
    return `${baseClasses} border-zinc-600`;
  }
</script>

<Sheet bind:open side="right" class="w-full border-zinc-800 bg-zinc-950 p-6 sm:max-w-lg">
  <!-- Sheet Header -->
  <div class="mb-6 flex items-start justify-between">
    <div>
      <h2 class="text-xl font-semibold text-zinc-50">{m.budget_config_title()}</h2>
      <p class="mt-1 text-sm text-zinc-400">Configure your hobby budget tracking settings.</p>
    </div>
    <button
      type="button"
      onclick={() => (open = false)}
      class="rounded-md p-1 text-zinc-400 hover:bg-zinc-900 hover:text-zinc-200"
    >
      <X size={20} />
    </button>
  </div>

  <form class="space-y-6" onsubmit={handleSubmit}>
    <!-- Budget Mode Toggle -->
    <fieldset class="space-y-3">
      <legend class="block text-sm font-medium text-zinc-300">
        {m.budget_config_mode_label()}
      </legend>
      <div class="flex gap-3">
        <button
          type="button"
          onclick={() => (mode = 'MONTHLY')}
          class={getModeButtonClasses(mode === 'MONTHLY')}
        >
          <div class={getRadioClasses(mode === 'MONTHLY')}>
            {#if mode === 'MONTHLY'}
              <div class="h-2 w-2 rounded-full bg-white"></div>
            {/if}
          </div>
          {m.budget_config_mode_monthly()}
        </button>

        <button
          type="button"
          onclick={() => (mode = 'YEARLY')}
          class={getModeButtonClasses(mode === 'YEARLY')}
        >
          <div class={getRadioClasses(mode === 'YEARLY')}>
            {#if mode === 'YEARLY'}
              <div class="h-2 w-2 rounded-full bg-white"></div>
            {/if}
          </div>
          {m.budget_config_mode_yearly()}
        </button>
      </div>
    </fieldset>

    <!-- Budget Amount Input -->
    <div class="space-y-3">
      <label for="amount" class="block text-sm font-medium text-zinc-300">
        {m.budget_config_amount_label()}
        ({mode === 'MONTHLY' ? m.budget_config_mode_monthly() : m.budget_config_mode_yearly()})
      </label>
      <Input
        id="amount"
        type="number"
        step="0.01"
        min="0"
        bind:value={amountInputValue}
        required
        disabled={saving}
        class="border-zinc-800 bg-zinc-900 text-zinc-100 placeholder-zinc-500 focus-visible:border-amber-500 focus-visible:ring-amber-500"
        placeholder="0.00"
      />
      <p class="text-xs text-zinc-500">
        {m.budget_config_amount_helper()}
      </p>
    </div>

    <!-- Derived Calculations Display -->
    <div class="space-y-3 rounded-lg border border-zinc-800 bg-zinc-900/50 p-4">
      <h3 class="text-sm font-semibold text-zinc-300">{m.budget_config_summary_title()}</h3>
      <div class="grid grid-cols-2 gap-4">
        <div>
          <p class="text-xs text-zinc-500">{m.budget_config_mode_monthly()}</p>
          <p class="text-lg font-bold text-amber-500">{formatCurrency(monthlyDisplay)}</p>
        </div>
        <div>
          <p class="text-xs text-zinc-500">{m.budget_config_mode_yearly()}</p>
          <p class="text-lg font-bold text-amber-500">{formatCurrency(yearlyDisplay)}</p>
        </div>
      </div>
    </div>

    <!-- Submit Button -->
    <div class="flex items-center gap-3 pt-4">
      <Button
        type="submit"
        variant="secondary"
        disabled={saving}
        class="flex-1 bg-amber-600 text-white hover:bg-amber-700"
      >
        <Check size={16} />
        <span>{saving ? m.budget_config_saving_button() : m.budget_config_save_button()}</span>
      </Button>

      {#if saving}
        <div class="flex items-center gap-2 text-sm text-zinc-400">
          <div
            class="h-4 w-4 animate-spin rounded-full border-2 border-zinc-700 border-t-amber-500"
          ></div>
        </div>
      {/if}
    </div>
  </form>
</Sheet>
