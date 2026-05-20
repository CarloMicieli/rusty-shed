<script lang="ts">
  import * as m from '$lib/paraglide/messages.js';
  import type { OnboardingConfig } from '../onboarding-state.svelte';

  interface Props {
    config: OnboardingConfig;
    disabled?: boolean;
  }

  let { config, disabled = false }: Props = $props();
</script>

<section class="space-y-6">
  <div>
    <h2 class="text-xl font-semibold">{m.onboarding_step_1_title()}</h2>
    <p class="text-sm text-muted-foreground">{m.onboarding_step_1_hint()}</p>
  </div>

  <div class="grid gap-3 sm:grid-cols-2">
    <button
      type="button"
      class={[
        'rounded-md border p-4 text-left transition-colors',
        config.theme === 'steampunk-light' ? 'border-primary bg-primary/10' : 'border-border'
      ]}
      aria-pressed={config.theme === 'steampunk-light'}
      {disabled}
      onclick={() => (config.theme = 'steampunk-light')}
    >
      <div class="text-sm font-semibold">{m.settings_theme_light()}</div>
    </button>

    <button
      type="button"
      class={[
        'rounded-md border p-4 text-left transition-colors',
        config.theme === 'steampunk-dark' ? 'border-primary bg-primary/10' : 'border-border'
      ]}
      aria-pressed={config.theme === 'steampunk-dark'}
      {disabled}
      onclick={() => (config.theme = 'steampunk-dark')}
    >
      <div class="text-sm font-semibold">{m.settings_theme_dark()}</div>
    </button>
  </div>

  <label class="block space-y-2">
    <span class="text-sm font-medium">{m.settings_language_label()}</span>
    <select
      class="w-full rounded-md border border-border bg-background px-3 py-2"
      bind:value={config.language}
      {disabled}
    >
      <option value="en">English</option>
      <option value="it">Italiano</option>
    </select>
  </label>
</section>
