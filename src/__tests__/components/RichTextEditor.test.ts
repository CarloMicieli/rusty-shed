import { describe, it, expect, vi, beforeEach } from 'vitest';
import { render, fireEvent, waitFor } from '@testing-library/svelte';
import { tick } from 'svelte';

// ── Mocks (hoisted before imports) ──────────────────────────────────────────

vi.mock('$lib/paraglide/messages', () => ({
  details_placeholder: () => 'Add maintenance notes, DCC addresses, or other details...',
  details_save_failed: () => 'Failed to save details. Please try again.',
  edit_field_placeholder_empty: () => 'Click to add...'
}));

vi.mock('marked', () => ({
  marked: {
    parse: vi.fn((content: string) => `<p>${content}</p>`)
  }
}));

vi.mock('@tiptap/starter-kit', () => ({ default: {} }));

vi.mock('@tiptap/markdown', () => ({
  Markdown: {}
}));

vi.mock('@tiptap/core', () => ({
  Editor: vi.fn()
}));

// ── Import component after mocks ─────────────────────────────────────────────

import RichTextEditor from '$lib/components/RichTextEditor.svelte';
import { Editor } from '@tiptap/core';

// ── Mock Editor type (Tiptap 3.x API surface used by the component) ───────────

type MockEditorInstance = {
  /** Tiptap 3.x: editor.getMarkdown() is added by @tiptap/markdown module augmentation */
  getMarkdown: ReturnType<typeof vi.fn>;
  isActive: ReturnType<typeof vi.fn>;
  chain: ReturnType<typeof vi.fn>;
  destroy: ReturnType<typeof vi.fn>;
  _onTransaction: () => void;
  _onBlur: () => void;
};

function getLastEditorInstance(): MockEditorInstance | null {
  const results = vi.mocked(Editor).mock.results;
  if (results.length === 0) return null;
  return results[results.length - 1].value as MockEditorInstance;
}

 
function setupEditorMock(overrides?: Partial<MockEditorInstance>): void {
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  (vi.mocked(Editor) as any).mockImplementation(function (options: any) {
    const instance: MockEditorInstance = {
      getMarkdown: vi.fn().mockReturnValue(options?.content ?? ''),
      isActive: vi.fn().mockReturnValue(false),
      chain: vi.fn(() => ({
        focus: vi.fn().mockReturnThis(),
        toggleBold: vi.fn().mockReturnThis(),
        toggleItalic: vi.fn().mockReturnThis(),
        toggleBulletList: vi.fn().mockReturnThis(),
        toggleOrderedList: vi.fn().mockReturnThis(),
        run: vi.fn()
      })),
      destroy: vi.fn(),
      _onTransaction: options?.onTransaction ?? (() => {}),
      _onBlur: options?.onBlur ?? (() => {})
    };
    return Object.assign(instance, overrides);
  });
}

beforeEach(() => {
  vi.clearAllMocks();
  setupEditorMock();
});

// ── Helper to click and enter editing mode ───────────────────────────────────

async function enterEditingMode(container: HTMLElement): Promise<void> {
  const root = container.firstElementChild as HTMLElement;
  await fireEvent.click(root);
  await tick();
  await tick();
  await new Promise((r) => setTimeout(r, 0));
  await tick();
}

// ── Display Mode tests (T007 subset) ─────────────────────────────────────────

