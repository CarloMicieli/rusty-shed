<script lang="ts">
  import type { Snippet } from 'svelte';

  interface Props {
    /** Extra classes on the row (e.g. rounded corners for accordion). */
    class?: string;
    /** Horizontal padding; default matches collection card. */
    paddingClass?: string;
    identity: Snippet;
    classification: Snippet;
    actions: Snippet;
  }

  const {
    class: className = '',
    paddingClass = 'px-4 py-3',
    identity,
    classification,
    actions
  }: Props = $props();
</script>

<div
  class="grid grid-cols-[1fr_minmax(0,1fr)_auto] items-center border-b border-border bg-card {paddingClass} {className}"
>
  {@render identity()}
  {@render classification()}
  {@render actions()}
</div>
