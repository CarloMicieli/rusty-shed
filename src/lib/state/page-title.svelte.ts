import { getContext, setContext } from 'svelte';

const PAGE_TITLE_CONTEXT_KEY = Symbol('page-title-context');

export type PageTitleState = {
  readonly title: string | null;
  setTitle: (value: string) => void;
  clearTitle: () => void;
};

export function createPageTitleState(): PageTitleState {
  let title = $state<string | null>(null);

  return {
    get title() {
      return title;
    },
    setTitle: (value: string) => {
      title = value;
    },
    clearTitle: () => {
      title = null;
    }
  };
}

export function setPageTitleContext(state: PageTitleState): void {
  setContext(PAGE_TITLE_CONTEXT_KEY, state);
}

export function getPageTitleContext(): PageTitleState {
  return getContext<PageTitleState>(PAGE_TITLE_CONTEXT_KEY);
}
