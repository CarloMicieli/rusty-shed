import { getContext, setContext } from 'svelte';
import { commands, type GlobalSearchResultView, type Language } from '$lib/bindings';

const SEARCH_CTX = Symbol('search-service');

/**
 * Reactive state service for global search.
 *
 * Holds the current query, results, and loading state.
 * Exposes a `search()` method that invokes the `global_search` Tauri command.
 */
export class SearchService {
  #results = $state<GlobalSearchResultView[]>([]);
  #isLoading = $state(false);
  #error = $state<string | null>(null);
  #lastQuery = $state('');

  /** The current list of search results. */
  get results(): GlobalSearchResultView[] {
    return this.#results;
  }

  /** True while a search request is in-flight. */
  get isLoading(): boolean {
    return this.#isLoading;
  }

  /** A non-null error message if the last search failed. */
  get error(): string | null {
    return this.#error;
  }

  /** The query that produced the current results. */
  get lastQuery(): string {
    return this.#lastQuery;
  }

  /**
   * Execute a global search.
   *
   * @param query - The user's search string (must be ≥ 2 characters)
   * @param lang  - Language for localised display names
   */
  async search(query: string, lang: Language): Promise<void> {
    const trimmed = query.trim();
    if (trimmed.length < 2) return;

    this.#isLoading = true;
    this.#error = null;
    this.#lastQuery = trimmed;

    try {
      const result = await commands.globalSearch({ query: trimmed, lang });
      if (result.status === 'ok') {
        this.#results = result.data;
      } else {
        this.#error = typeof result.error === 'string' ? result.error : 'Search failed.';
        this.#results = [];
      }
    } catch (err) {
      this.#error = err instanceof Error ? err.message : 'Search failed.';
      this.#results = [];
    } finally {
      this.#isLoading = false;
    }
  }

  /** Reset results and state. */
  reset(): void {
    this.#results = [];
    this.#error = null;
    this.#lastQuery = '';
  }
}

/** Provide a {@link SearchService} instance via Svelte context. */
export function setSearchContext(): SearchService {
  const service = new SearchService();
  setContext(SEARCH_CTX, service);
  return service;
}

/** Retrieve the {@link SearchService} from Svelte context. */
export function getSearchContext(): SearchService {
  return getContext<SearchService>(SEARCH_CTX);
}
