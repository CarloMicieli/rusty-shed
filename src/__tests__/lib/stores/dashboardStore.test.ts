import { describe, it, expect, vi, beforeEach } from 'vitest';

// Mock @tauri-apps/api/core BEFORE importing anything
vi.mock('@tauri-apps/api/core', () => ({
	invoke: vi.fn()
}));

// Mock toaster
vi.mock('$lib/toaster', () => ({
	toaster: { error: vi.fn() }
}));

// Now import after mocks
import { dashboardStore } from '$lib/stores/dashboardStore.svelte';
import type { DashboardSummary } from '$lib/stores/dashboardStore.svelte';
import { invoke } from '@tauri-apps/api/core';
import { toaster } from '$lib/toaster';

const mockInvoke = vi.mocked(invoke);
const mockToaster = vi.mocked(toaster);

// Helper for Tauri mock
const tauriMock = {
	handlers: new Map<string, (args?: Record<string, unknown>) => unknown>(),
	delays: new Map<string, number>(),
	
	mockCommand<T>(command: string, response: T) {
		this.handlers.set(command, () => response);
	},
	
	mockCommandError(command: string, error: unknown) {
		this.handlers.set(command, () => { throw error; });
	},
	
	mockCommandWithDelay<T>(command: string, delay: number, response: T) {
		this.delays.set(command, delay);
		this.mockCommand(command, response);
	},
	
	reset() {
		this.handlers.clear();
		this.delays.clear();
		mockInvoke.mockReset();
		// Re-apply the implementation
		mockInvoke.mockImplementation(async (command: string, args?: Record<string, unknown>) => {
			const handler = this.handlers.get(command);
			const delay = this.delays.get(command) || 0;
			
			if (!handler) {
				throw new Error(`Unmocked Tauri command: ${command}`);
			}
			
			if (delay > 0) {
				await new Promise((resolve) => setTimeout(resolve, delay));
			}
			
			return handler(args);
		});
	}
};

// Initial setup of mockInvoke implementation
mockInvoke.mockImplementation(async (command: string, args?: Record<string, unknown>) => {
	const handler = tauriMock.handlers.get(command);
	const delay = tauriMock.delays.get(command) || 0;
	
	if (!handler) {
		throw new Error(`Unmocked Tauri command: ${command}`);
	}
	
	if (delay > 0) {
		await new Promise((resolve) => setTimeout(resolve, delay));
	}
	
	return handler(args);
});

describe('DashboardStore', () => {
	beforeEach(() => {
		tauriMock.reset();
		vi.clearAllMocks();
		// Reset store state by creating a fresh load
		// Note: dashboardStore is a singleton, so we need to reset its internal state
		// For now, we'll rely on each test starting fresh
	});

	const mockDashboardData: DashboardSummary = {
		totals: {
			collection_items: 42,
			wishlists: 3,
			maintenance_due: 5,
			total_value: { amount: 1250.5, currency: 'EUR' }
		},
		recent_items: [
			{ id: '1', title: 'BR 185', subtitle: 'Electric Locomotive' },
			{ id: '2', title: 'ICE 3', subtitle: 'High Speed Train' }
		],
		depot_items: [
			{
				id: '1',
				manufacturer: 'Roco',
				productCode: '79894',
				category: 'locomotive',
				scale: 'H0',
				railwayCompany: 'DB AG',
				description: 'BR 185'
			}
		]
	};

	it('should initialize with null data and not loading', () => {
		expect(dashboardStore.data).toBeNull();
		expect(dashboardStore.isLoading).toBe(false);
		expect(dashboardStore.error).toBeNull();
	});

	it('should load dashboard data successfully', async () => {
		tauriMock.mockCommand('dashboard_summary', mockDashboardData);

		await dashboardStore.load();

		// Assert synchronously - $state updates immediately
		expect(dashboardStore.data).toEqual(mockDashboardData);
		expect(dashboardStore.isLoading).toBe(false);
		expect(dashboardStore.error).toBeNull();
	});

	it('should set loading state during fetch', async () => {
		// Mock with delay to observe loading state
		tauriMock.mockCommandWithDelay('dashboard_summary', 50, mockDashboardData);

		const loadPromise = dashboardStore.load();

		// Assert loading state immediately (synchronous $state update)
		expect(dashboardStore.isLoading).toBe(true);

		await loadPromise;

		expect(dashboardStore.isLoading).toBe(false);
	});

	it('should handle error when loading fails', async () => {
		const error = { DatabaseError: 'Connection timeout' };
		tauriMock.mockCommandError('dashboard_summary', error);

		const dataBefore = dashboardStore.data;
		await dashboardStore.load();

		// Assert synchronously - data should remain unchanged (not updated on error)
		expect(dashboardStore.data).toBe(dataBefore);
		expect(dashboardStore.isLoading).toBe(false);
		expect(dashboardStore.error).toBe('dashboard_load_failed');
	});

	it('should prevent concurrent load calls', async () => {
		tauriMock.mockCommandWithDelay('dashboard_summary', 50, mockDashboardData);

		// Start two loads simultaneously
		const load1 = dashboardStore.load();
		const load2 = dashboardStore.load();

		await Promise.all([load1, load2]);

		// Invoke should only be called once due to isLoading guard
		expect(mockInvoke).toHaveBeenCalledTimes(1);
	});

	it('should compute hasMaintenance derived value', async () => {
		tauriMock.mockCommand('dashboard_summary', mockDashboardData);

		await dashboardStore.load();

		// $derived values update synchronously
		expect(dashboardStore.hasMaintenance).toBe(true);
	});

	it('should compute hasMaintenance as false when no maintenance due', async () => {
		const dataWithNoMaintenance: DashboardSummary = {
			...mockDashboardData,
			totals: { ...mockDashboardData.totals, maintenance_due: 0 }
		};

		tauriMock.mockCommand('dashboard_summary', dataWithNoMaintenance);

		await dashboardStore.load();

		expect(dashboardStore.hasMaintenance).toBe(false);
	});

	it('should compute recentItemsCount', async () => {
		tauriMock.mockCommand('dashboard_summary', mockDashboardData);

		await dashboardStore.load();

		expect(dashboardStore.recentItemsCount).toBe(2);
	});

	it('should clear data and reload on retry', async () => {
		// Load initial data
		tauriMock.mockCommand('dashboard_summary', mockDashboardData);
		await dashboardStore.load();

		expect(dashboardStore.data).toEqual(mockDashboardData);

		// Prepare new data for retry
		const newData: DashboardSummary = {
			...mockDashboardData,
			totals: { ...mockDashboardData.totals, collection_items: 100 }
		};
		tauriMock.reset();
		tauriMock.mockCommand('dashboard_summary', newData);

		await dashboardStore.retry();

		expect(dashboardStore.data).toEqual(newData);
	});

	it('should clear old data immediately when retry is called', () => {
		tauriMock.mockCommand('dashboard_summary', mockDashboardData);

		// Load initial data
		dashboardStore.load();

		// Call retry
		dashboardStore.retry();

		// Data should be cleared before the async load completes
		expect(dashboardStore.data).toBeNull();
	});
});
