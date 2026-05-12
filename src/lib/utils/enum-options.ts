import type { Category, PowerMethod, Scale } from '$lib/bindings';
import * as m from '$lib/paraglide/messages.js';

// ── Localized Sort ────────────────────────────────────────────────────────────
// Uses the runtime locale automatically, matching Paraglide's active language.
const collator = new Intl.Collator(undefined, { sensitivity: 'base' });

export function sortLocalized<T>(items: T[], getLabel: (item: T) => string): T[] {
  return [...items].sort((a, b) => collator.compare(getLabel(a), getLabel(b)));
}

// ── Scales ────────────────────────────────────────────────────────────────────
// Technical notation — not locale-sensitive, so no Paraglide keys needed.
export const SCALE_DISPLAY_MAP: Record<Scale, string> = {
  H0: 'H0 (1:87)',
  H0m: 'H0m (1:87)',
  H0e: 'H0e (1:87)',
  N: 'N (1:160)',
  TT: 'TT (1:120)',
  Z: 'Z (1:220)',
  G: 'G (1:22.5)',
  '1': '1 (1:32)',
  '0': '0 (1:43.5)',
  '00': '00 (1:76.2)'
};

export const SCALES: Scale[] = Object.keys(SCALE_DISPLAY_MAP) as Scale[];

export function scaleOptions(): { value: Scale; label: string }[] {
  return SCALES.map((s) => ({ value: s, label: SCALE_DISPLAY_MAP[s] }));
}

// ── Power Methods ─────────────────────────────────────────────────────────────
// Maps are inside functions so Paraglide functions are only accessed at render
// time, not at module initialization (which would break Vitest mocks).
export const POWER_METHODS: PowerMethod[] = ['AC', 'DC', 'TRIX_EXPRESS'];

export function powerMethodLabel(pm: PowerMethod): string {
  const map: Record<PowerMethod, () => string> = {
    AC: m.enum_power_method_ac,
    DC: m.enum_power_method_dc,
    TRIX_EXPRESS: m.enum_power_method_trix_express
  };
  return map[pm]();
}

export function powerMethodOptions(): { value: PowerMethod; label: string }[] {
  return sortLocalized(
    POWER_METHODS.map((pm) => ({ value: pm, label: powerMethodLabel(pm) })),
    (item) => item.label
  );
}

// ── Categories ────────────────────────────────────────────────────────────────
export const CATEGORIES: Category[] = [
  'LOCOMOTIVES',
  'TRAIN_SETS',
  'STARTER_SETS',
  'FREIGHT_CARS',
  'PASSENGER_CARS',
  'ELECTRIC_MULTIPLE_UNITS',
  'RAILCARS'
];

export function categoryLabel(cat: Category): string {
  const map: Record<Category, () => string> = {
    LOCOMOTIVES: m.enum_category_locomotives,
    TRAIN_SETS: m.enum_category_train_sets,
    STARTER_SETS: m.enum_category_starter_sets,
    FREIGHT_CARS: m.enum_category_freight_cars,
    PASSENGER_CARS: m.enum_category_passenger_cars,
    ELECTRIC_MULTIPLE_UNITS: m.enum_category_electric_multiple_units,
    RAILCARS: m.enum_category_railcars
  };
  return map[cat]();
}

export function categoryOptions(): { value: Category; label: string }[] {
  return sortLocalized(
    CATEGORIES.map((cat) => ({ value: cat, label: categoryLabel(cat) })),
    (item) => item.label
  );
}
