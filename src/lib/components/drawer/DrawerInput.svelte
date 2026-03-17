<script lang="ts">
  import { cn } from '$lib/utils';

  interface Props {
    id?: string;
    type?: string;
    min?: string | number;
    max?: string | number;
    value?: string | number | null;
    placeholder?: string;
    disabled?: boolean;
    class?: string;
    'aria-invalid'?: boolean | 'true' | 'false';
    'aria-describedby'?: string;
    oninput?: (e: Event & { currentTarget: HTMLInputElement }) => void;
  }

  let {
    class: className,
    value = $bindable<string | number | null>(''),
    oninput,
    ...rest
  }: Props = $props();
</script>

<input
  bind:value
  {oninput}
  class={cn(
    'flex h-10 w-full rounded-[8px] border border-[#1F1F1F] bg-[#0F0F0F] px-3 py-2 text-sm text-[#E0E0E0] placeholder:text-[#808080] focus-visible:border-[#D48A42]/60 focus-visible:ring-2 focus-visible:ring-[#D48A42]/20 focus-visible:outline-none disabled:cursor-not-allowed disabled:opacity-50',
    className
  )}
  {...rest}
/>
