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

  import * as m from '$lib/paraglide/messages.js';
  import { Button, Input, Textarea } from '$lib/components';
  import type { BudgetState } from '../BudgetState.svelte';

  interface Props {
    budgetState: BudgetState;
    year: number;
    month: number;
    onClose?: () => void;
  }

  const { budgetState, year, month, onClose }: Props = $props();

  // Form state
  let amount = $state(0);
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
    if (!amount || amount <= 0) {
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
      const amountInCents = Math.round(amount * 100); // Convert to minor units

      await budgetState.addExtraBudget({
        year,
        month,
        amount: amountInCents,
        reason: reason || undefined
      });

      onClose?.();
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
    onClose?.();
  }
</script>

<div class="card w-full max-w-md p-6">
  <header class="mb-4">
    <h3 class="h3 text-surface-900 font-semibold">
      {m.budget_extra_modal_title()}
    </h3>
    <p class="text-surface-600 mt-1 text-sm">
      {monthName}
      {year}
    </p>
  </header>

  <form onsubmit={handleSubmit} class="space-y-4">
    <!-- Amount Input -->
    <div>
      <label for="extra-amount" class="text-surface-700 mb-1 block text-sm font-medium">
        {m.budget_extra_amount_label()}
      </label>
      <div class="relative">
        <span class="text-surface-500 absolute top-1/2 left-3 -translate-y-1/2">
          {budgetState.currency === 'EUR' ? '€' : '$'}
        </span>
        <Input
          id="extra-amount"
          type="number"
          step="0.01"
          min="0"
          value={amount}
          oninput={(e) => (amount = parseFloat(e.currentTarget.value) || 0)}
          disabled={isSubmitting}
          class="w-full py-2 pr-4 pl-8"
          placeholder="0.00"
          required
        />
      </div>
      {#if errors.amount}
        <p class="text-error-500 mt-1 text-sm">{errors.amount}</p>
      {/if}
    </div>

    <!-- Reason Input -->
    <div>
      <label for="extra-reason" class="text-surface-700 mb-1 block text-sm font-medium">
        {m.budget_extra_reason_label()}
      </label>
      <Textarea
        id="extra-reason"
        bind:value={reason}
        disabled={isSubmitting}
        class="w-full"
        rows={3}
        maxlength={500}
        placeholder={m.budget_extra_reason_placeholder()}
      />
      {#if errors.reason}
        <p class="text-error-500 mt-1 text-sm">{errors.reason}</p>
      {/if}
    </div>

    <!-- Action Buttons -->
    <footer class="flex justify-end gap-3 pt-4">
      <Button type="button" onclick={handleCancel} disabled={isSubmitting} variant="ghost">
        {m.budget_extra_cancel_button()}
      </Button>
      <Button type="submit" disabled={isSubmitting}>
        {isSubmitting ? m.budget_config_saving_button() : m.budget_extra_save_button()}
      </Button>
    </footer>
  </form>
</div>
