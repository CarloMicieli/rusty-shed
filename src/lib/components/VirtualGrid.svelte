<script lang="ts" generics="T">
  /**
   * VirtualGrid — Svelte 5 DOM virtualizer for responsive CSS grids.
   *
   * Automatically detects the nearest scrollable ancestor and renders only
   * the rows currently visible in the viewport (plus overscan). This keeps
   * the DOM node count at ~(columns × overscanRows × 2) regardless of how
   * many items are in the list.
   *
   * Usage:
   * ```svelte
   * <VirtualGrid items={filteredItems} itemHeight={320}>
   *   {#snippet children(item, idx)}
   *     <MyCard {item} />
   *   {/snippet}
   * </VirtualGrid>
   * ```
   */

  import { onMount } from 'svelte';
  import type { Snippet } from 'svelte';

  interface Props {
    /** All items to virtualise. Only visible rows are mounted in the DOM. */
    items: T[];
    /** Estimated height of a single row (card height) in px. */
    itemHeight: number;
    /** Minimum column width, matching the CSS `minmax()` value. Default: 240. */
    itemMinWidth?: number;
    /** Gap between items in px. Should match the Tailwind `gap-*` value. Default: 16. */
    gap?: number;
    /** Number of extra rows to render above and below the visible window. Default: 3. */
    overscan?: number;
    /** Render slot — receives each (item, index) in the visible window. */
    children: Snippet<[T, number]>;
  }

  let { items, itemHeight, itemMinWidth = 240, gap = 16, overscan = 3, children }: Props = $props();

  let containerEl = $state<HTMLDivElement | undefined>(undefined);
  let scrollParentEl = $state<HTMLElement | null>(null);

  let containerWidth = $state(0);
  let scrollTop = $state(0);
  let viewportHeight = $state(600);
  /**
   * The container's static vertical offset from the scroll parent's origin (px).
   * Changes only when layout above the grid shifts (e.g. stats row appears).
   */
  let containerOffset = $state(0);

  // ── Layout calculations ──────────────────────────────────────────────────────

  const columnCount = $derived(
    Math.max(1, Math.floor((containerWidth + gap) / (itemMinWidth + gap)))
  );
  const rowCount = $derived(Math.ceil(items.length / columnCount));
  const rowH = $derived(itemHeight + gap);
  const totalHeight = $derived(rowCount > 0 ? rowCount * rowH - gap : 0);

  // ── Visible row window (with overscan) ───────────────────────────────────────

  const relScrollTop = $derived(Math.max(0, scrollTop - containerOffset));
  const firstRow = $derived(Math.max(0, Math.floor(relScrollTop / rowH) - overscan));
  const lastRow = $derived(
    Math.min(rowCount - 1, Math.ceil((relScrollTop + viewportHeight) / rowH) + overscan)
  );
  const offsetTop = $derived(firstRow * rowH);

  const visibleItems = $derived.by(() => {
    const out: { item: T; idx: number }[] = [];
    for (let r = firstRow; r <= lastRow; r++) {
      for (let c = 0; c < columnCount; c++) {
        const idx = r * columnCount + c;
        if (idx < items.length) out.push({ item: items[idx], idx });
      }
    }
    return out;
  });

  // ── DOM helpers ──────────────────────────────────────────────────────────────

  /** Walk up the DOM to find the nearest element that actually scrolls. */
  function findScrollParent(el: HTMLElement): HTMLElement | null {
    let parent = el.parentElement;
    while (parent) {
      const { overflowY } = getComputedStyle(parent);
      if (overflowY === 'auto' || overflowY === 'scroll') return parent;
      parent = parent.parentElement;
    }
    return null;
  }

  /**
   * Compute and store the static offset from the scroll parent's top
   * to the container's top (in the scroll parent's coordinate system).
   * Must be called after any layout shift above the grid.
   */
  function measureOffset(sp: HTMLElement | null) {
    if (!containerEl) return;
    if (sp) {
      const cRect = containerEl.getBoundingClientRect();
      const pRect = sp.getBoundingClientRect();
      containerOffset = cRect.top - pRect.top + sp.scrollTop;
    } else {
      containerOffset = containerEl.getBoundingClientRect().top + window.scrollY;
    }
  }

  // Re-measure whenever items change — stats/loading state above the grid
  // can shift the container position when data loads or mutations occur.
  $effect(() => {
    void items.length;
    measureOffset(scrollParentEl);
  });

  onMount(() => {
    if (!containerEl) return;
    const sp = findScrollParent(containerEl);
    scrollParentEl = sp;

    // Initial measurements
    containerWidth = containerEl.offsetWidth;
    if (sp) {
      viewportHeight = sp.clientHeight;
      scrollTop = sp.scrollTop;
    } else {
      viewportHeight = window.innerHeight;
      scrollTop = window.scrollY;
    }
    measureOffset(sp);

    // Observe container width changes (responsive column recalculation)
    const ro = new ResizeObserver((entries) => {
      containerWidth = entries[0]?.contentRect.width ?? containerEl!.offsetWidth;
      measureOffset(sp);
    });
    ro.observe(containerEl);

    const scroller: HTMLElement | Window = sp ?? window;

    function onScroll() {
      scrollTop = sp ? sp.scrollTop : window.scrollY;
    }
    function onResize() {
      viewportHeight = sp ? sp.clientHeight : window.innerHeight;
      measureOffset(sp);
    }

    scroller.addEventListener('scroll', onScroll, { passive: true });
    window.addEventListener('resize', onResize, { passive: true });

    return () => {
      ro.disconnect();
      scroller.removeEventListener('scroll', onScroll);
      window.removeEventListener('resize', onResize);
    };
  });
</script>

<!--
  The outer div claims the full virtual height so the scroll container's
  scrollbar reflects the real content length. The inner div is absolutely
  positioned at the current visible window's offset.
-->
<div bind:this={containerEl} style="position: relative; height: {totalHeight}px;">
  {#if visibleItems.length > 0}
    <div
      style="position: absolute; top: {offsetTop}px; left: 0; right: 0; display: grid; gap: {gap}px; grid-template-columns: repeat({columnCount}, minmax(0, 1fr));"
    >
      {#each visibleItems as { item, idx } (idx)}
        {@render children(item, idx)}
      {/each}
    </div>
  {/if}
</div>
