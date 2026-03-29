import {
  LayoutDashboard,
  TrainFront,
  Wallet,
  Heart,
  Wrench,
  Warehouse,
  Cpu,
  TrainTrack,
  Combine
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
    href: '/dashboard',
    isPrimary: true
  },
  {
    id: 'collection',
    label: () => m.app_collection(),
    icon: TrainFront,
    href: '/collection',
    isPrimary: true,
    usePrefixMatch: true
  },
  {
    id: 'finance',
    label: () => m.app_finance(),
    icon: Wallet,
    href: '/finance',
    isPrimary: true
  },
  {
    id: 'wishlists',
    label: () => m.app_wishlists(),
    icon: Heart,
    href: '/wishlists',
    isPrimary: true,
    usePrefixMatch: true
  },
  {
    id: 'maintenance',
    label: () => m.app_maintenance(),
    icon: Wrench,
    href: '/maintenance',
    isPrimary: false,
    usePrefixMatch: true
  },
  {
    id: 'depot',
    label: () => m.app_depot(),
    icon: Warehouse,
    href: '/depot',
    isPrimary: false
  },
  {
    id: 'digital-dcc',
    label: () => m.app_digital_dcc(),
    icon: Cpu,
    href: '/digital-dcc',
    isPrimary: false
  },
  {
    id: 'railway-tracks',
    label: () => m.app_railway_tracks(),
    icon: TrainTrack,
    href: '/railway-tracks',
    isPrimary: false,
    usePrefixMatch: true
  },
  {
    id: 'train-formations',
    label: () => m.app_train_formations(),
    icon: Combine,
    href: '/train-formations',
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
