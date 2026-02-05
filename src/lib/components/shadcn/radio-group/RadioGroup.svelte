<script lang="ts">
  /**
   * RadioGroup Component (shadcn-svelte compatible)
   * Accessible radio button group with Steampunk theme
   * 
   * Feature: 012-shadcn-migration
   * Task: T046 - Create RadioGroup component
   */
  import { twMerge } from 'tailwind-merge';

  type Option = {
    value: string;
    label: string;
    disabled?: boolean;
  };

  type Props = {
    value?: string;
    options: Option[];
    name: string;
    disabled?: boolean;
    class?: string;
    onchange?: (value: string) => void;
  };

  let {
    value = $bindable(''),
    options,
    name,
    disabled = false,
    class: className = '',
    onchange
  }: Props = $props();

  const groupClass = $derived(twMerge('flex flex-col gap-3', className));

  function handleChange(newValue: string) {
    value = newValue;
    onchange?.(newValue);
  }
</script>

<div class={groupClass} role="radiogroup">
  {#each options as option}
    <label class="flex items-center gap-2 cursor-pointer">
      <input
        type="radio"
        {name}
        value={option.value}
        checked={value === option.value}
        disabled={disabled || option.disabled}
        onchange={() => handleChange(option.value)}
        class="h-4 w-4 border-surface-600 text-primary-600 focus:ring-2 focus:ring-primary-600 focus:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50"
      />
      <span class="text-sm {disabled || option.disabled ? 'opacity-50' : ''}">
        {option.label}
      </span>
    </label>
  {/each}
</div>
