/**
 * Steampunk Custom Transitions
 *
 * Custom Svelte transitions with steampunk mechanical aesthetic:
 * - heavyDoorSlide: Weighted, metallic sliding motion (for drawers, modals)
 * - leverToggle: Mechanical lever press effect (for buttons, switches)
 *
 * Feature: 011-steampunk-theme
 */

import { cubicOut, cubicInOut } from 'svelte/easing';

export interface TransitionOptions {
  delay?: number;
  duration?: number;
  easing?: (t: number) => number;
}

/**
 * Heavy Door Slide - Weighted sliding motion with metallic feel
 * Simulates opening a heavy metal door with momentum
 *
 * @param node - Element to animate
 * @param options - delay, duration, easing
 */
export const heavyDoorSlide = (
  node: Element,
  options: TransitionOptions & { direction?: 'x' | 'y' } = {}
) => {
  const { delay = 0, duration = 400, easing = cubicOut, direction = 'x' } = options;

  const style = window.getComputedStyle(node);
  const opacity = +style.opacity;
  const start = { scale: 0.95, opacity: 0 };
  const end = { scale: 1, opacity };

  return {
    delay,
    duration,
    easing,
    css: (t: number) => {
      const scale = start.scale + (end.scale - start.scale) * t;
      const opacity = start.opacity + (end.opacity - start.opacity) * t;

      if (direction === 'x') {
        return `transform: scaleX(${scale}); opacity: ${opacity}`;
      } else {
        return `transform: scaleY(${scale}); opacity: ${opacity}`;
      }
    }
  };
};

/**
 * Lever Toggle - Mechanical lever press effect
 * Simulates pressing a mechanical lever/button with rebound
 *
 * @param node - Element to animate
 * @param options - delay, duration
 */
export const leverToggle = (node: Element, options: TransitionOptions = {}) => {
  const { delay = 0, duration = 200, easing = cubicInOut } = options;

  return {
    delay,
    duration,
    easing,
    css: (t: number) => {
      // Press down (0 to 0.5) then spring back (0.5 to 1)
      const pressAmount = t < 0.5 ? t * 2 * 4 : (1 - t) * 2 * 4;
      const scale = 1 + pressAmount * 0.05;
      const rotate = pressAmount * -2;

      return `
        transform: scale(${scale}) rotate(${rotate}deg);
        opacity: ${t};
      `;
    }
  };
};

/**
 * Rusty Creak - Sounds mechanical and old (CSS animation only, no audio)
 * Creates a perception of movement through opacity and scale changes
 *
 * @param node - Element to animate
 * @param options - delay, duration
 */
export const rustCreak = (node: Element, options: TransitionOptions = {}) => {
  const { delay = 0, duration = 300, easing = cubicOut } = options;

  return {
    delay,
    duration,
    easing,
    css: (t: number) => {
      // Add subtle jitter for "creaky" feel
      const jitter = Math.sin(t * Math.PI * 4) * 0.02;
      const scale = 0.8 + t * 0.2 + jitter;

      return `
        transform: scale(${scale});
        opacity: ${t};
      `;
    }
  };
};
