<script lang="ts">
  /**
   * Toggle Component (shadcn-svelte compatible)
   * Toggle button/switch styled with Steampunk theme
   *
   * Feature: 012-shadcn-migration
   * Task: T047 - Create Toggle component
   */
  import { twMerge } from 'tailwind-merge';

  type Props = {
    pressed?: boolean;
    disabled?: boolean;
    class?: string;
    'aria-label'?: string;
    size?: 'default' | 'sm' | 'lg';
    onclick?: (pressed: boolean) => void;
  };

  let {
    pressed = $bindable(false),
    disabled = false,
    class: className = '',
    'aria-label': ariaLabel,
    size = 'default',
    onclick
  }: Props = $props();

  const sizeStyles = {
    sm: 'h-8 px-2 text-xs',
    default: 'h-10 px-3 text-sm',
    lg: 'h-11 px-4 text-base'
  };

  const baseStyles = `inline-flex items-center justify-center rounded-md font-medium ring-offset-background transition-colors hover:bg-surface-700 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-primary-600 focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50`;

  const variantStyles = $derived(
    pressed ? 'bg-primary-600 text-primary-foreground' : 'bg-transparent border border-surface-600'
  );

  const buttonClass = $derived(twMerge(baseStyles, sizeStyles[size], variantStyles, className));

  function handleClick() {
    if (disabled) return;
    pressed = !pressed;
    onclick?.(pressed);
  }
</script>

<button
  type="button"
  role="switch"
  aria-checked={pressed}
  aria-label={ariaLabel}
  {disabled}
  class={buttonClass}
  onclick={handleClick}
>
  <slot />
</button>
