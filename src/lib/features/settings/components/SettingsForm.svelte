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

  // Local state for form inputs
  let currency = $state(settingsState.settings.currency);
  let language = $state(settingsState.settings.language);
  let theme = $state(settingsState.settings.theme);
  let measureUnit = $state(settingsState.settings.measureUnit);
  let favouriteScale = $state(settingsState.settings.favouriteScale);
  let powerSystem = $state(settingsState.settings.powerSystem);

  let saving = $state(false);
  let saveError = $state<string | null>(null);
  let saveSuccess = $state(false);

  // Sync language changes with Paraglide
  $effect(() => {
    setLocale(language);
  });

  // Update local state when settings change
  $effect(() => {
    currency = settingsState.settings.currency;
    language = settingsState.settings.language;
    theme = settingsState.settings.theme;
    measureUnit = settingsState.settings.measureUnit;
    favouriteScale = settingsState.settings.favouriteScale;
    powerSystem = settingsState.settings.powerSystem;
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
        powerSystem
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
        value={powerSystem}
        onchange={(value) => (powerSystem = value)}
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
