import { describe, expect, it } from 'vitest';
import { createDrawerForm } from '$lib/components/drawer/createDrawerForm.svelte';

describe('createDrawerForm', () => {
  it('hides validation errors until touch() is called', () => {
    const form = createDrawerForm({
      initial: () => ({ name: '' }),
      validate: (values) => ({ name: values.name ? '' : 'Required' })
    });

    expect(form.errors).toEqual({});
    expect(form.isValid).toBe(false);

    form.touch();
    expect(form.errors).toEqual({ name: 'Required' });
  });

  it('tracks dirty state and reset lifecycle', () => {
    const form = createDrawerForm({
      initial: () => ({ quantity: 1 })
    });

    expect(form.isDirty).toBe(false);
    form.values.quantity = 2;
    expect(form.isDirty).toBe(true);

    form.reset();
    expect(form.values.quantity).toBe(1);
    expect(form.isDirty).toBe(false);
    expect(form.touched).toBe(false);
  });

  it('syncInitial() updates baseline for dirty checks', () => {
    const form = createDrawerForm({
      initial: () => ({ category: 'A' })
    });

    form.values.category = 'B';
    expect(form.isDirty).toBe(true);

    form.syncInitial();
    expect(form.isDirty).toBe(false);
  });

  it('reset(next) applies external state snapshot', () => {
    const form = createDrawerForm({
      initial: () => ({ amount: 100, note: '' })
    });

    form.touch();
    form.reset({ amount: 250, note: 'Updated' });

    expect(form.values).toEqual({ amount: 250, note: 'Updated' });
    expect(form.touched).toBe(false);
    expect(form.isDirty).toBe(false);
  });
});
