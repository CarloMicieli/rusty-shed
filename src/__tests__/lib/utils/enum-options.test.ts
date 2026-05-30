import { describe, expect, it } from 'vitest';
import { SCALES, scaleOptions } from '$lib/utils/enum-options';

describe('enum-options', () => {
  it('returns scale options sorted from bigger to smaller models', () => {
    const orderedValues = scaleOptions().map((opt) => opt.value);

    expect(orderedValues).toEqual(['G', '1', '0', '00', 'H0', 'H0m', 'H0e', 'TT', 'N', 'Z']);
  });

  it('returns the same set of values as the base scale list', () => {
    const optionValues = scaleOptions().map((opt) => opt.value);

    expect(new Set(optionValues)).toEqual(new Set(SCALES));
  });
});
