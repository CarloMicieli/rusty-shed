import { describe, expect, it } from 'vitest';
import { hasTraction, isTractionSlot } from '$lib/features/train-formations/domain/traction';
import { makeElement, makePrototype } from '../fixtures';

describe('train-formations traction domain', () => {
  it('returns false for an empty composition', () => {
    expect(hasTraction([])).toBe(false);
  });

  it('returns false for a coach-only slot', () => {
    expect(
      isTractionSlot(
        makeElement({
          prototype: makePrototype({ car_type: 'Coach', is_motorized: false })
        })
      )
    ).toBe(false);
  });

  it('counts a locomotive as traction', () => {
    expect(isTractionSlot(makeElement())).toBe(true);
  });

  it('counts a power car as traction', () => {
    expect(
      isTractionSlot(
        makeElement({
          prototype: makePrototype({ car_type: 'PowerCar', series_code: 'RAe TEE II' })
        })
      )
    ).toBe(true);
  });

  it('excludes default dummy motorized units', () => {
    expect(
      isTractionSlot(
        makeElement({
          prototype: makePrototype({ default_is_dummy: true })
        })
      )
    ).toBe(false);
  });

  it('force-includes a coach when override is 1', () => {
    expect(
      isTractionSlot(
        makeElement({
          prototype: makePrototype({ car_type: 'Coach', is_motorized: false }),
          traction_override: 1
        })
      )
    ).toBe(true);
  });

  it('force-excludes a locomotive when override is -1', () => {
    expect(isTractionSlot(makeElement({ traction_override: -1 }))).toBe(false);
  });

  it('returns false when all elements are force-excluded', () => {
    expect(
      hasTraction([
        makeElement({ id: 'a', traction_override: -1 }),
        makeElement({
          id: 'b',
          prototype: makePrototype({ car_type: 'PowerCar' }),
          traction_override: -1
        })
      ])
    ).toBe(false);
  });

  it('returns true when any element provides traction', () => {
    expect(
      hasTraction([
        makeElement({
          id: 'coach',
          prototype: makePrototype({ car_type: 'Coach', is_motorized: false })
        }),
        makeElement({ id: 'loco' })
      ])
    ).toBe(true);
  });

  it('keeps a non-motorized unit excluded without overrides', () => {
    expect(
      isTractionSlot(
        makeElement({
          prototype: makePrototype({ car_type: 'BaggageCar', is_motorized: false })
        })
      )
    ).toBe(false);
  });

  it('force-includes a dummy locomotive when override is 1', () => {
    expect(
      isTractionSlot(
        makeElement({
          prototype: makePrototype({ default_is_dummy: true }),
          traction_override: 1
        })
      )
    ).toBe(true);
  });
});
