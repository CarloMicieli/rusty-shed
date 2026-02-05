<script lang="ts">
  /**
   * Select Component (shadcn-svelte compatible)
   * Native HTML select styled with Steampunk theme
   *
   * Feature: 012-shadcn-migration
   * Task: T041 - Create select/dropdown components
   */
  import { twMerge } from 'tailwind-merge';

  type Props = {
    value?: string | number;
    disabled?: boolean;
    required?: boolean;
    class?: string;
    id?: string;
    name?: string;
    onchange?: (e: Event & { currentTarget: HTMLSelectElement }) => void;
    children?: import('svelte').Snippet;
  };

  let {
    value = $bindable(''),
    disabled = false,
    required = false,
    class: className = '',
    id,
    name,
    onchange,
    children
  }: Props = $props();

  const baseStyles =
    'flex h-10 w-full rounded-md border border-surface-600 bg-surface-800 px-3 py-2 text-sm ring-offset-background focus:outline-none focus:ring-2 focus:ring-primary-600 focus:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50';

  const selectClass = $derived(twMerge(baseStyles, className));

  function handleChange(e: Event & { currentTarget: HTMLSelectElement }) {
    value = e.currentTarget.value;
    onchange?.(e);
  }
</script>

<select {value} onchange={handleChange} {disabled} {required} {id} {name} class={selectClass}>
  {@render children?.()}
</select>
