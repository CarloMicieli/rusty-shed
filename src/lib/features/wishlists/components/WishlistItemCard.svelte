<script lang="ts">
  import * as m from '$lib/paraglide/messages.js';
  import { Trash2, ShoppingCart, MoveRight, TrainFront, Box, Users, Layers } from 'lucide-svelte';
  import type { WishlistItem, RailwayModelView, MonetaryAmount } from '$lib/bindings';
  import { Badge, Button, Card, CardContent, CardHeader } from '$lib/components';
  import { onMount } from 'svelte';
  import { commands } from '$lib/bindings';
  import { readFile } from '@tauri-apps/plugin-fs';
  import { goto } from '$app/navigation';
  import { getLocale } from '$lib/paraglide/runtime.js';
  import { regionalManager } from '$lib/features/settings/RegionalManager.svelte';

  interface Props {
    item: WishlistItem;
    wishlistId: string;
    onRemove?: (itemId: string) => void;
    onMove?: (itemId: string) => void;
    onPurchase?: (itemId: string) => void;
  }

  let { item, wishlistId, onRemove, onMove, onPurchase }: Props = $props();

  let photoUrl = $state<string | null>(null);
  let modelDetails = $state<RailwayModelView | null>(null);

  onMount(async () => {
    try {
      const [modelResult, imageResult] = await Promise.all([
        commands.getRailwayModelById(item.railwayModelId, getLocale()),
        commands.getRailwayModelImage(item.railwayModelId)
      ]);

      if (modelResult.status === 'ok' && modelResult.data) {
        modelDetails = modelResult.data;
      }

      if (imageResult.status === 'ok' && imageResult.data.hasImage && imageResult.data.imagePath) {
        const filePath = imageResult.data.imagePath;
        const ext = filePath.split('.').pop()?.toLowerCase() ?? 'jpg';
        const mimes: Record<string, string> = {
          jpg: 'image/jpeg',
          jpeg: 'image/jpeg',
          png: 'image/png',
          webp: 'image/webp'
        };
        const bytes = await readFile(filePath);
        photoUrl = URL.createObjectURL(new Blob([bytes], { type: mimes[ext] ?? 'image/jpeg' }));
      }
    } catch (e) {
      console.warn('Failed to load model details for wishlist item', e);
    }
  });

  const statusLabel = $derived(item.status);

  const desiredPriceStr = $derived.by(() => {
    if (!item.desiredPrice) return null;
    const price = item.desiredPrice as MonetaryAmount;
    return regionalManager.formatCurrencyWith(
      Number(price.amount || 0),
      (price.currency as string) || 'EUR'
    );
  });

  const PlaceholderIcon = $derived.by(() => {
    const cat = modelDetails?.category?.toLowerCase() ?? '';
    if (cat.includes('locomotive')) return TrainFront;
    if (cat.includes('freight')) return Box;
    if (cat.includes('passenger')) return Users;
    if (cat.includes('railcar') || cat.includes('train_set')) return Layers;
    return TrainFront;
  });
</script>

