/**
 * Currency utilities for formatting and symbol resolution.
 */

/**
 * Gets the symbol for a given currency code.
 * @param currency The currency code (e.g., 'EUR', 'USD')
 * @param locale The locale to use for formatting (defaults to system locale)
 * @returns The currency symbol or the original code if not found
 */
export function getCurrencySymbol(currency: string, locale: string = 'en-US'): string {
  try {
    return (
      new Intl.NumberFormat(locale, {
        style: 'currency',
        currency: currency
      })
        .formatToParts(0)
        .find((part) => part.type === 'currency')?.value || currency
    );
  } catch (e) {
    console.warn(`Failed to get currency symbol for ${currency}`, e);
    return currency;
  }
}

/**
 * Formats integer cents to a standard currency string.
 * @param cents The amount in cents
 * @param currency The currency code
 * @param locale The locale
 */
export function formatCents(
  cents: number | bigint | null | undefined,
  currency: string,
  locale: string = 'en-US'
): string {
  if (cents === null || cents === undefined) return '';

  return new Intl.NumberFormat(locale, {
    style: 'currency',
    currency: currency
  }).format(Number(cents) / 100);
}
