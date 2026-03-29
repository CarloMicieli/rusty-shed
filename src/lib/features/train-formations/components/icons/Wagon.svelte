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
  <path d="M2 16h20" />
  <rect x="3" y="9" width="18" height="7" />
  <path d="M8 9v7M16 9v7" />
  <circle cx="6" cy="17" r="1" />
  <circle cx="18" cy="17" r="1" />
</svg>
