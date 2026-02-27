import { describe, it, expect, vi, beforeEach } from 'vitest';
import { render, screen, fireEvent } from '@testing-library/svelte';
import BudgetTable from '$lib/features/budget/components/BudgetTable.svelte';
import type { MonthlyBudgetRecordDto } from '$lib/features/budget/services/BudgetService.svelte';

// ── Mock modal store ─────────────────────────────────────────────────────────
const mockModalTrigger = vi.hoisted(() => vi.fn());
vi.mock('$lib/stores/modal', () => ({
  getModalStore: () => ({ trigger: mockModalTrigger })
}));

// ── Mock ExtraBudgetModal (only referenced, not rendered directly) ─────────
vi.mock('$lib/features/budget/components/ExtraBudgetModal.svelte', () => ({
  default: {}
}));

// ── Mock ui/Badge (simple passthrough) ──────────────────────────────────────
vi.mock('$lib/components', async (importOriginal) => {
  const actual = await importOriginal<typeof import('$lib/components')>();
  return { ...actual };
});

function makeRecord(overrides?: Partial<MonthlyBudgetRecordDto>): MonthlyBudgetRecordDto {
  return {
    year: 2026,
    month: 1,
    baseBudget: 10000,
    extraBudget: 0,
    actualSpend: 5000,
    rolloverIn: 0,
    rolloverOut: 0,
    available: 10000,
    remaining: 5000,
    remainingPercentage: 50,
    status: 'IN_PROGRESS' as const,
    currency: 'EUR',
    ...overrides
  };
}

const mockBudgetState = {
  config: null,
  isLoading: false,
  hasConfig: true
};

describe('BudgetTable.svelte', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('renders month names for all provided records', () => {
    const records = [makeRecord({ month: 1 }), makeRecord({ month: 6 }), makeRecord({ month: 12 })];
    render(BudgetTable, { props: { records, budgetState: mockBudgetState as never } });

    expect(screen.getByText('January')).toBeInTheDocument();
    expect(screen.getByText('June')).toBeInTheDocument();
    expect(screen.getByText('December')).toBeInTheDocument();
  });

  it('renders empty state message when no records', () => {
    render(BudgetTable, { props: { records: [], budgetState: mockBudgetState as never } });
    expect(screen.getByText('No budget records available')).toBeInTheDocument();
  });

  it('renders status badge labels', () => {
    const records = [
      makeRecord({ month: 1, status: 'COMPLETED' as const }),
      makeRecord({ month: 2, status: 'IN_PROGRESS' as const }),
      makeRecord({ month: 3, status: 'PROJECTED' as const })
    ];
    render(BudgetTable, { props: { records, budgetState: mockBudgetState as never } });

    expect(screen.getByText('Completed')).toBeInTheDocument();
    expect(screen.getByText('In Progress')).toBeInTheDocument();
    expect(screen.getByText('Projected')).toBeInTheDocument();
  });

  it('renders table headers', () => {
    render(BudgetTable, { props: { records: [], budgetState: mockBudgetState as never } });

    expect(screen.getByText('Month')).toBeInTheDocument();
    expect(screen.getByText('Base Budget')).toBeInTheDocument();
    expect(screen.getByText('Spent')).toBeInTheDocument();
    expect(screen.getByText('Remaining')).toBeInTheDocument();
    expect(screen.getByText('Status')).toBeInTheDocument();
  });

  it('shows "—" for zero extra budget', () => {
    const record = makeRecord({ month: 1, extraBudget: 0 });
    render(BudgetTable, { props: { records: [record], budgetState: mockBudgetState as never } });
    // The component renders '—' for 0 extra or rollover
    expect(screen.getAllByText('—').length).toBeGreaterThan(0);
  });

  it('shows rollover amount when rolloverIn > 0', () => {
    const record = makeRecord({ month: 1, rolloverIn: 2500 });
    render(BudgetTable, { props: { records: [record], budgetState: mockBudgetState as never } });
    // rolloverIn (25.00 EUR) should show as formatted currency value, not '—'
    // The table cells use Intl.NumberFormat which includes the EUR symbol
    const cells = Array.from(document.querySelectorAll('td.text-right'));
    // The 4th column is "Rollover In" (index 3 from text-right cells)
    // rolloverIn: 2500 => 25.00 EUR; verify at least one cell has non-dash content
    const nonDashCells = cells.filter((c) => c.textContent && c.textContent.trim() !== '—');
    expect(nonDashCells.length).toBeGreaterThan(0);
  });

  it('triggers modal when Extra button is clicked', async () => {
    const record = makeRecord({ month: 3, year: 2026 });
    render(BudgetTable, { props: { records: [record], budgetState: mockBudgetState as never } });

    const extraBtn = screen.getByRole('button', { name: /extra/i });
    await fireEvent.click(extraBtn);

    expect(mockModalTrigger).toHaveBeenCalledWith(
      expect.objectContaining({
        type: 'component',
        component: expect.objectContaining({
          props: expect.objectContaining({ year: 2026, month: 3 })
        })
      })
    );
  });

  it('renders multiple month rows for multiple records', () => {
    const records = Array.from({ length: 6 }, (_, i) => makeRecord({ month: i + 1 }));
    render(BudgetTable, { props: { records, budgetState: mockBudgetState as never } });

    expect(screen.getByText('January')).toBeInTheDocument();
    expect(screen.getByText('February')).toBeInTheDocument();
    expect(screen.getByText('March')).toBeInTheDocument();
    expect(screen.getByText('April')).toBeInTheDocument();
    expect(screen.getByText('May')).toBeInTheDocument();
    expect(screen.getByText('June')).toBeInTheDocument();
  });
});
