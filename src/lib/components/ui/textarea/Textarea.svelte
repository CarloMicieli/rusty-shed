<script lang="ts">
  /**
   * Textarea Component (shadcn-svelte compatible)
   * Replaces Skeleton's textarea classes with a proper component
   * Supports Steampunk theme
   *
   * Feature: 012-shadcn-migration
   */
  import { twMerge } from 'tailwind-merge';

  type Props = {
    value?: string | null;
    placeholder?: string;
    disabled?: boolean;
    readonly?: boolean;
    required?: boolean;
    rows?: number;
    class?: string;
    id?: string;
    name?: string;
    maxlength?: number;
    oninput?: (e: Event & { currentTarget: HTMLTextAreaElement }) => void;
    onchange?: (e: Event & { currentTarget: HTMLTextAreaElement }) => void;
    onblur?: (e: FocusEvent & { currentTarget: HTMLTextAreaElement }) => void;
    onfocus?: (e: FocusEvent & { currentTarget: HTMLTextAreaElement }) => void;
  };

  let {
    value = $bindable(''),
    placeholder = '',
    disabled = false,
    readonly = false,
    required = false,
    rows = 3,
    class: className = '',
    id,
    name,
    maxlength,
    oninput,
    onchange,
    onblur,
    onfocus
  }: Props = $props();

  const baseStyles =
    'flex min-h-[80px] w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50 resize-y';

  const textareaClass = $derived(twMerge(baseStyles, className));

  function handleInput(e: Event & { currentTarget: HTMLTextAreaElement }) {
    value = e.currentTarget.value;
    oninput?.(e);
  }

  function handleChange(e: Event & { currentTarget: HTMLTextAreaElement }) {
    value = e.currentTarget.value;
    onchange?.(e);
  }
</script>

<textarea
  {value}
  oninput={handleInput}
  onchange={handleChange}
  {placeholder}
  {disabled}
  {readonly}
  {required}
  {rows}
  {id}
  {name}
  {maxlength}
  class={textareaClass}
  {onblur}
  {onfocus}
></textarea>
