<script lang="ts">
  import * as m from '$lib/paraglide/messages.js';
  import { DrawerSectionCard, ModelDetails } from '$lib/components/drawer';
  import type { Manufacturer } from '$lib/bindings';

  interface Props {
    manufacturerId: string | null;
    productCode: string;
    description: string;
    category: string | null;
    scale: string | null;
    powerMethod: string | null;
    epoch: string | null;
    manufacturers: Manufacturer[];
    errors?: {
      manufacturerId?: string;
      productCode?: string;
      description?: string;
      category?: string;
      scale?: string;
      powerMethod?: string;
      epoch?: string;
    };
    isLoading?: boolean;
    disabled?: boolean;
    onQuickAddManufacturer?: () => void;
  }

  let {
    manufacturerId = $bindable<string | null>(),
    productCode = $bindable(),
    description = $bindable(),
    category = $bindable<string | null>(),
    scale = $bindable<string | null>(),
    powerMethod = $bindable<string | null>(),
    epoch = $bindable<string | null>(),
    manufacturers,
    errors = {},
    isLoading = false,
    disabled = false,
    onQuickAddManufacturer
  }: Props = $props();
</script>

<DrawerSectionCard label={m.drawer_section_model_info()}>
  <ModelDetails
    bind:manufacturerId
    bind:productCode
    bind:description
    bind:category
    bind:scale
    bind:powerMethod
    bind:epoch
    {manufacturers}
    {isLoading}
    {disabled}
    {onQuickAddManufacturer}
    {errors}
  />
</DrawerSectionCard>
