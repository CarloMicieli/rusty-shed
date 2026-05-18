import { fireEvent, render, screen } from '@testing-library/svelte';
import { describe, expect, it, vi } from 'vitest';
import MergeEntityDialog from '$lib/features/settings/components/library/MergeEntityDialog.svelte';

describe('MergeEntities', () => {
  it('requires selecting a target before merge confirm is enabled', () => {
    render(MergeEntityDialog, {
      props: {
        open: true,
        sourceName: 'Source Seller',
        sourceId: 'source',
        options: [
          { id: 'source', name: 'Source Seller', countryCode: null, usageCount: 0, isSystemSeeded: false },
          { id: 'target', name: 'Target Seller', countryCode: null, usageCount: 0, isSystemSeeded: false }
        ],
        targetId: null,
        onTargetChange: vi.fn(),
        onOpenChange: vi.fn(),
        onConfirm: vi.fn()
      }
    });

    expect(screen.getByText('Choose a target entity to continue.')).toBeInTheDocument();
    expect(screen.getByRole('button', { name: 'Merge' })).toBeDisabled();
  });

  it('does not render source as merge target option', () => {
    render(MergeEntityDialog, {
      props: {
        open: true,
        sourceName: 'Source Seller',
        sourceId: 'source',
        options: [
          { id: 'source', name: 'Source Seller', countryCode: null, usageCount: 0, isSystemSeeded: false },
          { id: 'target', name: 'Target Seller', countryCode: null, usageCount: 0, isSystemSeeded: false }
        ],
        targetId: null,
        onTargetChange: vi.fn(),
        onOpenChange: vi.fn(),
        onConfirm: vi.fn()
      }
    });

    const optionLabels = Array.from(screen.getAllByRole('option')).map((el) => el.textContent);
    expect(optionLabels).toContain('Target Seller');
    expect(optionLabels).not.toContain('Source Seller');
  });

  it('calls onTargetChange when target is selected', async () => {
    const onTargetChange = vi.fn();

    render(MergeEntityDialog, {
      props: {
        open: true,
        sourceName: 'Source Seller',
        sourceId: 'source',
        options: [
          { id: 'target', name: 'Target Seller', countryCode: null, usageCount: 0, isSystemSeeded: false }
        ],
        targetId: null,
        onTargetChange,
        onOpenChange: vi.fn(),
        onConfirm: vi.fn()
      }
    });

    await fireEvent.change(screen.getByLabelText('Target entity'), {
      target: { value: 'target' }
    });

    expect(onTargetChange).toHaveBeenCalledWith('target');
  });
});
