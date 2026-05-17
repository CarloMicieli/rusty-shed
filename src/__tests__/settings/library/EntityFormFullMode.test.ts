import { render, screen } from '@testing-library/svelte';
import { describe, expect, it, vi } from 'vitest';
import QuickAddEntityForm from '$lib/features/quick-add/QuickAddEntityForm.svelte';

describe('EntityFormFullMode', () => {
  it('renders notes field in FULL mode', () => {
    render(QuickAddEntityForm, {
      props: {
        target: 'manufacturer',
        mode: 'FULL',
        existingNames: [],
        onSuccess: vi.fn(),
        onCancel: vi.fn()
      }
    });

    expect(screen.getByLabelText('Notes')).toBeInTheDocument();
  });

  it('does not render notes field in QUICK mode', () => {
    render(QuickAddEntityForm, {
      props: {
        target: 'manufacturer',
        mode: 'QUICK',
        existingNames: [],
        onSuccess: vi.fn(),
        onCancel: vi.fn()
      }
    });

    expect(screen.queryByLabelText('Notes')).not.toBeInTheDocument();
  });
});
