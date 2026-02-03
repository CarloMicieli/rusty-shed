<script lang="ts">
  import type { DigitalSummary } from '$lib/bindings';
  import * as m from '$lib/paraglide/messages';

  interface Props {
    summary: DigitalSummary | null;
    loading?: boolean;
  }

  let { summary = null, loading = false }: Props = $props();

  const formattedPercentage = $derived(summary ? summary.percentage.toFixed(1) : '0.0');
</script>

<div class="space-y-4 card p-6">
  <h2 class="h3 font-bold">{m.digital_roster_summary_title()}</h2>

  {#if loading}
    <div class="flex items-center justify-center py-8">
      <div class="h-8 w-8 animate-spin rounded-full border-b-2 border-primary-500"></div>
    </div>
  {:else if summary}
    <div class="grid grid-cols-1 gap-4 md:grid-cols-3">
      <!-- Percentage Card -->
      <div class="variant-ghost-primary card p-4 text-center">
        <div class="text-4xl font-bold text-primary-500">
          {formattedPercentage}%
        </div>
        <div class="mt-2 text-sm opacity-75">
          {m.digital_roster_percentage({ percentage: formattedPercentage })}
        </div>
      </div>

      <!-- Total Non-Dummy Count -->
      <div class="variant-ghost-surface card p-4 text-center">
        <div class="text-3xl font-semibold">
          {summary.total_non_dummy}
        </div>
      </div>

      <!-- Digital Count -->
      <div class="variant-ghost-success card p-4 text-center">
        <div class="text-3xl font-semibold text-success-500">
          {summary.digital_count}
        </div>
        <div class="mt-2 text-sm opacity-75">
          {m.digital_roster_digital_count({ count: summary.digital_count.toString() })}
        </div>
      </div>
    </div>
  {:else}
    <p class="text-center opacity-75">{m.digital_roster_loading()}</p>
  {/if}
</div>
