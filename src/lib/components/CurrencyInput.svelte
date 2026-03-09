<script lang="ts">
  /**
   * CurrencyInput Component
   * A specialized input for monetary values using integer cents.
   *
   * Features:
   * - Two-way binding for 'value' (integer cents)
   * - Free typing (formatting only on blur)
   * - Read-only currency symbol suffix
   * - Mobile-friendly numeric keyboard
   * - Svelte 5 Runes ($state, $derived, $props, $bindable)
   */
  import { Input } from '$lib/components/ui/input';
  import { cn } from '$lib/utils';

  interface Props {
    /** Integer cents amount, bound two-way */
    value: number | null | undefined;
    /** Display value (string representation), bound two-way */
    displayValue?: string;
    /** Whether the last input had invalid characters that were filtered, bound two-way */
    hasInvalidInput?: boolean;
    /** Currency symbol to display as suffix (e.g., €, $, CHF) */
    symbol: string;
    /** Form label for accessibility */
    label?: string;
    /** Unique identifier */
    id?: string;
    /** Placeholder when empty */
    placeholder?: string;
    /** Disable interaction */
    disabled?: boolean;
    /** Requirement flag */
    required?: boolean;
    /** Additional CSS classes for the container */
    class?: string;
    /** Additional CSS classes for the internal input element */
    inputClass?: string;
    /** Optional change notification */
    onchange?: (value: number | null) => void;
    /** Standard blur event */
    onblur?: (e: FocusEvent & { currentTarget: HTMLInputElement }) => void;
    /** Standard keydown event */
    onkeydown?: (e: KeyboardEvent & { currentTarget: HTMLInputElement }) => void;
    /** Automatically focus on mount */
    autofocus?: boolean;
  }

  let {
    value = $bindable(null),
    displayValue: bindableDisplayValue = $bindable(''),
    hasInvalidInput: bindableHasInvalidInput = $bindable(false),
    symbol,
    label,
    id,
    placeholder = '0.00',
    disabled = false,
    required = false,
    class: className = '',
    inputClass = '',
    onchange,
    onblur,
    onkeydown,
    autofocus = false
  }: Props = $props();

  /**
   * Helper: Format integer cents to decimal string (e.g., 1050 -> "10.50")
   */
  function toDecimalString(cents: number | null | undefined): string {
    if (cents === null || cents === undefined) return '';
    return (cents / 100).toFixed(2);
  }

  /**
   * Helper: Parse decimal string to integer cents (e.g., "10.50" -> 1050, "-5.00" -> -500)
   */
  function parseToCents(val: string): number | null {
    if (!val || val.trim() === '') return null;
    // Support both dot and comma (European style) separators
    const clean = val.replace(',', '.');
    const parsed = parseFloat(clean);
    return isNaN(parsed) ? null : Math.round(parsed * 100);
  }

  // Track the last value we synced from the parent to detect external changes
  let lastSyncedValue = $state(value);

  // Internal display state - what the user actually sees/types in the input
  let displayValue = $state(bindableDisplayValue || toDecimalString(value));

  // Keep internal display string in sync with external value if it changes programmatically
  $effect(() => {
    if (value !== lastSyncedValue) {
      displayValue = toDecimalString(value);
      lastSyncedValue = value;
    }
  });

  // Sync displayValue and hasInvalidInput back to parent
  $effect(() => {
    bindableDisplayValue = displayValue;
  });

  // Clear invalid flag when the user fixes the input
  $effect(() => {
    if (displayValue.length > 0) {
      // hasInvalidInput is already set by handleInput, no need to change it here
    }
  });

  /**
   * Handle blur event:
   * 1. Finalize the parsed value to integer cents
   * 2. Re-format the display string to ensure 2 decimal places
   */
  function handleBlur(e: FocusEvent & { currentTarget: HTMLInputElement }) {
    const nextCents = parseToCents(displayValue);
    value = nextCents;
    lastSyncedValue = nextCents;
    displayValue = toDecimalString(nextCents);
    onchange?.(nextCents);
    onblur?.(e);
  }

  /**
   * Handle input event:
   * Keep the internal displayValue state synchronized with keyboard input.
   * Filter characters to only allow digits, single dot, single comma, or leading minus.
   * Track if invalid characters were removed.
   */
  function handleInput(e: Event & { currentTarget: HTMLInputElement }) {
    const raw = e.currentTarget.value;
    // Allow digits, dot, comma, and leading minus sign
    let filtered = raw.replace(/[^\d.,-]/g, '');

    // Track if any characters were filtered out (indicating invalid input)
    bindableHasInvalidInput = raw.length > 0 && filtered.length < raw.length;

    // Ensure minus sign only appears at the start
    const minusCount = (filtered.match(/-/g) || []).length;
    if (minusCount > 1) {
      // Remove all minus signs and add back one at the start if original had any
      filtered = filtered.replace(/-/g, '');
      if (raw.startsWith('-')) {
        filtered = '-' + filtered;
      }
    } else if (minusCount === 1 && !filtered.startsWith('-')) {
      // Move minus to the start
      filtered = '-' + filtered.replace('-', '');
    }

    // Prevent multiple separators
    const parts = filtered.split(/[.,]/);
    let finalValue = filtered;
    if (parts.length > 2) {
      // Keep only first separator
      const firstSeparatorMatch = filtered.match(/[.,]/);
      const separator = firstSeparatorMatch ? firstSeparatorMatch[0] : '';
      const before = parts[0];
      const after = parts.slice(1).join('');
      finalValue = before + separator + after;
    }

    displayValue = finalValue;
    // Force the input element to show the filtered value
    e.currentTarget.value = finalValue;
  }

  /**
   * Handle keydown event:
   * 1. If Enter is pressed, sync the value immediately so handlers (like form submit) have latest data.
   */
  function handleKeyDownInternal(e: KeyboardEvent & { currentTarget: HTMLInputElement }) {
    if (e.key === 'Enter') {
      const nextCents = parseToCents(displayValue);
      value = nextCents;
      lastSyncedValue = nextCents;
      onchange?.(nextCents);
    }
    onkeydown?.(e);
  }
</script>

<div class={cn('group relative flex w-full items-center', className)}>
  <Input
    {id}
    type="text"
    inputmode="decimal"
    value={displayValue}
    oninput={handleInput}
    onblur={handleBlur}
    {placeholder}
    {disabled}
    {required}
    class={cn('pr-12 text-right font-mono', inputClass)}
    aria-label={label}
    onkeydown={handleKeyDownInternal}
    {autofocus}
  />
  <div
    class="pointer-events-none absolute right-3 flex h-full items-center pl-2 text-sm font-medium text-muted-foreground transition-colors group-focus-within:text-foreground"
    aria-hidden="true"
  >
    {symbol}
  </div>
</div>

<style>
  /* Optional: Ensure font-mono looks consistent for numbers across browsers */
  :global(.font-mono) {
    font-variant-numeric: tabular-nums;
  }
</style>
