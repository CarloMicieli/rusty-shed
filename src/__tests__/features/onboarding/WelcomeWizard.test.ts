import { describe, it, expect, vi, beforeEach } from 'vitest';
import { fireEvent, render, screen, waitFor } from '@testing-library/svelte';
import WelcomeWizard from '$lib/features/onboarding/WelcomeWizard.svelte';

vi.mock('$lib/paraglide/messages.js', async (importOriginal) => {
  const actual = (await importOriginal()) as Record<string, unknown>;
  return Object.fromEntries(
    Object.entries(actual).map(([k, v]) => [k, typeof v === 'function' ? () => k : v])
  );
});

const mockSaveOnboardingSettings = vi.hoisted(() => vi.fn());
vi.mock('$lib/services/settings', () => ({
  saveOnboardingSettings: mockSaveOnboardingSettings
}));

const mockRunLocalArchiveImport = vi.hoisted(() => vi.fn());
vi.mock('$lib/services/import/localImport', () => ({
  runLocalArchiveImport: mockRunLocalArchiveImport
}));

const mockRunGoogleDriveImport = vi.hoisted(() => vi.fn());
vi.mock('$lib/services/import/googleDriveImport', () => ({
  runGoogleDriveImport: mockRunGoogleDriveImport
}));

describe('WelcomeWizard', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    mockSaveOnboardingSettings.mockResolvedValue({ ok: true, data: {} });
    mockRunLocalArchiveImport.mockResolvedValue({ ok: true });
    mockRunGoogleDriveImport.mockResolvedValue({ ok: true });
  });

  it('supports Step 1 language and theme interactions', async () => {
    render(WelcomeWizard, { onComplete: vi.fn() });

    await fireEvent.change(screen.getByLabelText('settings_language_label'), {
      target: { value: 'it' }
    });

    await fireEvent.click(screen.getByText('settings_theme_light'));
    await fireEvent.click(screen.getByText('onboarding_continue'));

    await waitFor(() => expect(mockSaveOnboardingSettings).toHaveBeenCalled(), { timeout: 2000 });
  });

  it('supports Step 2 keyboard selection flow', async () => {
    render(WelcomeWizard, { onComplete: vi.fn() });

    await fireEvent.click(screen.getByText('onboarding_continue'));
    await waitFor(() => expect(screen.getByText('onboarding_step_2_title')).toBeInTheDocument(), {
      timeout: 2000
    });

    const [scaleGroup] = screen.getAllByRole('radiogroup');
    await fireEvent.keyDown(scaleGroup, { key: 'ArrowRight' });
    await fireEvent.keyDown(scaleGroup, { key: 'ArrowRight' });

    await fireEvent.click(screen.getByText('onboarding_continue'));
    await waitFor(() => expect(screen.getByText('onboarding_step_3_title')).toBeInTheDocument(), {
      timeout: 2000
    });
  });

  it('locks import actions while local import is running', async () => {
    const deferred = new Promise<{ ok: boolean }>((resolve) => {
      setTimeout(() => resolve({ ok: true }), 30);
    });
    mockRunLocalArchiveImport.mockReturnValue(deferred);

    render(WelcomeWizard, { onComplete: vi.fn() });
    await fireEvent.click(screen.getByText('onboarding_continue'));
    await fireEvent.click(screen.getByText('onboarding_continue'));

    const localButton = screen.getByText('onboarding_import_local');
    await fireEvent.click(localButton);

    expect(localButton.closest('button')).toBeDisabled();
    await waitFor(() => expect(mockRunLocalArchiveImport).toHaveBeenCalled(), { timeout: 2000 });
  });

  it('shows inline error banner when Google Drive restore fails', async () => {
    mockRunGoogleDriveImport.mockResolvedValue({ ok: false, error: 'OAuth failed' });

    render(WelcomeWizard, { onComplete: vi.fn() });
    await fireEvent.click(screen.getByText('onboarding_continue'));
    await fireEvent.click(screen.getByText('onboarding_continue'));

    await fireEvent.click(screen.getByText('onboarding_import_drive'));

    await waitFor(() => expect(screen.getByRole('alert')).toHaveTextContent('OAuth failed'), {
      timeout: 2000
    });
  });

  it('applies transition classes to the step panel container', () => {
    const { container } = render(WelcomeWizard, { onComplete: vi.fn() });
    const panel = container.querySelector('section');

    expect(panel).not.toBeNull();
    expect(panel).toHaveClass('transition-transform');
    expect(panel).toHaveClass('duration-150');
    expect(panel).toHaveClass('ease-out');
  });
});
