<script lang="ts">
  import * as m from '$lib/paraglide/messages.js';
  import type { OnboardingConfig } from '../onboarding-state.svelte';

  interface Props {
    config: OnboardingConfig;
    disabled?: boolean;
  }

  const scales = ['H0', 'N', 'TT', 'Z', 'G', '00'] as const;
  const powerMethods = ['DC', 'AC', 'TRIX_EXPRESS'] as const;

  let { config, disabled = false }: Props = $props();

  function moveScale(direction: 1 | -1): void {
    const index = scales.indexOf(config.favouriteScale as (typeof scales)[number]);
    const safeIndex = index >= 0 ? index : 0;
    const next = Math.max(0, Math.min(scales.length - 1, safeIndex + direction));
    config.favouriteScale = scales[next];
  }

  function movePower(direction: 1 | -1): void {
    const index = powerMethods.indexOf(config.powerMethod as (typeof powerMethods)[number]);
    const safeIndex = index >= 0 ? index : 0;
    const next = Math.max(0, Math.min(powerMethods.length - 1, safeIndex + direction));
    config.powerMethod = powerMethods[next];
  }

  function handleScaleKey(event: KeyboardEvent): void {
    if (disabled) return;
    if (event.key === 'ArrowRight') {
      event.preventDefault();
      moveScale(1);
    }
    if (event.key === 'ArrowLeft') {
      event.preventDefault();
      moveScale(-1);
    }
  }

  function handlePowerKey(event: KeyboardEvent): void {
    if (disabled) return;
    if (event.key === 'ArrowRight') {
      event.preventDefault();
      movePower(1);
    }
    if (event.key === 'ArrowLeft') {
      event.preventDefault();
      movePower(-1);
    }
  }
</script>

<section class="space-y-6">
  <div>
    <h2 class="text-xl font-semibold">{m.onboarding_step_2_title()}</h2>
    <p class="text-sm text-muted-foreground">{m.onboarding_step_2_hint()}</p>
  </div>

  <div class="space-y-2" onkeydown={handleScaleKey} tabindex="0" role="radiogroup">
    <span class="text-sm font-medium">{m.settings_scale_label()}</span>
    <div class="grid grid-cols-3 gap-2">
      {#each scales as scale (scale)}
        <button
          type="button"
          class={[
            'rounded-md border px-3 py-2 text-sm',
            config.favouriteScale === scale ? 'border-primary bg-primary/10' : 'border-border'
          ]}
          aria-pressed={config.favouriteScale === scale}
          {disabled}
          onclick={() => (config.favouriteScale = scale)}
        >
          {scale}
        </button>
      {/each}
    </div>
  </div>

  <div class="space-y-2">
    <span class="text-sm font-medium">{m.settings_unit_label()}</span>
    <div class="grid grid-cols-2 gap-2">
      <button
        type="button"
        class={[
          'rounded-md border px-3 py-2 text-sm',
          config.measureUnit === 'Metric' ? 'border-primary bg-primary/10' : 'border-border'
        ]}
        aria-pressed={config.measureUnit === 'Metric'}
        {disabled}
        onclick={() => (config.measureUnit = 'Metric')}
      >
        {m.settings_unit_metric()}
      </button>
      <button
        type="button"
        class={[
          'rounded-md border px-3 py-2 text-sm',
          config.measureUnit === 'Imperial' ? 'border-primary bg-primary/10' : 'border-border'
        ]}
        aria-pressed={config.measureUnit === 'Imperial'}
        {disabled}
        onclick={() => (config.measureUnit = 'Imperial')}
      >
        {m.settings_unit_imperial()}
      </button>
    </div>
  </div>

  <div class="space-y-2" onkeydown={handlePowerKey} tabindex="0" role="radiogroup">
    <span class="text-sm font-medium">{m.settings_power_label()}</span>
    <div class="grid grid-cols-3 gap-2">
      {#each powerMethods as method (method)}
        <button
          type="button"
          class={[
            'rounded-md border px-3 py-2 text-sm',
            config.powerMethod === method ? 'border-primary bg-primary/10' : 'border-border'
          ]}
          aria-pressed={config.powerMethod === method}
          {disabled}
          onclick={() => (config.powerMethod = method)}
        >
          {method === 'TRIX_EXPRESS' ? 'TRIX' : method}
        </button>
      {/each}
    </div>
  </div>
</section>
