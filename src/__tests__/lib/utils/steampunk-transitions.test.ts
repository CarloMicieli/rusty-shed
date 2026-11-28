import { beforeEach, describe, expect, it, vi } from 'vitest';
import { heavyDoorSlide, leverToggle, rustCreak } from '$lib/utils/steampunk-transitions';

describe('steampunk transitions', () => {
  beforeEach(() => {
    vi.restoreAllMocks();
  });

  it('builds heavyDoorSlide css for both axes and preserves timing options', () => {
    const node = document.createElement('div');
    vi.spyOn(window, 'getComputedStyle').mockReturnValue({ opacity: '0.8' } as CSSStyleDeclaration);

    const horizontal = heavyDoorSlide(node, { delay: 20, duration: 500, direction: 'x' });
    const vertical = heavyDoorSlide(node, { direction: 'y' });

    expect(horizontal.delay).toBe(20);
    expect(horizontal.duration).toBe(500);
    expect(horizontal.css(1)).toContain('scaleX(1)');
    expect(horizontal.css(1)).toContain('opacity: 0.8');

    expect(vertical.css(0.5)).toContain('scaleY(');
    expect(vertical.css(0.5)).toContain('opacity:');
  });

  it('applies leverToggle press-and-rebound transform', () => {
    const node = document.createElement('button');
    const transition = leverToggle(node, { duration: 240 });

    const mid = transition.css(0.25);
    const end = transition.css(1);

    expect(transition.duration).toBe(240);
    expect(mid).toContain('transform: scale(');
    expect(mid).toContain('rotate(');
    expect(end).toContain('opacity: 1');
  });

  it('adds jittered scale and opacity for rustCreak', () => {
    const node = document.createElement('div');
    const transition = rustCreak(node, { delay: 10, duration: 350 });

    const css = transition.css(0.6);

    expect(transition.delay).toBe(10);
    expect(transition.duration).toBe(350);
    expect(css).toContain('transform: scale(');
    expect(css).toContain('opacity: 0.6');
  });
});
