<script lang="ts">
  import { BarChart, LinearGradient, PieChart } from 'layerchart';
  import { BarChart2, Wallet } from 'lucide-svelte';
  import { resolve } from '$app/paths';
  import { Button } from '$lib/components';
  import * as m from '$lib/paraglide/messages.js';

  type MonthlySpendingPoint = { month: number; amount: number };

  interface DashboardChartsProps {
    /** ISO 4217 currency code for formatting. @default 'EUR' */
    currencyCode?: string;
    /** Chart data overrides. */
    data?: {
      budget?: number;
      monthlySpending?: MonthlySpendingPoint[];
    };
    /** Enable compact mode with reduced chart heights. @default false */
    compact?: boolean;
    /** Whether a budget has been configured. undefined = unknown/loading. false = no budget. */
    hasBudget?: boolean;
  }

  let {
    currencyCode: currencyCodeProp,
    data: dataProp,
    compact = false,
    hasBudget
  }: DashboardChartsProps = $props();

  // --- Reactive Derived Logic ---
  const currencyCode = $derived(currencyCodeProp ?? 'EUR');
  const data = $derived(dataProp ?? {});
  const noBudget = $derived(hasBudget === false);
  const budget = $derived<number>(data.budget ?? 0);
  const monthlySpending = $derived<MonthlySpendingPoint[]>(data.monthlySpending ?? []);
  const hasSpendingData = $derived(monthlySpending.some((d) => d.amount > 0));
  const currentYear = new Date().getFullYear();

  // --- Formatters ---
  const monthFormatter = new Intl.DateTimeFormat(undefined, { month: 'short' });
  const currencyFormatter = $derived(
    new Intl.NumberFormat(undefined, {
      style: 'currency',
      currency: currencyCode,
      maximumFractionDigits: 0
    })
  );

  const formatMonthIndex = (monthIndex: number) =>
    monthFormatter.format(new Date(2024, Math.max(0, Math.min(11, monthIndex)), 1));

  const formatCurrency = (value: number) => currencyFormatter.format(value ?? 0);

  const budgetGradientId = 'budget-gradient';
  const budgetPercent = $derived(Math.round(budget * 100));
  const monthlyYMax = $derived(Math.max(...monthlySpending.map((d) => d.amount), 1));

  // Compact mode reduces chart height by ~30%
  const chartHeight = $derived(compact ? 'h-44' : 'h-64');
  const chartCardClass =
    'card gauge-frame p-4 transition-colors duration-200 backdrop-blur-sm bg-zinc-900/50 border border-zinc-800';
</script>

<div class="gauge-frame flex h-full items-center p-4">
  <div class="grid w-full grid-cols-1 gap-4 lg:grid-cols-2">
    <!-- Budget Gauge (PieChart) -->
    <div class={chartCardClass}>
      <div class="mb-3 space-y-1">
        <p class="text-xs font-semibold tracking-wide uppercase opacity-80">
          {m.dashboard_chart_budget_label()}
        </p>
        <p class="text-lg font-bold">{m.dashboard_chart_budget_title()}</p>
      </div>

      <div class="relative {chartHeight} w-full">
        {#if noBudget}
          <div class="flex h-full flex-col items-center justify-center gap-3 text-center">
            <div class="rounded-full bg-zinc-800/60 p-3">
              <Wallet size={28} class="text-zinc-500" />
            </div>
            <p class="text-sm text-zinc-400">{m.dashboard_chart_budget_no_budget()}</p>
            <Button variant="outline" size="sm" href={resolve('/finance')}>
              {m.dashboard_chart_budget_set_cta()}
            </Button>
          </div>
        {:else}
          <PieChart
            data={[{ key: 'available', value: budget }]}
            key="key"
            value="value"
            maxValue={1}
            innerRadius={0.65}
            padAngle={0.02}
            tooltip={false}
            props={{
              svg: { class: 'w-full h-full overflow-visible' },
              arc: {
                fill: `url(#${budgetGradientId})`,
                track: { fill: 'rgba(63, 63, 70, 0.4)' },
                strokeWidth: 0
              }
            }}
          >
            {#snippet belowMarks(_snippetProps)}
              <LinearGradient
                id={budgetGradientId}
                stops={[
                  ['0%', '#d48a3e'],
                  ['100%', '#b87333']
                ]}
                vertical
              />
            {/snippet}
          </PieChart>

          <div
            class="pointer-events-none absolute inset-0 flex flex-col items-center justify-center"
          >
            <span class="text-3xl font-extrabold text-white">{budgetPercent}%</span>
            <span class="text-xs tracking-tighter uppercase opacity-60"
              >{m.dashboard_chart_budget_remaining()}</span
            >
          </div>
        {/if}
      </div>
    </div>

    <!-- Monthly Spending (BarChart) -->
    <div class={chartCardClass}>
      <div class="mb-3 space-y-1">
        <p class="text-xs font-semibold tracking-wide uppercase opacity-80">
          {m.dashboard_chart_spending_label()}
        </p>
        <p class="text-lg font-bold">{m.dashboard_chart_spending_title()}</p>
      </div>

      <div class="relative {chartHeight} w-full">
        {#if !hasSpendingData}
          <div
            class="absolute inset-0 z-10 flex flex-col items-center justify-center gap-2 text-center"
          >
            <BarChart2 size={28} class="text-zinc-700" />
            <p class="text-sm text-zinc-500">
              {m.dashboard_chart_spending_no_data({ year: currentYear })}
            </p>
          </div>
          <!-- Ghost bars to preserve chart shape -->
          <div class="flex h-full w-full items-end gap-1 px-8 opacity-10">
            {#each Array(12) as _, i (i)}
              <div class="flex-1 rounded-t bg-zinc-700" style="height: {20 + (i % 4) * 10}%"></div>
            {/each}
          </div>
        {:else}
          <BarChart
            data={monthlySpending}
            x={(d) => d.month}
            y={(d) => d.amount}
            yDomain={[0, monthlyYMax * 1.1]}
            padding={{ top: 10, right: 10, bottom: 30, left: 55 }}
            props={{
              svg: { class: 'w-full h-full overflow-visible' },
              bars: { fill: '#d48a3e', radius: 4, strokeWidth: 0 },
              grid: {
                y: { style: 'stroke: rgba(82, 82, 91, 0.3); stroke-dasharray: 4 4;' }
              },
              xAxis: {
                format: (v: unknown) => formatMonthIndex(Number(v))
              },
              yAxis: {
                format: (v: unknown) => formatCurrency(Number(v))
              },
              tooltip: {
                header: { format: (v: unknown) => formatMonthIndex(Number(v)) },
                item: { format: (v: unknown) => formatCurrency(Number(v)) }
              }
            }}
          />
        {/if}
      </div>
    </div>
  </div>
</div>
