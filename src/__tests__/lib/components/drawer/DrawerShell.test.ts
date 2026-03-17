import { describe, it, expect, vi, beforeEach } from 'vitest';
import { render, screen, waitFor, cleanup, fireEvent } from '@testing-library/svelte';
import userEvent from '@testing-library/user-event';

vi.mock('$lib/paraglide/messages.js', () => ({
  drawer_discard_title: () => 'Discard changes?',
  drawer_discard_description: () => 'You have unsaved changes. Discard them?',
  drawer_discard_confirm: () => 'Discard',
  drawer_discard_cancel: () => 'Keep editing'
}));

// Import wrapper AFTER mocks
import DrawerShellWrapper from './DrawerShellWrapper.svelte';

describe('DrawerShell', () => {
  beforeEach(() => {
    cleanup();
    vi.clearAllMocks();
  });

  // ── Visibility ──────────────────────────────────────────────────────────────

  it('renders nothing when open=false', () => {
    const onClose = vi.fn();
    render(DrawerShellWrapper, { props: { open: false, onClose } });

    expect(screen.queryByText('Test Drawer')).toBeNull();
    expect(screen.queryByText('Drawer body content')).toBeNull();
  });

  it('renders overlay, panel, header and body when open=true', async () => {
    const onClose = vi.fn();
    render(DrawerShellWrapper, { props: { open: true, onClose } });

    await waitFor(() => {
      expect(screen.getByText('Test Drawer')).toBeInTheDocument();
      expect(screen.getByText('Drawer body content')).toBeInTheDocument();
      expect(screen.getByText('Footer')).toBeInTheDocument();
    });
  });

  // ── Size classes ────────────────────────────────────────────────────────────

  it.each([
    ['md', 'max-w-lg'],
    ['lg', 'max-w-2xl'],
    ['xl', 'max-w-3xl']
  ] as const)('applies correct size class for size="%s"', async (size, expectedClass) => {
    const onClose = vi.fn();
    const { container } = render(DrawerShellWrapper, { props: { open: true, onClose, size } });

    await waitFor(() => {
      expect(screen.getByRole('dialog')).toBeInTheDocument();
    });

    const panel = container.querySelector('[role="dialog"]');
    expect(panel?.className).toContain(expectedClass);
  });

  // ── Overlay click (no changes) ───────────────────────────────────────────────

  it('calls onClose when overlay clicked and no changes', async () => {
    const onClose = vi.fn();
    const { container } = render(DrawerShellWrapper, {
      props: { open: true, onClose, hasChanges: false }
    });

    await waitFor(() => expect(screen.getByRole('dialog')).toBeInTheDocument());

    const overlay = container.querySelector('[role="presentation"]') as HTMLElement;
    fireEvent.click(overlay);

    expect(onClose).toHaveBeenCalledOnce();
  });

  // ── Overlay click (with changes) ─────────────────────────────────────────────

  it('shows discard dialog when overlay clicked with hasChanges=true', async () => {
    const onClose = vi.fn();
    const { container } = render(DrawerShellWrapper, {
      props: { open: true, onClose, hasChanges: true }
    });

    await waitFor(() => expect(screen.getByRole('dialog')).toBeInTheDocument());

    const overlay = container.querySelector('[role="presentation"]') as HTMLElement;
    fireEvent.click(overlay);

    await waitFor(() => {
      expect(screen.getByText('Discard changes?')).toBeInTheDocument();
    });
    expect(onClose).not.toHaveBeenCalled();
  });

  // ── Discard dialog actions ───────────────────────────────────────────────────

  it('calls onClose when discard confirm clicked', async () => {
    const user = userEvent.setup();
    const onClose = vi.fn();
    const { container } = render(DrawerShellWrapper, {
      props: { open: true, onClose, hasChanges: true }
    });

    await waitFor(() => expect(screen.getByRole('dialog')).toBeInTheDocument());

    const overlay = container.querySelector('[role="presentation"]') as HTMLElement;
    fireEvent.click(overlay);

    await waitFor(() => expect(screen.getByText('Discard changes?')).toBeInTheDocument());

    await user.click(screen.getByRole('button', { name: 'Discard' }));
    expect(onClose).toHaveBeenCalledOnce();
  });

  it('dismisses discard dialog when cancel clicked without calling onClose', async () => {
    const user = userEvent.setup();
    const onClose = vi.fn();
    const { container } = render(DrawerShellWrapper, {
      props: { open: true, onClose, hasChanges: true }
    });

    await waitFor(() => expect(screen.getByRole('dialog')).toBeInTheDocument());

    const overlay = container.querySelector('[role="presentation"]') as HTMLElement;
    fireEvent.click(overlay);

    await waitFor(() => expect(screen.getByText('Discard changes?')).toBeInTheDocument());

    await user.click(screen.getByRole('button', { name: 'Keep editing' }));

    await waitFor(() => {
      expect(screen.queryByText('Discard changes?')).toBeNull();
    });
    expect(onClose).not.toHaveBeenCalled();
  });

  // ── Custom discard strings ───────────────────────────────────────────────────

  it('uses custom discardTitle and discardConfirm props', async () => {
    const onClose = vi.fn();
    const { container } = render(DrawerShellWrapper, {
      props: {
        open: true,
        onClose,
        hasChanges: true,
        discardTitle: 'Discard wishlist item?',
        discardConfirm: 'Yes, discard'
      }
    });

    await waitFor(() => expect(screen.getByRole('dialog')).toBeInTheDocument());

    const overlay = container.querySelector('[role="presentation"]') as HTMLElement;
    fireEvent.click(overlay);

    await waitFor(() => {
      expect(screen.getByText('Discard wishlist item?')).toBeInTheDocument();
      expect(screen.getByRole('button', { name: 'Yes, discard' })).toBeInTheDocument();
    });
  });

  // ── Escape key ───────────────────────────────────────────────────────────────

  it('calls onClose on Escape when no changes', async () => {
    const onClose = vi.fn();
    render(DrawerShellWrapper, { props: { open: true, onClose, hasChanges: false } });

    await waitFor(() => expect(screen.getByRole('dialog')).toBeInTheDocument());

    fireEvent.keyDown(window, { key: 'Escape' });
    expect(onClose).toHaveBeenCalledOnce();
  });

  it('shows discard dialog on Escape when hasChanges=true', async () => {
    const onClose = vi.fn();
    render(DrawerShellWrapper, { props: { open: true, onClose, hasChanges: true } });

    await waitFor(() => expect(screen.getByRole('dialog')).toBeInTheDocument());

    fireEvent.keyDown(window, { key: 'Escape' });

    await waitFor(() => {
      expect(screen.getByText('Discard changes?')).toBeInTheDocument();
    });
    expect(onClose).not.toHaveBeenCalled();
  });

  // ── requestClose via header snippet ─────────────────────────────────────────

  it('header requestClose respects hasChanges guard', async () => {
    const user = userEvent.setup();
    const onClose = vi.fn();
    render(DrawerShellWrapper, { props: { open: true, onClose, hasChanges: true } });

    await waitFor(() => expect(screen.getByLabelText('close drawer')).toBeInTheDocument());

    await user.click(screen.getByLabelText('close drawer'));

    await waitFor(() => {
      expect(screen.getByText('Discard changes?')).toBeInTheDocument();
    });
    expect(onClose).not.toHaveBeenCalled();
  });

  it('header requestClose calls onClose directly when no changes', async () => {
    const user = userEvent.setup();
    const onClose = vi.fn();
    render(DrawerShellWrapper, { props: { open: true, onClose, hasChanges: false } });

    await waitFor(() => expect(screen.getByLabelText('close drawer')).toBeInTheDocument());

    await user.click(screen.getByLabelText('close drawer'));

    expect(onClose).toHaveBeenCalledOnce();
  });

  // ── Error prop ───────────────────────────────────────────────────────────────

  it('renders error message when error prop is set', async () => {
    const onClose = vi.fn();
    render(DrawerShellWrapper, {
      props: { open: true, onClose, error: 'Something went wrong' }
    });

    await waitFor(() => {
      expect(screen.getByText('Something went wrong')).toBeInTheDocument();
    });
  });

  it('does not render error area when error is null', async () => {
    const onClose = vi.fn();
    render(DrawerShellWrapper, { props: { open: true, onClose, error: null } });

    await waitFor(() => expect(screen.getByRole('dialog')).toBeInTheDocument());
    expect(screen.queryByText('Something went wrong')).toBeNull();
  });

  // ── Scroll lock ──────────────────────────────────────────────────────────────

  it('sets body overflow hidden when open', async () => {
    const onClose = vi.fn();
    render(DrawerShellWrapper, { props: { open: true, onClose } });

    await waitFor(() => expect(screen.getByRole('dialog')).toBeInTheDocument());
    expect(document.body.style.overflow).toBe('hidden');
  });
});
