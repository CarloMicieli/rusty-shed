import {
  LayoutDashboard,
  TrainFront,
  Wallet,
  Heart,
  Wrench,
  Warehouse,
  Cpu,
  TrainTrack
} from 'lucide-svelte';
import * as m from '$lib/paraglide/messages.js';
import type { NavigationItem } from './types';

/**
 * Complete navigation configuration for the application
 * Items are ordered as they should appear in the UI
 */
export const NAVIGATION_ITEMS: NavigationItem[] = [
  {
    id: 'home',
    label: () => m.app_home(),
    icon: LayoutDashboard,
    href: '/my-dashboard',
    isPrimary: true
  },
  {
    id: 'collection',
    label: () => m.app_collection(),
    icon: TrainFront,
    href: '/my-collection',
    isPrimary: true,
    additionalPrefixes: ['/collection']
  },
  {
    id: 'finance',
    label: () => m.app_finance(),
    icon: Wallet,
    href: '/my-budget',
    isPrimary: true
  },
  {
    id: 'wishlists',
    label: () => m.app_wishlists(),
    icon: Heart,
    href: '/my-wishlists',
    isPrimary: true
  },
  {
    id: 'maintenance',
    label: () => m.app_maintenance(),
    icon: Wrench,
    href: '/my-maintenance',
    isPrimary: false
  },
  {
    id: 'depot',
    label: () => m.app_depot(),
    icon: Warehouse,
    href: '/my-depot',
    isPrimary: false
  },
  {
    id: 'digital-dcc',
    label: () => m.app_digital_dcc(),
    icon: Cpu,
    href: '/my-digital-roster',
    isPrimary: false
  },
  {
    id: 'railway-tracks',
    label: () => m.app_railway_tracks(),
    icon: TrainTrack,
    href: '/my-tracks',
    isPrimary: false,
    usePrefixMatch: true
  }
];

/**
 * Filter helper: Get primary navigation items (mobile bottom bar)
 */
export const PRIMARY_ITEMS = NAVIGATION_ITEMS.filter((item) => item.isPrimary);

/**
 * Filter helper: Get secondary navigation items (More menu)
 */
export const SECONDARY_ITEMS = NAVIGATION_ITEMS.filter((item) => !item.isPrimary);

// Development-only validation
if (import.meta.env.DEV) {
  const primaryCount = NAVIGATION_ITEMS.filter((i) => i.isPrimary).length;
  if (primaryCount !== 4) {
    console.warn(`Navigation config: Expected 4 primary items, got ${primaryCount}`);
  }
}
