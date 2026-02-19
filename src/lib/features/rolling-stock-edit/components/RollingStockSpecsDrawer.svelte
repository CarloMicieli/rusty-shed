<script lang="ts">
  import { X } from 'lucide-svelte';
  import { Button } from '$lib/components/ui/button';
  import * as m from '$lib/paraglide/messages';
  import { toaster } from '$lib/toaster';
  import {
    commands,
    type RailwayModelId,
    type RollingStockId,
    type RollingStockView
  } from '$lib/bindings';

  interface Props {
    /** Controls drawer visibility. */
    open: boolean;
    /** The parent railway model. */
    railwayModelId: RailwayModelId;
    /** The rolling stock unit to edit. */
    rollingStockId: RollingStockId;
    /** Called after a successful save (to trigger parent refresh). */
    onSaved?: () => void;
    /** Called when the drawer requests to close. */
    onClose: () => void;
  }

  let { open, railwayModelId, rollingStockId, onSaved, onClose }: Props = $props();

  // ── Form state ──────────────────────────────────────────────────────────────
  interface FormState {
    seriesCode: string;
    roadNumber: string;
    livery: string;
    depot: string;
    flywheelFitted: boolean | null;
    bodyShell: string;
    chassis: string;
    interiorLights: string;
    lights: string;
    dccInterface: string;
    control: string;
    couplingSocket: string;
    closeCouplers: boolean | null;
    digitalShunting: boolean | null;
  }

  const emptyForm: FormState = {
    seriesCode: '',
    roadNumber: '',
    livery: '',
    depot: '',
    flywheelFitted: null,
    bodyShell: '',
    chassis: '',
    interiorLights: '',
    lights: '',
    dccInterface: '',
    control: '',
    couplingSocket: '',
    closeCouplers: null,
    digitalShunting: null
  };

  let form = $state<FormState>({ ...emptyForm });
  let originalForm = $state<FormState>({ ...emptyForm });
  let isLoading = $state(false);
  let isSaving = $state(false);
  let inlineError = $state<string | null>(null);
  let showDiscardDialog = $state(false);

  // ── Derived ─────────────────────────────────────────────────────────────────
  const isDirty = $derived(JSON.stringify(form) !== JSON.stringify(originalForm));

  // ── Option lists ────────────────────────────────────────────────────────────
  const boolOptions = [
    { value: 'true', label: 'Yes' },
    { value: 'false', label: 'No' }
  ];

  const bodyShellOptions = [
    { value: '', label: '—' },
    { value: 'PLASTIC', label: 'Plastic' },
    { value: 'METAL_DIE_CAST', label: 'Metal die-cast' }
  ];

  const chassisOptions = [
    { value: '', label: '—' },
    { value: 'PLASTIC', label: 'Plastic' },
    { value: 'METAL_DIE_CAST', label: 'Metal die-cast' }
  ];

  const featureFlagOptions = [
    { value: '', label: '—' },
    { value: 'YES', label: 'Yes' },
    { value: 'NO', label: 'No' }
  ];

  const controlOptions = [
    { value: '', label: '—' },
    { value: 'DCC_READY', label: 'DCC Ready' },
    { value: 'DCC_FITTED', label: 'DCC Fitted' },
    { value: 'DCC_SOUND', label: 'DCC Sound' },
    { value: 'NO_DCC', label: 'Analogue (No DCC)' }
  ];

  const dccInterfaceOptions = [
    { value: '', label: '—' },
    { value: 'NEM_651', label: 'NEM 651' },
    { value: 'NEM_652', label: 'NEM 652' },
    { value: 'NEM_654', label: 'NEM 654' },
    { value: 'PLUX_8', label: 'PLUX 8' },
    { value: 'PLUX_12', label: 'PLUX 12' },
    { value: 'PLUX_16', label: 'PLUX 16' },
    { value: 'PLUX_22', label: 'PLUX 22' },
    { value: 'NEXT_18', label: 'Next18' },
    { value: 'NEXT_18_S', label: 'Next18-S' },
    { value: 'MTC_21', label: 'MTC 21' }
  ];

  const couplingSockeOptions = [
    { value: '', label: '—' },
    { value: 'NONE', label: 'None' },
    { value: 'NEM_355', label: 'NEM 355' },
    { value: 'NEM_356', label: 'NEM 356' },
    { value: 'NEM_357', label: 'NEM 357' },
    { value: 'NEM_359', label: 'NEM 359' },
    { value: 'NEM_360', label: 'NEM 360' },
    { value: 'NEM_362', label: 'NEM 362' },
    { value: 'NEM_365', label: 'NEM 365' }
  ];

  // ── Data loading ─────────────────────────────────────────────────────────────
  function extractRsData(view: RollingStockView): Omit<FormState, never> {
    let rs;
    if ('locomotive' in view) rs = view.locomotive;
    else if ('electricMultipleUnit' in view) rs = view.electricMultipleUnit;
    else if ('freightCar' in view) rs = view.freightCar;
    else if ('passengerCar' in view) rs = view.passengerCar;
    else if ('railcar' in view) rs = view.railcar;
    else return { ...emptyForm };

    const ts = rs.technical_specifications;
    return {
      seriesCode: rs.series_code,
      roadNumber: rs.road_number ?? '',
      livery: rs.livery ?? '',
      depot: 'depot' in rs ? (rs.depot ?? '') : '',
      flywheelFitted:
        ts?.flywheel_fitted === 'YES' ? true : ts?.flywheel_fitted === 'NO' ? false : null,
      bodyShell: ts?.body_shell ?? '',
      chassis: ts?.chassis ?? '',
      interiorLights: ts?.interior_lights ?? '',
      lights: ts?.lights ?? '',
      dccInterface: 'dcc_interface' in rs ? (rs.dcc_interface ?? '') : '',
      control: 'control' in rs ? (rs.control ?? '') : '',
      couplingSocket: ts?.coupling?.socket ?? '',
      closeCouplers:
        ts?.coupling?.close_couplers === 'YES'
          ? true
          : ts?.coupling?.close_couplers === 'NO'
            ? false
            : null,
      digitalShunting:
        ts?.coupling?.digital_shunting === 'YES'
          ? true
          : ts?.coupling?.digital_shunting === 'NO'
            ? false
            : null
    };
  }

  $effect(() => {
    if (!open) return;
    void loadData();
  });

  async function loadData() {
    isLoading = true;
    inlineError = null;
    try {
      const result = await commands.getRailwayModelById(railwayModelId);
      if (result.status === 'error') {
        toaster.error(m.specs_drawer_save_error());
        onClose();
        return;
      }
      if (!result.data) {
        toaster.error(m.specs_drawer_save_error());
        onClose();
        return;
      }
      const rs = result.data.rollingStock.find((r) => {
        if ('locomotive' in r) return r.locomotive.id === rollingStockId;
        if ('electricMultipleUnit' in r) return r.electricMultipleUnit.id === rollingStockId;
        if ('freightCar' in r) return r.freightCar.id === rollingStockId;
        if ('passengerCar' in r) return r.passengerCar.id === rollingStockId;
        if ('railcar' in r) return r.railcar.id === rollingStockId;
        return false;
      });
      if (!rs) {
        toaster.error(m.specs_drawer_save_error());
        onClose();
        return;
      }
      const data = extractRsData(rs);
      form = { ...data };
      originalForm = { ...data };
    } finally {
      isLoading = false;
    }
  }

  // ── Save ────────────────────────────────────────────────────────────────────
  async function handleSave() {
    isSaving = true;
    inlineError = null;
    try {
      const result = await commands.updateRollingStockSpecifications({
        railwayModelId,
        rollingStockId,
        seriesCode: form.seriesCode,
        roadNumber: form.roadNumber || null,
        livery: form.livery || null,
        depot: form.depot || null,
        flywheelFitted: form.flywheelFitted,
        bodyShell: form.bodyShell || null,
        chassis: form.chassis || null,
        interiorLights: form.interiorLights || null,
        lights: form.lights || null,
        dccInterface: (form.dccInterface || null) as Parameters<
          typeof commands.updateRollingStockSpecifications
        >[0]['dccInterface'],
        control: (form.control || null) as Parameters<
          typeof commands.updateRollingStockSpecifications
        >[0]['control'],
        couplingSocket: form.couplingSocket || null,
        closeCouplers: form.closeCouplers,
        digitalShunting: form.digitalShunting
      });

      if (result.status === 'error') {
        inlineError = m.specs_drawer_save_error();
        return;
      }

      toaster.success(m.specs_drawer_save_success());
      originalForm = { ...form };
      onSaved?.();
      onClose();
    } finally {
      isSaving = false;
    }
  }

  // ── Close handling ──────────────────────────────────────────────────────────
  function requestClose() {
    if (isDirty) {
      showDiscardDialog = true;
    } else {
      onClose();
    }
  }

  function confirmDiscard() {
    showDiscardDialog = false;
    form = { ...originalForm };
    onClose();
  }

  function cancelDiscard() {
    showDiscardDialog = false;
  }

  // ── Helpers ─────────────────────────────────────────────────────────────────
  function boolValue(val: boolean | null): string {
    if (val === true) return 'true';
    if (val === false) return 'false';
    return '';
  }

  function parseBool(s: string): boolean | null {
    if (s === 'true') return true;
    if (s === 'false') return false;
    return null;
  }
