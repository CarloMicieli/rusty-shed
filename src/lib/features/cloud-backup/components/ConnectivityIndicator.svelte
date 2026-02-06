<script lang="ts">
  /**
   * ConnectivityIndicator Component
   *
   * Displays the current internet connectivity status.
   * Shows online/offline status with last checked timestamp.
   *
   * @component
   * @example
   * ```svelte
   * <ConnectivityIndicator testId="connectivity-badge" />
   * ```
   */

  import { Wifi, WifiOff } from 'lucide-svelte';
  import { connectivityStore } from '../stores/connectivity';
  import * as m from '$lib/paraglide/messages.js';

  /**
   * Props for the connectivity indicator
   */
  interface Props {
    /** Optional test id for automation/testing */
    testId?: string;
  }

  let { testId }: Props = $props();

  let status = $derived($connectivityStore);

  function formatCheckedAt(checkedAt: string): string {
    try {
      const date = new Date(checkedAt);
      return new Intl.DateTimeFormat(undefined, {
        hour: '2-digit',
        minute: '2-digit',
        second: '2-digit'
      }).format(date);
    } catch {
      return checkedAt;
    }
  }
</script>

<div
  class="flex items-center gap-2 rounded-md border border-gray-200 bg-white px-3 py-2 text-sm"
  data-testid={testId}
>
  {#if status}
    {#if status.isOnline}
      <Wifi class="h-4 w-4 text-green-600" />
      <span class="font-medium text-green-700">{m.cloud_backup_connectivity_online()}</span>
    {:else}
      <WifiOff class="h-4 w-4 text-amber-600" />
      <span class="font-medium text-amber-700">{m.cloud_backup_connectivity_offline()}</span>
    {/if}

    <span class="text-gray-500">
      {m.cloud_backup_connectivity_last_checked({
        timestamp: formatCheckedAt(status.checkedAt)
      })}
    </span>
  {:else}
    <Wifi class="h-4 w-4 text-gray-400" />
    <span class="text-gray-500">{m.cloud_backup_connectivity_checking()}</span>
  {/if}
</div>
