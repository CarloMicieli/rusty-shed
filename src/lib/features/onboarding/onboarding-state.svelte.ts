import type { Language, MeasureUnit, PowerMethod } from '$lib/bindings';
import type { ThemeValue, Scale } from '$lib/services/settings';

export interface OnboardingConfig {
  language: Language;
  theme: ThemeValue;
  measureUnit: MeasureUnit;
  favouriteScale: Scale;
  powerMethod: PowerMethod;
}

export interface OnboardingFlowState {
  currentStep: 1 | 2 | 3;
  isBusy: boolean;
  errorMessage: string | null;
}

export interface OnboardingRuntimeState {
  needsOnboarding: boolean;
  isChecking: boolean;
}

export const onboardingDefaultConfig: OnboardingConfig = {
  language: 'en',
  theme: 'steampunk-dark',
  measureUnit: 'Metric',
  favouriteScale: 'H0',
  powerMethod: 'DC'
};

export function createOnboardingFlowState(): OnboardingFlowState {
  return {
    currentStep: 1,
    isBusy: false,
    errorMessage: null
  };
}

export function createOnboardingRuntimeState(): OnboardingRuntimeState {
  return {
    needsOnboarding: false,
    isChecking: true
  };
}

export function clampStep(step: number): 1 | 2 | 3 {
  if (step <= 1) return 1;
  if (step >= 3) return 3;
  return step as 1 | 2 | 3;
}

export function canAdvance(step: 1 | 2 | 3, config: OnboardingConfig): boolean {
  if (step === 1) {
    return config.language.length > 0 && config.theme.length > 0;
  }

  if (step === 2) {
    return (
      config.favouriteScale.length > 0 &&
      config.measureUnit.length > 0 &&
      config.powerMethod.length > 0
    );
  }

  return true;
}

export function nextStep(state: OnboardingFlowState, config: OnboardingConfig): void {
  if (!canAdvance(state.currentStep, config)) return;
  state.currentStep = clampStep(state.currentStep + 1);
}

export function previousStep(state: OnboardingFlowState): void {
  state.currentStep = clampStep(state.currentStep - 1);
}

export function bootstrapNeedsOnboarding(settings: {
  has_completed_onboarding?: boolean | null;
}): boolean {
  return !settings.has_completed_onboarding;
}

export async function completeOnboardingStatus(settingsState: {
  markOnboardingCompleted: () => Promise<void>;
}): Promise<void> {
  await settingsState.markOnboardingCompleted();
}
