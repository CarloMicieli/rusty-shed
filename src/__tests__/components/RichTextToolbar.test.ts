import { describe, it, expect, vi } from 'vitest';
import { render, fireEvent } from '@testing-library/svelte';
import type { Editor } from '@tiptap/core';

import RichTextToolbar from '$lib/components/RichTextToolbar.svelte';

// ── Mock editor builder ───────────────────────────────────────────────────────

type ChainResult = {
  focus: ReturnType<typeof vi.fn>;
  toggleBold: ReturnType<typeof vi.fn>;
  toggleItalic: ReturnType<typeof vi.fn>;
  toggleBulletList: ReturnType<typeof vi.fn>;
  toggleOrderedList: ReturnType<typeof vi.fn>;
  run: ReturnType<typeof vi.fn>;
};

type MockEditor = {
  isActive: ReturnType<typeof vi.fn>;
  chain: ReturnType<typeof vi.fn>;
  _chainResult: ChainResult;
};

/**
 * Build a minimal mock editor that satisfies the API used by RichTextToolbar:
 *   editor?.isActive(type)
 *   editor?.chain().focus().toggleXxx().run()
 */
function buildMockEditor(
  activeStates: Partial<Record<'bold' | 'italic' | 'bulletList' | 'orderedList', boolean>> = {}
): MockEditor {
  const chainResult: ChainResult = {
    focus: vi.fn().mockReturnThis(),
    toggleBold: vi.fn().mockReturnThis(),
    toggleItalic: vi.fn().mockReturnThis(),
    toggleBulletList: vi.fn().mockReturnThis(),
    toggleOrderedList: vi.fn().mockReturnThis(),
    run: vi.fn()
  };

  return {
    isActive: vi.fn((type: string) => activeStates[type as keyof typeof activeStates] ?? false),
    chain: vi.fn(() => chainResult),
    _chainResult: chainResult
  };
}

/** Cast MockEditor to Editor for component prop usage */
function asEditor(mock: MockEditor): Editor {
  return mock as unknown as Editor;
}

// ── Render tests ─────────────────────────────────────────────────────────────

describe('RichTextToolbar - Render', () => {
  it('renders all four formatting buttons', () => {
    const mock = buildMockEditor();
    const { container } = render(RichTextToolbar, { props: { editor: asEditor(mock) } });
    expect(container.querySelectorAll('button').length).toBe(4);
  });

  it('all buttons have aria-label attributes', () => {
    const mock = buildMockEditor();
    const { container } = render(RichTextToolbar, { props: { editor: asEditor(mock) } });
    const buttons = Array.from(container.querySelectorAll('button'));
    for (const btn of buttons) {
      expect(btn.getAttribute('aria-label')).not.toBeNull();
      expect(btn.getAttribute('aria-label')!.length).toBeGreaterThan(0);
    }
  });

  it('renders Bold button with aria-label "Bold"', () => {
    const mock = buildMockEditor();
    const { getByRole } = render(RichTextToolbar, { props: { editor: asEditor(mock) } });
    expect(getByRole('button', { name: 'Bold' })).not.toBeNull();
  });

  it('renders Italic button with aria-label "Italic"', () => {
    const mock = buildMockEditor();
    const { getByRole } = render(RichTextToolbar, { props: { editor: asEditor(mock) } });
    expect(getByRole('button', { name: 'Italic' })).not.toBeNull();
  });

  it('renders Bullet list button with aria-label "Bullet list"', () => {
    const mock = buildMockEditor();
    const { getByRole } = render(RichTextToolbar, { props: { editor: asEditor(mock) } });
    expect(getByRole('button', { name: 'Bullet list' })).not.toBeNull();
  });

  it('renders Ordered list button with aria-label "Ordered list"', () => {
    const mock = buildMockEditor();
    const { getByRole } = render(RichTextToolbar, { props: { editor: asEditor(mock) } });
    expect(getByRole('button', { name: 'Ordered list' })).not.toBeNull();
  });

  it('renders with null editor without crashing', () => {
    const { container } = render(RichTextToolbar, { props: { editor: null } });
    expect(container.querySelectorAll('button').length).toBe(4);
  });
});

// ── Active state tests ────────────────────────────────────────────────────────

