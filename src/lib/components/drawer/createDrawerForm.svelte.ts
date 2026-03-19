/**
 * Svelte 5 Runes-based form utility for drawers.
 *
 * Creates a reactive form object with:
 * - `values`  — the current form state (deeply reactive via $state)
 * - `errors`  — validation errors (only exposed after `touch()`)
 * - `isValid` — whether the form passes validation (always evaluated)
 * - `isDirty` — whether values differ from the initial snapshot
 * - `touched` — whether the user has attempted to submit
 * - `touch()` — marks the form as touched (call on submit attempt)
 * - `reset()`  — resets to initial state and clears touched flag
 */
export function createDrawerForm<
  T extends object,
  E extends Record<string, unknown> = Record<string, string>
>(opts: { initial: () => T; validate?: (values: T) => E }) {
  let values = $state<T>(opts.initial());
  let touched = $state(false);
  let _initial = $state<T>(opts.initial());

  const errors = $derived(opts.validate ? opts.validate(values) : ({} as E));
  const isValid = $derived(Object.values(errors as Record<string, unknown>).every((e) => !e));
  const isDirty = $derived(JSON.stringify(values) !== JSON.stringify(_initial));

  return {
    get values() {
      return values;
    },
    get errors(): E {
      return touched ? errors : ({} as E);
    },
    get isValid() {
      return isValid;
    },
    get isDirty() {
      return isDirty;
    },
    get touched() {
      return touched;
    },
    touch() {
      touched = true;
    },
    reset(next?: T) {
      const v = next ?? opts.initial();
      values = v;
      _initial = structuredClone(v) as T;
      touched = false;
    },
    syncInitial() {
      _initial = JSON.parse(JSON.stringify(values)) as T;
    }
  };
}