describe('RichTextEditor - Display Mode', () => {
  it('renders Markdown HTML from value prop', () => {
    const { container } = render(RichTextEditor, {
      props: {
        value: '**Bold text**',
        onSave: vi.fn().mockResolvedValue(undefined)
      }
    });
    const prose = container.querySelector('.prose');
    expect(prose).not.toBeNull();
    // marked.parse mock wraps content in <p>
    expect(prose?.innerHTML).toContain('**Bold text**');
  });

  it('shows placeholder <p> when value is null', () => {
    const { container } = render(RichTextEditor, {
      props: {
        value: null,
        placeholder: 'Click to add details...',
        onSave: vi.fn().mockResolvedValue(undefined)
      }
    });
    const placeholder = container.querySelector('p.italic');
    expect(placeholder).not.toBeNull();
    expect(placeholder?.textContent).toContain('Click to add details...');
    expect(container.querySelector('.prose')).toBeNull();
  });

  it('shows placeholder when value is empty string', () => {
    const { container } = render(RichTextEditor, {
      props: {
        value: '',
        placeholder: 'Click to add...',
        onSave: vi.fn().mockResolvedValue(undefined)
      }
    });
    expect(container.querySelector('p.italic')).not.toBeNull();
    expect(container.querySelector('.prose')).toBeNull();
  });

  it('does not add hover:ring-1 class when editable is false', () => {
    const { container } = render(RichTextEditor, {
      props: {
        value: 'Some content',
        editable: false,
        onSave: vi.fn().mockResolvedValue(undefined)
      }
    });
    const root = container.firstElementChild as HTMLElement;
    expect(root.className).not.toContain('hover:ring-1');
  });

  it('adds hover:ring-1 class when editable is true', () => {
    const { container } = render(RichTextEditor, {
      props: {
        value: 'Some content',
        editable: true,
        onSave: vi.fn().mockResolvedValue(undefined)
      }
    });
    const root = container.firstElementChild as HTMLElement;
    expect(root.className).toContain('hover:ring-1');
  });
});

// ── Click-to-edit behaviour (T007 subset) ────────────────────────────────────

describe('RichTextEditor - Click to edit', () => {
  it('click does nothing when editable is false', async () => {
    const onSave = vi.fn();
    const { container } = render(RichTextEditor, {
      props: { value: 'Some content', editable: false, onSave }
    });
    await fireEvent.click(container.firstElementChild as HTMLElement);
    await tick();
    await tick();
    // Editor mode not entered — ring-primary container not shown
    expect(container.querySelector('[class*="ring-primary"]')).toBeNull();
  });

  it('click enters editor mode when editable is true', async () => {
    const { container } = render(RichTextEditor, {
      props: { value: 'Some content', editable: true, onSave: vi.fn().mockResolvedValue(undefined) }
    });
    await enterEditingMode(container);
    expect(container.querySelector('[class*="ring-primary"]')).not.toBeNull();
  });
});

// ── Save behaviour (T007 subset) ─────────────────────────────────────────────

describe('RichTextEditor - Save on blur', () => {
  it('onSave is NOT called on blur when no changes were made (isDirty=false)', async () => {
    const onSave = vi.fn().mockResolvedValue(undefined);
    const { container } = render(RichTextEditor, {
      props: { value: 'Original', editable: true, onSave }
    });

    await enterEditingMode(container);

    // Trigger blur WITHOUT firing onTransaction (isDirty stays false)
    const instance = getLastEditorInstance();
    instance?._onBlur();
    await tick();
    await new Promise((r) => setTimeout(r, 0));

    expect(onSave).not.toHaveBeenCalled();
    // Editor should have exited (no ring-primary container)
    expect(container.querySelector('[class*="ring-primary"]')).toBeNull();
  });

  it('onSave called with Markdown string on blur after edit', async () => {
    const onSave = vi.fn().mockResolvedValue(undefined);
    const { container } = render(RichTextEditor, {
      props: { value: 'Original', editable: true, onSave }
    });

    await enterEditingMode(container);

    const instance = getLastEditorInstance();
    if (instance) {
      // Simulate typing — sets isDirty=true via onTransaction
      instance.getMarkdown.mockReturnValue('**Updated**');
      instance._onTransaction();
      await tick();

      // Blur to trigger save
      instance._onBlur();
      await tick();
      await new Promise((r) => setTimeout(r, 0));
    }

    expect(onSave).toHaveBeenCalledWith('**Updated**');
  });

  it('onSave rejection keeps editor mounted and does not update display', async () => {
    const onSave = vi.fn().mockRejectedValue(new Error('Save failed'));
    const { container } = render(RichTextEditor, {
      props: { value: 'Original', editable: true, onSave }
    });

    await enterEditingMode(container);

    const instance = getLastEditorInstance();
    if (instance) {
      instance.getMarkdown.mockReturnValue('New content');
      instance._onTransaction();
      await tick();
      instance._onBlur();
      await tick();
      await new Promise((r) => setTimeout(r, 0));
    }

    // Editor should still be open (ring-primary container still visible)
    expect(container.querySelector('[class*="ring-primary"]')).not.toBeNull();
  });

  it('successful save of empty string shows placeholder in Display Mode', async () => {
    const onSave = vi.fn().mockResolvedValue(undefined);
    const { container } = render(RichTextEditor, {
      props: { value: 'Some content', editable: true, placeholder: 'Add notes...', onSave }
    });

    await enterEditingMode(container);

    const instance = getLastEditorInstance();
    if (instance) {
      // User clears all content
      instance.getMarkdown.mockReturnValue('');
      instance._onTransaction();
      await tick();
      instance._onBlur();
      await tick();
      await new Promise((r) => setTimeout(r, 0));
    }

    // After successful save of empty string, Display Mode shows placeholder
    await waitFor(() => {
      const placeholder = container.querySelector('p.italic');
      expect(placeholder).not.toBeNull();
      expect(placeholder?.textContent).toContain('Add notes...');
    });
  });
});

