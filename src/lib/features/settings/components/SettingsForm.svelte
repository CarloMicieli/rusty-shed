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

  interface SettingsDraft {
    currency: string;
    language: Language;
    theme: AppTheme;
    measureUnit: MeasureUnit;
    favouriteScale: string;
    powerMethod: PowerMethod;
  }

  let draft = $state(createSettingsDraft(settingsState.settings));
  let previousSettingsDraft: SettingsDraft | null = null;

  let saving = $state(false);
  let saveError = $state<string | null>(null);
  let saveSuccess = $state(false);

  // Sync language changes with Paraglide
  $effect(() => {
    setLocale(draft.language);
  });

  // Reconcile upstream settings changes without mirroring each field separately.
  $effect(() => {
    const nextSettings = createSettingsDraft(settingsState.settings);
    const previousSettings = previousSettingsDraft;

    if (previousSettings) {
      if (previousSettings.currency !== nextSettings.currency)
        draft.currency = nextSettings.currency;
      if (previousSettings.language !== nextSettings.language)
        draft.language = nextSettings.language;
      if (previousSettings.theme !== nextSettings.theme) draft.theme = nextSettings.theme;
      if (previousSettings.measureUnit !== nextSettings.measureUnit)
        draft.measureUnit = nextSettings.measureUnit;
      if (previousSettings.favouriteScale !== nextSettings.favouriteScale)
        draft.favouriteScale = nextSettings.favouriteScale;
      if (previousSettings.powerMethod !== nextSettings.powerMethod)
        draft.powerMethod = nextSettings.powerMethod;
    }

    previousSettingsDraft = nextSettings;
  });

  async function handleSubmit(event: Event) {
    event.preventDefault();
    log.debug('SettingsForm: Form submitted');

    saving = true;
    saveError = null;
    saveSuccess = false;

    try {
      const inputData = {
        currency: draft.currency,
        language: draft.language,
        theme: draft.theme,
        measureUnit: draft.measureUnit,
        favouriteScale: draft.favouriteScale,
        powerMethod: draft.powerMethod
      };

      await settingsState.update(inputData);

      const { themeState } = await import('$lib/stores/themeStore.svelte');
      await themeState.setTheme(draft.theme);

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

  function createSettingsDraft(settings: typeof settingsState.settings): SettingsDraft {
    return {
      currency: settings.currency ?? 'EUR',
      language: settings.language ?? 'en',
      theme: settings.theme ?? 'steampunk-dark',
      measureUnit: settings.measureUnit ?? 'Metric',
      favouriteScale: settings.favouriteScale ?? '',
      powerMethod: settings.powerMethod ?? 'DC'
    };
  }
</script>

<form onsubmit={handleSubmit} class="space-y-6">
  <div class="rounded-lg border border-border bg-card p-6">
    <h2 class="h3 mb-4">{m.settings_heading()}</h2>
    <p class="mb-6 text-muted-foreground">{m.settings_description()}</p>

    <div class="space-y-4">
      <CurrencySelector
        value={draft.currency}
        onchange={(value) => (draft.currency = value)}
        disabled={saving}
      />

      <LanguageSelector
        value={draft.language}
        onchange={(value) => (draft.language = value)}
        disabled={saving}
      />

      <MeasureUnitSelector
        value={draft.measureUnit}
        onchange={(value) => (draft.measureUnit = value)}
        disabled={saving}
      />

      <ThemeSelector
        value={draft.theme}
        onchange={(value) => (draft.theme = value)}
        disabled={saving}
      />

      <ScaleSelector
        value={draft.favouriteScale}
        onchange={(value) => (draft.favouriteScale = value)}
        disabled={saving}
      />

      <PowerSystemSelector
        value={draft.powerMethod}
        onchange={(value) => (draft.powerMethod = value)}
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
