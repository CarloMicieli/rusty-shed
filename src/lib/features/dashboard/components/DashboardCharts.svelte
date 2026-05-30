<script lang="ts">
  import { onMount } from 'svelte';
  import { BarChart2, Wallet } from 'lucide-svelte';
  import * as m from '$lib/paraglide/messages.js';
  import { regionalManager } from '$lib/features/settings/RegionalManager.svelte';

  type MonthlySpendingPoint = { month: number; amount: number };

  interface Props {
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

  const {
    currencyCode: currencyCodeProp,
    data: dataProp,
    compact = false,
    hasBudget
  }: Props = $props();

  // --- Reactive Derived Logic ---
  const currencyCode = $derived(currencyCodeProp ?? 'EUR');
  const data = $derived(dataProp ?? {});
  const noBudget = $derived(hasBudget === false);
  const budget = $derived<number>(data.budget ?? 0);
  const monthlySpending = $derived<MonthlySpendingPoint[]>(data.monthlySpending ?? []);
  const hasSpendingData = $derived(monthlySpending.some((d) => d.amount > 0));
  const currentYear = new Date().getFullYear();

  // --- Formatters ---
  const monthFormatter = $derived(
    new Intl.DateTimeFormat(regionalManager.locale, { month: 'short' })
  );
  const currencyFormatter = $derived(
    new Intl.NumberFormat(regionalManager.locale, {
      style: 'currency',
      currency: currencyCode,
      maximumFractionDigits: 0
    })
  );

  const formatMonthIndex = $derived((monthIndex: number) =>
    monthFormatter.format(new Date(2024, Math.max(0, Math.min(11, monthIndex)), 1))
  );

  const formatCurrency = (value: number) => currencyFormatter.format(value ?? 0);

  const budgetGradientId = 'budget-gradient';
  const budgetPercent = $derived(Math.round(budget * 100));
  const monthlyYMax = $derived(Math.max(...monthlySpending.map((d) => d.amount), 1));

  let isNarrowViewport = $state(false);

  onMount(() => {
    const mediaQuery = window.matchMedia('(max-width: 1023px)');

    const updateViewport = () => {
      isNarrowViewport = mediaQuery.matches;
    };

    updateViewport();
    mediaQuery.addEventListener('change', updateViewport);

    return () => {
      mediaQuery.removeEventListener('change', updateViewport);
    };
  });

  const useLightweightCharts = $derived(compact || isNarrowViewport);

  // Compact mode reduces chart height by ~30%
  const chartHeight = $derived(compact ? 'h-44' : 'h-64');
  const chartCardClass =
    'card gauge-frame p-4 transition-colors duration-200 backdrop-blur-sm bg-card/80 border border-border';

  type LayerchartModule = typeof import('layerchart');

  let PieChart = $state<LayerchartModule['PieChart'] | null>(null);
  let BarChart = $state<LayerchartModule['BarChart'] | null>(null);
  let LinearGradient = $state<LayerchartModule['LinearGradient'] | null>(null);
  let layerchartLoading = $state(false);

  $effect(() => {
    if (useLightweightCharts || PieChart || layerchartLoading) {
      return;
    }

    layerchartLoading = true;
    void import('layerchart')
      .then((mod) => {
        PieChart = mod.PieChart;
        BarChart = mod.BarChart;
        LinearGradient = mod.LinearGradient;
      })
      .catch((error) => {
        console.warn(`Failed to load layerchart: ${String(error)}`);
      })
      .finally(() => {
        layerchartLoading = false;
      });
  });
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
        {#if useLightweightCharts}
          {#if noBudget}
            <div class="flex h-full flex-col items-center justify-center gap-3 text-center">
              <div class="rounded-full bg-muted/60 p-3">
                <Wallet size={28} class="text-muted-foreground" />
              </div>
              <p class="text-sm text-muted-foreground">{m.dashboard_chart_budget_no_budget()}</p>
            </div>
          {:else}
            <div class="flex h-full flex-col items-center justify-center gap-4 text-center">
              <div
                class="relative grid h-28 w-28 place-items-center rounded-full border border-border bg-background"
                style:background={`conic-gradient(#d48a3e ${budgetPercent}%, rgba(63,63,70,0.25) 0)`}
              >
                <div class="grid h-20 w-20 place-items-center rounded-full bg-background">
                  <span class="text-2xl font-extrabold text-foreground">{budgetPercent}%</span>
                </div>
              </div>
              <span class="text-xs tracking-tighter uppercase opacity-60">
                {m.dashboard_chart_budget_remaining()}
              </span>
            </div>
          {/if}
        {:else if noBudget}
          <div class="flex h-full flex-col items-center justify-center gap-3 text-center">
            <div class="rounded-full bg-muted/60 p-3">
              <Wallet size={28} class="text-muted-foreground" />
            </div>
            <p class="text-sm text-muted-foreground">{m.dashboard_chart_budget_no_budget()}</p>
          </div>
        {:else if !PieChart}
          <div class="flex h-full flex-col items-center justify-center gap-3 text-center">
            <div class="rounded-full bg-muted/60 p-3">
              <Wallet size={28} class="text-muted-foreground" />
            </div>
            <p class="text-sm text-muted-foreground">{m.dashboard_chart_budget_title()}</p>
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
            <span class="text-3xl font-extrabold text-foreground">{budgetPercent}%</span>
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
        {#if useLightweightCharts}
          {#if !hasSpendingData}
            <div
              class="absolute inset-0 z-10 flex flex-col items-center justify-center gap-2 text-center"
            >
              <BarChart2 size={28} class="text-muted-foreground/50" />
              <p class="text-sm text-muted-foreground">
                {m.dashboard_chart_spending_no_data({ year: currentYear })}
              </p>
            </div>
            <div class="flex h-full w-full items-end gap-1 px-4 opacity-20">
              {#each Array(12) as _, i (i)}
                <div class="flex-1 rounded-t bg-muted" style:height={`${18 + (i % 4) * 8}%`}></div>
              {/each}
            </div>
          {:else}
            <div class="flex h-full w-full items-end gap-1 px-2">
              {#each monthlySpending as point (point.month)}
                <div class="flex flex-1 flex-col items-center justify-end gap-2">
                  <div
                    class="w-full rounded-t bg-primary/80"
                    style:height={`${Math.max(10, Math.round((point.amount / monthlyYMax) * 100))}%`}
                    title={formatCurrency(point.amount)}
                  ></div>
                  <span class="text-[0.65rem] text-muted-foreground"
                    >{formatMonthIndex(point.month)}</span
                  >
                </div>
              {/each}
            </div>
          {/if}
        {:else if !hasSpendingData}
          <div
            class="absolute inset-0 z-10 flex flex-col items-center justify-center gap-2 text-center"
          >
            <BarChart2 size={28} class="text-muted-foreground/50" />
            <p class="text-sm text-muted-foreground">
              {m.dashboard_chart_spending_no_data({ year: currentYear })}
            </p>
          </div>
          <!-- Ghost bars to preserve chart shape -->
          <div class="flex h-full w-full items-end gap-1 px-8 opacity-10">
            {#each Array(12) as _, i (i)}
              <div class="flex-1 rounded-t bg-muted" style="height: {20 + (i % 4) * 10}%"></div>
            {/each}
          </div>
        {:else if !BarChart}
          <div class="flex h-full w-full items-end gap-1 px-2 opacity-40">
            {#each monthlySpending as point (point.month)}
              <div class="flex flex-1 flex-col items-center justify-end gap-2">
                <div
                  class="w-full rounded-t bg-primary/50"
                  style:height={`${Math.max(10, Math.round((point.amount / monthlyYMax) * 100))}%`}
                ></div>
                <span class="text-[0.65rem] text-muted-foreground"
                  >{formatMonthIndex(point.month)}</span
                >
              </div>
            {/each}
          </div>
        {:else}
          <BarChart
            data={monthlySpending}
            x={(d: MonthlySpendingPoint) => d.month}
            y={(d: MonthlySpendingPoint) => d.amount}
            yDomain={[0, monthlyYMax * 1.1]}
            padding={{ top: 10, right: 10, bottom: 30, left: 55 }}
            props={{
              svg: { class: 'w-full h-full overflow-visible' },
              bars: { fill: '#d48a3e', radius: 4, strokeWidth: 0 },
              grid: {
                y: { style: 'stroke: rgba(31, 31, 31, 0.9); stroke-dasharray: 4 4;' }
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
