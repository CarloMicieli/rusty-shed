<script lang="ts">
  import { Button } from '$lib/components/ui/button';
  import { Card } from '$lib/components/ui/card';
  import * as m from '$lib/paraglide/messages.js';
  import {
    onboardingDefaultConfig,
    createOnboardingFlowState,
    nextStep,
    previousStep,
    type OnboardingConfig
  } from './onboarding-state.svelte';
  import { saveOnboardingSettings } from '$lib/services/settings';
  import BasicsStep from './steps/BasicsStep.svelte';
  import ModelingStep from './steps/ModelingStep.svelte';
  import ImportStep from './steps/ImportStep.svelte';
  import { runLocalArchiveImport } from '$lib/services/import/localImport';

  let { onComplete }: { onComplete: () => void } = $props();

  let config = $state<OnboardingConfig>({ ...onboardingDefaultConfig });
  let flow = $state(createOnboardingFlowState());
  let stepMetaLine = $derived(
    flow.currentStep === 1
      ? m.onboarding_step_1_meta()
      : flow.currentStep === 2
        ? m.onboarding_step_2_meta()
        : m.onboarding_step_3_meta()
  );

  async function handleContinue() {
    if (flow.isBusy) return;

    if (flow.currentStep === 1 || flow.currentStep === 2) {
      flow.isBusy = true;
      flow.errorMessage = null;
      const result = await saveOnboardingSettings(config);
      flow.isBusy = false;

      if (!result.ok) {
        flow.errorMessage = result.error.message;
        return;
      }
    }

    nextStep(flow, config);
  }

  function handleBack() {
    previousStep(flow);
  }

  async function handleLocalImport() {
    flow.isBusy = true;
    flow.errorMessage = null;
    const result = await runLocalArchiveImport();
    flow.isBusy = false;

    if (!result.ok) {
      flow.errorMessage = result.error ?? m.onboarding_import_local_failed();
      return;
    }

    onComplete();
  }

  async function handleCloudRestoreCompleted() {
    onComplete();
  }

  async function handleSkipAndStartFresh() {
    onComplete();
  }

  function handleWizardKeydown(event: KeyboardEvent): void {
    if (event.key !== 'Enter' || flow.currentStep >= 3 || flow.isBusy) {
      return;
    }

    event.preventDefault();
    void handleContinue();
  }
</script>

<div
  class="fixed inset-0 z-50 flex items-center justify-center bg-background p-4 sm:p-6"
  role="dialog"
  aria-modal="true"
  tabindex="0"
  onkeydown={handleWizardKeydown}
>
  <Card
    class="variant-steampunk-riveted relative w-full max-w-3xl overflow-hidden border border-border bg-card px-5 pt-12 pb-6 shadow-[0_25px_50px_-12px_hsl(var(--foreground)/0.45)] sm:px-8 sm:pt-14 sm:pb-8"
  >
    <div
      class="pointer-events-none absolute top-0 right-0 left-0 h-[2px] bg-gradient-to-r from-transparent via-primary to-transparent opacity-75"
      aria-hidden="true"
    ></div>

    <div
      class="pointer-events-none absolute top-2 right-3 z-10 sm:top-3 sm:right-4"
      aria-hidden="true"
    >
      <div class="w-20 opacity-95 drop-shadow-[0_10px_18px_hsl(var(--foreground)/0.35)] sm:w-24">
        <img src="/logo.png" alt="" class="h-auto w-full object-contain" />
      </div>
    </div>

    <header class="mb-6 border-b border-border/50 pb-4">
      <h1 class="font-bebas text-3xl tracking-widest text-foreground sm:text-4xl">
        {m.onboarding_title()}
      </h1>
      <p class="font-mono text-[11px] tracking-[0.22em] text-muted-foreground uppercase">
        {stepMetaLine}
      </p>
    </header>

    <div class="mb-8 flex items-center gap-2 sm:gap-3">
      {#each [1, 2, 3] as step (step)}
        <div
          class={[
            'h-2 flex-1 rounded-full border transition-all duration-150',
            flow.currentStep >= step
              ? 'border-primary bg-primary shadow-[0_0_0_3px_hsl(var(--primary)/0.16)]'
              : 'border-border bg-background'
          ]}
        ></div>
      {/each}
    </div>

    <section class="min-h-[220px] transition-transform duration-150 ease-out">
      {#if flow.currentStep === 1}
        <BasicsStep {config} disabled={flow.isBusy} />
      {:else if flow.currentStep === 2}
        <ModelingStep {config} disabled={flow.isBusy} />
      {:else}
        <ImportStep
          isBusy={flow.isBusy}
          errorMessage={flow.errorMessage}
          onLocalImport={handleLocalImport}
          onCloudRestoreCompleted={handleCloudRestoreCompleted}
          onSkipAndStartFresh={handleSkipAndStartFresh}
        />
      {/if}
    </section>

    <footer class="mt-8 flex items-center justify-between border-t border-border pt-6">
      <Button variant="ghost" onclick={handleBack} disabled={flow.currentStep === 1 || flow.isBusy}>
        {m.onboarding_back()}
      </Button>

      {#if flow.currentStep < 3}
        <Button onclick={handleContinue} disabled={flow.isBusy}>{m.onboarding_continue()}</Button>
      {/if}
    </footer>
  </Card>
</div>
