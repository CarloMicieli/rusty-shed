import { createToaster } from '@skeletonlabs/skeleton-svelte';

// Shared toaster instance used across the app
export const toaster = createToaster({
  placement: 'top-end',
  overlap: true,
  duration: 4000
});
