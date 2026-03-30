import { cleanup, render, screen } from '@testing-library/svelte';
import { beforeEach, describe, expect, it, vi } from 'vitest';
import TractionWarningHarness from './TractionWarningHarness.svelte';

vi.mock('$lib/paraglide/messages.js', () => ({
  formations_no_traction_warning: () => 'No traction',
  formations_traction_warning_tooltip: () => 'Add a motorized unit'
}));

describe('TractionWarning.svelte', () => {
  beforeEach(() => {
    cleanup();
  });

  it('renders the warning when traction is missing', () => {
    render(TractionWarningHarness, { props: { hasTraction: false } });
    expect(screen.getByText('No traction')).toBeInTheDocument();
  });

  it('does not render anything when traction is present', () => {
    render(TractionWarningHarness, { props: { hasTraction: true } });
    expect(screen.queryByText('No traction')).toBeNull();
  });

  it('keeps the alert semantics for the warning container', () => {
    render(TractionWarningHarness, { props: { hasTraction: false } });
    expect(screen.getByRole('alert')).toBeInTheDocument();
  });
});
