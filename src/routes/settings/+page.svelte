<script lang="ts">
  import { onMount } from 'svelte';
  import { getLocale, setLocale } from '$lib/paraglide/runtime.js';
  import { Button, PageHeader } from '$lib/components';
  import SettingsForm from '$lib/components/SettingsForm.svelte';
  import GoogleConnectButton from '$lib/features/cloud-backup/components/GoogleConnectButton.svelte';
  import ConnectivityIndicator from '$lib/features/cloud-backup/components/ConnectivityIndicator.svelte';
  import SyncButton from '$lib/features/cloud-backup/components/SyncButton.svelte';
  import BackupList from '$lib/features/cloud-backup/components/BackupList.svelte';
  import RestoreConfirmModal from '$lib/features/cloud-backup/components/RestoreConfirmModal.svelte';
  import { getCloudBackupController } from '$lib/features/cloud-backup';
  import DataManagementSection from '$lib/features/database-backup/components/DataManagementSection.svelte';
  import {
    fetchSettings,
    saveSettings,
    type SettingsDto,
    type UpdateSettingsPayload,
    type LanguageCode
  } from '$lib/services';
  import { toaster } from '$lib/toaster';
  import { getToastMessage } from '$lib/services/errors';
  import * as m from '$lib/paraglide/messages.js';
  import { setActiveLocale } from '$lib/stores/locale';

  let settings: SettingsDto | null = $state(null);
  let loading = $state(true);
  let saving = $state(false);
  let error: string | null = $state(null);

  const cloudBackupController = getCloudBackupController();
  let isConnected = $derived(cloudBackupController.isConnected);
  let lastSyncAt = $derived(cloudBackupController.lastSyncAt);
  let backups = $derived(cloudBackupController.backups);
  let backupCount = $derived(cloudBackupController.backups.length);

  // Restore modal state
  let showRestoreModal = $state(false);
  let selectedBackupId = $state<string | null>(null);
  let selectedBackupLabel = $state<string>('');

  onMount(async () => {
    await loadSettings();
  });

  async function loadSettings() {
    loading = true;
    const result = await fetchSettings();
    if (result.ok) {
      settings = result.data;
      await syncLocale(result.data.language);
      error = null;
    } else {
      error = getToastMessage(result.error);
    }
    loading = false;
  }

  import { themeStore } from '$lib/stores/themeStore.svelte';

  async function handleSubmit(payload: UpdateSettingsPayload) {
    saving = true;
    const result = await saveSettings(payload);
    if (result.ok) {
      settings = result.data;
      await syncLocale(result.data.language);
      await themeStore.setTheme(result.data.theme);
      toaster.success({ title: m.settings_saved_toast() });
    } else {
      toaster.error({ title: getToastMessage(result.error) });
    }
    saving = false;
  }

  async function syncLocale(language: LanguageCode) {
    const nextLocale = language === 'en' || language === 'it' ? language : null;
    if (!nextLocale) return;

    const current = getLocale();
    if (nextLocale === current) return;

    await setLocale(nextLocale, { reload: false });
    setActiveLocale(nextLocale);
  }

  function handleRestoreClick(backupId: string) {
    const backup = backups.find((b) => b.id === backupId);
    if (backup) {
      selectedBackupId = backupId;
      selectedBackupLabel = backup.label;
      showRestoreModal = true;
    }
  }

  async function handleRestoreConfirm() {
    // Restore was successful, reload the app
    toaster.success({
      title: m.cloud_backup_restore_success(),
      description: m.cloud_backup_restore_reload_notice()
    });

    // Wait a moment for the toast to show
    setTimeout(async () => {
      window.location.reload();
    }, 1000);
  }

  function handleRestoreCancel() {
    showRestoreModal = false;
    selectedBackupId = null;
    selectedBackupLabel = '';
  }
</script>

<svelte:head>
  <title>{m.app_name()} | {m.app_settings()}</title>
</svelte:head>

<div class="flex flex-col">
  <div
    class="-mx-4 -mt-4 mb-6 border-b border-border bg-card/50 px-6 py-4 lg:-mx-8 lg:-mt-8 lg:mb-8"
  >
    <PageHeader
      title={m.settings_heading()}
      subtitle={m.app_settings()}
      description={m.settings_description()}
    />
  </div>

  <div class="space-y-6">
    {#if loading}
      <div class="rounded-lg border border-border bg-card p-8 shadow-xl">
        <div class="animate-pulse space-y-4">
          <div class="h-4 w-1/3 rounded bg-muted"></div>
          <div class="h-4 w-1/2 rounded bg-muted"></div>
          <div class="grid gap-4 md:grid-cols-2">
            {#each Array(4) as _item, index (index)}
              <div class="h-20 rounded bg-muted/60"></div>
            {/each}
          </div>
        </div>
      </div>
    {:else if error}
      <div class="rounded-lg border border-destructive/30 bg-destructive/10 p-6">
        <p class="font-semibold text-destructive">{error}</p>
        <Button variant="default" class="mt-4" onclick={loadSettings}>
          {m.errors_retry_page()}
        </Button>
      </div>
    {:else if settings}
      <div class="space-y-6">
        {#key `${settings.language}-${settings.currency}-${settings.measureUnit}-${settings.favouriteScale}-${settings.powerSystem}`}
          <SettingsForm {settings} {saving} onsubmit={handleSubmit} />
        {/key}

        <!-- Data Management Section -->
        <DataManagementSection />

        <!-- Cloud Backup Section -->
        <div class="rounded-lg border border-border bg-card p-6 shadow-xl">
          <div class="space-y-4">
            <div>
              <h2 class="text-xl font-bold">{m.cloud_backup_title()}</h2>
              <p class="mt-1 text-sm text-muted-foreground">{m.cloud_backup_subtitle()}</p>
            </div>

            <GoogleConnectButton />

            {#if isConnected}
              <div class="border-t border-border pt-4">
                <ConnectivityIndicator />
                <SyncButton />

                {#if lastSyncAt}
                  <div class="text-sm text-muted-foreground">
                    <p>
                      {m.cloud_backup_last_sync({
                        timestamp: new Date(lastSyncAt).toLocaleString()
                      })}
                    </p>
                  </div>
                {/if}

                {#if backupCount > 0}
                  <div class="text-sm text-muted-foreground">
                    {#if backupCount === 1}
                      <p>{m.cloud_backup_backups_count_single({ count: backupCount })}</p>
                    {:else}
                      <p>{m.cloud_backup_backups_count_multiple({ count: backupCount })}</p>
                    {/if}
                  </div>
                {/if}

                <!-- Backup List Section -->
                <div class="border-t border-border pt-4">
                  <BackupList onRestore={handleRestoreClick} />
                </div>
              </div>
            {/if}
          </div>
        </div>
      </div>
    {/if}
  </div>
</div>

<!-- Restore Confirmation Modal -->
{#if selectedBackupId}
  <RestoreConfirmModal
    bind:open={showRestoreModal}
    backupId={selectedBackupId}
    backupLabel={selectedBackupLabel}
    onConfirm={handleRestoreConfirm}
    onCancel={handleRestoreCancel}
  />
{/if}
