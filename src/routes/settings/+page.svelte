<script lang="ts">
  import { onMount } from 'svelte';
  import { settingsState } from '$lib/features/settings/SettingsState.svelte';
  import SettingsForm from '$lib/features/settings/components/SettingsForm.svelte';
  import * as m from '$lib/paraglide/messages';

  let loading = $state(true);
  let loadError = $state<string | null>(null);

  onMount(async () => {
    console.log('[SettingsPage] onMount called, loading settings...');
    try {
      await settingsState.load();
      console.log('[SettingsPage] Settings loaded successfully');
    } catch (err) {
      loadError = String(err);
      console.error('[SettingsPage] Failed to load settings:', err);
    } finally {
      loading = false;
      console.log('[SettingsPage] Loading complete, rendering form...');
    }
  });
</script>

<div class="container mx-auto max-w-3xl p-4">
  <div class="mb-6">
    <h1 class="h1">{m.settings_title()}</h1>
    <p class="text-surface-600-300-token">{m.settings_subtitle()}</p>
  </div>

  {#if loading}
    <div class="card p-6">
      <div class="placeholder animate-pulse">
        <div class="bg-surface-300-600-token mb-4 h-4 w-3/4 rounded"></div>
        <div class="bg-surface-300-600-token mb-4 h-4 w-1/2 rounded"></div>
        <div class="bg-surface-300-600-token h-4 w-2/3 rounded"></div>
      </div>
    </div>
  {:else if loadError}
    <div class="alert variant-filled-error" role="alert">
      <p>{m.settings_load_failed()}: {loadError}</p>
    </div>
  {:else}
    <SettingsForm />
  {/if}
</div>