</script>

<svelte:window onkeydown={(e) => e.key === 'Escape' && !showDiscardDialog && requestClose()} />

{#if open}
  <!-- Backdrop -->
  <div
    class="fixed inset-0 z-40 bg-black/40"
    role="presentation"
    tabindex="-1"
    onclick={requestClose}
    onkeydown={(e) => e.key === 'Escape' && requestClose()}
  ></div>

  <!-- Drawer panel -->
  <div
    class="fixed inset-y-0 right-0 z-50 flex w-full max-w-xl flex-col overflow-hidden border-l border-zinc-800 bg-[#0C0C0C] shadow-2xl"
    role="dialog"
    tabindex="-1"
    aria-modal="true"
    aria-label={m.specs_drawer_title()}
    onclick={(e) => e.stopPropagation()}
    onkeydown={(e) => e.key !== 'Escape' && e.stopPropagation()}
  >
    <!-- Header -->
    <div class="flex items-center justify-between border-b border-zinc-800 px-6 py-4">
      <div>
        <p class="text-[10px] font-medium tracking-widest text-zinc-500 uppercase">
          {m.specs_drawer_title()}
        </p>
      </div>
      <button
        type="button"
        class="rounded p-1 text-zinc-500 transition-colors hover:bg-zinc-800 hover:text-zinc-200"
        onclick={requestClose}
        aria-label={m.specs_drawer_cancel()}
      >
        <X size={16} />
      </button>
    </div>

    <!-- Content -->
    <div class="flex-1 overflow-y-auto px-6 py-4">
      {#if isLoading}
        <div class="flex h-32 items-center justify-center">
          <div
            class="h-6 w-6 animate-spin rounded-full border-2 border-[#E2994F] border-t-transparent"
          ></div>
        </div>
      {:else}
        {#if inlineError}
          <div
            class="mb-4 rounded-lg border border-red-800/50 bg-red-950/40 px-4 py-3 text-sm text-red-400"
          >
            {inlineError}
          </div>
        {/if}

        <div class="space-y-6">
          <!-- ── Identification section ──────────────────────────────────── -->
          <section>
            <h3 class="mb-3 text-[10px] font-semibold tracking-widest text-zinc-500 uppercase">
              {m.specs_drawer_section_identification()}
            </h3>
            <div class="grid grid-cols-2 gap-3">
              <div class="col-span-2">
                <label
                  class="mb-1 block text-xs font-medium text-zinc-400"
                  for="drawer-series-code"
                >
                  {m.rolling_stock_field_series_code()} <span class="text-red-400">*</span>
                </label>
                <input
                  id="drawer-series-code"
                  type="text"
                  bind:value={form.seriesCode}
                  class="w-full rounded-md border border-zinc-700 bg-zinc-900 px-3 py-1.5 text-sm text-zinc-100 outline-none focus:border-[#E2994F] focus:ring-1 focus:ring-[#E2994F]/30"
                />
              </div>
              <div>
                <label
                  class="mb-1 block text-xs font-medium text-zinc-400"
                  for="drawer-road-number"
                >
                  {m.rolling_stock_field_road_number()}
                </label>
                <input
                  id="drawer-road-number"
                  type="text"
                  bind:value={form.roadNumber}
                  class="w-full rounded-md border border-zinc-700 bg-zinc-900 px-3 py-1.5 text-sm text-zinc-100 outline-none focus:border-[#E2994F] focus:ring-1 focus:ring-[#E2994F]/30"
                />
              </div>
              <div>
                <label class="mb-1 block text-xs font-medium text-zinc-400" for="drawer-livery">
                  {m.rolling_stock_field_livery()}
                </label>
                <input
                  id="drawer-livery"
                  type="text"
                  bind:value={form.livery}
                  class="w-full rounded-md border border-zinc-700 bg-zinc-900 px-3 py-1.5 text-sm text-zinc-100 outline-none focus:border-[#E2994F] focus:ring-1 focus:ring-[#E2994F]/30"
                />
              </div>
              <div>
                <label class="mb-1 block text-xs font-medium text-zinc-400" for="drawer-depot">
                  {m.rolling_stock_field_depot()}
                </label>
                <input
                  id="drawer-depot"
                  type="text"
                  bind:value={form.depot}
                  class="w-full rounded-md border border-zinc-700 bg-zinc-900 px-3 py-1.5 text-sm text-zinc-100 outline-none focus:border-[#E2994F] focus:ring-1 focus:ring-[#E2994F]/30"
                />
              </div>
            </div>
          </section>

          <!-- ── Technical section ───────────────────────────────────────── -->
          <section>
            <h3 class="mb-3 text-[10px] font-semibold tracking-widest text-zinc-500 uppercase">
              {m.specs_drawer_section_technical()}
            </h3>
            <div class="grid grid-cols-2 gap-3">
              <div>
                <label class="mb-1 block text-xs font-medium text-zinc-400" for="drawer-flywheel">
                  {m.specs_drawer_field_flywheel()}
                </label>
                <select
                  id="drawer-flywheel"
                  value={boolValue(form.flywheelFitted)}
                  onchange={(e) => {
                    form.flywheelFitted = parseBool((e.target as HTMLSelectElement).value);
                  }}
                  class="w-full rounded-md border border-zinc-700 bg-zinc-900 px-3 py-1.5 text-sm text-zinc-100 outline-none focus:border-[#E2994F] focus:ring-1 focus:ring-[#E2994F]/30"
                >
                  <option value="">—</option>
                  {#each boolOptions as opt (opt.value)}
                    <option value={opt.value}>{opt.label}</option>
                  {/each}
                </select>
              </div>
              <div>
                <label class="mb-1 block text-xs font-medium text-zinc-400" for="drawer-body-shell">
                  {m.specs_drawer_field_body_material()}
                </label>
                <select
                  id="drawer-body-shell"
                  bind:value={form.bodyShell}
                  class="w-full rounded-md border border-zinc-700 bg-zinc-900 px-3 py-1.5 text-sm text-zinc-100 outline-none focus:border-[#E2994F] focus:ring-1 focus:ring-[#E2994F]/30"
                >
                  {#each bodyShellOptions as opt (opt.value)}
                    <option value={opt.value}>{opt.label}</option>
                  {/each}
                </select>
              </div>
              <div>
                <label class="mb-1 block text-xs font-medium text-zinc-400" for="drawer-chassis">
                  {m.specs_drawer_field_chassis_material()}
                </label>
                <select
                  id="drawer-chassis"
                  bind:value={form.chassis}
                  class="w-full rounded-md border border-zinc-700 bg-zinc-900 px-3 py-1.5 text-sm text-zinc-100 outline-none focus:border-[#E2994F] focus:ring-1 focus:ring-[#E2994F]/30"
                >
                  {#each chassisOptions as opt (opt.value)}
                    <option value={opt.value}>{opt.label}</option>
                  {/each}
                </select>
              </div>
              <div>
                <label
                  class="mb-1 block text-xs font-medium text-zinc-400"
                  for="drawer-interior-lights"
                >
                  {m.specs_drawer_field_lighting()} (interior)
                </label>
                <select
                  id="drawer-interior-lights"
                  bind:value={form.interiorLights}
                  class="w-full rounded-md border border-zinc-700 bg-zinc-900 px-3 py-1.5 text-sm text-zinc-100 outline-none focus:border-[#E2994F] focus:ring-1 focus:ring-[#E2994F]/30"
                >
                  {#each featureFlagOptions as opt (opt.value)}
                    <option value={opt.value}>{opt.label}</option>
                  {/each}
                </select>
              </div>
              <div>
                <label class="mb-1 block text-xs font-medium text-zinc-400" for="drawer-lights">
                  {m.specs_drawer_field_lighting()} (headlights)
                </label>
                <select
                  id="drawer-lights"
                  bind:value={form.lights}
                  class="w-full rounded-md border border-zinc-700 bg-zinc-900 px-3 py-1.5 text-sm text-zinc-100 outline-none focus:border-[#E2994F] focus:ring-1 focus:ring-[#E2994F]/30"
                >
                  {#each featureFlagOptions as opt (opt.value)}
                    <option value={opt.value}>{opt.label}</option>
                  {/each}
                </select>
              </div>
            </div>
          </section>

          <!-- ── Control section ────────────────────────────────────────── -->
          <section>
            <h3 class="mb-3 text-[10px] font-semibold tracking-widest text-zinc-500 uppercase">
              {m.specs_drawer_section_control()}
            </h3>
            <div class="grid grid-cols-2 gap-3">
              <div>
                <label class="mb-1 block text-xs font-medium text-zinc-400" for="drawer-control">
                  {m.specs_drawer_field_control_type()}
                </label>
                <select
                  id="drawer-control"
                  bind:value={form.control}
                  class="w-full rounded-md border border-zinc-700 bg-zinc-900 px-3 py-1.5 text-sm text-zinc-100 outline-none focus:border-[#E2994F] focus:ring-1 focus:ring-[#E2994F]/30"
                >
                  {#each controlOptions as opt (opt.value)}
                    <option value={opt.value}>{opt.label}</option>
                  {/each}
                </select>
              </div>
              <div>
                <label
                  class="mb-1 block text-xs font-medium text-zinc-400"
                  for="drawer-dcc-interface"
                >
                  {m.specs_drawer_field_dcc_interface()}
                </label>
                <select
                  id="drawer-dcc-interface"
                  bind:value={form.dccInterface}
                  class="w-full rounded-md border border-zinc-700 bg-zinc-900 px-3 py-1.5 text-sm text-zinc-100 outline-none focus:border-[#E2994F] focus:ring-1 focus:ring-[#E2994F]/30"
                >
                  {#each dccInterfaceOptions as opt (opt.value)}
                    <option value={opt.value}>{opt.label}</option>
                  {/each}
                </select>
              </div>
            </div>
          </section>

          <!-- ── Coupling section ───────────────────────────────────────── -->
          <section>
            <h3 class="mb-3 text-[10px] font-semibold tracking-widest text-zinc-500 uppercase">
              {m.specs_drawer_section_coupling()}
            </h3>
            <div class="grid grid-cols-2 gap-3">
              <div>
                <label
                  class="mb-1 block text-xs font-medium text-zinc-400"
                  for="drawer-coupling-socket"
                >
                  {m.specs_drawer_field_coupling_socket()}
                </label>
                <select
                  id="drawer-coupling-socket"
                  bind:value={form.couplingSocket}
                  class="w-full rounded-md border border-zinc-700 bg-zinc-900 px-3 py-1.5 text-sm text-zinc-100 outline-none focus:border-[#E2994F] focus:ring-1 focus:ring-[#E2994F]/30"
                >
                  {#each couplingSockeOptions as opt (opt.value)}
                    <option value={opt.value}>{opt.label}</option>
                  {/each}
                </select>
              </div>
              <div>
                <label
                  class="mb-1 block text-xs font-medium text-zinc-400"
                  for="drawer-close-couplers"
                >
                  {m.specs_drawer_field_close_coupling()}
                </label>
                <select
                  id="drawer-close-couplers"
                  value={boolValue(form.closeCouplers)}
                  onchange={(e) => {
                    form.closeCouplers = parseBool((e.target as HTMLSelectElement).value);
                  }}
                  class="w-full rounded-md border border-zinc-700 bg-zinc-900 px-3 py-1.5 text-sm text-zinc-100 outline-none focus:border-[#E2994F] focus:ring-1 focus:ring-[#E2994F]/30"
                >
                  <option value="">—</option>
                  {#each boolOptions as opt (opt.value)}
                    <option value={opt.value}>{opt.label}</option>
                  {/each}
                </select>
              </div>
              <div>
                <label
                  class="mb-1 block text-xs font-medium text-zinc-400"
                  for="drawer-digital-shunting"
                >
                  {m.specs_drawer_field_digital_shunting()}
                </label>
                <select
                  id="drawer-digital-shunting"
                  value={boolValue(form.digitalShunting)}
                  onchange={(e) => {
                    form.digitalShunting = parseBool((e.target as HTMLSelectElement).value);
                  }}
                  class="w-full rounded-md border border-zinc-700 bg-zinc-900 px-3 py-1.5 text-sm text-zinc-100 outline-none focus:border-[#E2994F] focus:ring-1 focus:ring-[#E2994F]/30"
                >
                  <option value="">—</option>
                  {#each boolOptions as opt (opt.value)}
                    <option value={opt.value}>{opt.label}</option>
                  {/each}
                </select>
              </div>
            </div>
          </section>
        </div>
      {/if}
    </div>

    <!-- Footer -->
    <div class="flex items-center justify-end gap-3 border-t border-zinc-800 px-6 py-4">
      <Button variant="ghost" onclick={requestClose} disabled={isSaving}>
        {m.specs_drawer_cancel()}
      </Button>
      <Button
        onclick={handleSave}
        disabled={isSaving || isLoading || !form.seriesCode.trim()}
        class="bg-[#E2994F] text-black hover:bg-[#E2994F]/90"
      >
        {#if isSaving}
          <span
            class="mr-2 inline-block h-3 w-3 animate-spin rounded-full border-2 border-black border-t-transparent"
          ></span>
        {/if}
        {m.specs_drawer_save()}
      </Button>
    </div>
  </div>

  <!-- Discard confirmation dialog -->
  {#if showDiscardDialog}
    <div
      class="fixed inset-0 z-60 flex items-center justify-center bg-black/60"
      role="presentation"
      tabindex="-1"
    >
      <div
        class="mx-4 w-full max-w-sm rounded-xl border border-zinc-700 bg-zinc-900 p-6 shadow-2xl"
        role="dialog"
        aria-modal="true"
      >
        <h2 class="mb-2 text-base font-semibold text-zinc-100">
          {m.specs_drawer_unsaved_title()}
        </h2>
        <p class="mb-6 text-sm text-zinc-400">{m.specs_drawer_unsaved_message()}</p>
        <div class="flex justify-end gap-3">
          <Button variant="ghost" onclick={cancelDiscard}>
            {m.specs_drawer_cancel()}
          </Button>
          <Button variant="destructive" onclick={confirmDiscard}>
            {m.specs_drawer_unsaved_confirm()}
          </Button>
        </div>
      </div>
    </div>
  {/if}
{/if}
