/**
 * Unit tests for viewport state management
 *
 * @vitest-environment happy-dom
 */

import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import { render } from '@testing-library/svelte';
import { tick } from 'svelte';
import ViewportTestWrapper from './ViewportTestWrapper.svelte';
import type { createViewport } from './viewport.svelte';

// Mock Tauri API modules
vi.mock('@tauri-apps/api/event', () => ({
  listen: vi.fn(() => Promise.resolve(() => {}))
}));

vi.mock('@tauri-apps/api/webviewWindow', () => ({
  getCurrentWebviewWindow: vi.fn(() => ({
    scaleFactor: vi.fn(() => Promise.resolve(1.0)),
    innerSize: vi.fn(() => Promise.resolve({ width: 800, height: 600 }))
  }))
}));

describe('viewport', () => {
  beforeEach(() => {
    // Clear all mocks before each test
    vi.clearAllMocks();

    // Mock document.documentElement for CSS variable tests
    if (typeof document !== 'undefined') {
      document.documentElement.style.setProperty = vi.fn();
    }
  });

  afterEach(() => {
    vi.restoreAllMocks();
  });

  it('should initialize with default values', async () => {
    let viewport: ReturnType<typeof createViewport>;
    render(ViewportTestWrapper, {
      props: {
        onViewport: (v) => {
          viewport = v;
        }
      }
    });
    await tick();

    expect(viewport!.scaleFactor).toBe(1.0);
    expect(viewport!.width).toBe(800);
    expect(viewport!.height).toBe(600);
    expect(viewport!.isMobile).toBe(false);
  });

  it('should correctly compute isHighDPI based on scaleFactor', async () => {
    const { getCurrentWebviewWindow } = await import('@tauri-apps/api/webviewWindow');

    // Mock scale factor >= 1.5 for high DPI
    vi.mocked(getCurrentWebviewWindow).mockReturnValue({
      scaleFactor: vi.fn(() => Promise.resolve(2.0)),
      innerSize: vi.fn(() => Promise.resolve({ width: 1920, height: 1080 }))
    } as unknown as ReturnType<typeof getCurrentWebviewWindow>);

    let viewport: ReturnType<typeof createViewport>;
    render(ViewportTestWrapper, {
      props: {
        onViewport: (v) => {
          viewport = v;
        }
      }
    });

    await tick();
    await new Promise((resolve) => setTimeout(resolve, 20));

    expect(viewport!.isHighDPI).toBe(true);
  });

  it('should detect mobile platform correctly', async () => {
    // Mock Android user agent
    const originalUserAgent = navigator.userAgent;
    Object.defineProperty(navigator, 'userAgent', {
      value: 'Mozilla/5.0 (Linux; Android 10) AppleWebKit/537.36',
      configurable: true
    });

    let viewport: ReturnType<typeof createViewport>;
    render(ViewportTestWrapper, {
      props: {
        onViewport: (v) => {
          viewport = v;
        }
      }
    });

    await tick();
    await new Promise((resolve) => setTimeout(resolve, 20));

    expect(viewport!.isMobile).toBe(true);

    // Restore original user agent
    Object.defineProperty(navigator, 'userAgent', {
      value: originalUserAgent,
      configurable: true
    });
  });

  it('should set up event listeners for scale-change and resize', async () => {
    const { listen } = await import('@tauri-apps/api/event');

    render(ViewportTestWrapper, {
      props: { onViewport: () => {} }
    });

    await tick();
    await new Promise((resolve) => setTimeout(resolve, 20));

    expect(listen).toHaveBeenCalledWith('tauri://scale-change', expect.any(Function));
    expect(listen).toHaveBeenCalledWith('tauri://resize', expect.any(Function));
  });

  it('should apply CSS custom properties', async () => {
    const { getCurrentWebviewWindow } = await import('@tauri-apps/api/webviewWindow');

    vi.mocked(getCurrentWebviewWindow).mockReturnValue({
      scaleFactor: vi.fn(() => Promise.resolve(1.5)),
      innerSize: vi.fn(() => Promise.resolve({ width: 1024, height: 768 }))
    } as unknown as ReturnType<typeof getCurrentWebviewWindow>);

    render(ViewportTestWrapper, {
      props: { onViewport: () => {} }
    });

    await tick();
    await new Promise((resolve) => setTimeout(resolve, 20));

    if (typeof document !== 'undefined') {
      const setPropertyMock = document.documentElement.style.setProperty as ReturnType<
        typeof vi.fn
      >;
      expect(setPropertyMock).toHaveBeenCalledWith('--app-scale', expect.any(String));
      expect(setPropertyMock).toHaveBeenCalledWith('--app-width', expect.any(String));
      expect(setPropertyMock).toHaveBeenCalledWith('--app-height', expect.any(String));
    }
  });

  it('should handle errors gracefully when Tauri API fails', async () => {
    const { getCurrentWebviewWindow } = await import('@tauri-apps/api/webviewWindow');

    vi.mocked(getCurrentWebviewWindow).mockReturnValue({
      scaleFactor: vi.fn(() => Promise.reject(new Error('API Error'))),
      innerSize: vi.fn(() => Promise.reject(new Error('API Error')))
    } as unknown as ReturnType<typeof getCurrentWebviewWindow>);
    const consoleSpy = vi.spyOn(console, 'error').mockImplementation(() => {});

    let viewport: ReturnType<typeof createViewport>;
    render(ViewportTestWrapper, {
      props: {
        onViewport: (v) => {
          viewport = v;
        }
      }
    });

    await tick();
    await new Promise((resolve) => setTimeout(resolve, 20));

    expect(viewport!.scaleFactor).toBe(1.0);
    expect(viewport!.width).toBe(800);
    expect(viewport!.height).toBe(600);

    consoleSpy.mockRestore();
  });
});
