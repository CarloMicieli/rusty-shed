<script lang="ts">
  import { ChevronLeft } from 'lucide-svelte';
  import { resolve } from '$app/paths';
  import { twMerge } from 'tailwind-merge';

  type DetailBackPath = '/collection' | '/maintenance' | '/railway-tracks' | '/wishlists';

  interface Props {
    path: DetailBackPath;
    ariaLabel: string;
    class?: string;
  }

  let { path, ariaLabel, class: className = '' }: Props = $props();

  const linkClass = $derived(
    twMerge(
      'flex h-10 w-10 items-center justify-center rounded-sm border border-border bg-card text-muted-foreground transition-colors hover:bg-background hover:text-foreground',
      className
    )
  );
</script>

<a href={resolve(path)} aria-label={ariaLabel} class={linkClass}>
  <ChevronLeft size={22} />
</a>
