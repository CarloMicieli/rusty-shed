<script lang="ts">
  import type { Language } from '$lib/bindings';
  import LanguageFallbackBadge from '$lib/components/LanguageFallbackBadge.svelte';
  import RichTextEditor from '$lib/components/RichTextEditor.svelte';
  import * as m from '$lib/paraglide/messages';

  interface Props {
    details: string;
    detailsLang: Language | null;
    currentLocale: Language;
    editable: boolean;
    onSave: (value: string) => Promise<void>;
  }

  const { details, detailsLang, currentLocale, editable, onSave }: Props = $props();
</script>

<div class="rounded-lg border border-border bg-card/40 p-4">
  {#if detailsLang && detailsLang !== currentLocale}
    <div class="mb-2 flex items-center gap-1 text-xs text-zinc-500">
      <span>{m.railway_model_field_details()}</span>
      <LanguageFallbackBadge lang={detailsLang} />
    </div>
  {/if}
  <RichTextEditor value={details} {editable} placeholder={m.details_placeholder()} {onSave} />
</div>
