<script lang="ts">
  import * as m from '$lib/paraglide/messages.js';
  import { Button } from '$lib/components/ui/button';
  import { Input } from '$lib/components/ui/input';
  import { Label } from '$lib/components/ui/label';
  import * as Select from '$lib/components/ui/select';
  import { Checkbox } from '$lib/components/ui/checkbox';
  import type { TrainFormationState } from '../TrainFormationState.svelte.js';

  const CAR_TYPES = [
    'Locomotive',
    'Coach',
    'Wagon',
    'PowerCar',
    'ControlCar',
    'MotorUnitCar',
    'DrivingTrailer',
    'TrailerCar',
    'CateringCar'
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
  let carType = $state('Coach');
  let serviceLevel = $state('');
  let category = $state('');
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
        car_type: carType,
        service_level: serviceLevel.trim() || null,
        category: category.trim() || 'Freight',
        is_motorized: isMotorized,
        default_is_dummy: defaultIsDummy,
        notes: null
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
    <Label for="cp-type" class="text-xs">{m.formations_prototype_car_type()}</Label>
    <Select.Root type="single" bind:value={carType}>
      <Select.Trigger id="cp-type" class="h-8 text-sm">{carType}</Select.Trigger>
      <Select.Content>
        {#each CAR_TYPES as ct (ct)}
          <Select.Item value={ct}>{ct}</Select.Item>
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
