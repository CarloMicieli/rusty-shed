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
    paddingClass = 'px-4 py-2',
    identity,
    classification,
    actions
  }: Props = $props();
</script>

<div
  class="grid grid-cols-[1fr_auto_1fr] items-center border-b border-border bg-card {paddingClass} {className}"
>
  {@render identity()}
  {@render classification()}
  {@render actions()}
</div>
