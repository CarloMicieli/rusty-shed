<script lang="ts">
  import { BarChart, LinearGradient, PieChart, ScatterChart } from 'layerchart';
  import * as m from '$lib/paraglide/messages.js';

  type MonthlySpendingPoint = { month: number; amount: number };
  type HistoryPoint = { year: number; month: number; value: number };

  interface DashboardChartsProps {
    /** ISO 4217 currency code for formatting. @default 'EUR' */
    currencyCode?: string;
    /** Chart data overrides – falls back to mock data when omitted. */
    data?: {
      budget?: number;
      monthlySpending?: MonthlySpendingPoint[];
      history?: HistoryPoint[];
    };
  }

  let { currencyCode: currencyCodeProp, data: dataProp }: DashboardChartsProps = $props();

  // --- Mock Data ---
  const budgetMock = 0.75;
  const monthlySpendingMock: MonthlySpendingPoint[] = [
    { month: 0, amount: 1200 },
    { month: 1, amount: 980 },
    { month: 2, amount: 1050 },
    { month: 3, amount: 1250 },
    { month: 4, amount: 1325 },
    { month: 5, amount: 1210 },
    { month: 6, amount: 1400 },
    { month: 7, amount: 1360 },
    { month: 8, amount: 1280 },
    { month: 9, amount: 1175 },
    { month: 10, amount: 1230 },
    { month: 11, amount: 1315 }
  ];

  const currentYear = new Date().getFullYear();
  const historyMock: HistoryPoint[] = Array.from({ length: 60 }, (_, index) => {
    const year = currentYear - 4 + Math.floor(index / 12);
    const month = (index % 12) + 1;
    const value = 20 + ((index * 7) % 80);
    return { year, month, value };
  });

  // --- Reactive Derived Logic ---
  const currencyCode = $derived(currencyCodeProp ?? 'EUR');
  const data = $derived(dataProp ?? {});
  const budget = $derived<number>(data.budget ?? budgetMock);
  const monthlySpending = $derived<MonthlySpendingPoint[]>(
    data.monthlySpending ?? monthlySpendingMock
  );
  const historyData = $derived<HistoryPoint[]>(data.history ?? historyMock);

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
  const formatMonthNumber = (monthNumber: number) =>
    monthFormatter.format(new Date(2024, Math.max(0, Math.min(11, monthNumber - 1)), 1));
  const formatCurrency = (value: number) => currencyFormatter.format(value ?? 0);

  const budgetGradientId = 'budget-gradient';
  const budgetPercent = $derived(Math.round(budget * 100));
  const monthlyYMax = $derived(Math.max(...monthlySpending.map((d) => d.amount), 1));
  const historyValueMax = $derived(Math.max(...historyData.map((d) => d.value), 1));

  const chartCardClass =
    'card gauge-frame p-4 transition-colors duration-200 backdrop-blur-sm bg-zinc-900/50 border border-zinc-800';
</script>

<div class="grid grid-cols-1 gap-4 lg:grid-cols-3">
  <!-- Budget Gauge (PieChart) -->
  <div class={chartCardClass}>
    <div class="mb-3 space-y-1">
      <p class="text-xs font-semibold tracking-wide uppercase opacity-80">
        {m.dashboard_chart_budget_label()}
      </p>
      <p class="text-lg font-bold">{m.dashboard_chart_budget_title()}</p>
    </div>

    <div class="relative h-64 w-full">
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

      <div class="pointer-events-none absolute inset-0 flex flex-col items-center justify-center">
        <span class="text-3xl font-extrabold text-white">{budgetPercent}%</span>
        <span class="text-xs tracking-tighter uppercase opacity-60"
          >{m.dashboard_chart_budget_remaining()}</span
        >
      </div>
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

    <div class="relative h-64 w-full">
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
    </div>
  </div>

  <!-- Punchcard (ScatterChart) -->
  <div class={chartCardClass}>
    <div class="mb-3 space-y-1">
      <p class="text-xs font-semibold tracking-wide uppercase opacity-80">
        {m.dashboard_chart_punchcard_label()}
      </p>
      <p class="text-lg font-bold">{m.dashboard_chart_punchcard_title()}</p>
    </div>

    <div class="relative h-64 w-full">
      <ScatterChart
        data={historyData}
        x={(d) => d.month}
        y={(d) => d.year}
        r={(d) => d.value}
        rDomain={[0, historyValueMax]}
        rRange={[3, 12]}
        padding={{ top: 10, right: 15, bottom: 30, left: 45 }}
        props={{
          svg: { class: 'w-full h-full overflow-visible' },
          points: {
            fill: '#b87333',
            fillOpacity: 0.4,
            stroke: '#d48a3e',
            strokeWidth: 1
          },
          grid: {
            x: { style: 'stroke: rgba(82, 82, 91, 0.3); stroke-dasharray: 2 2;' },
            y: { style: 'stroke: rgba(82, 82, 91, 0.3); stroke-dasharray: 2 2;' }
          },
          xAxis: {
            format: (v: unknown) => formatMonthNumber(Number(v))
          },
          tooltip: {
            header: { format: (v: unknown) => formatMonthNumber(Number(v)) }
          }
        }}
      />
    </div>
  </div>
</div>