describe('RichTextToolbar - Active states', () => {
  it('Bold button: isActive("bold") is called to determine variant', () => {
    const mock = buildMockEditor({ bold: true });
    render(RichTextToolbar, { props: { editor: asEditor(mock) } });
    expect(mock.isActive.mock.calls.some((args) => args[0] === 'bold')).toBe(true);
  });

  it('Bold button renders when bold is inactive', () => {
    const mock = buildMockEditor({ bold: false });
    const { getByRole } = render(RichTextToolbar, { props: { editor: asEditor(mock) } });
    expect(getByRole('button', { name: 'Bold' })).not.toBeNull();
  });

  it('Italic button: isActive("italic") is called to determine variant', () => {
    const mock = buildMockEditor({ italic: true });
    const { getByRole } = render(RichTextToolbar, { props: { editor: asEditor(mock) } });
    expect(getByRole('button', { name: 'Italic' })).not.toBeNull();
    expect(mock.isActive.mock.calls.some((args) => args[0] === 'italic')).toBe(true);
  });

  it('BulletList button: isActive("bulletList") is called to determine variant', () => {
    const mock = buildMockEditor({ bulletList: true });
    const { getByRole } = render(RichTextToolbar, { props: { editor: asEditor(mock) } });
    expect(getByRole('button', { name: 'Bullet list' })).not.toBeNull();
    expect(mock.isActive.mock.calls.some((args) => args[0] === 'bulletList')).toBe(true);
  });

  it('OrderedList button: isActive("orderedList") is called to determine variant', () => {
    const mock = buildMockEditor({ orderedList: true });
    const { getByRole } = render(RichTextToolbar, { props: { editor: asEditor(mock) } });
    expect(getByRole('button', { name: 'Ordered list' })).not.toBeNull();
    expect(mock.isActive.mock.calls.some((args) => args[0] === 'orderedList')).toBe(true);
  });
});

// ── Click behaviour ──────────────────────────────────────────────────────────

describe('RichTextToolbar - Button clicks', () => {
  it('clicking Bold calls editor.chain().focus().toggleBold().run()', async () => {
    const mock = buildMockEditor();
    const { getByRole } = render(RichTextToolbar, { props: { editor: asEditor(mock) } });
    await fireEvent.click(getByRole('button', { name: 'Bold' }));
    expect(mock.chain).toHaveBeenCalled();
    expect(mock._chainResult.focus).toHaveBeenCalled();
    expect(mock._chainResult.toggleBold).toHaveBeenCalled();
    expect(mock._chainResult.run).toHaveBeenCalled();
  });

  it('clicking Italic calls editor.chain().focus().toggleItalic().run()', async () => {
    const mock = buildMockEditor();
    const { getByRole } = render(RichTextToolbar, { props: { editor: asEditor(mock) } });
    await fireEvent.click(getByRole('button', { name: 'Italic' }));
    expect(mock.chain).toHaveBeenCalled();
    expect(mock._chainResult.toggleItalic).toHaveBeenCalled();
    expect(mock._chainResult.run).toHaveBeenCalled();
  });

  it('clicking Bullet list calls editor.chain().focus().toggleBulletList().run()', async () => {
    const mock = buildMockEditor();
    const { getByRole } = render(RichTextToolbar, { props: { editor: asEditor(mock) } });
    await fireEvent.click(getByRole('button', { name: 'Bullet list' }));
    expect(mock._chainResult.toggleBulletList).toHaveBeenCalled();
    expect(mock._chainResult.run).toHaveBeenCalled();
  });

  it('clicking Ordered list calls editor.chain().focus().toggleOrderedList().run()', async () => {
    const mock = buildMockEditor();
    const { getByRole } = render(RichTextToolbar, { props: { editor: asEditor(mock) } });
    await fireEvent.click(getByRole('button', { name: 'Ordered list' }));
    expect(mock._chainResult.toggleOrderedList).toHaveBeenCalled();
    expect(mock._chainResult.run).toHaveBeenCalled();
  });

  it('clicking Bold with null editor does not throw', async () => {
    const { getByRole } = render(RichTextToolbar, { props: { editor: null } });
    await expect(fireEvent.click(getByRole('button', { name: 'Bold' }))).resolves.not.toThrow();
  });
});
