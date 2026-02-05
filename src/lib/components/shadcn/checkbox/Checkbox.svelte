<script lang="ts">
  /**
   * Checkbox Component (shadcn-svelte compatible)
   * Replaces Skeleton's checkbox classes with a proper component
   * Supports Steampunk theme
   * 
   * Feature: 012-shadcn-migration
   */
  import { twMerge } from 'tailwind-merge';
  import { Check } from 'lucide-svelte';

  type Props = {
    checked?: boolean;
    disabled?: boolean;
    required?: boolean;
    class?: string;
    id?: string;
    name?: string;
    value?: string;
    onchange?: (e: Event & { currentTarget: HTMLInputElement }) => void;
  };

  const {
    checked = $bindable(false),
    disabled = false,
    required = false,
    class: className = '',
    id,
    name,
    value,
    onchange
  }: Props = $props();

  const baseStyles = 'peer h-4 w-4 shrink-0 rounded-sm border border-surface-600 ring-offset-background focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-primary-600 focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50 data-[state=checked]:bg-primary-600 data-[state=checked]:text-white';

  const checkboxClass = $derived(twMerge(baseStyles, className));
</script>

<div class="relative inline-flex items-center">
  <input
    type="checkbox"
    bind:checked
    {disabled}
    {required}
    {id}
    {name}
    {value}
    class="sr-only peer"
    onchange={onchange}
    data-state={checked ? 'checked' : 'unchecked'}
  />
  <div class={checkboxClass} data-state={checked ? 'checked' : 'unchecked'}>
    {#if checked}
      <Check class="h-4 w-4" />
    {/if}
  </div>
</div>
