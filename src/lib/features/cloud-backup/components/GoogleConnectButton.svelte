<script lang="ts">
  /**
   * GoogleConnectButton Component
   *
   * Displays a button to connect or disconnect Google Drive account.
   * Shows current connection status and user email when connected.
   *
   * @component
   * @example
   * ```svelte
   * <GoogleConnectButton class="w-full" />
   * ```
   */

  import { Button } from '$lib/components';
  import { getCloudBackupController } from '../controllers/cloudBackup.svelte';
  import * as m from '$lib/paraglide/messages';
  import { AlertCircle, CheckCircle2, Loader2 } from 'lucide-svelte';

  interface Props {
    /** Optional CSS class for styling */
    class?: string;
  }

  let { class: className }: Props = $props();

  const controller = getCloudBackupController();

  const isConnected = $derived(controller.isConnected);
  const userEmail = $derived(controller.userEmail);
  const isLoading = $derived(controller.isLoading);
  const error = $derived(controller.error);

  async function handleConnect() {
    try {
      await controller.connectGoogle();
    } catch (err) {
      console.error('Failed to connect Google account:', err);
    }
  }

  async function handleDisconnect() {
    try {
      await controller.disconnectGoogle();
    } catch (err) {
      console.error('Failed to disconnect Google account:', err);
    }
  }
</script>

<div class="{className} space-y-4">
  {#if error}
    <div
      class="flex items-center gap-2 rounded-md bg-destructive/15 px-4 py-2 text-sm text-destructive"
    >
      <AlertCircle class="h-4 w-4" />
      <span>{error}</span>
    </div>
  {/if}

  {#if isConnected && userEmail}
    <div class="space-y-3">
      <div
        class="flex items-center gap-2 rounded-md bg-green-500/10 px-4 py-2 text-sm text-green-600 dark:text-green-400"
      >
        <CheckCircle2 class="h-4 w-4" />
        <span>{m.cloud_backup_connected_as({ email: userEmail })}</span>
      </div>

      <Button variant="outline" onclick={handleDisconnect} disabled={isLoading} class="w-1/4">
        {#if isLoading}
          <Loader2 class="mr-2 h-4 w-4 animate-spin" />
        {/if}
        {m.cloud_backup_disconnect()}
      </Button>
    </div>
  {:else}
    <Button onclick={handleConnect} disabled={isLoading} class="w-1/4">
      {#if isLoading}
        <Loader2 class="mr-2 h-4 w-4 animate-spin" />
      {/if}
      {m.cloud_backup_connect_google()}
    </Button>
  {/if}
</div>
