<script lang="ts">
  import { ChevronLeft } from 'lucide-svelte';
  import { resolve } from '$app/paths';
  import { SvelteURLSearchParams } from 'svelte/reactivity';
  import { twMerge } from 'tailwind-merge';

  type DetailBackPath = '/collection' | '/maintenance' | '/railway-tracks' | '/wishlists';

  interface Props {
    path: DetailBackPath;
    ariaLabel: string;
    query?: Record<string, string | null | undefined>;
    class?: string;
  }

  let { path, ariaLabel, query, class: className = '' }: Props = $props();

  const linkClass = $derived(
    twMerge(
      'flex h-10 w-10 items-center justify-center rounded-sm border border-border bg-card text-muted-foreground transition-colors hover:bg-background hover:text-foreground',
      className
    )
  );

  const queryString = $derived.by(() => {
    if (!query) return '';
    const params = new SvelteURLSearchParams();
    for (const [key, value] of Object.entries(query)) {
      if (value && value.trim().length > 0) {
        params.set(key, value);
      }
    }

    const search = params.toString();
    return search.length > 0 ? `?${search}` : '';
  });
</script>

<a
  href={queryString ? `${resolve(path)}${queryString}` : resolve(path)}
  aria-label={ariaLabel}
  class={linkClass}
>
  <ChevronLeft size={22} />
</a>
