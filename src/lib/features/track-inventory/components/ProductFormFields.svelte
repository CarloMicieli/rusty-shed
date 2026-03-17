<script lang="ts">
  import * as m from '$lib/paraglide/messages';
  import { Ruler, Compass, X } from 'lucide-svelte';
  import type { ManufacturerId, TrackType, TrackCode } from '$lib/bindings';
  import { Input } from '$lib/components';
  import { FormSelect } from '$lib/components/drawer';

  interface Props {
    manufacturerId?: string;
    productCode?: string;
    description?: string;
    trackType?: TrackType;
    trackCode?: TrackCode;
    withRoadbed?: boolean;
    length?: string;
    radius?: string;
    submitting?: boolean;
    error?: string | null;
    manufacturers?: Array<{ id: ManufacturerId; name: string }>;
  }

  const trackTypes: TrackType[] = ['STRAIGHT', 'CURVE', 'TURNOUT', 'FLEX_TRACK'];
  const trackCodes: TrackCode[] = ['CODE_70', 'CODE_75', 'CODE_83', 'CODE_100'];

  let {
    manufacturerId = $bindable(''),
    productCode = $bindable(''),
    description = $bindable(''),
    trackType = $bindable('STRAIGHT' as TrackType),
    trackCode = $bindable('CODE_83' as TrackCode),
    withRoadbed = $bindable(false),
    length = $bindable(''),
    radius = $bindable(''),
    submitting = false,
    error = null,
    manufacturers = []
  }: Props = $props();

  // Map manufacturers to { value, label }[] for FormSelect
  const manufacturerOptions = $derived(
    manufacturers.map((mfr) => ({ value: mfr.id, label: mfr.name }))
  );

  const trackTypeOptions = $derived(
    trackTypes.map((type) => ({ value: type, label: type.replace('_', ' ') }))
  );

  const trackCodeOptions = $derived(
    trackCodes.map((code) => ({ value: code, label: code.replace('CODE_', 'Code ') }))
  );

  // Local string mirrors for the typed union props (FormSelect operates on string | null).
  // These must stay writable ($state) so FormSelect can bind:value back to them.
  // The $effect sync below is intentional — $derived cannot be used on writable locals.
  /* eslint-disable svelte/prefer-writable-derived */
  let trackTypeStr = $state('');
  let trackCodeStr = $state('');

  // Sync prop → local when parent updates externally (also runs on mount)
  $effect(() => {
    trackTypeStr = trackType;
  });
  $effect(() => {
    trackCodeStr = trackCode;
  });
  /* eslint-enable svelte/prefer-writable-derived */

  // Propagate local string back to typed prop
  $effect(() => {
    if (trackTypeStr) trackType = trackTypeStr as TrackType;
  });
  $effect(() => {
    if (trackCodeStr) trackCode = trackCodeStr as TrackCode;
  });
</script>

<div class="space-y-8">
  <div class="grid grid-cols-1 gap-6 sm:grid-cols-2">
    <!-- Manufacturer -->
    <FormSelect
      id="manufacturer-select"
      label="{m.track_product_field_manufacturer()} *"
      options={manufacturerOptions}
      bind:value={manufacturerId}
      placeholder={m.wishlist_modal_manufacturer_placeholder()}
      disabled={submitting}
      required
    />

    <!-- Product Code -->
    <div class="space-y-2">
      <label for="product-code-input" class="text-xs text-zinc-400">
        {m.track_product_field_product_code()} *
      </label>
      <Input
        id="product-code-input"
        bind:value={productCode}
        disabled={submitting}
        required
        placeholder={m.wishlist_modal_product_code_placeholder()}
        class="h-12 rounded-xl border-white/10 bg-zinc-950 text-zinc-100 focus:border-white/20 focus:ring-0"
      />
    </div>
  </div>

  <!-- Description -->
  <div class="space-y-2">
    <label
      for="track-description-input"
      class="ml-1 text-[10px] font-bold tracking-[0.2em] text-zinc-500 uppercase"
    >
      {m.track_product_field_description()}
    </label>
    <Input
      id="track-description-input"
      bind:value={description}
      disabled={submitting}
      required
      placeholder="e.g. Straight Track 111.06mm"
      class="h-12 rounded-xl border-white/10 bg-zinc-950 text-zinc-100 focus:border-white/20 focus:ring-0"
    />
  </div>

  <div class="grid grid-cols-1 gap-6 sm:grid-cols-2">
    <!-- Track Type -->
    <FormSelect
      id="track-type-select"
      label={m.track_product_field_track_type()}
      options={trackTypeOptions}
      bind:value={trackTypeStr}
      disabled={submitting}
      required
    />

    <!-- Track Code -->
    <FormSelect
      id="track-code-select"
      label={m.track_product_field_track_code()}
      options={trackCodeOptions}
      bind:value={trackCodeStr}
      disabled={submitting}
      required
    />
  </div>

  <div class="grid grid-cols-1 gap-6 sm:grid-cols-3">
    <!-- Length -->
    <div class="space-y-2">
      <label
        class="ml-1 flex items-center gap-2 text-[10px] font-bold tracking-[0.2em] text-zinc-500 uppercase"
      >
        <Ruler size={10} />
        Length (mm)
      </label>
      <Input
        type="number"
        step="0.01"
        bind:value={length}
        disabled={submitting}
        placeholder="0.00"
        class="h-12 rounded-xl border-white/10 bg-zinc-950 font-mono text-zinc-100 placeholder:text-zinc-700"
      />
    </div>

    <!-- Radius -->
    <div class="space-y-2">
      <label
        class="ml-1 flex items-center gap-2 text-[10px] font-bold tracking-[0.2em] text-zinc-500 uppercase"
      >
        <Compass size={10} />
        Radius (mm)
      </label>
      <Input
        type="number"
        step="0.01"
        bind:value={radius}
        disabled={submitting}
        placeholder="0.00"
        class="h-12 rounded-xl border-white/10 bg-zinc-950 font-mono text-zinc-100 placeholder:text-zinc-700"
      />
    </div>

    <!-- With Roadbed -->
    <div class="flex items-center justify-center">
      <label class="group flex cursor-pointer items-center gap-3">
        <div class="relative flex items-center justify-center">
          <input
            type="checkbox"
            bind:checked={withRoadbed}
            disabled={submitting}
            class="peer absolute h-6 w-6 cursor-pointer rounded-lg opacity-0"
          />
          <div
            class="h-6 w-6 rounded-lg border-2 border-white/10 bg-zinc-900 transition-all peer-checked:border-amber-500 peer-checked:bg-amber-500"
          ></div>
          <X
            size={14}
            class="absolute scale-150 rotate-45 text-black opacity-0 transition-all peer-checked:opacity-100"
          />
        </div>
        <span
          class="text-[10px] font-bold tracking-widest text-zinc-500 uppercase transition-colors group-hover:text-zinc-300"
          >With Roadbed</span
        >
      </label>
    </div>
  </div>

  {#if error}
    <div
      class="rounded-xl border border-red-500/20 bg-red-500/10 p-4 text-xs font-bold tracking-widest text-red-500 uppercase"
    >
      Catalog Error: {error}
    </div>
  {/if}
</div>
