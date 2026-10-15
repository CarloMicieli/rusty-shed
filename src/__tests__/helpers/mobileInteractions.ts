import { fireEvent } from '@testing-library/svelte';
import { expect } from 'vitest';

const MIN_TOUCH_TARGET = 44;
const MIN_CHIP_REMOVE_TARGET = 36;

export async function tap(element: Element): Promise<void> {
  await fireEvent.pointerDown(element, { pointerType: 'touch', button: 0 });
  await fireEvent.pointerUp(element, { pointerType: 'touch', button: 0 });
  await fireEvent.click(element);
}

export function assertTouchTarget(
  element: HTMLElement,
  options: { chipRemove?: boolean } = {}
): void {
  const rect = element.getBoundingClientRect();
  const requiredSize = options.chipRemove ? MIN_CHIP_REMOVE_TARGET : MIN_TOUCH_TARGET;

  expect(rect.width).toBeGreaterThanOrEqual(requiredSize);
  expect(rect.height).toBeGreaterThanOrEqual(requiredSize);
}

export function setBoundingClientRect(
  element: HTMLElement,
  dimensions: { width: number; height: number }
): void {
  element.getBoundingClientRect = () =>
    ({
      width: dimensions.width,
      height: dimensions.height,
      top: 0,
      right: dimensions.width,
      bottom: dimensions.height,
      left: 0,
      x: 0,
      y: 0,
      toJSON: () => ({})
    }) as DOMRect;
}
