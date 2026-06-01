<script lang="ts">
  import type { Category as RailwayCategory, ModelCard as ModelCardType } from '$lib/bindings';
  import RailwayModelPreviewCard, {
    type RailwayModelCardData
  } from '$lib/components/RailwayModelPreviewCard.svelte';

  let {
    card,
    purchaseDate,
    onclick
  }: { card: ModelCardType; purchaseDate: string | null; onclick?: () => void } = $props();

  function mapCategory(category: RailwayCategory): RailwayModelCardData['category'] {
    switch (category) {
      case 'LOCOMOTIVES':
        return 'SteamLocomotive';
      case 'FREIGHT_CARS':
        return 'FreightCar';
      case 'PASSENGER_CARS':
        return 'PassengerCar';
      case 'RAILCARS':
        return 'Railcar';
      case 'TRAIN_SETS':
      case 'STARTER_SETS':
      case 'ELECTRIC_MULTIPLE_UNITS':
        return 'TrainSet';
      default:
        return 'Unknown';
    }
  }

  const sharedCard = $derived.by<RailwayModelCardData>(() => ({
    id: card.id,
    manufacturer: card.manufacturer,
    productCode: card.productCode,
    description: card.description,
    category: mapCategory(card.category),
    scale: card.scale,
    powerMethod: card.powerMethod,
    condition: card.condition,
    era: card.era,
    purchaseDate,
    price: card.price,
    photoUrl: card.thumbnailPath,
    isSold: false,
    unitCount: null,
    digitalFeatures: []
  }));
</script>

<div
  role="button"
  tabindex={0}
  class="cursor-pointer"
  onclick={onclick}
  onkeydown={(e) => {
    if (e.key === 'Enter' || e.key === ' ') {
      e.preventDefault();
      onclick?.();
    }
  }}
>
  <RailwayModelPreviewCard model={sharedCard} class="h-full" />
</div>
