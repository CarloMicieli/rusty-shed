import { describe, it, expect, vi } from 'vitest';

// Mock Paraglide messages: each function returns its own key name for easy assertion
vi.mock('$lib/paraglide/messages.js', () => ({
  module_label_yard_overview: () => 'module_label_yard_overview',
  module_label_collection_depot: () => 'module_label_collection_depot',
  module_label_wishlist: () => 'module_label_wishlist',
  module_label_maintenance_log: () => 'module_label_maintenance_log',
  module_label_finance_ledger: () => 'module_label_finance_ledger',
  module_label_global_search: () => 'module_label_global_search',
  module_label_settings: () => 'module_label_settings',
  module_label_signal_box: () => 'module_label_signal_box'
}));

import { getModuleLabel } from '$lib/services/module-label';

describe('getModuleLabel', () => {
  it('/dashboard → yard overview label', () => {
    expect(getModuleLabel('/dashboard')).toBe('module_label_yard_overview');
  });

  it('/dashboard/sub-page → yard overview label', () => {
    expect(getModuleLabel('/dashboard/stats')).toBe('module_label_yard_overview');
  });

  it('/collection → collection depot label', () => {
    expect(getModuleLabel('/collection')).toBe('module_label_collection_depot');
  });

  it('/wishlist → wishlist label', () => {
    expect(getModuleLabel('/wishlist')).toBe('module_label_wishlist');
  });

  it('/wishlists → wishlist label (prefix match)', () => {
    expect(getModuleLabel('/wishlists/123')).toBe('module_label_wishlist');
  });

  it('/maintenance → maintenance log label', () => {
    expect(getModuleLabel('/maintenance')).toBe('module_label_maintenance_log');
  });

  it('/finance → finance ledger label', () => {
    expect(getModuleLabel('/finance')).toBe('module_label_finance_ledger');
  });

  it('/search → global search label', () => {
    expect(getModuleLabel('/search')).toBe('module_label_global_search');
  });

  it('/settings → settings label', () => {
    expect(getModuleLabel('/settings')).toBe('module_label_settings');
  });

  it('/unknown-path → signal box fallback', () => {
    expect(getModuleLabel('/unknown-path')).toBe('module_label_signal_box');
  });

  it('empty string → signal box fallback', () => {
    expect(getModuleLabel('')).toBe('module_label_signal_box');
  });
});
