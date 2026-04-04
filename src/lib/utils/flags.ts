/**
 * Convert an ISO country code string (e.g., "IT") into a Unicode flag emoji.
 * Uses the Regional Indicator Symbol logic where each character is offset by 127397.
 *
 * @param code - The ISO 3166-1 alpha-2 country code (e.g., "IT", "GB", "DE").
 * @returns The flag emoji or a globe emoji fallback if the code is null/invalid.
 */
export const getFlag = (code: string | null | undefined): string => {
  if (!code || code.length !== 2) {
    return '\u{1F310}'; // 🌐 International / Default
  }

  return code
    .toUpperCase()
    .split('')
    .map((char) => String.fromCodePoint(char.charCodeAt(0) + 127397))
    .join('');
};
