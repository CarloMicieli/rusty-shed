<script lang="ts">
  import Locomotive from './Locomotive.svelte';
  import Coach from './Coach.svelte';
  import Wagon from './Wagon.svelte';
  import type { Component } from 'svelte';

  let {
    type,
    isOwned = false,
    class: className = ''
  }: { type: string; isOwned?: boolean; class?: string } = $props();

  const iconMap: Record<string, Component> = {
    Locomotive,
    PowerCar: Locomotive,
    Coach,
    Couchette: Coach,
    Dining: Coach,
    Sleeping: Coach,
    ControlCar: Coach,
    BaggageCar: Wagon,
    FreightWagon: Wagon
  } as const;

  const SelectedIcon = $derived(iconMap[type] ?? Wagon);
</script>

<div
  class="relative flex items-center justify-center p-1
    {isOwned ? 'text-blue-600' : 'text-gray-400 opacity-60'} {className}"
>
  <SelectedIcon size="size-10" />

  {#if !isOwned}
    <div class="absolute inset-0 rounded-sm border border-dashed border-gray-300"></div>
  {/if}
</div>
