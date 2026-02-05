<script lang="ts">
  /**
   * Historical Archive Component
   *
   * Accordion displaying budget records for the past 5 years.
   * Each year shows a full BudgetTable with 12-month breakdown.
   */

  import {
    Accordion,
    AccordionItem,
    AccordionItemTrigger,
    AccordionItemContent
  } from '$lib/components/accordion';
  import type { BudgetState } from '../BudgetState.svelte';
  import BudgetTable from './BudgetTable.svelte';
  import * as m from '$lib/paraglide/messages.js';

  interface Props {
    budgetState: BudgetState;
    currentYear: number;
    onYearSelect: (year: number) => Promise<void>;
  }

  let { budgetState, currentYear, onYearSelect }: Props = $props();

  // Generate list of past 5 years (excluding current year)
  const historicalYears = $derived(
    Array.from({ length: 5 }, (_, i) => currentYear - (i + 1)).filter((y) => y > 0)
  );

  let loadingYear: number | null = $state(null);

  async function handleYearExpand(year: number) {
    if (loadingYear) return; // Prevent concurrent loads

    loadingYear = year;
    try {
      await onYearSelect(year);
    } catch (error) {
      console.error(`Failed to load records for year ${year}:`, error);
    } finally {
      loadingYear = null;
    }
  }

  function formatYearRange(year: number): string {
    return `${year}`;
  }
</script>

<div class="historical-archive space-y-4">
  <h3 class="text-surface-100 mb-4 text-lg font-semibold">Historical Budget Data</h3>

  {#if historicalYears.length === 0}
    <p class="text-surface-400 text-sm">No historical data available.</p>
  {:else}
    <Accordion>
      {#snippet children(_context: { isExpanded: (value: string) => boolean })}
        {#each historicalYears as year (year)}
          <AccordionItem value={year.toString()}>
            {#snippet children({ isExpanded }: { isExpanded: boolean })}
              <AccordionItemTrigger value={year.toString()}>
                <div class="flex w-full items-center justify-between">
                  <span class="text-surface-100 font-semibold">{formatYearRange(year)}</span>
                  {#if loadingYear === year}
                    <span class="text-surface-400 text-xs"
                      >{m.budget_loading?.() || 'Loading...'}</span
                    >
                  {/if}
                </div>
              </AccordionItemTrigger>

              <AccordionItemContent value={year.toString()} {isExpanded}>
                <div class="p-4">
                  {#if budgetState.hasRecords && budgetState.monthlyRecords.length > 0 && budgetState.monthlyRecords[0].year === year}
                    <BudgetTable
                      records={budgetState.monthlyRecords}
                      {budgetState}
                      currency={budgetState.currency}
                    />
                  {:else}
                    <button
                      type="button"
                      class="variant-ghost-primary btn btn-sm"
                      onclick={() => handleYearExpand(year)}
                      disabled={loadingYear !== null}
                    >
                      {loadingYear === year
                        ? m.budget_loading?.() || 'Loading...'
                        : m.budget_mode_yearly?.() || `Load ${year} data`}
                    </button>
                  {/if}
                </div>
              </AccordionItemContent>
            {/snippet}
          </AccordionItem>
        {/each}
      {/snippet}
    </Accordion>
  {/if}
</div>
