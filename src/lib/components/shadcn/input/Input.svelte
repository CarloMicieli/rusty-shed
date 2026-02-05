<script lang="ts">
  /**
   * Input Component (shadcn-svelte compatible)
   * Replaces Skeleton's input classes with a proper component
   * Supports Steampunk theme
   * 
   * Feature: 012-shadcn-migration
   */
  import { twMerge } from 'tailwind-merge';

  type Props = {
    type?: 'text' | 'email' | 'password' | 'number' | 'tel' | 'url' | 'search';
    value?: string | number;
    placeholder?: string;
    disabled?: boolean;
    readonly?: boolean;
    required?: boolean;
    class?: string;
    id?: string;
    name?: string;
    autocomplete?: string;
    oninput?: (e: Event & { currentTarget: HTMLInputElement }) => void;
    onchange?: (e: Event & { currentTarget: HTMLInputElement }) => void;
    onblur?: (e: FocusEvent & { currentTarget: HTMLInputElement }) => void;
    onfocus?: (e: FocusEvent & { currentTarget: HTMLInputElement }) => void;
  };

  const {
    type = 'text',
    value = $bindable(''),
    placeholder = '',
    disabled = false,
    readonly = false,
    required = false,
    class: className = '',
    id,
    name,
    autocomplete,
    oninput,
    onchange,
    onblur,
    onfocus
  }: Props = $props();

  const baseStyles = 'flex h-10 w-full rounded-md border border-surface-600 bg-surface-800 px-3 py-2 text-sm ring-offset-background file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-surface-400 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-primary-600 focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50';

  const inputClass = $derived(twMerge(baseStyles, className));
</script>

<input
  {type}
  bind:value
  {placeholder}
  {disabled}
  {readonly}
  {required}
  {id}
  {name}
  {autocomplete}
  class={inputClass}
  oninput={oninput}
  onchange={onchange}
  onblur={onblur}
  onfocus={onfocus}
/>
