import { cleanup, render } from '@testing-library/svelte';
import { beforeEach, describe, expect, it } from 'vitest';
import PrototypeIcon from '$lib/features/train-formations/components/icons/PrototypeIcon.svelte';

function renderIcon(type: string, isOwned = false, className = '') {
  return render(PrototypeIcon, {
    props: {
      type,
      isOwned,
      class: className
    }
  });
}

describe('PrototypeIcon.svelte', () => {
  beforeEach(() => {
    cleanup();
  });

  it('renders the locomotive icon for Locomotive', () => {
    const { container } = renderIcon('Locomotive');
    expect(container.innerHTML).toContain('M2 17h20v-4H2v4Z');
  });

  it('renders the locomotive icon for PowerCar', () => {
    const { container } = renderIcon('PowerCar');
    expect(container.innerHTML).toContain('M2 17h20v-4H2v4Z');
  });

  it('renders the coach icon for Coach', () => {
    const { container } = renderIcon('Coach');
    expect(container.innerHTML).toContain('width="20" height="9" rx="1"');
  });

  it('renders the coach icon for Couchette', () => {
    const { container } = renderIcon('Couchette');
    expect(container.innerHTML).toContain('width="20" height="9" rx="1"');
  });

  it('renders the coach icon for Dining', () => {
    const { container } = renderIcon('Dining');
    expect(container.innerHTML).toContain('width="20" height="9" rx="1"');
  });

  it('renders the coach icon for Sleeping', () => {
    const { container } = renderIcon('Sleeping');
    expect(container.innerHTML).toContain('width="20" height="9" rx="1"');
  });

  it('renders the coach icon for ControlCar', () => {
    const { container } = renderIcon('ControlCar');
    expect(container.innerHTML).toContain('width="20" height="9" rx="1"');
  });

  it('renders the wagon icon for BaggageCar', () => {
    const { container } = renderIcon('BaggageCar');
    expect(container.innerHTML).toContain('M2 16h20');
  });

  it('renders the wagon icon for FreightWagon', () => {
    const { container } = renderIcon('FreightWagon');
    expect(container.innerHTML).toContain('M2 16h20');
  });

  it('falls back to the wagon icon for unknown types', () => {
    const { container } = renderIcon('UnknownType');
    expect(container.innerHTML).toContain('M2 16h20');
  });

  it('uses the owned color when the unit is owned', () => {
    const { container } = renderIcon('Locomotive', true);
    expect(container.firstElementChild).toHaveClass('text-blue-600');
  });

  it('uses the unowned color when the unit is not owned', () => {
    const { container } = renderIcon('Locomotive', false);
    expect(container.firstElementChild).toHaveClass('text-gray-400');
  });

  it('shows the dashed overlay for unowned units', () => {
    const { container } = renderIcon('Coach', false);
    expect(container.querySelector('.border-dashed')).not.toBeNull();
  });

  it('omits the dashed overlay for owned units', () => {
    const { container } = renderIcon('Coach', true);
    expect(container.querySelector('.border-dashed')).toBeNull();
  });

  it('applies custom classes to the wrapper', () => {
    const { container } = renderIcon('Locomotive', false, 'ring-1');
    expect(container.firstElementChild).toHaveClass('ring-1');
  });

  it('renders an svg element', () => {
    const { container } = renderIcon('Coach');
    expect(container.querySelector('svg')).not.toBeNull();
  });

  it('keeps the wrapper layout classes', () => {
    const { container } = renderIcon('Coach');
    expect(container.firstElementChild).toHaveClass('relative', 'flex', 'items-center');
  });
});
