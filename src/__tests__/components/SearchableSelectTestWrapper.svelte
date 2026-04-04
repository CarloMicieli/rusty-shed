<script lang="ts">
  import SearchableSelect, { type SelectOption } from '$lib/components/SearchableSelect.svelte';
  import type { Snippet } from 'svelte';

  interface Props {
    options: SelectOption[];
    value: string;
    item?: Snippet<[SelectOption]>;
    trigger?: Snippet<[SelectOption | undefined]>;
  }

  let { options, value = $bindable(), item, trigger }: Props = $props();
</script>

<SearchableSelect {options} bind:value {item} {trigger} />
