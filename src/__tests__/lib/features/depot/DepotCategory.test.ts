import { describe, it, expect, vi, beforeEach } from 'vitest';
import { render, screen } from '@testing-library/svelte';
import DepotCategoryWrapper from './DepotCategoryWrapper.svelte';
import { TrainFront } from 'lucide-svelte';

// ── Mock @tauri-apps/plugin-fs ───────────────────────────────────────────────
vi.mock('@tauri-apps/plugin-fs', () => ({
  readFile: vi.fn().mockRejectedValue(new Error('Not found')),
  BaseDirectory: { AppLocalData: 'AppLocalData' }
}));

const defaultProps = {
  value: 'locomotives',
  title: 'Locomotives',
  icon: TrainFront,
  items: [{ id: '1' }, { id: '2' }, { id: '3' }],
  categoryId: 'LOCO-001'
};

describe('DepotCategory.svelte', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('renders category title when items array is non-empty', () => {
    render(DepotCategoryWrapper, { props: defaultProps });
    expect(screen.getByText('Locomotives')).toBeInTheDocument();
  });

  it('renders categoryId text', () => {
    render(DepotCategoryWrapper, { props: defaultProps });
    expect(screen.getByText('LOCO-001')).toBeInTheDocument();
  });

  it('renders item count badge with correct unit count', () => {
    render(DepotCategoryWrapper, { props: defaultProps });
    expect(screen.getByText('3 UNITS')).toBeInTheDocument();
  });

  it('does not render anything when items array is empty', () => {
    render(DepotCategoryWrapper, {
      props: { ...defaultProps, items: [] }
    });
    // With empty items, the {#if items.length > 0} block should hide everything
    expect(screen.queryByText('Locomotives')).toBeNull();
  });

  it('renders singular item count correctly', () => {
    render(DepotCategoryWrapper, {
      props: { ...defaultProps, items: [{ id: '1' }] }
    });
    expect(screen.getByText('1 UNITS')).toBeInTheDocument();
  });

  it('renders with different category data', () => {
    render(DepotCategoryWrapper, {
      props: {
        ...defaultProps,
        title: 'Passenger Cars',
        categoryId: 'PASS-002',
        items: Array.from({ length: 10 }, (_, i) => ({ id: String(i) }))
      }
    });
    expect(screen.getByText('Passenger Cars')).toBeInTheDocument();
    expect(screen.getByText('PASS-002')).toBeInTheDocument();
    expect(screen.getByText('10 UNITS')).toBeInTheDocument();
  });
});
