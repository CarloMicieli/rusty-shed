import { cleanup, fireEvent, render, screen } from '@testing-library/svelte';
import { beforeEach, describe, expect, it, vi } from 'vitest';
import FormationCellHarness from './FormationCellHarness.svelte';
import { makeElement, makePrototype } from '../fixtures';

vi.mock('$lib/paraglide/messages.js', () => ({
  formations_stock_not_found: () => 'Stock not found',
  formations_quick_assign: () => 'Quick assign',
  formations_assign_model: () => 'Assign model',
  formations_unassign_model: () => 'Unassign model',
  formations_traction_override_disable: () => 'Disable traction override',
  formations_traction_override_enable: () => 'Enable traction override',
  formation_toggle_traction: () => 'Toggle traction override',
  formation_remove_element: () => 'Remove element'
}));

describe('FormationCell.svelte', () => {
  beforeEach(() => {
    cleanup();
    vi.clearAllMocks();
  });

  it('renders company, series code, and service level', () => {
    render(FormationCellHarness, {
      props: {
        element: makeElement({
          prototype: makePrototype({
            company_name: 'FS',
            series_code: 'UIC-X',
            service_level: '1st Class'
          })
        })
      }
    });

    expect(screen.getByText('FS')).toBeInTheDocument();
    expect(screen.getByText('UIC-X')).toBeInTheDocument();
    expect(screen.getByText('1st Class')).toBeInTheDocument();
  });

  it('shows quick assign when exactly one owned model matches', async () => {
    const onOpenPicker = vi.fn();
    render(FormationCellHarness, {
      props: {
        element: makeElement({ owned_count_for_prototype: 1 }),
        onOpenPicker
      }
    });

    await fireEvent.click(screen.getByText('Quick assign'));
    expect(onOpenPicker).toHaveBeenCalledWith('trn:element:1');
  });

  it('shows assign model when multiple owned models match', () => {
    render(FormationCellHarness, {
      props: {
        element: makeElement({ owned_count_for_prototype: 3 })
      }
    });

    expect(screen.getByText('Assign model')).toBeInTheDocument();
  });

  it('shows unassign when a specific model is already assigned', async () => {
    const onOpenPicker = vi.fn();
    render(FormationCellHarness, {
      props: {
        element: makeElement({ owned_rolling_stock_id: 'stock-1' }),
        onOpenPicker
      }
    });

    await fireEvent.click(screen.getByText('Unassign model'));
    expect(onOpenPicker).toHaveBeenCalledWith('trn:element:1');
  });

  it('renders the stock-not-found indicator when the assignment tombstone exists', () => {
    const { container } = render(FormationCellHarness, {
      props: {
        element: makeElement({
          stock_not_found: true,
          snapshot_series_code: 'Deleted Stock',
          snapshot_company_name: 'SBB'
        })
      }
    });

    expect(container.querySelector('.bg-red-500')).not.toBeNull();
  });

  it('cycles traction override from neutral to included', async () => {
    const onTractionToggle = vi.fn();
    render(FormationCellHarness, {
      props: {
        element: makeElement({ traction_override: 0 }),
        onTractionToggle
      }
    });

    await fireEvent.click(screen.getByLabelText('Toggle traction override'));
    expect(onTractionToggle).toHaveBeenCalledWith('trn:element:1', 1);
  });

  it('calls onRemove when the remove button is pressed', async () => {
    const onRemove = vi.fn();
    const { container } = render(FormationCellHarness, {
      props: {
        element: makeElement(),
        onRemove
      }
    });

    const removeButton = container.querySelector('button[aria-label="Remove element"]');
    expect(removeButton).not.toBeNull();
    await fireEvent.click(removeButton as HTMLButtonElement);
    expect(onRemove).toHaveBeenCalledWith('trn:element:1');
  });
});
