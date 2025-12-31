import { describe, it, expect, vi, beforeEach } from 'vitest';

// Mock the invoke function BEFORE importing anything that uses it
vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn()
}));

// Now import after mocks are set up
import { safeInvoke, invokeOrThrow } from '$lib/services/tauri';
import { invoke, type InvokeArgs, type InvokeOptions } from '@tauri-apps/api/core';

const mockInvoke = vi.mocked(invoke);
type InvokeArgType = InvokeArgs | undefined;
type InvokeOptionType = InvokeOptions | undefined;

// Helper to create mock responses
const tauriMock = {
  handlers: new Map<string, (args?: InvokeArgType) => unknown>(),
  delays: new Map<string, number>(),

  mockCommand<T>(command: string, response: T | ((args?: InvokeArgType) => T)) {
    const handler: (args?: InvokeArgType) => unknown =
      typeof response === 'function' ? (response as (args?: InvokeArgType) => T) : () => response;
    this.handlers.set(command, handler);
  },

  mockCommandError(command: string, error: unknown) {
    this.handlers.set(command, () => {
      throw error;
    });
  },

  mockCommandWithDelay<T>(command: string, delay: number, response: T) {
    this.delays.set(command, delay);
    this.mockCommand(command, response);
  },

  mockCommandErrorWithDelay(command: string, delay: number, error: unknown) {
    this.delays.set(command, delay);
    this.mockCommandError(command, error);
  },

  reset() {
    this.handlers.clear();
    this.delays.clear();
    mockInvoke.mockReset();
    // Re-apply the implementation
    mockInvoke.mockImplementation(
      async (command: string, args?: InvokeArgType, _options?: InvokeOptionType) => {
        const handler = this.handlers.get(command);
        const delay = this.delays.get(command) || 0;

        if (!handler) {
          throw new Error(`Unmocked Tauri command: ${command}`);
        }

        if (delay > 0) {
          await new Promise((resolve) => setTimeout(resolve, delay));
        }

        return handler(args);
      }
    );
  }
};

// Initial setup of mockInvoke implementation
mockInvoke.mockImplementation(
  async (command: string, args?: InvokeArgType, _options?: InvokeOptionType) => {
    const handler = tauriMock.handlers.get(command);
    const delay = tauriMock.delays.get(command) || 0;

    if (!handler) {
      throw new Error(`Unmocked Tauri command: ${command}`);
    }

    if (delay > 0) {
      await new Promise((resolve) => setTimeout(resolve, delay));
    }

    return handler(args);
  }
);

