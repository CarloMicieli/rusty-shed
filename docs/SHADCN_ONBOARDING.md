# Getting Started with shadcn-svelte

**Developer Onboarding Guide for Rusty Shed**  
**Feature**: 012-shadcn-migration  
**Created**: 2026-02-05

## Overview

Rusty Shed uses [shadcn-svelte](https://www.shadcn-svelte.com/) v1.1.1 as its UI component library. This guide will help you get started with using and customizing these components.

## Quick Start

### 1. Import Components

Components can be imported from the central component index:

```svelte
<script lang="ts">
  import { Button, Badge, Input, Dialog } from '$lib/components';
</script>

<Button variant="default">Click me</Button>
<Badge variant="secondary">New</Badge>
```

### 2. Use Svelte 5 Runes

All components are built with Svelte 5 Runes. Make sure to use the new reactive syntax:

```svelte
<script lang="ts">
  import { Button, Input } from '$lib/components';
  
  // Use $state for reactive variables
  let name = $state('');
  
  // Use $derived for computed values
  const greeting = $derived(`Hello, ${name}!`);
  
  function handleClick() {
    console.log('Button clicked!');
  }
</script>

<Input bind:value={name} placeholder="Enter your name" />
<p>{greeting}</p>
<Button onclick={handleClick}>Submit</Button>
```

### 3. Apply the Steampunk Theme

All components automatically use the Steampunk theme via CSS variables:

- **Primary colors**: Brass/Copper tones (`--color-primary-*`)
- **Surface colors**: Dark/Light backgrounds (`--color-surface-*`)
- **Accent colors**: Furnace orange (`--color-accent-*`)

No manual theme configuration needed—just use the components!

## Component Reference

### Button

**Variants**: `default`, `destructive`, `outline`, `secondary`, `ghost`, `link`  
**Sizes**: `default`, `sm`, `lg`, `icon`

```svelte
<Button variant="default" size="sm" onclick={handleClick}>
  Save Changes
</Button>

<!-- Link button (renders as <a> tag) -->
<Button href="/settings" variant="link">Go to Settings</Button>

<!-- Icon button -->
<Button variant="ghost" size="icon" aria-label="Delete">
  <TrashIcon />
</Button>
```

**Props**:
- `variant?: ButtonVariant` - Visual style
- `size?: ButtonSize` - Button size
- `disabled?: boolean` - Disable interaction
- `href?: string` - Render as link
- `onclick?: (e: MouseEvent) => void` - Click handler
- `aria-label?: string` - Accessibility label
- `class?: string` - Additional CSS classes

### Badge

**Variants**: `default`, `secondary`, `destructive`, `outline`, `success`

```svelte
<Badge variant="success">Active</Badge>
<Badge variant="destructive">Error</Badge>
<Badge variant="outline">Pending</Badge>
```

**Props**:
- `variant?: BadgeVariant` - Visual style
- `class?: string` - Additional CSS classes

### Input

**Supports all standard HTML input types** (`text`, `email`, `password`, `number`, `date`, etc.)

```svelte
<script lang="ts">
  let email = $state('');
  let password = $state('');
</script>

<Input
  type="email"
  bind:value={email}
  placeholder="Enter your email"
  required
/>

<Input
  type="password"
  bind:value={password}
  placeholder="Password"
  aria-label="Password input"
/>
```

**Props**:
- `type?: string` - Input type
- `value?: string | number` - Input value (use `bind:value`)
- `placeholder?: string` - Placeholder text
- `disabled?: boolean` - Disable input
- `readonly?: boolean` - Read-only state
- `required?: boolean` - Required field
- `oninput?: (e: Event) => void` - Input event handler
- `onchange?: (e: Event) => void` - Change event handler
- `class?: string` - Additional CSS classes

### Textarea

```svelte
<script lang="ts">
  let description = $state('');
</script>

<Textarea
  bind:value={description}
  placeholder="Enter description"
  rows={5}
/>
```

**Props**: Same as Input (except `type`)

### Checkbox

```svelte
<script lang="ts">
  let agreed = $state(false);
</script>

<div class="flex items-center gap-2">
  <Checkbox bind:checked={agreed} id="terms" />
  <label for="terms">I agree to the terms and conditions</label>
</div>
```

**Props**:
- `checked?: boolean` - Checked state (use `bind:checked`)
- `disabled?: boolean` - Disable checkbox
- `aria-label?: string` - Accessibility label
- `class?: string` - Additional CSS classes

### Dialog (Modal)

```svelte
<script lang="ts">
  import { Dialog } from '$lib/components';
  
  let open = $state(false);
</script>

<Button onclick={() => open = true}>Open Dialog</Button>

<Dialog bind:open={open}>
  <div class="p-6 space-y-4">
    <h2 id="dialog-title" class="text-lg font-semibold">Confirm Action</h2>
    <p id="dialog-description">Are you sure you want to proceed?</p>
    <div class="flex gap-2 justify-end">
      <Button variant="ghost" onclick={() => open = false}>Cancel</Button>
      <Button variant="default" onclick={() => open = false}>Confirm</Button>
    </div>
  </div>
</Dialog>
```

**Props**:
- `open?: boolean` - Open state (use `bind:open`)
- `onOpenChange?: (open: boolean) => void` - Open state change handler
- `aria-labelledby?: string` - ID of title element
- `aria-describedby?: string` - ID of description element
- `class?: string` - Additional CSS classes

### Table

```svelte
<script lang="ts">
  import { Table, TableHeader, TableBody, TableHead, TableRow, TableCell } from '$lib/components';
  
  const users = [
    { id: 1, name: 'Alice', email: 'alice@example.com' },
    { id: 2, name: 'Bob', email: 'bob@example.com' }
  ];
</script>

<Table>
  <TableHeader>
    <TableRow>
      <TableHead>Name</TableHead>
      <TableHead>Email</TableHead>
    </TableRow>
  </TableHeader>
  <TableBody>
    {#each users as user (user.id)}
      <TableRow>
        <TableCell>{user.name}</TableCell>
        <TableCell>{user.email}</TableCell>
      </TableRow>
    {/each}
  </TableBody>
</Table>
```

### Card

```svelte
<script lang="ts">
  import { Card, CardHeader, CardTitle, CardDescription, CardContent, CardFooter } from '$lib/components';
</script>

<Card>
  <CardHeader>
    <CardTitle>Settings</CardTitle>
    <CardDescription>Manage your account settings</CardDescription>
  </CardHeader>
  <CardContent>
    <!-- Content here -->
  </CardContent>
  <CardFooter>
    <Button variant="default">Save</Button>
  </CardFooter>
</Card>
```

## Customization Patterns

### 1. Extending Component Styles

Use the `class` prop to add Tailwind utility classes:

```svelte
<Button class="w-full mt-4">Full Width Button</Button>

<Badge class="text-xs font-mono">Custom Badge</Badge>
```

### 2. Component Composition

Build complex UIs by composing smaller components:

```svelte
<script lang="ts">
  import { Card, CardHeader, CardTitle, CardContent, Input, Button } from '$lib/components';
  
  let email = $state('');
  
  function handleSubmit() {
    console.log('Email:', email);
  }
</script>

<Card class="max-w-md">
  <CardHeader>
    <CardTitle>Subscribe</CardTitle>
  </CardHeader>
  <CardContent class="space-y-4">
    <Input type="email" bind:value={email} placeholder="Enter email" />
    <Button class="w-full" onclick={handleSubmit}>Subscribe</Button>
  </CardContent>
</Card>
```

### 3. Theming & Dark Mode

The Steampunk theme automatically switches between light and dark modes based on user preference. Theme switching is managed by the theme store:

```svelte
<script lang="ts">
  import { themeStore } from '$lib/stores/themeStore.svelte';
</script>

<Button onclick={() => themeStore.toggle()}>
  Toggle Theme
</Button>
```

### 4. Accessibility Best Practices

Always include proper ARIA labels and semantic HTML:

```svelte
<!-- Good: Proper labeling -->
<Button aria-label="Delete item" variant="destructive">
  <TrashIcon />
</Button>

<!-- Good: Associated label -->
<label for="username">Username</label>
<Input id="username" bind:value={username} />

<!-- Good: Dialog with proper ARIA -->
<Dialog 
  bind:open={dialogOpen} 
  aria-labelledby="dialog-title"
  aria-describedby="dialog-desc"
>
  <h2 id="dialog-title">Title</h2>
  <p id="dialog-desc">Description</p>
</Dialog>
```

## Common Tasks

### Adding a New Feature with shadcn-svelte

1. **Create a new route/component** (e.g., `src/routes/my-feature/+page.svelte`)
2. **Import needed components** from `$lib/components`
3. **Use Svelte 5 Runes** for reactivity (`$state`, `$derived`, `$props`)
4. **Apply Steampunk theme** automatically via CSS variables
5. **Test accessibility** with keyboard navigation and screen readers

### Example: Creating a Settings Form

```svelte
<script lang="ts">
  import { Card, CardHeader, CardTitle, CardContent, Input, Checkbox, Button } from '$lib/components';
  
  let settings = $state({
    username: '',
    email: '',
    notifications: false
  });
  
  function handleSave() {
    console.log('Saving settings:', settings);
  }
</script>

<Card>
  <CardHeader>
    <CardTitle>Account Settings</CardTitle>
  </CardHeader>
  <CardContent class="space-y-4">
    <div>
      <label for="username" class="block text-sm font-medium mb-1">Username</label>
      <Input id="username" bind:value={settings.username} />
    </div>
    
    <div>
      <label for="email" class="block text-sm font-medium mb-1">Email</label>
      <Input id="email" type="email" bind:value={settings.email} />
    </div>
    
    <div class="flex items-center gap-2">
      <Checkbox bind:checked={settings.notifications} id="notifications" />
      <label for="notifications">Enable notifications</label>
    </div>
    
    <Button onclick={handleSave}>Save Settings</Button>
  </CardContent>
</Card>
```

## TypeScript Support

All components include TypeScript types. Use them for better IDE support:

```typescript
import type { ButtonVariant, BadgeVariant } from '$lib/components/shadcn/button/Button.svelte';

let variant: ButtonVariant = 'default';
```

## Testing Components

Use Vitest and Testing Library to test your components:

```typescript
import { render, fireEvent } from '@testing-library/svelte';
import { Button } from '$lib/components';

test('Button renders and handles clicks', async () => {
  let clicked = false;
  const { getByRole } = render(Button, {
    props: {
      onclick: () => clicked = true
    },
    children: 'Click me'
  });
  
  const button = getByRole('button');
  await fireEvent.click(button);
  
  expect(clicked).toBe(true);
});
```

## Resources

- **shadcn-svelte Documentation**: https://www.shadcn-svelte.com/
- **Svelte 5 Runes**: https://svelte-5-preview.vercel.app/docs/runes
- **Tailwind CSS**: https://tailwindcss.com/docs
- **Component Source**: `/src/lib/components/shadcn/`
- **Theme Files**: `/src/lib/themes/`

## Getting Help

- Check the component source code in `src/lib/components/shadcn/`
- Review the shadcn-svelte README: `src/lib/components/shadcn/README.md`
- Refer to existing usage in the codebase (search for component imports)
- Ask the team for clarification on Steampunk theme customizations

---

**Happy coding!** 🚂⚙️
