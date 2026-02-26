import { describe, it, expect, vi } from 'vitest';
import { render, screen } from '@testing-library/svelte';
import LocalizedFieldInput from '$lib/features/catalogue/components/LocalizedFieldInput.svelte';

// ── Paraglide messages ───────────────────────────────────────────────────────
vi.mock('$lib/paraglide/messages.js', () => ({
  translation_section_english: () => 'English',
  translation_section_italian: () => 'Italian',
  translation_section_required: () => 'Required',
  translation_section_optional: () => 'Optional'
}));

describe('LocalizedFieldInput.svelte', () => {
  it('renders the label text', () => {
    render(LocalizedFieldInput, {
      props: { lang: 'en', label: 'Description', value: null }
    });
    expect(screen.getByText('Description')).toBeInTheDocument();
  });

  it('shows "English" language badge for lang="en"', () => {
    render(LocalizedFieldInput, {
      props: { lang: 'en', label: 'Description', value: null }
    });
    expect(screen.getByText('English')).toBeInTheDocument();
  });

  it('shows "Italian" language badge for lang="it"', () => {
    render(LocalizedFieldInput, {
      props: { lang: 'it', label: 'Descrizione', value: null }
    });
    expect(screen.getByText('Italian')).toBeInTheDocument();
  });

  it('shows "Required" label when required=true', () => {
    render(LocalizedFieldInput, {
      props: { lang: 'en', label: 'Description', value: null, required: true }
    });
    expect(screen.getByText('Required')).toBeInTheDocument();
  });

  it('shows "Optional" label when required=false (default)', () => {
    render(LocalizedFieldInput, {
      props: { lang: 'en', label: 'Description', value: null }
    });
    expect(screen.getByText('Optional')).toBeInTheDocument();
  });

  it('does not show "Required" when required=false', () => {
    render(LocalizedFieldInput, {
      props: { lang: 'en', label: 'Description', value: null, required: false }
    });
    expect(screen.queryByText('Required')).toBeNull();
  });

  it('renders a textarea element', () => {
    render(LocalizedFieldInput, {
      props: { lang: 'en', label: 'Notes', value: 'some text', placeholder: 'Enter notes...' }
    });
    const textarea = document.querySelector('textarea');
    expect(textarea).not.toBeNull();
  });

  it('renders textarea placeholder', () => {
    render(LocalizedFieldInput, {
      props: { lang: 'en', label: 'Notes', value: null, placeholder: 'Enter notes...' }
    });
    expect(screen.getByPlaceholderText('Enter notes...')).toBeInTheDocument();
  });

  it('uses amber background class for English language badge', () => {
    render(LocalizedFieldInput, {
      props: { lang: 'en', label: 'Description', value: null }
    });
    const badge = screen.getByText('English');
    // The 'en' badge should have bg-amber-500 class via Svelte class: directive
    expect(badge.classList.contains('bg-amber-500')).toBe(true);
  });

  it('uses zinc background class for Italian language badge', () => {
    render(LocalizedFieldInput, {
      props: { lang: 'it', label: 'Descrizione', value: null }
    });
    const badge = screen.getByText('Italian');
    expect(badge.classList.contains('bg-zinc-700')).toBe(true);
  });
});
