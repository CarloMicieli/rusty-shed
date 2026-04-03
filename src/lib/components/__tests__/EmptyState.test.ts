import { render, screen, cleanup } from '@testing-library/svelte';
import userEvent from '@testing-library/user-event';
import { describe, it, expect, vi, beforeEach } from 'vitest';
import { Wrench, Plus, Microchip } from 'lucide-svelte';
import EmptyState from '../EmptyState.svelte';

describe('EmptyState', () => {
  beforeEach(() => {
    cleanup();
    vi.clearAllMocks();
  });

  // ── Content rendering ───────────────────────────────────────────────────────

  it('renders the title', () => {
    render(EmptyState, {
      props: { icon: Wrench, title: 'No Items', description: 'Add your first item.' }
    });
    expect(screen.getByText('No Items')).toBeInTheDocument();
  });

  it('renders the description', () => {
    render(EmptyState, {
      props: { icon: Wrench, title: 'No Items', description: 'Add your first item.' }
    });
    expect(screen.getByText('Add your first item.')).toBeInTheDocument();
  });

  it('renders the icon container', () => {
    render(EmptyState, {
      props: { icon: Wrench, title: 'No Items', description: 'Add your first item.' }
    });
    // Icon is wrapped in a rounded div — confirm SVG is present
    expect(document.querySelector('svg')).toBeInTheDocument();
  });

  // ── CTA button visibility ───────────────────────────────────────────────────

  it('does not render a button when ctaLabel and onCta are absent', () => {
    render(EmptyState, {
      props: { icon: Wrench, title: 'No Items', description: 'Add your first item.' }
    });
    expect(screen.queryByRole('button')).not.toBeInTheDocument();
  });

  it('does not render a button when ctaLabel is provided but onCta is absent', () => {
    render(EmptyState, {
      props: {
        icon: Wrench,
        title: 'No Items',
        description: 'Add your first item.',
        ctaLabel: 'Create'
      }
    });
    expect(screen.queryByRole('button')).not.toBeInTheDocument();
  });

  it('does not render a button when onCta is provided but ctaLabel is absent', () => {
    render(EmptyState, {
      props: {
        icon: Wrench,
        title: 'No Items',
        description: 'Add your first item.',
        onCta: vi.fn()
      }
    });
    expect(screen.queryByRole('button')).not.toBeInTheDocument();
  });

  it('renders a button when both ctaLabel and onCta are provided', () => {
    render(EmptyState, {
      props: {
        icon: Wrench,
        title: 'No Items',
        description: 'Add your first item.',
        ctaLabel: 'Create Item',
        onCta: vi.fn()
      }
    });
    expect(screen.getByRole('button', { name: /create item/i })).toBeInTheDocument();
  });

  it('renders the ctaLabel text inside the button', () => {
    render(EmptyState, {
      props: {
        icon: Wrench,
        title: 'No Items',
        description: 'Add your first item.',
        ctaLabel: 'Add Now',
        onCta: vi.fn()
      }
    });
    expect(screen.getByText('Add Now')).toBeInTheDocument();
  });

  // ── CTA button interaction ──────────────────────────────────────────────────

  it('calls onCta when the button is clicked', async () => {
    const onCta = vi.fn();
    render(EmptyState, {
      props: {
        icon: Wrench,
        title: 'No Items',
        description: 'Add your first item.',
        ctaLabel: 'Create',
        onCta
      }
    });

    await userEvent.click(screen.getByRole('button', { name: /create/i }));
    expect(onCta).toHaveBeenCalledOnce();
  });

  // ── ctaIcon override ────────────────────────────────────────────────────────

  it('renders without error when ctaIcon differs from icon', () => {
    expect(() =>
      render(EmptyState, {
        props: {
          icon: Microchip,
          ctaIcon: Plus,
          title: 'No Formations',
          description: 'Create your first formation.',
          ctaLabel: 'New Formation',
          onCta: vi.fn()
        }
      })
    ).not.toThrow();
    expect(screen.getByRole('button', { name: /new formation/i })).toBeInTheDocument();
  });
});
