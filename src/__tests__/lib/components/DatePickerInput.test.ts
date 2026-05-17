import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import { render, screen, cleanup } from '@testing-library/svelte';
import { CalendarDate } from '@internationalized/date';

// Mock the regionalManager to control date formatting
vi.mock('$lib/features/settings/RegionalManager.svelte', () => ({
  regionalManager: {
    formatDate: (iso: string) => {
      const [y, mo, d] = iso.split('-').map(Number);
      const date = new Date(y, mo - 1, d);
      return date.toLocaleDateString('en-US', { year: 'numeric', month: 'short', day: 'numeric' });
    },
    getCurrencySymbol: (currency: string) => {
      const map: Record<string, string> = { EUR: '€', USD: '$', GBP: '£' };
      return map[currency] || currency;
    }
  }
}));

vi.mock('lucide-svelte', () => ({
  Calendar: () => null
}));

import DatePickerInput from '$lib/components/DatePickerInput.svelte';

describe('DatePickerInput', () => {
  beforeEach(() => {
    cleanup();
    vi.clearAllMocks();
  });

  afterEach(() => {
    cleanup();
  });

  // ── Basic Rendering ──────────────────────────────────────────────────

  it('renders without crashing', () => {
    const { container } = render(DatePickerInput, {
      props: {}
    });
    expect(container).toBeInTheDocument();
  });

  it('renders with default placeholder prop value', () => {
    render(DatePickerInput, {
      props: {}
    });

    expect(screen.getByText('Select date')).toBeInTheDocument();
  });

  it('renders with custom placeholder', () => {
    render(DatePickerInput, {
      props: {
        placeholder: 'Pick a date'
      }
    });

    expect(screen.getByText('Pick a date')).toBeInTheDocument();
  });

  it('renders a button element for the trigger', () => {
    render(DatePickerInput, {
      props: {}
    });

    const button = screen.getByRole('button');
    expect(button).toBeInTheDocument();
  });

  it('applies disabled attribute when disabled prop is true', () => {
    render(DatePickerInput, {
      props: {
        disabled: true
      }
    });

    const button = screen.getByRole('button');
    expect(button).toBeDisabled();
  });

  it('does not apply disabled when disabled prop is false', () => {
    render(DatePickerInput, {
      props: {
        disabled: false
      }
    });

    const button = screen.getByRole('button');
    expect(button).not.toBeDisabled();
  });

  it('applies custom id to the button', () => {
    render(DatePickerInput, {
      props: {
        id: 'purchase-date-picker'
      }
    });

    const button = screen.getByRole('button');
    expect(button).toHaveAttribute('id', 'purchase-date-picker');
  });

  // ── Value Display & Formatting ──────────────────────────────────────

  it('displays formatted date when value prop is set', () => {
    render(DatePickerInput, {
      props: {
        value: '2026-05-17'
      }
    });

    expect(screen.getByText('May 17, 2026')).toBeInTheDocument();
  });

  it('displays placeholder when value prop is null', () => {
    render(DatePickerInput, {
      props: {
        value: null,
        placeholder: 'Choose a date'
      }
    });

    expect(screen.getByText('Choose a date')).toBeInTheDocument();
  });

  it('displays placeholder when value prop is empty string', () => {
    render(DatePickerInput, {
      props: {
        value: '',
        placeholder: 'Select a date'
      }
    });

    expect(screen.getByText('Select a date')).toBeInTheDocument();
  });

  it('formats dates in proper locale format', () => {
    render(DatePickerInput, {
      props: {
        value: '2024-02-29'
      }
    });

    expect(screen.getByText('Feb 29, 2024')).toBeInTheDocument();
  });

  it('handles single-digit months and days', () => {
    render(DatePickerInput, {
      props: {
        value: '2026-01-05'
      }
    });

    expect(screen.getByText('Jan 5, 2026')).toBeInTheDocument();
  });

  // ── Styling & Layout ─────────────────────────────────────────────────

  it('applies proper height and border classes', () => {
    const { container } = render(DatePickerInput, {
      props: {}
    });

    const button = container.querySelector('button');
    expect(button?.className).toContain('h-9');
    expect(button?.className).toContain('rounded-sm');
    expect(button?.className).toContain('border');
    expect(button?.className).toContain('border-border');
  });

  it('applies flex layout classes for content alignment', () => {
    const { container } = render(DatePickerInput, {
      props: {}
    });

    const button = container.querySelector('button');
    expect(button?.className).toContain('flex');
    expect(button?.className).toContain('items-center');
    expect(button?.className).toContain('justify-between');
  });

  it('applies transition classes', () => {
    const { container } = render(DatePickerInput, {
      props: {}
    });

    const button = container.querySelector('button');
    expect(button?.className).toContain('transition-all');
    expect(button?.className).toContain('duration-150');
    expect(button?.className).toContain('ease-out');
  });

  it('applies disabled styling classes when disabled', () => {
    const { container } = render(DatePickerInput, {
      props: {
        disabled: true
      }
    });

    const button = container.querySelector('button');
    expect(button?.className).toContain('disabled:cursor-not-allowed');
    expect(button?.className).toContain('disabled:opacity-50');
  });

  it('uses monospace font for date display', () => {
    const { container } = render(DatePickerInput, {
      props: {
        value: '2026-05-17'
      }
    });

    const dateSpan = container.querySelector('.font-mono');
    expect(dateSpan).toBeInTheDocument();
    expect(dateSpan?.textContent).toContain('May 17, 2026');
  });

  it('uses muted-foreground color for placeholder', () => {
    const { container } = render(DatePickerInput, {
      props: {
        value: null
      }
    });

    const placeholderSpan = container.querySelector('.text-muted-foreground');
    expect(placeholderSpan).toBeInTheDocument();
    expect(placeholderSpan?.textContent).toContain('Select date');
  });

  // ── Props Binding & Reactivity ──────────────────────────────────────

  it('can display date from initial prop', () => {
    render(DatePickerInput, {
      props: {
        value: '2026-03-15'
      }
    });

    expect(screen.getByText('Mar 15, 2026')).toBeInTheDocument();
  });

  it('can display different date with different prop value', () => {
    render(DatePickerInput, {
      props: {
        value: '2026-06-20'
      }
    });

    expect(screen.getByText('Jun 20, 2026')).toBeInTheDocument();
  });

  it('can display placeholder with null value', () => {
    render(DatePickerInput, {
      props: {
        value: null,
        placeholder: 'Pick a date'
      }
    });

    expect(screen.getByText('Pick a date')).toBeInTheDocument();
  });

  it('displays appropriate text based on prop state', () => {
    render(DatePickerInput, {
      props: {
        value: '2026-05-17',
        placeholder: 'Pick a date'
      }
    });

    expect(screen.getByText('May 17, 2026')).toBeInTheDocument();
    expect(screen.queryByText('Pick a date')).not.toBeInTheDocument();
  });

  // ── Date Constraints (Props) ────────────────────────────────────────

  it('accepts maxValue prop as CalendarDate', () => {
    const maxValue = new CalendarDate(2026, 12, 31);

    const { container } = render(DatePickerInput, {
      props: {
        value: '2026-06-15',
        maxValue
      }
    });

    expect(container).toBeInTheDocument();
  });

  it('accepts minValue prop as CalendarDate', () => {
    const minValue = new CalendarDate(2026, 1, 1);

    const { container } = render(DatePickerInput, {
      props: {
        value: '2026-06-15',
        minValue
      }
    });

    expect(container).toBeInTheDocument();
  });

  it('accepts both min and max constraints together', () => {
    const minValue = new CalendarDate(2026, 1, 1);
    const maxValue = new CalendarDate(2026, 12, 31);

    const { container } = render(DatePickerInput, {
      props: {
        value: '2026-06-15',
        minValue,
        maxValue
      }
    });

    expect(container).toBeInTheDocument();
  });

  // ── Popover Alignment ────────────────────────────────────────────────

  it('uses default align prop value "end"', () => {
    const { container } = render(DatePickerInput, {
      props: {}
    });

    // Component renders without error with default align
    expect(container).toBeInTheDocument();
  });

  it('accepts custom align prop', () => {
    const { container } = render(DatePickerInput, {
      props: {
        align: 'start'
      }
    });

    expect(container).toBeInTheDocument();
  });

  it('accepts "center" align prop', () => {
    const { container } = render(DatePickerInput, {
      props: {
        align: 'center'
      }
    });

    expect(container).toBeInTheDocument();
  });

  // ── Custom Class Props ──────────────────────────────────────────────

  it('applies custom class to trigger button', () => {
    const { container } = render(DatePickerInput, {
      props: {
        class: 'my-custom-class'
      }
    });

    const button = container.querySelector('button');
    expect(button?.className).toContain('my-custom-class');
  });

  it('maintains base classes when custom class is added', () => {
    const { container } = render(DatePickerInput, {
      props: {
        class: 'custom-class'
      }
    });

    const button = container.querySelector('button');
    const classString = button?.className || '';
    expect(classString).toContain('h-9');
    expect(classString).toContain('custom-class');
  });

  // ── Complex Scenarios ────────────────────────────────────────────────

  it('handles drawer workflow: set value → display formatted', () => {
    render(DatePickerInput, {
      props: {
        value: '2026-03-15',
        id: 'drawer-purchase-date'
      }
    });

    expect(screen.getByText('Mar 15, 2026')).toBeInTheDocument();
    const trigger = screen.getByRole('button');
    expect(trigger).toHaveAttribute('id', 'drawer-purchase-date');
  });

  it('maintains consistency with multiple props set', () => {
    render(DatePickerInput, {
      props: {
        value: '2026-05-17',
        disabled: false,
        placeholder: 'Pick a date',
        id: 'test-picker'
      }
    });

    expect(screen.getByText('May 17, 2026')).toBeInTheDocument();
    const button = screen.getByRole('button');
    expect(button).not.toBeDisabled();
    expect(button).toHaveAttribute('id', 'test-picker');
  });
});