<!-- svelte-ignore a11y_click_events_have_key_events -->
<!-- svelte-ignore a11y_no_static_element_interactions -->
<div class="cursor-pointer" onclick={() => goto(`/wishlists/${wishlistId}/items/${item.id}`)}>
  <Card
    class="group relative flex flex-col overflow-hidden border-2 border-border/20 bg-card transition-all duration-300 hover:border-primary/30 hover:shadow-[0_0_30px_rgba(245,158,11,0.07)]"
  >
    <!-- Priority Badge overlay -->
    {#if item.priority === 'HIGH'}
      <div class="absolute top-3 left-3 z-20">
        <Badge
          class="bg-amber-500 text-[9px] font-bold text-black uppercase shadow-lg shadow-amber-500/20"
        >
          {m.wishlist_priority_high()} Priority
        </Badge>
      </div>
    {/if}

    <CardHeader class="p-4 pb-2">
      <div class="flex items-start justify-between gap-2">
        <div class="min-w-0 flex-1">
          <div class="flex items-center gap-1.5 overflow-hidden">
            <span
              class="truncate text-[10px] font-bold tracking-widest text-muted-foreground uppercase"
            >
              {modelDetails?.manufacturer.display ?? 'Searching...'}
            </span>
          </div>
          <h3 class="truncate text-sm font-bold text-foreground">
            {modelDetails?.description ?? item.railwayModelId}
          </h3>
        </div>

        <button
          onclick={(e) => {
            e.stopPropagation();
            onRemove?.(item.id);
          }}
          class="text-muted-foreground transition-colors hover:text-red-400"
        >
          <Trash2 size={14} />
        </button>
      </div>
    </CardHeader>

    <CardContent class="flex flex-1 flex-col p-4 pt-0">
      <!-- Image Area -->
      <div
        class="relative aspect-[4/3] w-full overflow-hidden rounded-xl bg-muted/50"
        style="background-image: linear-gradient(rgba(255,255,255,0.03) 1px, transparent 1px), linear-gradient(90deg, rgba(255,255,255,0.03) 1px, transparent 1px); background-size: 15px 15px;"
      >
        {#if photoUrl}
          <img
            src={photoUrl}
            alt=""
            aria-hidden="true"
            class="absolute inset-0 h-full w-full scale-110 object-cover opacity-60 blur-xl"
          />
          <img
            src={photoUrl}
            alt="Model"
            class="absolute inset-0 z-10 h-full w-full object-contain transition-transform duration-500 group-hover:scale-105"
          />
        {:else}
          <div class="flex h-full w-full items-center justify-center opacity-20">
            <PlaceholderIcon size={40} class="text-zinc-400" />
          </div>
        {/if}

        <!-- Status Overlay -->
        <div class="absolute right-2 bottom-2 z-20">
          <Badge
            variant="outline"
            class="border-border/20 bg-background/60 font-mono text-[9px] backdrop-blur-md"
          >
            {statusLabel}
          </Badge>
        </div>
      </div>

      <!-- Specs row -->
      <div class="mt-3 grid grid-cols-2 gap-2">
        <div class="rounded-lg bg-muted/40 p-2 text-center">
          <p class="text-[8px] font-bold tracking-tighter text-muted-foreground uppercase">
            Price Target
          </p>
          <p class="font-mono text-xs font-bold text-amber-500">
            {desiredPriceStr ?? '—'}
          </p>
        </div>
        <div class="rounded-lg bg-muted/40 p-2 text-center">
          <p class="text-[8px] font-bold tracking-tighter text-muted-foreground uppercase">
            Product Code
          </p>
          <p class="font-mono text-xs text-foreground">
            {modelDetails?.productCode ?? '—'}
          </p>
        </div>
      </div>

      <!-- Action Buttons -->
      <div class="mt-auto flex gap-2 pt-4">
        <Button
          variant="secondary"
          size="sm"
          class="h-8 flex-1 border-border bg-muted text-[11px] font-bold hover:bg-muted/80"
          onclick={(e: MouseEvent) => {
            e.stopPropagation();
            onMove?.(item.id);
          }}
        >
          <MoveRight size={12} />
          <span class="ml-1.5">Move</span>
        </Button>
        {#if item.status === 'WANTED' || item.status === 'ON_ORDER'}
          <Button
            variant="secondary"
            size="sm"
            class="h-8 flex-1 border-border bg-muted text-[11px] font-bold hover:bg-muted/80"
            onclick={(e: MouseEvent) => {
              e.stopPropagation();
              onPurchase?.(item.id);
            }}
          >
            <ShoppingCart size={12} />
            <span class="ml-1.5">Purchase</span>
          </Button>
        {/if}
      </div>
    </CardContent>
  </Card>
</div>
