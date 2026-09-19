type MatchMediaRecord = {
  media: string;
  matches: boolean;
};

export type MobileViewportSetup = {
  restore: () => void;
  setWidth: (width: number) => void;
  setSafeAreaInset: (position: 'top' | 'right' | 'bottom' | 'left', valuePx: number) => void;
};

const MOBILE_BREAKPOINT = 768;

export function setupMobileViewport(initialWidth = 375): MobileViewportSetup {
  const originalInnerWidth = window.innerWidth;
  const originalMatchMedia = window.matchMedia;
  const safeAreaInsets: Record<'top' | 'right' | 'bottom' | 'left', number> = {
    top: 0,
    right: 0,
    bottom: 0,
    left: 0
  };

  let width = initialWidth;

  const records: MatchMediaRecord[] = [];

  const installMatchMedia = () => {
    Object.defineProperty(window, 'matchMedia', {
      configurable: true,
      writable: true,
      value: (query: string): MediaQueryList => {
        const matches = evaluateMediaQuery(query, width);
        records.push({ media: query, matches });

        return {
          media: query,
          matches,
          onchange: null,
          addEventListener: () => {},
          removeEventListener: () => {},
          addListener: () => {},
          removeListener: () => {},
          dispatchEvent: () => false
        } as MediaQueryList;
      }
    });
  };

  const setWidth = (nextWidth: number) => {
    width = nextWidth;
    Object.defineProperty(window, 'innerWidth', {
      configurable: true,
      writable: true,
      value: nextWidth
    });
    installMatchMedia();
  };

  const setSafeAreaInset = (position: 'top' | 'right' | 'bottom' | 'left', valuePx: number) => {
    safeAreaInsets[position] = Math.max(0, valuePx);
    document.documentElement.style.setProperty(
      `--test-safe-area-${position}`,
      `${safeAreaInsets[position]}px`
    );
  };

  setWidth(initialWidth);

  const restore = () => {
    Object.defineProperty(window, 'innerWidth', {
      configurable: true,
      writable: true,
      value: originalInnerWidth
    });

    Object.defineProperty(window, 'matchMedia', {
      configurable: true,
      writable: true,
      value: originalMatchMedia
    });

    for (const side of ['top', 'right', 'bottom', 'left'] as const) {
      document.documentElement.style.removeProperty(`--test-safe-area-${side}`);
    }
  };

  return {
    restore,
    setWidth,
    setSafeAreaInset
  };
}

export function isMobileWidth(width: number): boolean {
  return width < MOBILE_BREAKPOINT;
}

function evaluateMediaQuery(query: string, width: number): boolean {
  if (query.includes('max-width')) {
    return width < MOBILE_BREAKPOINT;
  }

  if (query.includes('min-width')) {
    return width >= MOBILE_BREAKPOINT;
  }

  return false;
}
