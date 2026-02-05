<script lang="ts">
  import { onMount } from 'svelte';
  import { getLocale, setLocale } from '$lib/paraglide/runtime.js';
  import { Button } from '$lib/components';
  import SettingsForm from '$lib/components/SettingsForm.svelte';
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

  onMount(async () => {
    await loadSettings();
  });

  async function loadSettings() {
    loading = true;
    const result = await fetchSettings();
    if (result.ok) {
      settings = result.data;
      await syncLocale(result.data.languageCode);
      error = null;
    } else {
      error = getToastMessage(result.error);
    }
    loading = false;
  }

  async function handleSubmit(payload: UpdateSettingsPayload) {
    saving = true;
    const result = await saveSettings(payload);
    if (result.ok) {
      settings = result.data;
      await syncLocale(result.data.languageCode);
      toaster.success({ title: m.settings_saved_toast() });
    } else {
      toaster.error({ title: getToastMessage(result.error) });
    }
    saving = false;
  }

  async function syncLocale(languageCode: LanguageCode) {
    const nextLocale = languageCode === 'en' || languageCode === 'it' ? languageCode : null;
    if (!nextLocale) return;

    const current = getLocale();
    if (nextLocale === current) return;

    await setLocale(nextLocale, { reload: false });
    setActiveLocale(nextLocale);
  }
</script>

<svelte:head>
  <title>{m.app_name()} | {m.app_settings()}</title>
</svelte:head>

<div class="space-y-6">
  <div class="flex items-center justify-between">
    <div>
      <p class="text-surface-400 text-sm font-semibold tracking-widest uppercase">
        {m.app_settings()}
      </p>
      <h1 class="h2 text-surface-50 font-bold">{m.settings_heading()}</h1>
      <p class="text-surface-300">{m.settings_description()}</p>
    </div>
    <a class="text-accent-500 text-sm font-semibold hover:underline" href="/my-dashboard">
      {m.settings_back_to_dashboard()}
    </a>
  </div>

  {#if loading}
    <div class="card border-surface-700/40 border p-8 shadow-xl">
      <div class="animate-pulse space-y-4">
        <div class="bg-surface-700/60 h-4 w-1/3 rounded"></div>
        <div class="bg-surface-700/60 h-4 w-1/2 rounded"></div>
        <div class="grid gap-4 md:grid-cols-2">
          {#each Array(4) as _item, index (index)}
            <div class="bg-surface-700/40 h-20 rounded"></div>
          {/each}
        </div>
      </div>
    </div>
  {:else if error}
    <div class="variant-soft-error rounded-container border-error-500/30 border p-6">
      <p class="text-error-200 font-semibold">{error}</p>
      <Button variant="default" class="mt-4" onclick={loadSettings}>
        {m.errors_retry_page()}
      </Button>
    </div>
  {:else if settings}
    {#key `${settings.languageCode}-${settings.currency}-${settings.lengthUnit}-${settings.favoriteScale}-${settings.favoritePowerMethod}`}
      <SettingsForm {settings} {saving} onsubmit={handleSubmit} />
    {/key}
  {/if}
</div>
