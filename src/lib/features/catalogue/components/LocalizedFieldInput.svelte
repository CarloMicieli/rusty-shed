<script lang="ts">
  import * as m from '$lib/paraglide/messages.js';
  import { Textarea } from '$lib/components/ui/textarea';
  import { Input } from '$lib/components/ui/input';
  import { Label } from '$lib/components/ui/label';

  interface Props {
    lang: 'en' | 'it';
    label: string;
    value: string | null;
    required?: boolean;
    placeholder?: string;
    rows?: number;
    singleLine?: boolean;
  }

  let {
    lang,
    label,
    value = $bindable(null),
    required = false,
    placeholder = '',
    rows = 3,
    singleLine = false
  }: Props = $props();

  const langLabel = $derived(
    lang === 'en' ? m.translation_section_english() : m.translation_section_italian()
  );

  const requiredLabel = $derived(
    required ? m.translation_section_required() : m.translation_section_optional()
  );
</script>

<div class="flex flex-col gap-1.5">
  <div class="flex items-center gap-2">
    <Label class="text-sm font-medium">{label}</Label>
    <span
      class="rounded px-1.5 py-0.5 text-xs font-semibold"
      class:bg-amber-500={lang === 'en'}
      class:text-black={lang === 'en'}
      class:bg-zinc-700={lang === 'it'}
      class:text-zinc-300={lang === 'it'}
    >
      {langLabel}
    </span>
    {#if required}
      <span class="text-xs text-red-400">{requiredLabel}</span>
    {:else}
      <span class="text-xs text-zinc-500">{requiredLabel}</span>
    {/if}
  </div>
  {#if singleLine}
    <Input
      type="text"
      value={value ?? ''}
      oninput={(e) => (value = e.currentTarget.value || null)}
      {placeholder}
    />
  {:else}
    <Textarea bind:value {placeholder} {rows} class="resize-none" />
  {/if}
</div>
