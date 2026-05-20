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
  import { runGoogleDriveImport } from '$lib/services/import/googleDriveImport';

  let { onComplete }: { onComplete: () => void } = $props();

  let config = $state<OnboardingConfig>({ ...onboardingDefaultConfig });
  let flow = $state(createOnboardingFlowState());

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

  async function handleGoogleDriveImport() {
    flow.isBusy = true;
    flow.errorMessage = null;
    const result = await runGoogleDriveImport();
    flow.isBusy = false;

    if (!result.ok) {
      flow.errorMessage = result.error ?? m.onboarding_import_drive_failed();
      return;
    }

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
  class="fixed inset-0 z-50 flex items-center justify-center bg-background p-6"
  role="dialog"
  aria-modal="true"
  tabindex="0"
  onkeydown={handleWizardKeydown}
>
  <Card class="w-full max-w-3xl border border-border bg-card p-8 shadow-xl">
    <header class="mb-6">
      <h1 class="text-2xl font-bold tracking-tight">{m.onboarding_title()}</h1>
    </header>

    <div class="mb-8 flex items-center gap-3">
      {#each [1, 2, 3] as step}
        <div
          class={[
            'h-2 flex-1 rounded-full transition-all duration-150',
            flow.currentStep >= step ? 'bg-primary' : 'bg-muted'
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
          onGoogleDriveImport={handleGoogleDriveImport}
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
