import { describe, expect, it, vi } from 'vitest';
import { render, screen } from '@testing-library/svelte';
import { CalendarDate } from '@internationalized/date';
import CalendarCaption from '$lib/components/ui/calendar/calendar-caption.svelte';

vi.mock('$lib/components/ui/calendar/calendar-month-select.svelte', async () => {
  const module = await import('../../../../stubs/CalendarMonthSelectStub.svelte');
  return { default: module.default };
});

vi.mock('$lib/components/ui/calendar/calendar-year-select.svelte', async () => {
  const module = await import('../../../../stubs/CalendarYearSelectStub.svelte');
  return { default: module.default };
});

describe('calendar-caption', () => {
  const baseProps = {
    months: [1, 2, 3],
    years: [2024, 2025],
    monthFormat: 'long' as const,
    yearFormat: 'numeric' as const,
    locale: 'en-US',
    monthIndex: 0
  };

  it('renders month and year label when captionLayout is label', () => {
    render(CalendarCaption, {
      props: {
        ...baseProps,
        captionLayout: 'label',
        month: new CalendarDate(2025, 3, 1),
        placeholder: new CalendarDate(2025, 3, 1)
      }
    });

    expect(screen.getByText(/March/i)).toBeInTheDocument();
    expect(screen.getByText(/2025/)).toBeInTheDocument();
  });

  it('shows year text alongside month dropdown when captionLayout is dropdown-months', () => {
    render(CalendarCaption, {
      props: {
        ...baseProps,
        captionLayout: 'dropdown-months',
        month: new CalendarDate(2024, 6, 1),
        placeholder: new CalendarDate(2024, 6, 1)
      }
    });

    expect(screen.getByTestId('calendar-month-select')).toBeInTheDocument();
    expect(screen.getByText('2024')).toBeInTheDocument();
  });

  it('shows month text alongside year dropdown when captionLayout is dropdown-years', () => {
    render(CalendarCaption, {
      props: {
        ...baseProps,
        captionLayout: 'dropdown-years',
        month: new CalendarDate(2024, 6, 1),
        placeholder: new CalendarDate(2024, 6, 1)
      }
    });

    expect(screen.getByText(/June/i)).toBeInTheDocument();
    expect(screen.getByTestId('calendar-year-select')).toBeInTheDocument();
  });
});
