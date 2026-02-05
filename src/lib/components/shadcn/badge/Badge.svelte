<script lang="ts">
  /**
   * Badge Component (shadcn-svelte compatible)
   * Replaces Skeleton's badge classes with a proper component
   * Supports Steampunk theme variants
   * 
   * Feature: 012-shadcn-migration
   */
  import { twMerge } from 'tailwind-merge';

  type BadgeVariant = 'default' | 'secondary' | 'destructive' | 'outline' | 'success';

  type Props = {
    variant?: BadgeVariant;
    class?: string;
    children?: any;
  };

  const {
    variant = 'default',
    class: className = '',
    children
  }: Props = $props();

  const baseStyles = 'inline-flex items-center gap-1 rounded-full border px-2.5 py-0.5 text-xs font-semibold transition-colors focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2';

  const variantStyles: Record<BadgeVariant, string> = {
    default: 'border-transparent bg-primary-600 text-white hover:bg-primary-700',
    secondary: 'border-transparent bg-secondary-600 text-white hover:bg-secondary-700',
    destructive: 'border-transparent bg-error-600 text-white hover:bg-error-700',
    outline: 'border-surface-600 text-surface-100',
    success: 'border-transparent bg-green-600 text-white hover:bg-green-700'
  };

  const badgeClass = twMerge(
    baseStyles,
    variantStyles[variant],
    className
  );
</script>

<span class={badgeClass}>
  {#if children}
    {@render children()}
  {/if}
</span>
