<script lang="ts">
  interface Props {
    size?: number;
    class?: string;
  }

  let { size = 32, class: className = '' }: Props = $props();
</script>

<svg
  class={className}
  width={size}
  height={size}
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
  aria-hidden="true"
>
  <!-- Boiler body -->
  <rect x="2" y="8" width="14" height="7" rx="2" />
  <!-- Cab -->
  <rect x="14" y="6" width="6" height="9" rx="1" />
  <!-- Smokestack -->
  <line x1="5" y1="8" x2="5" y2="5" />
  <line x1="4" y1="5" x2="6" y2="5" />
  <!-- Rail / running line -->
  <line x1="1" y1="17" x2="23" y2="17" />
  <!-- Drive wheels (large) -->
  <circle cx="7" cy="17" r="2" />
  <circle cx="13" cy="17" r="2" />
  <!-- Trailing wheel (small) -->
  <circle cx="18" cy="17" r="1.5" />
  <!-- Cowcatcher -->
  <line x1="2" y1="15" x2="1" y2="17" />
</svg>
