<script lang="ts">
  /**
   * Extra Budget Modal
   *
   * Allows users to add a one-time budget injection to a specific month.
   * Features:
   * - Amount input with currency formatting
   * - Optional reason field
   * - Form validation
   * - Error handling with toast feedback
   */

  // @ts-expect-error - Skeleton types not available but package works at runtime
  import { getModalStore } from '@skeletonlabs/skeleton';
  import * as m from '$lib/paraglide/messages.js';
  import type { BudgetState } from '../BudgetState.svelte';

  interface Props {
    budgetState: BudgetState;
    year: number;
    month: number;
  }

  const { budgetState, year, month }: Props = $props();

  const modalStore = getModalStore();

  // Form state
  let amount = $state('');
  let reason = $state('');
  let isSubmitting = $state(false);
  let errors = $state<Record<string, string>>({});

  // Convert month number to name for display
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
  const monthName = $derived(monthNames[month - 1] || 'Unknown');

  /**
   * Validate form inputs.
   */
  function validateForm(): boolean {
    const newErrors: Record<string, string> = {};

    // Validate amount
    const amountValue = parseFloat(amount);
    if (!amount || isNaN(amountValue) || amountValue <= 0) {
      newErrors.amount = 'Please enter a valid positive amount';
    }

    // Validate reason length
    if (reason && reason.length > 500) {
      newErrors.reason = 'Reason must be 500 characters or less';
    }

    errors = newErrors;
    return Object.keys(newErrors).length === 0;
  }

  /**
   * Handle form submission.
   */
  async function handleSubmit(event: Event) {
    event.preventDefault();

    if (!validateForm()) {
      return;
    }

    isSubmitting = true;

    try {
      const amountValue = parseFloat(amount);
      const amountInCents = Math.round(amountValue * 100); // Convert to minor units

      await budgetState.addExtraBudget({
        year,
        month,
        amount: amountInCents,
        reason: reason || undefined
      });

      modalStore.close();
    } catch (error) {
      console.error('Failed to add extra budget:', error);
      // Error toast is already shown by the service
    } finally {
      isSubmitting = false;
    }
  }

  /**
   * Handle cancel action.
   */
  function handleCancel() {
    modalStore.close();
  }
</script>

<div class="w-full max-w-md card p-6">
  <header class="mb-4">
    <h3 class="h3 font-semibold text-surface-900">
      {m.budget_extra_modal_title()}
    </h3>
    <p class="mt-1 text-sm text-surface-600">
      {monthName}
      {year}
    </p>
  </header>

  <form onsubmit={handleSubmit} class="space-y-4">
    <!-- Amount Input -->
    <div>
      <label for="extra-amount" class="mb-1 block text-sm font-medium text-surface-700">
        {m.budget_extra_amount_label()}
      </label>
      <div class="relative">
        <span class="absolute top-1/2 left-3 -translate-y-1/2 text-surface-500">
          {budgetState.currency === 'EUR' ? '€' : '$'}
        </span>
        <input
          id="extra-amount"
          type="number"
          step="0.01"
          min="0"
          bind:value={amount}
          disabled={isSubmitting}
          class="input w-full py-2 pr-4 pl-8"
          placeholder="0.00"
          required
        />
      </div>
      {#if errors.amount}
        <p class="mt-1 text-sm text-error-500">{errors.amount}</p>
      {/if}
    </div>

    <!-- Reason Input -->
    <div>
      <label for="extra-reason" class="mb-1 block text-sm font-medium text-surface-700">
        {m.budget_extra_reason_label()}
      </label>
      <textarea
        id="extra-reason"
        bind:value={reason}
        disabled={isSubmitting}
        class="textarea w-full"
        rows="3"
        maxlength="500"
        placeholder={m.budget_extra_reason_placeholder()}
      ></textarea>
      {#if errors.reason}
        <p class="mt-1 text-sm text-error-500">{errors.reason}</p>
      {/if}
    </div>

    <!-- Action Buttons -->
    <footer class="flex justify-end gap-3 pt-4">
      <button
        type="button"
        onclick={handleCancel}
        disabled={isSubmitting}
        class="variant-ghost-surface btn"
      >
        {m.budget_extra_cancel_button()}
      </button>
      <button type="submit" disabled={isSubmitting} class="variant-filled-primary btn">
        {isSubmitting ? m.budget_config_saving_button() : m.budget_extra_save_button()}
      </button>
    </footer>
  </form>
</div>

<style>
  .input,
  .textarea {
    border-radius: 0.5rem;
    border-width: 1px;
    border-color: rgb(212 212 216);
  }

  .input:focus,
  .textarea:focus {
    border-color: rgb(59 130 246);
    box-shadow: 0 0 0 2px rgba(59, 130, 246, 0.5);
    outline: none;
  }

  .input:disabled,
  .textarea:disabled {
    cursor: not-allowed;
    opacity: 0.5;
  }
</style>
