import { describe, it, expect, vi, beforeEach } from 'vitest';
import type { GlobalSearchResultView, Language } from '$lib/bindings';

// Mock the commands from bindings
vi.mock('$lib/bindings', () => {
  return {
    commands: {
      globalSearch: vi.fn()
    }
  };
});

// Mock svelte context functions
vi.mock('svelte', async () => {
  const actual = await vi.importActual('svelte');
  return {
    ...actual,
    getContext: vi.fn(),
    setContext: vi.fn()
  };
});

// Import after mocks are set up
import { SearchService } from '$lib/features/search/SearchService.svelte';
import { commands } from '$lib/bindings';

const mockCommands = commands;

describe('SearchService', () => {
  let service: SearchService;

  beforeEach(() => {
    service = new SearchService();
    vi.clearAllMocks();
  });

  describe('initial state', () => {
    it('should initialize with empty results', () => {
      expect(service.results).toEqual([]);
    });

    it('should initialize with isLoading as false', () => {
      expect(service.isLoading).toBe(false);
    });

    it('should initialize with error as null', () => {
      expect(service.error).toBeNull();
    });

    it('should initialize with empty lastQuery', () => {
      expect(service.lastQuery).toBe('');
    });
  });

  describe('search', () => {
    it('should execute a successful search and set results', async () => {
      const mockResults: GlobalSearchResultView[] = [
        {
          id: 'result-1',
          type: 'railway_model',
          title: 'Marklin Steam Locomotive',
          description: 'A classic steam locomotive',
          imageUrl: null
        },
        {
          id: 'result-2',
          type: 'rolling_stock',
          title: 'Freight Car',
          description: 'Standard freight car',
          imageUrl: null
        }
      ];

      mockCommands.globalSearch.mockResolvedValue({
        status: 'ok',
        data: mockResults
      });

      const lang: Language = 'en';
      await service.search('steam', lang);

      expect(service.results).toEqual(mockResults);
      expect(service.lastQuery).toBe('steam');
      expect(service.error).toBeNull();
      expect(service.isLoading).toBe(false);
      expect(mockCommands.globalSearch).toHaveBeenCalledWith({
        query: 'steam',
        lang
      });
    });

    it('should trim whitespace from query', async () => {
      mockCommands.globalSearch.mockResolvedValue({
        status: 'ok',
        data: []
      });

      const lang: Language = 'en';
      await service.search('  freight  ', lang);

      expect(service.lastQuery).toBe('freight');
      expect(mockCommands.globalSearch).toHaveBeenCalledWith({
        query: 'freight',
        lang
      });
    });

    it('should not search if query is too short', async () => {
      const lang: Language = 'en';
      await service.search('a', lang);

      expect(mockCommands.globalSearch).not.toHaveBeenCalled();
      expect(service.results).toEqual([]);
      expect(service.lastQuery).toBe('');
    });

    it('should not search if query is whitespace only', async () => {
      const lang: Language = 'en';
      await service.search('   ', lang);

      expect(mockCommands.globalSearch).not.toHaveBeenCalled();
      expect(service.results).toEqual([]);
    });

    it('should handle search errors from command', async () => {
      mockCommands.globalSearch.mockResolvedValue({
        status: 'error',
        error: 'Database error'
      });

      const lang: Language = 'en';
      await service.search('test', lang);

      expect(service.error).toBe('Database error');
      expect(service.results).toEqual([]);
      expect(service.isLoading).toBe(false);
    });

    it('should handle search errors with non-string error object', async () => {
      mockCommands.globalSearch.mockResolvedValue({
        status: 'error',
        error: { code: 'ERROR_CODE' }
      });

      const lang: Language = 'en';
      await service.search('test', lang);

      expect(service.error).toBe('Search failed.');
      expect(service.results).toEqual([]);
    });

    it('should handle exceptions during search', async () => {
      mockCommands.globalSearch.mockRejectedValue(
        new Error('Network connection failed')
      );

      const lang: Language = 'en';
      await service.search('test', lang);

      expect(service.error).toBe('Network connection failed');
      expect(service.results).toEqual([]);
      expect(service.isLoading).toBe(false);
    });

    it('should handle non-Error exceptions', async () => {
      mockCommands.globalSearch.mockRejectedValue('Unknown error');

      const lang: Language = 'en';
      await service.search('test', lang);

      expect(service.error).toBe('Search failed.');
      expect(service.results).toEqual([]);
    });

    it('should set isLoading to true during search', async () => {
      mockCommands.globalSearch.mockImplementation(
        () =>
          new Promise((resolve) => {
            setTimeout(() => resolve({ status: 'ok', data: [] }), 10);
          })
      );

      const lang: Language = 'en';
      const searchPromise = service.search('test', lang);

      // Check loading state was set
      expect(service.isLoading).toBe(true);

      await searchPromise;

      expect(service.isLoading).toBe(false);
    });

    it('should clear error on successful search', async () => {
      // First do a failed search
      mockCommands.globalSearch.mockResolvedValue({
        status: 'error',
        error: 'First error'
      });

      const lang: Language = 'en';
      await service.search('test', lang);

      expect(service.error).toBe('First error');

      // Now do a successful search
      mockCommands.globalSearch.mockResolvedValue({
        status: 'ok',
        data: []
      });

      await service.search('test2', lang);

      expect(service.error).toBeNull();
    });

    it('should support multiple language options', async () => {
      mockCommands.globalSearch.mockResolvedValue({
        status: 'ok',
        data: []
      });

      const languages: Language[] = ['en', 'de', 'fr', 'es'];

      for (const lang of languages) {
        await service.search('test', lang);
        expect(mockCommands.globalSearch).toHaveBeenCalledWith({
          query: 'test',
          lang
        });
      }
    });

    it('should handle empty search results', async () => {
      mockCommands.globalSearch.mockResolvedValue({
        status: 'ok',
        data: []
      });

      const lang: Language = 'en';
      await service.search('nonexistent', lang);

      expect(service.results).toEqual([]);
      expect(service.error).toBeNull();
      expect(service.lastQuery).toBe('nonexistent');
    });

    it('should handle large result sets', async () => {
      const largeResults: GlobalSearchResultView[] = Array.from(
        { length: 100 },
        (_, i) => ({
          id: `result-${i}`,
          type: 'railway_model',
          title: `Model ${i}`,
          description: `Description ${i}`,
          imageUrl: null
        })
      );

      mockCommands.globalSearch.mockResolvedValue({
        status: 'ok',
        data: largeResults
      });

      const lang: Language = 'en';
      await service.search('many', lang);

      expect(service.results).toHaveLength(100);
      expect(service.results).toEqual(largeResults);
    });
  });

  describe('reset', () => {
    it('should clear all state', async () => {
      // First populate state
      mockCommands.globalSearch.mockResolvedValue({
        status: 'ok',
        data: [
          {
            id: 'result-1',
            type: 'railway_model',
            title: 'Test',
            description: 'Test description',
            imageUrl: null
          }
        ]
      });

      const lang: Language = 'en';
      await service.search('test', lang);

      expect(service.results.length).toBeGreaterThan(0);
      expect(service.lastQuery).toBe('test');

      // Reset
      service.reset();

      expect(service.results).toEqual([]);
      expect(service.error).toBeNull();
      expect(service.lastQuery).toBe('');
      expect(service.isLoading).toBe(false);
    });

    it('should clear error state on reset', async () => {
      mockCommands.globalSearch.mockResolvedValue({
        status: 'error',
        error: 'Test error'
      });

      const lang: Language = 'en';
      await service.search('test', lang);

      expect(service.error).toBe('Test error');

      service.reset();

      expect(service.error).toBeNull();
    });

    it('should be safe to call multiple times', () => {
      service.reset();
      service.reset();
      service.reset();

      expect(service.results).toEqual([]);
      expect(service.error).toBeNull();
      expect(service.lastQuery).toBe('');
    });
  });

  describe('getters', () => {
    it('should expose results getter', async () => {
      const mockResults: GlobalSearchResultView[] = [
        {
          id: 'r1',
          type: 'railway_model',
          title: 'Test Model',
          description: 'Test',
          imageUrl: null
        }
      ];

      mockCommands.globalSearch.mockResolvedValue({
        status: 'ok',
        data: mockResults
      });

      await service.search('test', 'en');

      const results = service.results;
      expect(results).toEqual(mockResults);
      expect(Array.isArray(results)).toBe(true);
    });

    it('should expose isLoading getter', async () => {
      mockCommands.globalSearch.mockImplementation(
        () =>
          new Promise((resolve) => {
            setTimeout(() => resolve({ status: 'ok', data: [] }), 10);
          })
      );

      expect(service.isLoading).toBe(false);

      const searchPromise = service.search('test', 'en');
      expect(service.isLoading).toBe(true);

      await searchPromise;
      expect(service.isLoading).toBe(false);
    });

    it('should expose error getter', async () => {
      mockCommands.globalSearch.mockResolvedValue({
        status: 'error',
        error: 'Test error'
      });

      expect(service.error).toBeNull();

      await service.search('test', 'en');
      expect(service.error).toBe('Test error');
    });

    it('should expose lastQuery getter', async () => {
      mockCommands.globalSearch.mockResolvedValue({
        status: 'ok',
        data: []
      });

      expect(service.lastQuery).toBe('');

      await service.search('locomotive', 'en');
      expect(service.lastQuery).toBe('locomotive');

      await service.search('  freight  ', 'en');
      expect(service.lastQuery).toBe('freight');
    });
  });

  describe('edge cases', () => {
    it('should handle queries with special characters', async () => {
      mockCommands.globalSearch.mockResolvedValue({
        status: 'ok',
        data: []
      });

      await service.search('test@#$%', 'en');

      expect(mockCommands.globalSearch).toHaveBeenCalledWith({
        query: 'test@#$%',
        lang: 'en'
      });
    });

    it('should handle very long queries', async () => {
      const longQuery = 'a'.repeat(500);
      mockCommands.globalSearch.mockResolvedValue({
        status: 'ok',
        data: []
      });

      await service.search(longQuery, 'en');

      expect(service.lastQuery).toBe(longQuery);
      expect(mockCommands.globalSearch).toHaveBeenCalledWith({
        query: longQuery,
        lang: 'en'
      });
    });

    it('should handle unicode queries', async () => {
      mockCommands.globalSearch.mockResolvedValue({
        status: 'ok',
        data: []
      });

      await service.search('Märklin', 'de');

      expect(service.lastQuery).toBe('Märklin');
      expect(mockCommands.globalSearch).toHaveBeenCalledWith({
        query: 'Märklin',
        lang: 'de'
      });
    });

    it('should handle concurrent searches', async () => {
      mockCommands.globalSearch.mockResolvedValue({
        status: 'ok',
        data: [
          {
            id: 'r1',
            type: 'railway_model',
            title: 'Model',
            description: 'Desc',
            imageUrl: null
          }
        ]
      });

      // Fire multiple searches
      const search1 = service.search('query1', 'en');
      const search2 = service.search('query2', 'en');
      const search3 = service.search('query3', 'en');

      await Promise.all([search1, search2, search3]);

      // Only the last query should be stored
      expect(service.lastQuery).toBe('query3');
      expect(mockCommands.globalSearch).toHaveBeenCalledTimes(3);
    });
  });
});
