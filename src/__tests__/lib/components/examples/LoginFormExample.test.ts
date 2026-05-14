import { describe, expect, it, vi } from 'vitest';
import { fireEvent, render, screen } from '@testing-library/svelte';
import LoginFormExample from '$lib/components/examples/LoginFormExample.svelte';

describe('LoginFormExample', () => {
  it('keeps submit disabled until email and password become valid', async () => {
    render(LoginFormExample);

    const email = screen.getByLabelText('Email');
    const password = screen.getByLabelText('Password');
    const submit = screen.getByRole('button', { name: 'Sign In' });

    expect(submit).toBeDisabled();

    await fireEvent.input(email, { target: { value: 'invalid' } });
    await fireEvent.input(password, { target: { value: '1234' } });
    expect(submit).toBeDisabled();

    await fireEvent.input(email, { target: { value: 'user@example.com' } });
    await fireEvent.input(password, { target: { value: 'password123' } });
    expect(submit).toBeEnabled();
  });

  it('shows validation messages for invalid input and submits on Enter when valid', async () => {
    const logSpy = vi.spyOn(console, 'log').mockImplementation(() => {});
    render(LoginFormExample);

    const email = screen.getByLabelText('Email');
    const password = screen.getByLabelText('Password');

    await fireEvent.input(email, { target: { value: 'bad' } });
    await fireEvent.input(password, { target: { value: '123' } });

    expect(screen.getByText('Please enter a valid email address')).toBeInTheDocument();
    expect(screen.getByText('Password must be at least 8 characters')).toBeInTheDocument();

    await fireEvent.input(email, { target: { value: 'ok@example.com' } });
    await fireEvent.input(password, { target: { value: 'longenough' } });
    await fireEvent.keyDown(password, { key: 'Enter' });

    expect(logSpy).toHaveBeenCalledWith('Form submitted:', {
      email: 'ok@example.com',
      password: 'longenough',
      rememberMe: false
    });
  });
});
