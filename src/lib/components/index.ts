/**
 * Component Index - Central export for all shadcn-svelte components
 *
 * This file provides a single import point for all UI components used in the app.
 * Import components like: import { Button, Card } from '$lib/components';
 *
 * Feature: 012-shadcn-migration
 */

// shadcn-svelte UI Components
export { Button } from './ui/button';
export { Badge } from './ui/badge';
export { Input } from './ui/input';
export { Textarea } from './ui/textarea';
export { Checkbox } from './ui/checkbox';
export { RadioGroup } from './ui/radio-group';
export { Toggle } from './ui/toggle';
export { Sheet } from './ui/sheet';
export { Card, CardHeader, CardTitle, CardDescription, CardContent, CardFooter } from './ui/card';
export { Table, TableHeader, TableBody, TableRow, TableHead, TableCell } from './ui/table';
export { Alert, AlertTitle, AlertDescription } from './ui/alert';

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
export { default as SellerForm } from './SellerForm.svelte';
export { default as SettingsForm } from './SettingsForm.svelte';

// Re-export shadcn-svelte UI components
export * as Accordion from './ui/accordion';
export * as Form from './ui/form';
export * as Dialog from './ui/dialog';
export * as Select from './ui/select';
export { Toaster } from './ui/sonner';
