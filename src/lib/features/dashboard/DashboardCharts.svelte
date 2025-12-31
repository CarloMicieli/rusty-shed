<script lang="ts">
  import { BarChart, LinearGradient, PieChart, ScatterChart } from 'layerchart';
  import * as m from '$lib/paraglide/messages.js';

  type MonthlySpendingPoint = { month: number; amount: number };
  type HistoryPoint = { year: number; month: number; value: number };

  const props = $props<{
    currencyCode?: string;
    data?: {
      budget?: number;
      monthlySpending?: MonthlySpendingPoint[];
      history?: HistoryPoint[];
    };
  }>();

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

  const currencyCode = $derived(props.currencyCode ?? 'EUR');
  const data = $derived(props.data ?? {});

  const budget = $derived<number>(data.budget ?? budgetMock);
  const monthlySpending = $derived<MonthlySpendingPoint[]>(
    data.monthlySpending ?? monthlySpendingMock
  );
  const historyData = $derived<HistoryPoint[]>(data.history ?? historyMock);

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

  const monthlyYMax = $derived(
    Math.max(...monthlySpending.map((d: MonthlySpendingPoint) => d.amount), 1)
  );
  const historyValueMax = $derived(Math.max(...historyData.map((d: HistoryPoint) => d.value), 1));
  const chartCardClass =
    'variant-filled-surface card border border-surface-700/60 p-4 transition-colors duration-200 backdrop-blur-sm';
</script>

<div class="grid grid-cols-1 gap-4 lg:grid-cols-3">
  <div class={chartCardClass}>
    <div class="mb-3 space-y-1">
      <p class="text-xs font-semibold tracking-wide uppercase opacity-80">
        {m.dashboard_chart_budget_label()}
      </p>
      <p class="text-lg font-bold">{m.dashboard_chart_budget_title()}</p>
    </div>

    <div class="relative h-64">
      <PieChart
        data={[{ key: 'available', label: m.dashboard_chart_budget_title(), value: budget }]}
        series={[
          {
            key: 'budget',
            data: [{ key: 'available', label: m.dashboard_chart_budget_title(), value: budget }],
            maxValue: 1,
            color: `url(#${budgetGradientId})`
          }
        ]}
        key="key"
        label="label"
        value="value"
        maxValue={1}
        innerRadius={0.62}
        padAngle={0.015}
        props={{
          arc: { stroke: 'transparent', track: { fill: '#e5e7eb' } },
          svg: { class: 'w-full h-full', style: 'background: transparent' },
          tooltip: {
            header: {
              format: (value: number) => `${Math.round(Number(value ?? 0) * 100)}%`
            },
            item: {
              format: (value: number) => `${Math.round(Number(value ?? 0) * 100)}%`,
              valueAlign: 'right'
            }
          }
        }}
      >
        <LinearGradient
          slot="belowMarks"
          id={budgetGradientId}
          stops={[
            ['0%', '#22c55e'],
            ['100%', '#ef4444']
          ]}
        />
      </PieChart>

      <div class="pointer-events-none absolute inset-0 flex flex-col items-center justify-center">
        <span class="text-3xl font-extrabold">{budgetPercent}%</span>
        <span class="text-xs opacity-80">{m.dashboard_chart_budget_remaining()}</span>
      </div>
    </div>
  </div>

  <div class={chartCardClass}>
    <div class="mb-3 space-y-1">
      <p class="text-xs font-semibold tracking-wide uppercase opacity-80">
        {m.dashboard_chart_spending_label()}
      </p>
      <p class="text-lg font-bold">{m.dashboard_chart_spending_title()}</p>
    </div>

    <div class="h-64">
      <BarChart
        data={monthlySpending}
        x={(d: MonthlySpendingPoint) => d.month}
        y={(d: MonthlySpendingPoint) => d.amount}
        yDomain={[0, monthlyYMax * 1.1]}
        bandPadding={0.25}
        props={{
          svg: { class: 'h-full w-full', style: 'background: transparent' },
          bars: { fill: '#3b82f6' },
          grid: { y: true, x: false },
          xAxis: { format: (value: number) => formatMonthIndex(Number(value)) },
          yAxis: { format: (value: number) => formatCurrency(Number(value)) },
          tooltip: {
            header: {
              format: (value: number) => formatMonthIndex(Number(value))
            },
            item: {
              format: (value: number) => formatCurrency(Number(value)),
              valueAlign: 'right'
            }
          }
        }}
      />
    </div>
  </div>

  <div class={chartCardClass}>
    <div class="mb-3 space-y-1">
      <p class="text-xs font-semibold tracking-wide uppercase opacity-80">
        {m.dashboard_chart_punchcard_label()}
      </p>
      <p class="text-lg font-bold">{m.dashboard_chart_punchcard_title()}</p>
    </div>

    <div class="h-64">
      <ScatterChart
        data={historyData}
        x={(d: HistoryPoint) => d.month}
        y={(d: HistoryPoint) => d.year}
        r={(d: HistoryPoint) => d.value}
        rDomain={[0, historyValueMax]}
        rRange={[4, 16]}
        props={{
          svg: { class: 'h-full w-full', style: 'background: transparent' },
          grid: { x: true, y: true },
          points: {
            fill: '#10b981',
            fillOpacity: 0.32,
            stroke: '#10b981'
          },
          xAxis: { format: (value: number) => formatMonthNumber(Number(value)) },
          yAxis: { format: (value: number) => `${value}` },
          tooltip: {
            header: {
              format: (value: number) => `${formatMonthNumber(Number(value))}`
            },
            item: {
              format: (value: number) => formatCurrency(Number(value)),
              valueAlign: 'right'
            }
          }
        }}
      />
    </div>
  </div>
</div>
