<script lang="ts">
  import * as m from '$lib/paraglide/messages.js';

  interface Props {
    /** The actual language of the content (e.g. "en") */
    lang: string;
  }

  let { lang }: Props = $props();
</script>

{#if lang === 'en'}
  <span
    class="ml-1 inline-flex items-center rounded bg-muted px-1.5 py-0.5 text-xs font-medium text-muted-foreground"
    title="Content displayed in English (fallback)"
  >
    {m.railway_model_content_in_english_fallback()}
  </span>
{/if}
