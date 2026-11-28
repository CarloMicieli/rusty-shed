import { cleanup, fireEvent, render, screen, waitFor } from '@testing-library/svelte';
import { beforeEach, describe, expect, it, vi } from 'vitest';
import IdentityCard from '$lib/features/train-formations/components/IdentityCard.svelte';
import type { TrainFormationState } from '$lib/features/train-formations/TrainFormationState.svelte';

const gotoMock = vi.hoisted(() => vi.fn());

vi.mock('$app/navigation', () => ({
  goto: gotoMock
}));

vi.mock('$app/paths', () => ({
  resolve: (path: string) => path
}));

vi.mock('$lib/paraglide/messages.js', async (importOriginal) => {
  const actual = (await importOriginal()) as Record<string, unknown>;
  return Object.fromEntries(
    Object.entries(actual).map(([key, value]) => [
      key,
      typeof value === 'function'
        ? (...args: unknown[]) => (args.length === 0 ? key : `${key}:${String(args[0])}`)
        : value
    ])
  );
});

vi.mock('$lib/components/ui/tooltip', () => ({
  Root: function TooltipRootStub() {},
  Trigger: function TooltipTriggerStub() {},
  Content: function TooltipContentStub() {}
}));

type TrainFormationCtx = {
  detail: {
    id: string;
    name: string;
    category: { id: string } | null;
    start_year: number | null;
    end_year: number | null;
    epoch: string | null;
    notes: string | null;
  } | null;
  categories: Array<{ id: string; name: string }>;
  hasTraction: boolean;
  update: ReturnType<typeof vi.fn>;
  delete: ReturnType<typeof vi.fn>;
};

function createCtx(overrides?: Partial<TrainFormationCtx>): TrainFormationCtx {
  return {
    detail: {
      id: 'formation-1',
      name: 'Night Express',
      category: { id: 'cat-1' },
      start_year: 1980,
      end_year: 1990,
      epoch: 'IV',
      notes: 'Mainline service'
    },
    categories: [{ id: 'cat-1', name: 'InterCity' }],
    hasTraction: true,
    update: vi.fn().mockResolvedValue(true),
    delete: vi.fn().mockResolvedValue(true),
    ...overrides
  };
}

describe('IdentityCard', () => {
  beforeEach(() => {
    cleanup();
    vi.clearAllMocks();
  });

  it('renders a skeleton while no formation is selected', () => {
    render(IdentityCard, {
      props: {
        state: createCtx({ detail: null }) as unknown as TrainFormationState
      }
    });

    expect(document.querySelector('.animate-pulse')).toBeInTheDocument();
  });

  it('renders formation metadata and traction warning state', () => {
    render(IdentityCard, {
      props: {
        state: createCtx({ hasTraction: false }) as unknown as TrainFormationState
      }
    });

    expect(screen.getByText('Night Express')).toBeInTheDocument();
    expect(screen.getByText('formations_meta_era')).toBeInTheDocument();
    expect(screen.getByText('formations_meta_years')).toBeInTheDocument();
  });

  it('edits name inline and saves trimmed value on Enter', async () => {
    const ctx = createCtx();
    render(IdentityCard, { props: { state: ctx as unknown as TrainFormationState } });

    await fireEvent.click(screen.getByRole('button', { name: /Night Express/ }));
    const input = screen.getAllByLabelText('formations_edit_formation')[0] as HTMLInputElement;
    await fireEvent.input(input, { target: { value: '  Alpine Express  ' } });
    await fireEvent.keyDown(input, { key: 'Enter' });

    await waitFor(() => {
      expect(ctx.update).toHaveBeenCalledWith(
        'formation-1',
        expect.objectContaining({ name: 'Alpine Express' })
      );
    });
  });

  it('cancels name editing on Escape or blank value', async () => {
    const ctx = createCtx();
    render(IdentityCard, { props: { state: ctx as unknown as TrainFormationState } });

    await fireEvent.click(screen.getByRole('button', { name: /Night Express/ }));
    const input = screen.getAllByLabelText('formations_edit_formation')[0] as HTMLInputElement;
    await fireEvent.input(input, { target: { value: 'Temp' } });
    await fireEvent.keyDown(input, { key: 'Escape' });
    expect(ctx.update).not.toHaveBeenCalled();

    await fireEvent.click(screen.getByRole('button', { name: /Night Express/ }));
    const secondInput = screen.getAllByLabelText(
      'formations_edit_formation'
    )[0] as HTMLInputElement;
    await fireEvent.input(secondInput, { target: { value: '   ' } });
    await fireEvent.blur(secondInput);
    expect(ctx.update).not.toHaveBeenCalled();
  });

  it('deletes the formation from confirm dialog and navigates back', async () => {
    const ctx = createCtx();
    render(IdentityCard, { props: { state: ctx as unknown as TrainFormationState } });

    await fireEvent.click(
      screen.getAllByRole('button', { name: 'formations_delete_formation' })[0]
    );
    const deleteButtons = screen.getAllByRole('button', { name: 'formations_delete_formation' });
    await fireEvent.click(deleteButtons[deleteButtons.length - 1]);

    await waitFor(() => {
      expect(ctx.delete).toHaveBeenCalledWith('formation-1');
      expect(gotoMock).toHaveBeenCalledWith('/train-formations');
    });
  });
});
