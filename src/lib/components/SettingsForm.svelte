<script lang="ts">
  import { Check } from 'lucide-svelte';
  import * as m from '$lib/paraglide/messages.js';
  import type {
    SettingsDto,
    UpdateSettingsPayload,
    Currency,
    MeasureUnit,
    PowerMethod,
    Scale,
    LanguageCode
  } from '$lib/services';

  let {
    settings: initialSettings,
    saving = false,
    onsubmit
  } = $props<{
    settings: SettingsDto;
    saving?: boolean;
    onsubmit: (payload: UpdateSettingsPayload) => void;
  }>();

  const initialForm = $derived.by(
    (): UpdateSettingsPayload => ({
      currency: initialSettings.currency,
      lengthUnit: initialSettings.lengthUnit,
      favoriteScale: initialSettings.favoriteScale,
      favoritePowerMethod: initialSettings.favoritePowerMethod,
      languageCode: initialSettings.languageCode
    })
  );

  let form = $state<UpdateSettingsPayload>({
    currency: 'EUR',
    lengthUnit: 'MILLIMETERS',
    favoriteScale: 'H0',
    favoritePowerMethod: 'AC',
    languageCode: 'en'
  });

  $effect(() => {
    const next = initialForm;
    form.currency = next.currency;
    form.lengthUnit = next.lengthUnit;
    form.favoriteScale = next.favoriteScale;
    form.favoritePowerMethod = next.favoritePowerMethod;
    form.languageCode = next.languageCode;
  });

  const currencyOptions: { label: string; value: Currency }[] = [
    { label: 'EUR (€)', value: 'EUR' },
    { label: 'USD ($)', value: 'USD' },
    { label: 'GBP (£)', value: 'GBP' },
    { label: 'JPY (¥)', value: 'JPY' }
  ];

  const lengthUnitOptions: { label: string; value: MeasureUnit }[] = [
    { label: m.settings_unit_metric(), value: 'MILLIMETERS' },
    { label: m.settings_unit_imperial(), value: 'INCHES' }
  ];

  const powerMethodOptions: { label: string; value: PowerMethod; helper?: string }[] = [
    { label: m.settings_power_ac(), value: 'AC', helper: m.settings_power_ac_helper() },
    { label: m.settings_power_dc(), value: 'DC', helper: m.settings_power_dc_helper() },
    {
      label: m.settings_power_trix(),
      value: 'TRIX_EXPRESS',
      helper: m.settings_power_trix_helper()
    }
  ];

  const scaleOptions: { label: string; value: Scale }[] = [
    { label: 'H0', value: 'H0' },
    { label: 'N', value: 'N' },
    { label: 'TT', value: 'TT' },
    { label: 'Z', value: 'Z' },
    { label: 'G', value: 'G' },
    { label: '0', value: '0' },
    { label: '00', value: '00' },
    { label: '1', value: '1' },
    { label: 'H0m', value: 'H0m' },
    { label: 'H0e', value: 'H0e' }
  ];

  const languageOptions: { label: string; value: LanguageCode }[] = [
    { label: 'English', value: 'en' },
    { label: 'Italiano', value: 'it' }
  ];

  function handleSubmit(event: Event) {
    event.preventDefault();
    onsubmit(form);
  }
</script>

<section class="card border-surface-700/60 bg-surface-900/50 border shadow-xl">
  <header class="border-surface-700/60 flex items-center justify-between gap-4 border-b p-6">
    <div>
      <p class="text-surface-400 text-sm font-semibold tracking-widest uppercase">
        {m.settings_title()}
      </p>
      <h2 class="h3 text-surface-50 font-bold">{m.settings_subtitle()}</h2>
    </div>
    <div class="variant-soft-primary badge font-semibold tracking-wide uppercase">
      <Check class="mr-2 h-4 w-4" />
      {m.settings_saved_badge()}
    </div>
  </header>

  <form class="space-y-8 p-6" onsubmit={handleSubmit}>
    <div class="grid gap-6 md:grid-cols-2">
      <div class="space-y-2">
        <label class="text-sm font-semibold tracking-wide text-surface-200" for="currency">
          {m.currency_label()}
        </label>
        <select
          id="currency"
          class="variant-filled-primary-500 select w-full"
          bind:value={form.currency}
        >
          {#each currencyOptions as option (option.value)}
            <option value={option.value}>{option.label}</option>
          {/each}
        </select>
      </div>

      <div class="space-y-2">
        <label class="text-sm font-semibold tracking-wide text-surface-200" for="language">
          {m.settings_language_label()}
        </label>
        <select
          id="language"
          class="variant-filled-primary-500 select w-full"
          bind:value={form.languageCode}
        >
          {#each languageOptions as option (option.value)}
            <option value={option.value}>{option.label}</option>
          {/each}
        </select>
      </div>
    </div>

    <div class="grid gap-6 md:grid-cols-2">
      <div class="space-y-2">
        <label class="text-sm font-semibold tracking-wide text-surface-200" for="length-unit">
          {m.settings_unit_label()}
        </label>
        <select
          id="length-unit"
          class="variant-filled-primary-500 select w-full"
          bind:value={form.lengthUnit}
        >
          {#each lengthUnitOptions as option (option.value)}
            <option value={option.value}>{option.label}</option>
          {/each}
        </select>
      </div>

      <div class="space-y-2">
        <label class="text-sm font-semibold tracking-wide text-surface-200" for="scale">
          {m.settings_scale_label()}
        </label>
        <select
          id="scale"
          class="variant-filled-primary-500 select w-full"
          bind:value={form.favoriteScale}
        >
          {#each scaleOptions as option (option.value)}
            <option value={option.value}>{option.label}</option>
          {/each}
        </select>
      </div>
    </div>

    <div class="space-y-3">
      <p class="text-sm font-semibold tracking-wide text-surface-200">
        {m.settings_power_label()}
      </p>
      <div class="grid gap-3" style="grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));">
        {#each powerMethodOptions as option (option.value)}
          <label
            class={[
              'group rounded-container relative flex gap-3 border p-4 transition',
              'bg-surface-800/60 hover:border-primary-400/70 hover:bg-surface-800/90',
              form.favoritePowerMethod === option.value
                ? 'border-primary-400/90 ring-primary-500/30 ring-1'
                : 'border-surface-700/60'
            ]}
          >
            <input
              class="sr-only"
              type="radio"
              name="power-method"
              value={option.value}
              bind:group={form.favoritePowerMethod}
            />

            <span
              aria-hidden="true"
              class={[
                'mt-1 inline-flex h-4 w-4 items-center justify-center rounded-full border transition',
                form.favoritePowerMethod === option.value
                  ? 'border-primary-300 bg-primary-500/30'
                  : 'border-surface-500 bg-surface-900'
              ]}
            >
              <span
                class={[
                  'bg-primary-400 h-2 w-2 rounded-full transition',
                  form.favoritePowerMethod === option.value ? 'opacity-100' : 'opacity-0'
                ]}
              ></span>
            </span>

            <div class="space-y-1">
              <p class="text-surface-50 text-sm font-semibold">{option.label}</p>
              {#if option.helper}
                <p class="text-surface-400 text-xs leading-relaxed">{option.helper}</p>
              {/if}
            </div>
          </label>
        {/each}
      </div>
    </div>

    <div class="border-surface-700/60 flex items-center justify-end gap-4 border-t pt-4">
      <button class="variant-filled-primary btn" type="submit" disabled={saving}>
        {saving ? m.settings_saving_button() : m.save_button()}
      </button>
    </div>
  </form>
</section>
