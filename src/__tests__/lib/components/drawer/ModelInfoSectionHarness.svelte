<script lang="ts">
  import * as Tooltip from '$lib/components/ui/tooltip';
  import ModelInfoSection from '$lib/components/drawer/sections/ModelInfoSection.svelte';
  import type { Manufacturer } from '$lib/bindings';

  let {
    manufacturerId = '',
    productCode = '',
    description = '',
    category = 'LOCOMOTIVES',
    scale = 'H0',
    powerMethod = 'DC',
    epoch = '',
    manufacturers = [],
    isLoading = false,
    errors = undefined
  }: {
    manufacturerId?: string;
    productCode?: string;
    description?: string;
    category?: string;
    scale?: string;
    powerMethod?: string;
    epoch?: string;
    manufacturers?: Manufacturer[];
    isLoading?: boolean;
    errors?: Record<string, string>;
  } = $props();
</script>

<Tooltip.Provider>
  <ModelInfoSection
    {manufacturerId}
    {productCode}
    {description}
    {category}
    {scale}
    {powerMethod}
    {epoch}
    {manufacturers}
    {isLoading}
    {errors}
  />
</Tooltip.Provider>
