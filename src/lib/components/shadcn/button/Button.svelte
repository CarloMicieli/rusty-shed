<script lang="ts">
  /**
   * Button Component (shadcn-svelte compatible)
   * Replaces Skeleton's btn classes with a proper component
   * Supports Steampunk theme variants
   *
   * Feature: 012-shadcn-migration
   */
  import { twMerge } from 'tailwind-merge';

  type ButtonVariant = 'default' | 'destructive' | 'outline' | 'secondary' | 'ghost' | 'link';
  type ButtonSize = 'default' | 'sm' | 'lg' | 'icon';

  type Props = {
    variant?: ButtonVariant;
    size?: ButtonSize;
    class?: string;
    type?: 'button' | 'submit' | 'reset';
    disabled?: boolean;
    href?: string;
    onclick?: (e: MouseEvent) => void;
    'aria-label'?: string;
    title?: string;
    children?: import('svelte').Snippet;
  };

  const {
    variant = 'default',
    size = 'default',
    class: className = '',
    type = 'button',
    disabled = false,
    href,
    onclick,
    'aria-label': ariaLabel,
    title,
    children
  }: Props = $props();

  const baseStyles =
    'inline-flex items-center justify-center gap-2 rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50';

  const variantStyles: Record<ButtonVariant, string> = {
    default: 'bg-primary text-primary-foreground hover:bg-primary/90 active:bg-primary/80',
    destructive:
      'bg-destructive text-destructive-foreground hover:bg-destructive/90 active:bg-destructive/80',
    outline: 'border border-input bg-transparent hover:bg-accent hover:text-accent-foreground',
    secondary:
      'bg-secondary text-secondary-foreground hover:bg-secondary/80 active:bg-secondary/70',
    ghost: 'hover:bg-accent hover:text-accent-foreground',
    link: 'text-primary underline-offset-4 hover:underline'
  };

  const sizeStyles: Record<ButtonSize, string> = {
    default: 'h-10 px-4 py-2',
    sm: 'h-9 rounded-md px-3',
    lg: 'h-11 rounded-md px-8',
    icon: 'h-10 w-10'
  };

  const buttonClass = $derived(
    twMerge(baseStyles, variantStyles[variant], sizeStyles[size], className)
  );
</script>

{#if href}
  <a {href} class={buttonClass} aria-disabled={disabled} aria-label={ariaLabel} {onclick}>
    {#if children}
      {@render children()}
    {/if}
  </a>
{:else}
  <button {type} class={buttonClass} {disabled} aria-label={ariaLabel} {title} {onclick}>
    {#if children}
      {@render children()}
    {/if}
  </button>
{/if}
