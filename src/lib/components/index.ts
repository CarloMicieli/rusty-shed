/**
 * Component Index - Central export for all shadcn-svelte components
 *
 * This file provides a single import point for all UI components used in the app.
 * Import components like: import { Button, Card } from '$lib/components';
 *
 * Feature: 012-shadcn-migration
 */

// shadcn-svelte UI Components
export { Button } from './shadcn/button';
export { Badge } from './shadcn/badge';

export { default as ToastHost } from './ToastHost.svelte';

// UI components (for form fields)
export { default as FormField } from './ui/FormField.svelte';

// Application-specific components
export { default as BottomNavigation } from './BottomNavigation.svelte';
export { default as SidebarNavigation } from './SidebarNavigation.svelte';
export { default as SearchBar } from './SearchBar.svelte';
export { default as PageHeader } from './PageHeader.svelte';
export { default as StatsCard } from './StatsCard.svelte';
export { default as StatusBadge } from './StatusBadge.svelte';
export { default as RecentItemCard } from './RecentItemCard.svelte';
export { default as QuickActionButtons } from './QuickActionButtons.svelte';
export { default as SmartImage } from './SmartImage.svelte';

// Feature-specific components
export { default as AddWishlistItemModal } from './AddWishlistItemModal.svelte';
export { default as DepotListCard } from './DepotListCard.svelte';
export { default as DepotTable } from './DepotTable.svelte';
export { default as DepotView } from './DepotView.svelte';
export { default as SellerForm } from './SellerForm.svelte';
export { default as SettingsForm } from './SettingsForm.svelte';

// Re-export accordion components for convenience
export * from './accordion';
