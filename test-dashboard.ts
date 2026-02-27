import { test, vi } from 'vitest';
vi.mock('lucide-svelte', () => new Proxy({}, { get: () => () => '' }));
import DashboardPage from './src/__tests__/routes/dashboard-page.test.ts';
