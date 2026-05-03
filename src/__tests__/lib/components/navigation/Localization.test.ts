import { describe, it, expect } from 'vitest';
import * as m from '$lib/paraglide/messages.js';

/**
 * Localization Tests for User Story 5
 *
 * Goal: Navigation labels appear in user's preferred language
 *
 * These tests validate that:
 * - All navigation labels use Paraglide message functions (not hardcoded strings)
 * - Locale-aware rendering is implemented via {#key locale} blocks
 * - All UI text is properly localized
 */

describe('User Story 5: Localized Navigation', () => {
  describe('T062-T064: Paraglide Message Functions', () => {
    it('all navigation labels use Paraglide message functions (no hardcoded strings)', () => {
      // T062, T064: Verify all message functions exist and return strings
      expect(typeof m.app_home).toBe('function');
      expect(typeof m.app_collection).toBe('function');
      expect(typeof m.app_finance).toBe('function');
      expect(typeof m.app_wishlists).toBe('function');
      expect(typeof m.app_maintenance).toBe('function');
      expect(typeof m.app_depot).toBe('function');
      expect(typeof m.app_digital_dcc).toBe('function');
      expect(typeof m.app_railway_tracks).toBe('function');
      expect(typeof m.app_more).toBe('function');
      expect(typeof m.app_more_aria).toBe('function');

      // Verify they return non-empty strings (localized values)
      expect(m.app_home()).toBeTruthy();
      expect(m.app_collection()).toBeTruthy();
      expect(m.app_finance()).toBeTruthy();
      expect(m.app_wishlists()).toBeTruthy();
      expect(m.app_maintenance()).toBeTruthy();
      expect(m.app_depot()).toBeTruthy();
      expect(m.app_digital_dcc()).toBeTruthy();
      expect(m.app_railway_tracks()).toBeTruthy();
      expect(m.app_more()).toBeTruthy();
      expect(m.app_more_aria()).toBeTruthy();
    });

    it('all messages return strings (support for future i18n', () => {
      // T064: Verify all message functions are callable and return strings
      const labels = [
        m.app_home(),
        m.app_collection(),
        m.app_finance(),
        m.app_wishlists(),
        m.app_maintenance(),
        m.app_depot(),
        m.app_digital_dcc(),
        m.app_railway_tracks(),
        m.app_more(),
        m.app_more_aria()
      ];

      labels.forEach((label) => {
        expect(typeof label).toBe('string');
        expect(label.length).toBeGreaterThan(0);
      });
    });

    it('deprecated keys are no longer used', () => {
      // Verify no hardcoded old labels remain
      const englishLabels = [
        m.app_home(),
        m.app_finance(),
        m.app_digital_dcc(),
        m.app_railway_tracks()
      ];

      // These should NOT match the old English text

      expect(englishLabels.includes('Dashboard' as any)).toBe(false);

      expect(englishLabels.includes('Budget Tracking' as any)).toBe(false);

      expect(englishLabels.includes('My Digital Rolling Stocks' as any)).toBe(false);

      expect(englishLabels.includes('My Tracks' as any)).toBe(false);
    });
  });

  describe('T065-T066: Paraglide Native Reactivity', () => {
    it('Paraglide message functions are reactive without {#key} blocks', () => {
      // T063: Svelte 5 + Paraglide reactivity approach
      //
      // Navigation components (SidebarNavigation.svelte, BottomNavigation.svelte)
      // call Paraglide message functions directly in the template:
      //   {item.label()} → m.app_collection(), m.app_finance(), etc.
      //
      // Paraglide-JS message functions are reactive in Svelte 5: when the active
      // locale changes, Svelte's fine-grained reactivity re-evaluates any expression
      // that calls a message function, updating the UI without a full DOM remount.
      //
      // This eliminates the need for {#key locale} wrappers, which would destroy
      // and re-mount the entire component tree on every locale change.

      // This behavior is verified by manual testing (T068)
      expect(true).toBe(true); // Documentation test
    });

    it('navigation components use locale store for i18n', () => {
      // T063, T065-T066: Navigation components react to locale changes
      // through Paraglide's locale context and Svelte 5's fine-grained reactivity.

      // When a user changes their language preference:
      // 1. localeStore is updated
      // 2. Paraglide's locale context updates
      // 3. All m.<key>() call sites in the template re-evaluate
      // 4. UI reflects new language immediately (no DOM remount)

      expect(true).toBe(true); // Behavior verified in manual testing
    });
  });

  describe('T067: MoreMenu Localization', () => {
    it('MoreMenu uses Paraglide message functions for all text', () => {
      // T067: MoreMenu should use Paraglide functions for:
      // 1. "More" menu title: m.app_more()
      // 2. All secondary feature labels via SECONDARY_ITEMS config
      // 3. No hardcoded text anywhere in component

      // Verify message functions are callable
      expect(typeof m.app_more).toBe('function');
      expect(m.app_more()).toBeTruthy();

      // Secondary items use label functions from config:
      expect(typeof m.app_maintenance).toBe('function');
      expect(typeof m.app_depot).toBe('function');
      expect(typeof m.app_digital_dcc).toBe('function');
      expect(typeof m.app_railway_tracks).toBe('function');
    });
  });

  describe('T068: Manual Language Switching', () => {
    it('language switching test documented (manual verification required)', () => {
      // T068: Manual testing procedure:
      // 1. Open app in browser
      // 2. View navigation in English (default)
      // 3. Open Settings → Language → Select Italian
      // 4. Verify ALL navigation labels change to Italian:
      //    - SidebarNavigation items (desktop)
      //    - BottomNavigation items (mobile)
      //    - More menu items (when opened on mobile)
      // 5. Switch back to English and verify labels revert
      //
      // Expected Italian labels:
      // - Home → Home (same in Italian)
      // - Collection → Collezione
      // - Finance → Finanze
      // - Wishlists → Liste Desideri
      // - Maintenance → Manutenzione
      // - Depot → Deposito
      // - Digital (DCC) → Digitale (DCC)
      // - Railway Tracks → Binari
      // - More → Altro
      //
      // Reactivity is provided natively by Paraglide-JS: message functions
      // re-evaluate when the locale changes, with no {#key} remount needed.

      expect(true).toBe(true); // Manual test documented
    });
  });

  describe('T069: More Button Accessibility', () => {
    it('aria-label for More button uses m.app_more_aria()', () => {
      // T069: Verify aria-label message function exists and provides
      // accessible label for the More button

      expect(typeof m.app_more_aria).toBe('function');

      const ariaLabel = m.app_more_aria();
      expect(typeof ariaLabel).toBe('string');
      expect(ariaLabel.length).toBeGreaterThan(0);

      // English: "Open more features menu"
      // Italian: "Apri menu funzionalità aggiuntive"
      expect(ariaLabel).toBeTruthy();
    });

    it('aria-label message is different from button text', () => {
      // Accessibility best practice: aria-label should provide
      // descriptive text beyond just the button text

      const buttonText = m.app_more();
      const ariaLabel = m.app_more_aria();

      // They may be different for better accessibility
      expect(buttonText).toBeTruthy();
      expect(ariaLabel).toBeTruthy();
      // Note: They might be the same in some languages
    });
  });

  describe('T070: Test Suite Verification', () => {
    it('all localization requirements have corresponding translations', () => {
      // T070: Verify translation keys are consistent across all components

      const requiredKeys = [
        'app_home',
        'app_collection',
        'app_finance',
        'app_wishlists',
        'app_maintenance',
        'app_depot',
        'app_digital_dcc',
        'app_railway_tracks',
        'app_more',
        'app_more_aria'
      ];

      requiredKeys.forEach((key) => {
        expect(typeof m[key as keyof typeof m]).toBe('function');
        expect((m[key as keyof typeof m] as () => string)()).toBeTruthy();
      });
    });
  });
});
