<script lang="ts">
  import * as m from '$lib/paraglide/messages.js';
  import { Button } from '$lib/components/ui/button';
  import { Input } from '$lib/components/ui/input';
  import { Label } from '$lib/components/ui/label';
  import * as Select from '$lib/components/ui/select';
  import { Checkbox } from '$lib/components/ui/checkbox';
  import type { TrainFormationState } from '../TrainFormationState.svelte.js';

  const SPECIFICATION_TYPES = [
    'LOCOMOTIVE',
    'PASSENGER_CAR',
    'FREIGHT_CAR',
    'RAILCAR',
    'ELECTRIC_MULTIPLE_UNIT'
  ] as const;

  let {
    state: ctx,
    onCreated,
    onCancel
  }: {
    state: TrainFormationState;
    onCreated: (prototypeId: string) => void;
    onCancel: () => void;
  } = $props();

  let railwayCompanyId = $state('');
  let seriesCode = $state('');
  let specificationType = $state('PASSENGER_CAR');
  let serviceLevel = $state('');
  let isMotorized = $state(false);
  let defaultIsDummy = $state(false);
  let submitting = $state(false);

  async function handleSubmit(e: Event) {
    e.preventDefault();
    if (!seriesCode.trim()) return;
    submitting = true;
    try {
      const proto = await ctx.createCustomPrototype({
        railway_company_id: railwayCompanyId.trim(),
        series_code: seriesCode.trim(),
        friendly_name: null,
        specification_type: specificationType,
        service_level: serviceLevel.trim() || null,
        is_motorized: isMotorized,
        default_is_dummy: defaultIsDummy,
        notes: null,
        locomotive_type: null,
        locomotive_series: null,
        passenger_car_type: null,
        freight_car_type: null,
        railcar_type: null,
        electric_multiple_unit_type: null,
        elements_count: null,
        is_permanently_coupled: null
      });
      if (proto) onCreated(proto.id);
    } finally {
      submitting = false;
    }
  }
</script>

<form onsubmit={handleSubmit} class="space-y-3 rounded-lg border p-3">
  <p class="text-sm font-medium">{m.formations_add_prototype_action()}</p>

  <div class="space-y-1">
    <Label for="cp-company" class="text-xs">Railway Company ID</Label>
    <Input
      id="cp-company"
      bind:value={railwayCompanyId}
      placeholder="trn:railway-company:sbb-cff-ffs"
      class="h-8 text-sm"
    />
  </div>

  <div class="space-y-1">
    <Label for="cp-series" class="text-xs">{m.formations_prototype_series_code()}</Label>
    <Input
      id="cp-series"
      bind:value={seriesCode}
      placeholder="Re 4/4 II"
      class="h-8 text-sm"
      required
    />
  </div>

  <div class="space-y-1">
    <Label for="cp-type" class="text-xs">{m.formations_prototype_specification_type()}</Label>
    <Select.Root type="single" bind:value={specificationType}>
      <Select.Trigger id="cp-type" class="h-8 text-sm">{specificationType}</Select.Trigger>
      <Select.Content>
        {#each SPECIFICATION_TYPES as st (st)}
          <Select.Item value={st}>{st}</Select.Item>
        {/each}
      </Select.Content>
    </Select.Root>
  </div>

  <div class="space-y-1">
    <Label for="cp-service" class="text-xs">{m.formations_prototype_service_level()}</Label>
    <Input id="cp-service" bind:value={serviceLevel} placeholder="1st class" class="h-8 text-sm" />
  </div>

  <div class="flex items-center gap-2">
    <Checkbox id="cp-motorized" bind:checked={isMotorized} />
    <Label for="cp-motorized" class="text-xs">Motorized</Label>
    <Checkbox id="cp-dummy" bind:checked={defaultIsDummy} />
    <Label for="cp-dummy" class="text-xs">Default dummy</Label>
  </div>

  <div class="flex gap-2 pt-1">
    <Button type="submit" size="sm" disabled={submitting}>{m.formations_save()}</Button>
    <Button type="button" variant="ghost" size="sm" onclick={onCancel}
      >{m.formations_cancel()}</Button
    >
  </div>
</form>
