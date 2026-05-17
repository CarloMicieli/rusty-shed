<script lang="ts">
  import * as m from '$lib/paraglide/messages.js';

  interface Props {
    value: string;
    onChange: (value: string) => void;
  }

  let { value, onChange }: Props = $props();
</script>

<div class="space-y-1">
  <input
    type="search"
    class="h-9 w-full rounded-sm border border-border bg-background px-3 text-sm"
    placeholder={m.settings_library_search_placeholder()}
    value={value}
    oninput={(event) => onChange((event.currentTarget as HTMLInputElement).value)}
  />
</div>
