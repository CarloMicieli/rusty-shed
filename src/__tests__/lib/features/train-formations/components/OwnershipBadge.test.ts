import { cleanup, render, screen } from '@testing-library/svelte';
import { beforeEach, describe, expect, it } from 'vitest';
import OwnershipBadge from '$lib/features/train-formations/components/OwnershipBadge.svelte';

describe('OwnershipBadge.svelte', () => {
  beforeEach(() => {
    cleanup();
  });

  it('shows Assigned when an owned model is linked', () => {
    render(OwnershipBadge, { props: { ownedCount: 3, assignedId: 'stock-1' } });
    expect(screen.getByText('Assigned')).toBeInTheDocument();
  });

  it('shows the owned count when models are available but none are assigned', () => {
    render(OwnershipBadge, { props: { ownedCount: 2, assignedId: null } });
    expect(screen.getByText('2 owned')).toBeInTheDocument();
  });

  it('shows Planned when no models are owned', () => {
    render(OwnershipBadge, { props: { ownedCount: 0, assignedId: null } });
    expect(screen.getByText('Planned')).toBeInTheDocument();
  });

  it('prioritizes Assigned over owned count', () => {
    render(OwnershipBadge, { props: { ownedCount: 5, assignedId: 'stock-2' } });
    expect(screen.getByText('Assigned')).toBeInTheDocument();
    expect(screen.queryByText('5 owned')).toBeNull();
  });

  it('uses the outline styling for planned units', () => {
    render(OwnershipBadge, { props: { ownedCount: 0, assignedId: null } });
    expect(screen.getByText('Planned')).toHaveClass('border-border');
  });
});
