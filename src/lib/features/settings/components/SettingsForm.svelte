<script lang="ts">
  import { settingsState } from '../SettingsState.svelte';
  import { setLocale } from '$lib/paraglide/runtime.js';
  import * as m from '$lib/paraglide/messages';
  import { log } from '$lib/tauri-logger';
  import LanguageSelector from './LanguageSelector.svelte';
  import CurrencySelector from './CurrencySelector.svelte';
  import MeasureUnitSelector from './MeasureUnitSelector.svelte';
  import ScaleSelector from './ScaleSelector.svelte';
  import PowerSystemSelector from './PowerSystemSelector.svelte';
  import ThemeSelector from './ThemeSelector.svelte';
  import { Button } from '$lib/components';
  import type { AppTheme, Language, MeasureUnit, PowerMethod } from '$lib/bindings';

  // Local state for form inputs
  let currency = $state(settingsState.settings.currency ?? 'EUR');
  let language = $state<Language>(settingsState.settings.language ?? 'en');
  let theme = $state<AppTheme>(settingsState.settings.theme ?? 'steampunk-dark');
  let measureUnit = $state<MeasureUnit>(settingsState.settings.measureUnit ?? 'Metric');
  let favouriteScale = $state(settingsState.settings.favouriteScale ?? '');
  let powerMethod = $state<PowerMethod>(settingsState.settings.powerMethod ?? 'DC');

  let saving = $state(false);
  let saveError = $state<string | null>(null);
  let saveSuccess = $state(false);

  // Sync language changes with Paraglide
  $effect(() => {
    setLocale(language);
  });

  // Update local state when settings change
  $effect(() => {
    currency = settingsState.settings.currency ?? 'EUR';
    language = settingsState.settings.language ?? 'en';
    theme = settingsState.settings.theme ?? 'steampunk-dark';
    measureUnit = settingsState.settings.measureUnit ?? 'Metric';
    favouriteScale = settingsState.settings.favouriteScale ?? '';
    powerMethod = settingsState.settings.powerMethod ?? 'DC';
  });

  async function handleSubmit(event: Event) {
    event.preventDefault();
    log.debug('SettingsForm: Form submitted');

    saving = true;
    saveError = null;
    saveSuccess = false;

    try {
      const inputData = {
        currency,
        language,
        theme,
        measureUnit,
        favouriteScale,
        powerMethod
      };

      await settingsState.update(inputData);

      const { themeStore } = await import('$lib/stores/themeStore.svelte');
      await themeStore.setTheme(theme);

      log.debug('SettingsForm: Save successful');
      saveSuccess = true;

      // Clear success message after 3 seconds
      setTimeout(() => {
        saveSuccess = false;
      }, 3000);
    } catch (err) {
      saveError = String(err);
      log.error(`SettingsForm: Failed to save settings: ${String(err)}`);
    } finally {
      saving = false;
    }
  }
</script>

<form onsubmit={handleSubmit} class="space-y-6">
  <div class="rounded-lg border border-border bg-card p-6">
    <h2 class="h3 mb-4">{m.settings_heading()}</h2>
    <p class="mb-6 text-muted-foreground">{m.settings_description()}</p>

    <div class="space-y-4">
      <CurrencySelector
        value={currency}
        onchange={(value) => (currency = value)}
        disabled={saving}
      />

      <LanguageSelector
        value={language}
        onchange={(value) => (language = value)}
        disabled={saving}
      />

      <MeasureUnitSelector
        value={measureUnit}
        onchange={(value) => (measureUnit = value)}
        disabled={saving}
      />

      <ThemeSelector value={theme} onchange={(value) => (theme = value)} disabled={saving} />

      <ScaleSelector
        value={favouriteScale}
        onchange={(value) => (favouriteScale = value)}
        disabled={saving}
      />

      <PowerSystemSelector
        value={powerMethod}
        onchange={(value) => (powerMethod = value)}
        disabled={saving}
      />
    </div>

    {#if saveError}
      <div class="mt-4 rounded-md border border-destructive/40 bg-destructive/10 p-3" role="alert">
        <p>{m.settings_update_failed()}: {saveError}</p>
      </div>
    {/if}

    {#if saveSuccess}
      <div class="mt-4 rounded-md border border-emerald-500/40 bg-emerald-500/10 p-3" role="alert">
        <p>{m.settings_saved_toast()}</p>
      </div>
    {/if}

    <div class="mt-6">
      <Button
        type="submit"
        variant="default"
        disabled={saving}
        onclick={() => console.log('[SettingsForm] Button clicked!')}
      >
        {#if saving}
          {m.settings_saving_button()}
        {:else}
          {m.save_button()}
        {/if}
      </Button>
    </div>
  </div>
</form>
