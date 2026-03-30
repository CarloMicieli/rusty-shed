<script lang="ts">
  let { size = 'size-8', class: className = '' }: { size?: string; class?: string } = $props();
</script>

<svg
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="1.5"
  class="{size} {className}"
>
  <rect x="2" y="8" width="20" height="9" rx="1" />
  <rect x="5" y="10" width="2" height="3" />
  <rect x="9" y="10" width="2" height="3" />
  <rect x="13" y="10" width="2" height="3" />
  <rect x="17" y="10" width="2" height="3" />
  <path d="M5 18a1 1 0 1 0 0-2 1 1 0 0 0 0 2Z" />
  <path d="M19 18a1 1 0 1 0 0-2 1 1 0 0 0 0 2Z" />
</svg>
