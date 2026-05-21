<script lang="ts">
  import * as m from '$lib/paraglide/messages.js';
  import * as Select from '$lib/components/ui/select';
  import type { OnboardingConfig } from '../onboarding-state.svelte';

  interface Props {
    config: OnboardingConfig;
    disabled?: boolean;
  }

  let { config, disabled = false }: Props = $props();

  const languageOptions = [
    { value: 'en', label: m.settings_language_option_english() },
    { value: 'it', label: m.settings_language_option_italian() }
  ] as const;

  const selectedLanguageLabel = $derived(
    languageOptions.find((language) => language.value === config.language)?.label ??
      m.settings_language_option_english()
  );
</script>

<section class="space-y-6">
  <div>
    <h2 class="font-bebas text-3xl tracking-widest text-foreground">
      {m.onboarding_step_1_title()}
    </h2>
    <p class="font-mono text-xs text-muted-foreground">{m.onboarding_step_1_hint()}</p>
  </div>

  <div class="space-y-2">
    <p class="font-mono text-[11px] tracking-[0.16em] text-muted-foreground uppercase">
      {m.settings_theme_label()}
    </p>

    <div class="grid gap-3 sm:grid-cols-2">
      <button
        type="button"
        class={[
          'rounded-sm border p-4 text-left transition-all duration-150 ease-out',
          config.theme === 'steampunk-light'
            ? 'border-primary bg-primary/10 text-primary shadow-[0_0_0_3px_hsl(var(--primary)/0.14)]'
            : 'border-border bg-background/40 text-muted-foreground hover:border-primary/40 hover:bg-background/80'
        ]}
        aria-pressed={config.theme === 'steampunk-light'}
        {disabled}
        onclick={() => (config.theme = 'steampunk-light')}
      >
        <div class="font-bebas text-xl tracking-wider">{m.settings_theme_light()}</div>
        <div class="font-mono text-[10px] tracking-wider uppercase opacity-75">
          {m.onboarding_theme_light_caption()}
        </div>
      </button>

      <button
        type="button"
        class={[
          'rounded-sm border p-4 text-left transition-all duration-150 ease-out',
          config.theme === 'steampunk-dark'
            ? 'border-primary bg-primary/10 text-primary shadow-[0_0_0_3px_hsl(var(--primary)/0.14)]'
            : 'border-border bg-background/40 text-muted-foreground hover:border-primary/40 hover:bg-background/80'
        ]}
        aria-pressed={config.theme === 'steampunk-dark'}
        {disabled}
        onclick={() => (config.theme = 'steampunk-dark')}
      >
        <div class="font-bebas text-xl tracking-wider">{m.settings_theme_dark()}</div>
        <div class="font-mono text-[10px] tracking-wider uppercase opacity-75">
          {m.onboarding_theme_dark_caption()}
        </div>
      </button>
    </div>
  </div>

  <label class="block space-y-2">
    <span class="font-mono text-[11px] tracking-[0.16em] text-muted-foreground uppercase">
      {m.settings_language_label()}
    </span>

    <Select.Root
      type="single"
      value={config.language}
      {disabled}
      onValueChange={(nextLanguage) => {
        config.language = nextLanguage as OnboardingConfig['language'];
      }}
    >
      <Select.Trigger
        aria-label={m.settings_language_label()}
        class="h-10 w-full rounded-sm border border-border bg-background px-3 font-mono text-xs text-foreground hover:bg-background/80"
      >
        <span>{selectedLanguageLabel}</span>
      </Select.Trigger>

      <Select.Content
        class="border border-border bg-card font-mono text-xs text-card-foreground shadow-xl"
      >
        {#each languageOptions as language (language.value)}
          <Select.Item value={language.value} label={language.label}>
            {language.label}
          </Select.Item>
        {/each}
      </Select.Content>
    </Select.Root>
  </label>
</section>