describe('safeInvoke', () => {
  beforeEach(() => {
    tauriMock.reset();
  });

  it('should return success result when command succeeds', async () => {
    const mockData = { id: '123', name: 'Test Item' };
    tauriMock.mockCommand('get_item', mockData);

    const result = await safeInvoke<typeof mockData>('get_item', { id: '123' });

    expect(result.ok).toBe(true);
    if (result.ok) {
      expect(result.data).toEqual(mockData);
    }
  });

  it('should normalize DatabaseError into error result', async () => {
    const error = { DatabaseError: 'Connection failed' };
    tauriMock.mockCommandError('save_item', error);

    const result = await safeInvoke<void>('save_item', { id: '123' });

    expect(result.ok).toBe(false);
    if (!result.ok) {
      expect(result.error.kind).toBe('database');
      expect(result.error.message).toBe('Connection failed');
    }
  });

  it('should normalize NotFound error', async () => {
    const error = { NotFound: 'Item with id 999 not found' };
    tauriMock.mockCommandError('get_item', error);

    const result = await safeInvoke<unknown>('get_item', { id: '999' });

    expect(result.ok).toBe(false);
    if (!result.ok) {
      expect(result.error.kind).toBe('not_found');
      expect(result.error.message).toBe('Item with id 999 not found');
    }
  });

  it('should normalize ValidationError with fields', async () => {
    const error = {
      ValidationError: {
        name: 'Name is required',
        email: 'Invalid email format'
      }
    };
    tauriMock.mockCommandError('create_user', error);

    const result = await safeInvoke<unknown>('create_user', {
      name: '',
      email: 'invalid'
    });

    expect(result.ok).toBe(false);
    if (!result.ok) {
      expect(result.error.kind).toBe('validation');
      expect(result.error.message).toBe('Validation failed');
      expect(result.error.fields).toEqual({
        name: 'Name is required',
        email: 'Invalid email format'
      });
    }
  });

  it('should normalize PermissionDenied error', async () => {
    const error = { PermissionDenied: 'Insufficient permissions' };
    tauriMock.mockCommandError('delete_admin', error);

    const result = await safeInvoke<void>('delete_admin', { id: '123' });

    expect(result.ok).toBe(false);
    if (!result.ok) {
      expect(result.error.kind).toBe('permission_denied');
      expect(result.error.message).toBe('Insufficient permissions');
    }
  });

  it('should normalize Unknown error', async () => {
    const error = { Unknown: 'Something went wrong' };
    tauriMock.mockCommandError('mystery_command', error);

    const result = await safeInvoke<void>('mystery_command');

    expect(result.ok).toBe(false);
    if (!result.ok) {
      expect(result.error.kind).toBe('unknown');
      expect(result.error.message).toBe('Something went wrong');
    }
  });

  it('should handle JavaScript Error objects', async () => {
    tauriMock.mockCommandError('broken_command', new Error('JavaScript error'));

    const result = await safeInvoke<void>('broken_command');

    expect(result.ok).toBe(false);
    if (!result.ok) {
      expect(result.error.kind).toBe('unknown');
      expect(result.error.message).toBe('JavaScript error');
    }
  });

  it('should handle completely unknown error types', async () => {
    tauriMock.mockCommandError('weird_command', 'plain string error');

    const result = await safeInvoke<void>('weird_command');

    expect(result.ok).toBe(false);
    if (!result.ok) {
      expect(result.error.kind).toBe('unknown');
      expect(result.error.message).toBe('plain string error');
    }
  });

  it('should pass arguments to the command', async () => {
    tauriMock.mockCommand('test_command', { success: true });

    await safeInvoke('test_command', { arg1: 'value1', arg2: 42 });

    expect(mockInvoke).toHaveBeenCalledWith('test_command', { arg1: 'value1', arg2: 42 });
  });

  it('should handle commands with no arguments', async () => {
    tauriMock.mockCommand('no_args_command', { result: 'ok' });

    await safeInvoke('no_args_command');

    expect(mockInvoke).toHaveBeenCalledWith('no_args_command', {});
  });
});

describe('invokeOrThrow', () => {
  beforeEach(() => {
    tauriMock.reset();
  });

  it('should return data when command succeeds', async () => {
    const mockData = { id: '456', value: 'success' };
    tauriMock.mockCommand('get_data', mockData);

    const data = await invokeOrThrow<typeof mockData>('get_data');

    expect(data).toEqual(mockData);
  });

  it('should throw normalized error when command fails', async () => {
    const error = { NotFound: 'Resource not found' };
    tauriMock.mockCommandError('get_missing', error);

    await expect(invokeOrThrow('get_missing')).rejects.toMatchObject({
      kind: 'not_found',
      message: 'Resource not found'
    });
  });

  it('should throw with validation fields when validation fails', async () => {
    const error = {
      ValidationError: {
        field1: 'Error 1',
        field2: 'Error 2'
      }
    };
    tauriMock.mockCommandError('validate_form', error);

    await expect(invokeOrThrow('validate_form')).rejects.toMatchObject({
      kind: 'validation',
      message: 'Validation failed',
      fields: {
        field1: 'Error 1',
        field2: 'Error 2'
      }
    });
  });
});
