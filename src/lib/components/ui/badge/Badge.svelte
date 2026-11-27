<script lang="ts">
  /**
   * Badge Component (shadcn-svelte compatible)
   * Replaces Skeleton's badge classes with a proper component
   * Supports Steampunk theme variants
   *
   * Feature: 012-shadcn-migration
   */
  import { twMerge } from 'tailwind-merge';

  export type BadgeVariant = 'default' | 'secondary' | 'destructive' | 'outline' | 'success';

  type Props = {
    variant?: BadgeVariant;
    class?: string;
    onclick?: (e: MouseEvent) => void;
    title?: string;
    children?: import('svelte').Snippet;
  };

  const { variant = 'default', class: className = '', onclick, title, children }: Props = $props();

  const baseStyles =
    'inline-flex items-center gap-1 rounded-full border px-2.5 py-0.5 text-xs font-semibold transition-colors focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2';

  const variantStyles: Record<BadgeVariant, string> = {
    default: 'border-transparent bg-primary text-primary-foreground hover:bg-primary/90',
    secondary: 'border-transparent bg-secondary text-secondary-foreground hover:bg-secondary/80',
    destructive:
      'border-transparent bg-destructive text-destructive-foreground hover:bg-destructive/90',
    outline: 'border-border text-foreground',
    success: 'border-transparent bg-success-600 text-white'
  };

  const badgeClass = $derived(twMerge(baseStyles, variantStyles[variant], className));
</script>

{#if onclick}
  <button type="button" class={badgeClass} {onclick} {title}>
    {#if children}
      {@render children()}
    {/if}
  </button>
{:else}
  <span class={badgeClass} {title}>
    {#if children}
      {@render children()}
    {/if}
  </span>
{/if}