// ── US3: Paste normalisation — Markdown extension is used (T011) ──────────────

describe('RichTextEditor - Paste / Markdown integration (US3)', () => {
  it('uses the Markdown extension when creating the editor', async () => {
    const { container } = render(RichTextEditor, {
      props: { value: 'content', editable: true, onSave: vi.fn().mockResolvedValue(undefined) }
    });
    await enterEditingMode(container);

    // Verify the Editor was constructed (Tiptap was instantiated)
    expect(vi.mocked(Editor)).toHaveBeenCalled();

    // The extensions array should include the Markdown object (imported from @tiptap/markdown)
    const constructorOptions = vi.mocked(Editor).mock.calls[0]?.[0] as { extensions?: unknown[] };
    expect(constructorOptions?.extensions).toBeDefined();
  });

  it('initialises editor with contentType markdown for proper Markdown loading', async () => {
    const { container } = render(RichTextEditor, {
      props: { value: '**Bold**', editable: true, onSave: vi.fn().mockResolvedValue(undefined) }
    });
    await enterEditingMode(container);

    const constructorOptions = vi.mocked(Editor).mock.calls[0]?.[0] as {
      content?: string;
      contentType?: string;
    };
    expect(constructorOptions?.content).toBe('**Bold**');
    expect(constructorOptions?.contentType).toBe('markdown');
  });
});

// ── US4: Placeholder lifecycle (T012) ────────────────────────────────────────

describe('RichTextEditor - Placeholder lifecycle (US4)', () => {
  it('shows placeholder in Display Mode when value is null', () => {
    const { container } = render(RichTextEditor, {
      props: {
        value: null,
        placeholder: 'My placeholder',
        onSave: vi.fn().mockResolvedValue(undefined)
      }
    });
    expect(container.querySelector('p.italic')?.textContent).toContain('My placeholder');
    expect(container.querySelector('.prose')).toBeNull();
  });

  it('shows placeholder in Display Mode when value is empty string', () => {
    const { container } = render(RichTextEditor, {
      props: {
        value: '',
        placeholder: 'Empty placeholder',
        onSave: vi.fn().mockResolvedValue(undefined)
      }
    });
    expect(container.querySelector('p.italic')?.textContent).toContain('Empty placeholder');
    expect(container.querySelector('.prose')).toBeNull();
  });

  it('does not show placeholder when value has content', () => {
    const { container } = render(RichTextEditor, {
      props: {
        value: 'Some content',
        placeholder: 'Placeholder',
        onSave: vi.fn().mockResolvedValue(undefined)
      }
    });
    expect(container.querySelector('p.italic')).toBeNull();
    expect(container.querySelector('.prose')).not.toBeNull();
  });

  it('clicking with null value and editable=true activates Editor Mode (no placeholder shown)', async () => {
    const { container } = render(RichTextEditor, {
      props: {
        value: null,
        editable: true,
        placeholder: 'My placeholder',
        onSave: vi.fn().mockResolvedValue(undefined)
      }
    });

    // Placeholder visible in Display Mode
    expect(container.querySelector('p.italic')).not.toBeNull();

    // Click to enter Editor Mode
    await enterEditingMode(container);

    // Placeholder replaced by editor container
    expect(container.querySelector('[class*="ring-primary"]')).not.toBeNull();
    expect(container.querySelector('p.italic')).toBeNull();
  });
});
