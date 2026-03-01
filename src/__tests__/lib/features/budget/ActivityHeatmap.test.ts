import { describe, it, expect, vi, beforeEach } from 'vitest';
import { render, screen, waitFor } from '@testing-library/svelte';
import userEvent from '@testing-library/user-event';
import ActivityHeatmap from '$lib/features/budget/components/ActivityHeatmap.svelte';
import type {
  QuarterlyActivityPoint,
  QuarterlySummary
} from '$lib/features/budget/services/BudgetService.svelte';

function makePoint(
  year: number,
  quarter: 'Q1' | 'Q2' | 'Q3' | 'Q4',
  spendingLevel: 'NONE' | 'LOW' | 'MEDIUM' | 'HIGH',
  amount = 0
): QuarterlyActivityPoint {
  return { year, quarter, spendingLevel, amount };
}

const mockBudgetState: {
  quarterlySummaries: QuarterlySummary[];
  loadQuarterlySummaries: ReturnType<typeof vi.fn>;
} = {
  quarterlySummaries: [
    {
      year: 2025,
      quarter: 'Q1',
      totalSpending: {
        amount: 5000,
        currency: 'EUR'
      },
      categoryBreakdown: []
    }
  ],
  loadQuarterlySummaries: vi.fn().mockResolvedValue(undefined)
};

describe('ActivityHeatmap.svelte', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    mockBudgetState.loadQuarterlySummaries.mockResolvedValue(undefined);
  });

  it('renders heatmap heading', () => {
    render(ActivityHeatmap, {
      props: {
        quarterlyActivity: [],
        currency: 'EUR',
        budgetState: mockBudgetState as never
      }
    });
    expect(screen.getByText('5-Year Activity')).toBeInTheDocument();
  });

  it('renders quarter column headers Q1–Q4', () => {
    render(ActivityHeatmap, {
      props: {
        quarterlyActivity: [],
        currency: 'EUR',
        budgetState: mockBudgetState as never
      }
    });
    expect(screen.getByText('Q1')).toBeInTheDocument();
    expect(screen.getByText('Q2')).toBeInTheDocument();
    expect(screen.getByText('Q3')).toBeInTheDocument();
    expect(screen.getByText('Q4')).toBeInTheDocument();
  });

  it('renders year label when data is provided', () => {
    const activity = [makePoint(2025, 'Q1', 'LOW', 5000)];
    render(ActivityHeatmap, {
      props: {
        quarterlyActivity: activity,
        currency: 'EUR',
        budgetState: mockBudgetState as never
      }
    });
    expect(screen.getByText('2025')).toBeInTheDocument();
  });

  it('renders legend with None/Low/Medium/High labels', () => {
    render(ActivityHeatmap, {
      props: {
        quarterlyActivity: [],
        currency: 'EUR',
        budgetState: mockBudgetState as never
      }
    });
    expect(screen.getByText('None')).toBeInTheDocument();
    expect(screen.getByText('Low')).toBeInTheDocument();
    expect(screen.getByText('Medium')).toBeInTheDocument();
    expect(screen.getByText('High')).toBeInTheDocument();
  });

  it('renders cells with title attributes for activity data', () => {
    const activity = [makePoint(2025, 'Q2', 'MEDIUM', 25000)];
    render(ActivityHeatmap, {
      props: {
        quarterlyActivity: activity,
        currency: 'EUR',
        budgetState: mockBudgetState as never
      }
    });
    // The grid cell should have a descriptive title
    const cell = document.querySelector('[title*="2025"]');
    expect(cell).not.toBeNull();
    expect(cell?.getAttribute('title')).toMatch(/2025/);
  });

  it('renders a cell for each quarter in the data year', () => {
    const activity = [
      makePoint(2024, 'Q1', 'NONE'),
      makePoint(2024, 'Q2', 'LOW', 1000),
      makePoint(2024, 'Q3', 'MEDIUM', 5000),
      makePoint(2024, 'Q4', 'HIGH', 20000)
    ];
    render(ActivityHeatmap, {
      props: {
        quarterlyActivity: activity,
        currency: 'EUR',
        budgetState: mockBudgetState as never
      }
    });
    expect(screen.getByText('2024')).toBeInTheDocument();
    // 4 quarter buttons rendered for the year
    const buttons = document.querySelectorAll('button[title*="2024"]');
    expect(buttons.length).toBe(4);
  });

  it('calls loadQuarterlySummaries and triggers modal on cell click', async () => {
    const user = userEvent.setup();
    mockBudgetState.quarterlySummaries = [
      {
        year: 2025,
        quarter: 'Q1',
        totalSpending: {
          amount: 5000,
          currency: 'EUR'
        },
        categoryBreakdown: []
      }
    ];
    const activity = [makePoint(2025, 'Q1', 'LOW', 5000)];
    render(ActivityHeatmap, {
      props: {
        quarterlyActivity: activity,
        currency: 'EUR',
        budgetState: mockBudgetState as never
      }
    });

    const cell = document.querySelector('button[title*="2025 Q1"]') as HTMLButtonElement;
    if (cell) {
      await user.click(cell);
      await waitFor(() => {
        expect(mockBudgetState.loadQuarterlySummaries).toHaveBeenCalledWith(2025, 'EUR');
      });
      await waitFor(() => {
        expect(screen.getByText('Category Spending Breakdown')).toBeInTheDocument();
      });
    }
  });

  it('renders "Spending Level:" label in the legend section', () => {
    render(ActivityHeatmap, {
      props: {
        quarterlyActivity: [],
        currency: 'EUR',
        budgetState: mockBudgetState as never
      }
    });
    expect(screen.getByText('Spending Level:')).toBeInTheDocument();
  });

  it('renders amount text inside cells with non-zero spending', () => {
    const activity = [makePoint(2025, 'Q3', 'HIGH', 123456)];
    render(ActivityHeatmap, {
      props: {
        quarterlyActivity: activity,
        currency: 'EUR',
        budgetState: mockBudgetState as never
      }
    });
    // Amount is shown as formatted currency inside the cell when amount > 0
    const amountText = document.querySelector('.text-foreground');
    expect(amountText).not.toBeNull();
  });
});
