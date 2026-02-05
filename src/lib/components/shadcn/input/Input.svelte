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
    type?:
      | 'text'
      | 'email'
      | 'password'
      | 'number'
      | 'tel'
      | 'url'
      | 'search'
      | 'date'
      | 'time'
      | 'datetime-local';
    value?: string | number | null;
    placeholder?: string;
    disabled?: boolean;
    readonly?: boolean;
    required?: boolean;
    class?: string;
    id?: string;
    name?: string;
    autocomplete?: AutoFill;
    min?: string | number;
    max?: string | number;
    step?: string | number;
    autofocus?: boolean;
    autoFocus?: boolean; // Alias for React-style compatibility
    'aria-describedby'?: string;
    onkeydown?: (e: KeyboardEvent & { currentTarget: HTMLInputElement }) => void;
    oninput?: (e: Event & { currentTarget: HTMLInputElement }) => void;
    onchange?: (e: Event & { currentTarget: HTMLInputElement }) => void;
    onblur?: (e: FocusEvent & { currentTarget: HTMLInputElement }) => void;
    onfocus?: (e: FocusEvent & { currentTarget: HTMLInputElement }) => void;
  };

  type AutoFill = HTMLInputElement['autocomplete'];

  let {
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
    min,
    max,
    step,
    autofocus,
    autoFocus,
    'aria-describedby': ariaDescribedby,
    onkeydown,
    oninput,
    onchange,
    onblur,
    onfocus
  }: Props = $props();

  // Support both autofocus and autoFocus (React-style)
  const shouldAutofocus = $derived(autofocus || autoFocus);

  const baseStyles =
    'flex h-10 w-full rounded-md border border-surface-600 bg-surface-800 px-3 py-2 text-sm ring-offset-background file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-surface-400 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-primary-600 focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50';

  const inputClass = $derived(twMerge(baseStyles, className));

  function handleInput(e: Event & { currentTarget: HTMLInputElement }) {
    value = e.currentTarget.value;
    oninput?.(e);
  }

  function handleChange(e: Event & { currentTarget: HTMLInputElement }) {
    value = e.currentTarget.value;
    onchange?.(e);
  }
</script>

<!-- svelte-ignore a11y_autofocus -->
<input
  {type}
  {value}
  oninput={handleInput}
  onchange={handleChange}
  {placeholder}
  {disabled}
  {readonly}
  {required}
  {id}
  {name}
  {autocomplete}
  {min}
  {max}
  {step}
  autofocus={shouldAutofocus}
  aria-describedby={ariaDescribedby}
  class={inputClass}
  {onkeydown}
  {onblur}
  {onfocus}
/>
