// Re-export toast store for backward compatibility
// This replaces the Skeleton toaster with a shadcn-svelte compatible implementation
export { toastStore as toaster, type Toast } from './stores/toast';
