<script lang="ts">
  import { untrack } from 'svelte';
  import { Check } from 'lucide-svelte';
  import * as m from '$lib/paraglide/messages.js';
  import { superForm } from 'sveltekit-superforms';
  import { zodClient } from 'sveltekit-superforms/adapters';
  import { settingsSchema } from '$lib/schemas/settings';
  import { Button } from '$lib/components/ui/button';
  import { Badge } from '$lib/components/ui/badge';
  import type {
    SettingsDto,
    UpdateSettingsPayload,
    Currency,
    MeasureUnit,
    PowerMethod,
    Scale,
    ThemeValue
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

  // Capture initial values as snapshot to avoid Svelte 5 reactivity warnings
  const settingsSnapshot = untrack(() => $state.snapshot(initialSettings));

  const formObj = superForm(
    {
      currency: settingsSnapshot.currency,
      measureUnit: settingsSnapshot.measureUnit,
      favouriteScale: settingsSnapshot.favouriteScale,
      powerSystem: settingsSnapshot.powerSystem,
      language: settingsSnapshot.language,
      theme: settingsSnapshot.theme
    },
    {
      SPA: true,
      dataType: 'json',
      // eslint-disable-next-line @typescript-eslint/no-explicit-any
      validators: zodClient(settingsSchema as any),
      onUpdate: async ({ form: formData }) => {
        console.log('[SettingsForm] onUpdate', formData);
        if (formData.valid) {
          console.log('[SettingsForm] form is valid, submitting...');
          onsubmit(formData.data);
        } else {
          console.error('[SettingsForm] form is invalid', formData.errors);
        }
      },
      onError: (err) => {
        console.error('[SettingsForm] superForm error', err);
      }
    }
  );

  const { form, enhance, tainted, errors } = formObj;

  const hasUnsavedChanges = $derived(
    typeof $tainted === 'boolean' ? $tainted : Object.keys($tainted ?? {}).length > 0
  );

  const currencyOptions: { label: string; value: Currency }[] = [
    { label: 'EUR (€)', value: 'EUR' },
    { label: 'USD ($)', value: 'USD' },
    { label: 'GBP (£)', value: 'GBP' },
    { label: 'JPY (¥)', value: 'JPY' }
  ];

  const lengthUnitOptions: { label: string; value: MeasureUnit }[] = [
    { label: m.settings_unit_metric(), value: 'Metric' },
    { label: m.settings_unit_imperial(), value: 'Imperial' }
  ];

  const powerMethodOptions: { label: string; value: PowerMethod; helper?: string }[] = [
    { label: m.settings_power_ac(), value: 'AC', helper: m.settings_power_ac_helper() },
    { label: m.settings_power_dc(), value: 'DC', helper: m.settings_power_dc_helper() },
    {
      label: m.settings_power_dcc(),
      value: 'DCC',
      helper: m.settings_power_dcc_helper()
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

  const themeOptions: { label: string; value: ThemeValue }[] = [
    { label: m.settings_theme_light(), value: 'steampunk-light' },
    { label: m.settings_theme_dark(), value: 'steampunk-dark' }
  ];

  const languageOptions: { label: string; value: 'en' | 'it' }[] = [
    { label: 'English', value: 'en' },
    { label: 'Italiano', value: 'it' }
  ];
</script>

<section class="card border-surface-700/60 bg-surface-900/50 border shadow-xl">
  <header class="border-surface-700/60 flex items-center justify-between gap-4 border-b p-6">
    <div>
      <p class="text-surface-400 text-sm font-semibold tracking-widest uppercase">
        {m.settings_title()}
      </p>
      <h2 class="h3 text-surface-50 font-bold">{m.settings_subtitle()}</h2>
    </div>
    <Badge variant="secondary" class="font-semibold tracking-wide uppercase">
      <Check class="mr-2 h-4 w-4" />
      {m.settings_saved_badge()}
    </Badge>
  </header>

  <form class="space-y-8 p-6" method="POST" use:enhance>
    <div class="grid gap-6 md:grid-cols-2">
      <div class="space-y-2">
        <label class="text-surface-200 text-sm font-semibold tracking-wide" for="currency">
          {m.currency_label()}
        </label>
        <select
          id="currency"
          class="variant-filled-primary-500 select w-full"
          bind:value={$form.currency}
          class:input-error={$errors.currency}
        >
          {#each currencyOptions as option (option.value)}
            <option value={option.value}>{option.label}</option>
          {/each}
        </select>
        {#if $errors.currency}
          <p class="text-error-500 text-xs">{$errors.currency}</p>
        {/if}
      </div>

      <div class="space-y-2">
        <label class="text-surface-200 text-sm font-semibold tracking-wide" for="language">
          {m.settings_language_label()}
        </label>
        <select
          id="language"
          class="variant-filled-primary-500 select w-full"
          bind:value={$form.language}
          class:input-error={$errors.language}
        >
          {#each languageOptions as option (option.value)}
            <option value={option.value}>{option.label}</option>
          {/each}
        </select>
        {#if $errors.language}
          <p class="text-error-500 text-xs">{$errors.language}</p>
        {/if}
      </div>
    </div>

    <div class="grid gap-6 md:grid-cols-2">
      <div class="space-y-2">
        <label class="text-surface-200 text-sm font-semibold tracking-wide" for="length-unit">
          {m.settings_unit_label()}
        </label>
        <select
          id="length-unit"
          class="variant-filled-primary-500 select w-full"
          bind:value={$form.measureUnit}
          class:input-error={$errors.measureUnit}
        >
          {#each lengthUnitOptions as option (option.value)}
            <option value={option.value}>{option.label}</option>
          {/each}
        </select>
        {#if $errors.measureUnit}
          <p class="text-error-500 text-xs">{$errors.measureUnit}</p>
        {/if}
      </div>

      <div class="space-y-2">
        <label class="text-surface-200 text-sm font-semibold tracking-wide" for="theme">
          {m.settings_theme_label()}
        </label>
        <select
          id="theme"
          class="variant-filled-primary-500 select w-full"
          bind:value={$form.theme}
          class:input-error={$errors.theme}
        >
          {#each themeOptions as option (option.value)}
            <option value={option.value}>{option.label}</option>
          {/each}
        </select>
        {#if $errors.theme}
          <p class="text-error-500 text-xs">{$errors.theme}</p>
        {/if}
      </div>
    </div>

    <div class="grid gap-6 md:grid-cols-2">
      <div class="space-y-2">
        <label class="text-surface-200 text-sm font-semibold tracking-wide" for="scale">
          {m.settings_scale_label()}
        </label>
        <select
          id="scale"
          class="variant-filled-primary-500 select w-full"
          bind:value={$form.favouriteScale}
          class:input-error={$errors.favouriteScale}
        >
          {#each scaleOptions as option (option.value)}
            <option value={option.value}>{option.label}</option>
          {/each}
        </select>
        {#if $errors.favouriteScale}
          <p class="text-error-500 text-xs">{$errors.favouriteScale}</p>
        {/if}
      </div>
    </div>

    <div class="space-y-3">
      <p class="text-surface-200 text-sm font-semibold tracking-wide">
        {m.settings_power_label()}
      </p>
      {#if $errors.powerSystem}
        <p class="text-error-500 text-xs">{$errors.powerSystem}</p>
      {/if}
      <div class="grid gap-3" style="grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));">
        {#each powerMethodOptions as option (option.value)}
          <label
            class={[
              'group rounded-container relative flex gap-3 border p-4 transition',
              'bg-surface-800/60 hover:border-primary-400/70 hover:bg-surface-800/90',
              $form.powerSystem === option.value
                ? 'border-primary-400/90 ring-primary-500/30 ring-1'
                : 'border-surface-700/60',
              $errors.powerSystem ? 'border-error-500' : ''
            ]}
          >
            <input
              class="sr-only"
              type="radio"
              name="power-method"
              value={option.value}
              bind:group={$form.powerSystem}
            />

            <span
              aria-hidden="true"
              class={[
                'mt-1 inline-flex h-4 w-4 items-center justify-center rounded-full border transition',
                $form.powerSystem === option.value
                  ? 'border-primary-300 bg-primary-500/30'
                  : 'border-surface-500 bg-surface-900'
              ]}
            >
              <span
                class={[
                  'bg-primary-400 h-2 w-2 rounded-full transition',
                  $form.powerSystem === option.value ? 'opacity-100' : 'opacity-0'
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

    {#if hasUnsavedChanges}
      <p class="text-warning text-sm">You have unsaved changes</p>
    {/if}

    <div class="border-surface-700/60 flex items-center justify-end gap-4 border-t pt-4">
      <Button variant="default" type="submit" disabled={saving}>
        {saving ? m.settings_saving_button() : m.save_button()}
      </Button>
    </div>
  </form>
</section>
