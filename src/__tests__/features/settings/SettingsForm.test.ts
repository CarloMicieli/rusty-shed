import { describe, it, expect, vi, beforeEach } from 'vitest';
import { render, fireEvent, screen, waitFor } from '@testing-library/svelte';
import { get, writable, type Writable } from 'svelte/store';
import SettingsForm from '$lib/components/SettingsForm.svelte';
import type { SettingsDto, UpdateSettingsPayload } from '$lib/services';
import * as m from '$lib/paraglide/messages.js';

type FormShape = UpdateSettingsPayload;

const mockSuperFormState = vi.hoisted(() => {
  let latestFormStore: Writable<FormShape> | null = null;
  let currentValid = true;
  let currentErrors: Partial<Record<keyof FormShape, string>> = {};
  let currentTainted: boolean | Record<string, boolean> = false;

  return {
    setValid(value: boolean) {
      currentValid = value;
    },
    setErrors(value: Partial<Record<keyof FormShape, string>>) {
      currentErrors = value;
    },
    setTainted(value: boolean | Record<string, boolean>) {
      currentTainted = value;
    },
    updateForm(updater: (value: FormShape) => FormShape) {
      if (!latestFormStore) throw new Error('superForm store not initialized');
      latestFormStore.update((current) => updater(current));
    },
    createStore(initial: FormShape) {
      latestFormStore = writable({ ...initial });
      return latestFormStore;
    },
    getSnapshot() {
      if (!latestFormStore) throw new Error('superForm store not initialized');
      return get(latestFormStore);
    },
    getValid() {
      return currentValid;
    },
    getErrors() {
      return currentErrors;
    },
    getTainted() {
      return currentTainted;
    },
    reset() {
      latestFormStore = null;
      currentValid = true;
      currentErrors = {};
      currentTainted = false;
    }
  };
});

vi.mock('$lib/paraglide/messages.js', async (importOriginal) => {
  const actual = (await importOriginal()) as Record<string, unknown>;
  return Object.fromEntries(
    Object.entries(actual).map(([key, value]) => [
      key,
      typeof value === 'function' ? () => key : value
    ])
  );
});

vi.mock('sveltekit-superforms/adapters', () => ({
  zodClient: vi.fn((schema: unknown) => schema)
}));

vi.mock('sveltekit-superforms', () => ({
  superForm: vi.fn(
    (
      initial: FormShape,
      options: {
        onUpdate?: (ctx: {
          form: { valid: boolean; data: FormShape; errors: unknown };
        }) => Promise<void> | void;
      }
    ) => {
      const form = mockSuperFormState.createStore(initial);
      const tainted = writable(mockSuperFormState.getTainted());
      const errors = writable(mockSuperFormState.getErrors());

      const enhance = (node: HTMLFormElement) => {
        const onSubmit = async (event: Event) => {
          event.preventDefault();
          tainted.set(mockSuperFormState.getTainted());
          errors.set(mockSuperFormState.getErrors());
          await options.onUpdate?.({
            form: {
              valid: mockSuperFormState.getValid(),
              data: get(form),
              errors: mockSuperFormState.getErrors()
            }
          });
        };

        node.addEventListener('submit', onSubmit);
        return {
          destroy() {
            node.removeEventListener('submit', onSubmit);
          }
        };
      };

      return { form, tainted, errors, enhance };
    }
  )
}));

vi.mock('$lib/components/ui/select', () => ({
  Root: function SelectRoot() {},
  Trigger: function SelectTrigger() {},
  Content: function SelectContent() {},
  Item: function SelectItem() {}
}));

const baseSettings: SettingsDto = {
  currency: 'EUR',
  measureUnit: 'Metric',
  favouriteScale: 'H0',
  powerMethod: 'DC',
  language: 'en',
  theme: 'steampunk-dark',
  has_completed_onboarding: false
};

describe('SettingsForm.svelte', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    mockSuperFormState.reset();
  });

  it('renders with provided settings and default save state', () => {
    const onsubmit = vi.fn();
    render(SettingsForm, { settings: baseSettings, onsubmit });

    expect(screen.getByText(m.settings_title())).toBeInTheDocument();
    expect(screen.getByText(m.settings_subtitle())).toBeInTheDocument();
    expect(screen.getByRole('button', { name: m.save_button() })).toBeEnabled();
    expect(screen.queryByText(m.settings_saving_button())).not.toBeInTheDocument();
  });

  it('calls onsubmit with latest form data when form update is valid', async () => {
    const onsubmit = vi.fn();
    render(SettingsForm, { settings: baseSettings, onsubmit });

    mockSuperFormState.setTainted({ currency: true });
    mockSuperFormState.updateForm((current) => ({ ...current, currency: 'USD' }));

    await fireEvent.click(screen.getByRole('button', { name: m.save_button() }));

    await waitFor(() => {
      expect(onsubmit).toHaveBeenCalledTimes(1);
      expect(onsubmit).toHaveBeenCalledWith({
        measureUnit: 'Metric',
        favouriteScale: 'H0',
        powerMethod: 'DC',
        language: 'en',
        theme: 'steampunk-dark',
        currency: 'USD'
      });
    });
  });

  it('does not call onsubmit when superform reports invalid update', async () => {
    const onsubmit = vi.fn();
    mockSuperFormState.setValid(false);
    mockSuperFormState.setErrors({ currency: 'errors_currency_required' });

    render(SettingsForm, { settings: baseSettings, onsubmit });

    await fireEvent.click(screen.getByRole('button', { name: m.save_button() }));

    await waitFor(() => {
      expect(onsubmit).not.toHaveBeenCalled();
    });
  });

  it('shows saving state and disables submit button when saving is true', () => {
    const onsubmit = vi.fn();
    render(SettingsForm, { settings: baseSettings, onsubmit, saving: true });

    expect(screen.getByRole('button', { name: m.settings_saving_button() })).toBeDisabled();
    expect(screen.queryByRole('button', { name: m.save_button() })).not.toBeInTheDocument();
  });
});
