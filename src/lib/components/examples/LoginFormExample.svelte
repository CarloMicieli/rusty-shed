<!--
  Example: Simple Form with shadcn-svelte components

  This example demonstrates:
  - Form input handling with Svelte 5 Runes
  - Component composition
  - Validation
  - Accessibility best practices
-->
<script lang="ts">
  import {
    Button,
    Input,
    Checkbox,
    Card,
    CardHeader,
    CardTitle,
    CardContent
  } from '$lib/components';

  // Form state using $state rune
  let form = $state({
    email: '',
    password: '',
    rememberMe: false
  });

  // Validation using $derived rune
  const isValidEmail = $derived(form.email.length > 0 && form.email.includes('@'));

  const isValidPassword = $derived(form.password.length >= 8);

  const canSubmit = $derived(isValidEmail && isValidPassword);

  // Form submission handler
  function handleSubmit() {
    if (!canSubmit) return;

    console.log('Form submitted:', form);
    // Handle authentication here
  }

  function handleKeyDown(e: KeyboardEvent) {
    if (e.key === 'Enter' && canSubmit) {
      handleSubmit();
    }
  }
</script>

<div class="flex min-h-screen items-center justify-center p-4">
  <Card class="w-full max-w-md">
    <CardHeader>
      <CardTitle>Login</CardTitle>
    </CardHeader>
    <CardContent class="space-y-4">
      <!-- Email Field -->
      <div class="space-y-1">
        <label for="email" class="block text-sm font-medium"> Email </label>
        <Input
          id="email"
          type="email"
          bind:value={form.email}
          placeholder="you@example.com"
          required
          aria-invalid={form.email.length > 0 && !isValidEmail}
          aria-describedby="email-error"
          onkeydown={handleKeyDown}
        />
        {#if form.email.length > 0 && !isValidEmail}
          <p id="email-error" class="text-error-600 text-sm">Please enter a valid email address</p>
        {/if}
      </div>

      <!-- Password Field -->
      <div class="space-y-1">
        <label for="password" class="block text-sm font-medium"> Password </label>
        <Input
          id="password"
          type="password"
          bind:value={form.password}
          placeholder="Enter your password"
          required
          aria-invalid={form.password.length > 0 && !isValidPassword}
          aria-describedby="password-error"
          onkeydown={handleKeyDown}
        />
        {#if form.password.length > 0 && !isValidPassword}
          <p id="password-error" class="text-error-600 text-sm">
            Password must be at least 8 characters
          </p>
        {/if}
      </div>

      <!-- Remember Me Checkbox -->
      <div class="flex items-center gap-2">
        <Checkbox bind:checked={form.rememberMe} id="remember" aria-label="Remember me" />
        <label for="remember" class="text-sm"> Remember me </label>
      </div>

      <!-- Submit Button -->
      <Button class="w-full" disabled={!canSubmit} onclick={handleSubmit}>Sign In</Button>
    </CardContent>
  </Card>
</div>
