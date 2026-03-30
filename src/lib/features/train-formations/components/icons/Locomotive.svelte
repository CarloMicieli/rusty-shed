<script lang="ts">
  let { size = 'size-8', class: className = '' }: { size?: string; class?: string } = $props();
</script>

<svg
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="1.5"
  stroke-linecap="round"
  stroke-linejoin="round"
  class="{size} {className}"
>
  <path d="M2 17h20v-4H2v4Z" />
  <path d="M4 13V7h8v6" />
  <path d="M12 9h10v4" />
  <rect x="6" y="9" width="4" height="2" />
  <circle cx="6" cy="18" r="1.5" />
  <circle cx="18" cy="18" r="1.5" />
</svg>
