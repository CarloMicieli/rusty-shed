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
    onclick?: (e: MouseEvent) => void;
    children?: any;
  };

  const {
    variant = 'default',
    size = 'default',
    class: className = '',
    type = 'button',
    disabled = false,
    onclick,
    children
  }: Props = $props();

  const baseStyles = 'inline-flex items-center justify-center gap-2 rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50';

  const variantStyles: Record<ButtonVariant, string> = {
    default: 'bg-primary-600 text-white hover:bg-primary-700 active:bg-primary-800',
    destructive: 'bg-error-600 text-white hover:bg-error-700 active:bg-error-800',
    outline: 'border border-surface-600 bg-transparent hover:bg-surface-700 hover:text-surface-50',
    secondary: 'bg-secondary-600 text-white hover:bg-secondary-700 active:bg-secondary-800',
    ghost: 'hover:bg-surface-700 hover:text-surface-50',
    link: 'text-primary-600 underline-offset-4 hover:underline'
  };

  const sizeStyles: Record<ButtonSize, string> = {
    default: 'h-10 px-4 py-2',
    sm: 'h-9 rounded-md px-3',
    lg: 'h-11 rounded-md px-8',
    icon: 'h-10 w-10'
  };

  const buttonClass = twMerge(
    baseStyles,
    variantStyles[variant],
    sizeStyles[size],
    className
  );
</script>

<button
  {type}
  class={buttonClass}
  {disabled}
  onclick={onclick}
>
  {#if children}
    {@render children()}
  {/if}
</button>
