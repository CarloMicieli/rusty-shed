import { cleanup, fireEvent, render, screen } from '@testing-library/svelte';
import { beforeEach, describe, expect, it, vi } from 'vitest';
import FormationList from '$lib/features/train-formations/components/FormationList.svelte';
import type { TrainFormationState } from '$lib/features/train-formations/TrainFormationState.svelte';
import { makeSummary } from '../fixtures';

const mockedGoto = vi.hoisted(() => vi.fn());

vi.mock('$app/navigation', () => ({ goto: mockedGoto }));

vi.mock('$lib/paraglide/messages.js', () => ({
  formations_page_title: () => 'Train formations',
  formations_new_formation: () => 'New formation',
  formations_description: () => 'Organize your locomotives and rolling stock into consist groups.',
  formations_empty_list: () => 'No formations yet',
  formations_empty_heading: () => 'Create your first formation',
  formations_empty_sub: () =>
    'Start organizing your locomotives and rolling stock into consist groups.',
  formations_element_count: ({ n }: { n: number }) => `${n} elements`,
  formations_owned_count: ({ n }: { n: number }) => `${n} owned`
}));

function makeState(overrides: Partial<Record<string, unknown>> = {}): TrainFormationState {
  return {
    isLoading: false,
    summaries: [],
    categories: [],
    create: vi.fn().mockResolvedValue(null),
    ...overrides
  } as unknown as TrainFormationState;
}

describe('FormationList.svelte', () => {
  beforeEach(() => {
    cleanup();
    vi.clearAllMocks();
  });

  it('renders loading placeholders while the list is loading', () => {
    const { container } = render(FormationList, {
      props: { state: makeState({ isLoading: true }) }
    });

    expect(container.querySelectorAll('.animate-pulse')).toHaveLength(3);
  });

  it('renders the empty state when there are no formations', () => {
    render(FormationList, {
      props: { state: makeState() }
    });

    expect(screen.getByText('Create your first formation')).toBeInTheDocument();
  });

  it('renders the available formation cards', () => {
    render(FormationList, {
      props: {
        state: makeState({
          summaries: [
            makeSummary({ id: 'one', name: 'Formation One' }),
            makeSummary({ id: 'two', name: 'Formation Two' })
          ]
        })
      }
    });

    expect(screen.getByText('Formation One')).toBeInTheDocument();
    expect(screen.getByText('Formation Two')).toBeInTheDocument();
  });

  it('navigates to the detail page when a card is clicked', async () => {
    render(FormationList, {
      props: {
        state: makeState({
          summaries: [makeSummary({ id: 'formation-42', name: 'Clickable Formation' })]
        })
      }
    });

    await fireEvent.click(screen.getByRole('button', { name: /clickable formation/i }));
    expect(mockedGoto).toHaveBeenCalledWith('/train-formations/formation-42');
  });
});
