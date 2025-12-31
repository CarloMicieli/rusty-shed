import { vi } from 'vitest';
import type { InvokeArgs, InvokeOptions } from '@tauri-apps/api/core';

export type MockInvokeResponse = {
  success: boolean;
  data?: unknown;
  error?: unknown;
};

type MockInvokeHandler = (args?: InvokeArgs) => unknown;

class TauriMock {
  private handlers = new Map<string, MockInvokeHandler>();
  private delays = new Map<string, number>();

  /**
   * Mock a Tauri command with a response
   */
  mockCommand<T>(command: string, response: T | ((args?: InvokeArgs) => T)) {
    const handler: MockInvokeHandler =
      typeof response === 'function' ? (response as (args?: InvokeArgs) => T) : () => response;
    this.handlers.set(command, handler);
  }

  /**
   * Mock a command that rejects with an error
   */
  mockCommandError(command: string, error: unknown) {
    this.handlers.set(command, () => {
      throw error;
    });
  }

  /**
   * Mock a command with a delay (useful for testing race conditions)
   */
  mockCommandWithDelay<T>(command: string, delay: number, response: T) {
    this.delays.set(command, delay);
    this.mockCommand(command, response);
  }

  /**
   * Mock a command that fails after a delay
   */
  mockCommandErrorWithDelay(command: string, delay: number, error: unknown) {
    this.delays.set(command, delay);
    this.mockCommandError(command, error);
  }

  /**
   * Get the mock invoke function
   */
  getInvokeMock() {
    return vi.fn(async (command: string, args?: InvokeArgs, _options?: InvokeOptions) => {
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

  /**
   * Clear all mocked commands
   */
  clear() {
    this.handlers.clear();
    this.delays.clear();
  }

  /**
   * Reset to default state
   */
  reset() {
    this.clear();
  }
}

// Singleton instance
export const tauriMock = new TauriMock();

// Mock the @tauri-apps/api/core module
export const invoke = tauriMock.getInvokeMock();

// Helper to create normalized error responses matching your error.ts types
export function createMockError(
  kind: 'database' | 'not_found' | 'validation' | 'permission_denied' | 'unknown',
  message: string,
  fields?: Record<string, string>
) {
  const error: Record<string, unknown> = { kind, message };
  if (fields) {
    error.fields = fields;
  }
  return error;
}
