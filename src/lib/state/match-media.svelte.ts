import { SvelteSet } from 'svelte/reactivity';

export type MatchMediaState = {
  readonly query: string;
  readonly matches: boolean;
  subscribe: (listener: (matches: boolean) => void) => () => void;
  destroy: () => void;
};

export function createMatchMediaState(query: string): MatchMediaState {
  let matches = $state(false);
  const listeners = new SvelteSet<(value: boolean) => void>();
  let mediaQueryList: MediaQueryList | null = null;
  let removeListener: (() => void) | null = null;

  const notify = () => {
    for (const listener of listeners) {
      listener(matches);
    }
  };

  const setMatches = (nextValue: boolean) => {
    if (matches === nextValue) {
      return;
    }

    matches = nextValue;
    notify();
  };

  const attach = () => {
    if (typeof window === 'undefined' || typeof window.matchMedia !== 'function') {
      return;
    }

    mediaQueryList = window.matchMedia(query);
    setMatches(mediaQueryList.matches);

    const handleChange = (event: MediaQueryListEvent) => {
      setMatches(event.matches);
    };

    mediaQueryList.addEventListener('change', handleChange);
    removeListener = () => {
      mediaQueryList?.removeEventListener('change', handleChange);
    };
  };

  const detach = () => {
    removeListener?.();
    removeListener = null;
    mediaQueryList = null;
  };

  attach();

  return {
    get query() {
      return query;
    },
    get matches() {
      return matches;
    },
    subscribe: (listener) => {
      listeners.add(listener);
      listener(matches);

      return () => {
        listeners.delete(listener);
      };
    },
    destroy: () => {
      listeners.clear();
      detach();
    }
  };
}

export function createMobileMatchMediaState(): MatchMediaState {
  return createMatchMediaState('(max-width: 767px)');
}
