import { render, screen } from '@testing-library/svelte';
import { describe, expect, it, vi } from 'vitest';
import EntityCards from '$lib/features/settings/components/library/EntityCards.svelte';
import EntityTabs from '$lib/features/settings/components/library/EntityTabs.svelte';
import type { LibraryEntityRow } from '$lib/services/entityLibrary';

const rows: LibraryEntityRow[] = [
  { id: 'row-1', name: 'Acme', countryCode: 'DE', usageCount: 2, isSystemSeeded: false },
  { id: 'row-2', name: 'Protected Co', countryCode: null, usageCount: 0, isSystemSeeded: true }
];

const paginationProps = {
  totalItems: rows.length,
  totalPages: 1,
  pageStart: 1,
  pageEnd: rows.length,
  currentPage: 1,
  onPageChange: vi.fn()
};

describe('EntityCards – mobile card layout', () => {
  it('renders all rows as card items', () => {
    render(EntityCards, {
      props: { rows, onEdit: vi.fn(), onDelete: vi.fn(), onMerge: vi.fn() }
    });

    expect(screen.getByText('Acme')).toBeInTheDocument();
    expect(screen.getByText('Protected Co')).toBeInTheDocument();
  });

  it('includes edit, delete, and merge buttons per row (action parity)', () => {
    render(EntityCards, {
      props: { rows: [rows[0]], onEdit: vi.fn(), onDelete: vi.fn(), onMerge: vi.fn() }
    });

    expect(screen.getByRole('button', { name: /edit acme/i })).toBeInTheDocument();
    expect(screen.getByRole('button', { name: /delete acme/i })).toBeInTheDocument();
    expect(screen.getByRole('button', { name: /merge acme/i })).toBeInTheDocument();
  });

  it('disables edit and merge for protected rows', () => {
    render(EntityCards, {
      props: { rows: [rows[1]], onEdit: vi.fn(), onDelete: vi.fn(), onMerge: vi.fn() }
    });

    expect(screen.getByRole('button', { name: /edit protected co/i })).toBeDisabled();
    expect(screen.getByRole('button', { name: /merge protected co/i })).toBeDisabled();
    expect(screen.getByRole('button', { name: /delete protected co/i })).toBeDisabled();
  });

  it('shows country code when present', () => {
    render(EntityCards, {
      props: { rows: [rows[0]], onEdit: vi.fn(), onDelete: vi.fn(), onMerge: vi.fn() }
    });

    expect(screen.getByText('DE')).toBeInTheDocument();
  });
});

describe('EntityTabs – both layouts are rendered to DOM', () => {
  it('renders a desktop wrapper and a mobile wrapper when rows are present', () => {
    render(EntityTabs, {
      props: {
        activeTab: 'manufacturers',
        onTabChange: vi.fn(),
        onEdit: vi.fn(),
        onDelete: vi.fn(),
        onMerge: vi.fn(),
        rows: [rows[0]],
        ...paginationProps
      }
    });

    expect(document.querySelector('[data-layout="desktop"]')).toBeInTheDocument();
    expect(document.querySelector('[data-layout="mobile"]')).toBeInTheDocument();
    expect(screen.getByLabelText(/table pagination/i)).toBeInTheDocument();
  });

  it('shows empty state when tab has no rows', () => {
    render(EntityTabs, {
      props: {
        activeTab: 'sellers',
        onTabChange: vi.fn(),
        onEdit: vi.fn(),
        onDelete: vi.fn(),
        onMerge: vi.fn(),
        rows: [],
        totalItems: 0,
        totalPages: 0,
        pageStart: 0,
        pageEnd: 0,
        currentPage: 1,
        onPageChange: vi.fn()
      }
    });

    expect(document.querySelector('[data-layout="desktop"]')).not.toBeInTheDocument();
    expect(document.querySelector('[data-layout="mobile"]')).not.toBeInTheDocument();
  });
});
