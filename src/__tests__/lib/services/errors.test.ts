import { describe, it, expect } from 'vitest';
import {
	getErrorMessage,
	getToastMessage,
	isValidationError,
	isNotFoundError,
	isRetryableError,
	type NormalizedError
} from '$lib/services/errors';

describe('getErrorMessage', () => {
	it('should return message for non-validation errors', () => {
		const error: NormalizedError = {
			kind: 'database',
			message: 'Connection failed'
		};

		expect(getErrorMessage(error)).toBe('Connection failed');
	});

	it('should combine message and fields for validation errors', () => {
		const error: NormalizedError = {
			kind: 'validation',
			message: 'Validation failed',
			fields: {
				name: 'Name is required',
				email: 'Invalid email'
			}
		};

		const result = getErrorMessage(error);
		expect(result).toContain('Validation failed');
		expect(result).toContain('name: Name is required');
		expect(result).toContain('email: Invalid email');
	});

	it('should handle validation errors with empty fields', () => {
		const error: NormalizedError = {
			kind: 'validation',
			message: 'Validation failed',
			fields: {}
		};

		expect(getErrorMessage(error)).toBe('Validation failed');
	});

	it('should handle validation errors without fields property', () => {
		const error: NormalizedError = {
			kind: 'validation',
			message: 'Validation failed'
		};

		expect(getErrorMessage(error)).toBe('Validation failed');
	});
});

describe('getToastMessage', () => {
	it('should return short message for not_found', () => {
		const error: NormalizedError = {
			kind: 'not_found',
			message: 'Item with id 123 not found'
		};

		expect(getToastMessage(error)).toBe('Resource not found');
	});

	it('should return short message for validation', () => {
		const error: NormalizedError = {
			kind: 'validation',
			message: 'Multiple fields failed validation',
			fields: { name: 'Required' }
		};

		expect(getToastMessage(error)).toBe('Validation failed');
	});

	it('should return short message for permission_denied', () => {
		const error: NormalizedError = {
			kind: 'permission_denied',
			message: 'You do not have permission to delete this resource'
		};

		expect(getToastMessage(error)).toBe('Permission denied');
	});

	it('should return short message for database', () => {
		const error: NormalizedError = {
			kind: 'database',
			message: 'SQLSTATE[HY000]: General error: 1 disk I/O error'
		};

		expect(getToastMessage(error)).toBe('Database error occurred');
	});

	it('should return short message for unknown', () => {
		const error: NormalizedError = {
			kind: 'unknown',
			message: 'Something unexpected happened'
		};

		expect(getToastMessage(error)).toBe('An unexpected error occurred');
	});
});

describe('isValidationError', () => {
	it('should return true for validation errors', () => {
		const error: NormalizedError = {
			kind: 'validation',
			message: 'Validation failed',
			fields: { name: 'Required' }
		};

		expect(isValidationError(error)).toBe(true);
	});

	it('should return false for non-validation errors', () => {
		const error: NormalizedError = {
			kind: 'database',
			message: 'Connection failed'
		};

		expect(isValidationError(error)).toBe(false);
	});

	it('should narrow type when true', () => {
		const error: NormalizedError = {
			kind: 'validation',
			message: 'Validation failed',
			fields: { email: 'Invalid' }
		};

		if (isValidationError(error)) {
			// Type should be narrowed to include fields
			expect(error.fields.email).toBe('Invalid');
		}
	});
});

describe('isNotFoundError', () => {
	it('should return true for not_found errors', () => {
		const error: NormalizedError = {
			kind: 'not_found',
			message: 'Resource not found'
		};

		expect(isNotFoundError(error)).toBe(true);
	});

	it('should return false for other error kinds', () => {
		const errors: NormalizedError[] = [
			{ kind: 'database', message: 'DB error' },
			{ kind: 'validation', message: 'Validation error' },
			{ kind: 'permission_denied', message: 'Permission error' },
			{ kind: 'unknown', message: 'Unknown error' }
		];

		errors.forEach((error) => {
			expect(isNotFoundError(error)).toBe(false);
		});
	});
});

describe('isRetryableError', () => {
	it('should return true for database errors', () => {
		const error: NormalizedError = {
			kind: 'database',
			message: 'Connection timeout'
		};

		expect(isRetryableError(error)).toBe(true);
	});

	it('should return true for unknown errors', () => {
		const error: NormalizedError = {
			kind: 'unknown',
			message: 'Something went wrong'
		};

		expect(isRetryableError(error)).toBe(true);
	});

	it('should return false for non-retryable errors', () => {
		const errors: NormalizedError[] = [
			{ kind: 'not_found', message: 'Not found' },
			{ kind: 'validation', message: 'Validation failed', fields: {} },
			{ kind: 'permission_denied', message: 'Permission denied' }
		];

		errors.forEach((error) => {
			expect(isRetryableError(error)).toBe(false);
		});
	});
});
