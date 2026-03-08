<script lang="ts">
  import * as m from '$lib/paraglide/messages';

  interface BoolOption {
    value: string;
    label: string;
  }

  interface SelectOption {
    value: string;
    label: string;
  }

  interface Props {
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
    boolOptions: BoolOption[];
    bodyShellOptions: SelectOption[];
    chassisOptions: SelectOption[];
    featureFlagOptions: SelectOption[];
    controlOptions: SelectOption[];
    dccInterfaceOptions: SelectOption[];
    couplingSockeOptions: SelectOption[];
    onFlywheelChange: (value: boolean | null) => void;
    onCloseCouplersChange: (value: boolean | null) => void;
    onDigitalShuntingChange: (value: boolean | null) => void;
    boolValue: (val: boolean | null) => string;
    parseBool: (s: string) => boolean | null;
  }

  let {
    flywheelFitted,
    bodyShell,
    chassis,
    interiorLights,
    lights,
    dccInterface,
    control,
    couplingSocket,
    closeCouplers,
    digitalShunting,
    boolOptions,
    bodyShellOptions,
    chassisOptions,
    featureFlagOptions,
    controlOptions,
    dccInterfaceOptions,
    couplingSockeOptions,
    onFlywheelChange,
    onCloseCouplersChange,
    onDigitalShuntingChange,
    boolValue,
    parseBool
  }: Props = $props();
</script>

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
        value={boolValue(flywheelFitted)}
        onchange={(e) => {
          onFlywheelChange(parseBool((e.target as HTMLSelectElement).value));
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
        bind:value={bodyShell}
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
        bind:value={chassis}
        class="w-full rounded-md border border-zinc-700 bg-zinc-900 px-3 py-1.5 text-sm text-zinc-100 outline-none focus:border-[#E2994F] focus:ring-1 focus:ring-[#E2994F]/30"
      >
        {#each chassisOptions as opt (opt.value)}
          <option value={opt.value}>{opt.label}</option>
        {/each}
      </select>
    </div>
    <div>
      <label class="mb-1 block text-xs font-medium text-zinc-400" for="drawer-interior-lights">
        {m.specs_drawer_field_lighting()} (interior)
      </label>
      <select
        id="drawer-interior-lights"
        bind:value={interiorLights}
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
        bind:value={lights}
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
        bind:value={control}
        class="w-full rounded-md border border-zinc-700 bg-zinc-900 px-3 py-1.5 text-sm text-zinc-100 outline-none focus:border-[#E2994F] focus:ring-1 focus:ring-[#E2994F]/30"
      >
        {#each controlOptions as opt (opt.value)}
          <option value={opt.value}>{opt.label}</option>
        {/each}
      </select>
    </div>
    <div>
      <label class="mb-1 block text-xs font-medium text-zinc-400" for="drawer-dcc-interface">
        {m.specs_drawer_field_dcc_interface()}
      </label>
      <select
        id="drawer-dcc-interface"
        bind:value={dccInterface}
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
      <label class="mb-1 block text-xs font-medium text-zinc-400" for="drawer-coupling-socket">
        {m.specs_drawer_field_coupling_socket()}
      </label>
      <select
        id="drawer-coupling-socket"
        bind:value={couplingSocket}
        class="w-full rounded-md border border-zinc-700 bg-zinc-900 px-3 py-1.5 text-sm text-zinc-100 outline-none focus:border-[#E2994F] focus:ring-1 focus:ring-[#E2994F]/30"
      >
        {#each couplingSockeOptions as opt (opt.value)}
          <option value={opt.value}>{opt.label}</option>
        {/each}
      </select>
    </div>
    <div>
      <label class="mb-1 block text-xs font-medium text-zinc-400" for="drawer-close-couplers">
        {m.specs_drawer_field_close_coupling()}
      </label>
      <select
        id="drawer-close-couplers"
        value={boolValue(closeCouplers)}
        onchange={(e) => {
          onCloseCouplersChange(parseBool((e.target as HTMLSelectElement).value));
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
      <label class="mb-1 block text-xs font-medium text-zinc-400" for="drawer-digital-shunting">
        {m.specs_drawer_field_digital_shunting()}
      </label>
      <select
        id="drawer-digital-shunting"
        value={boolValue(digitalShunting)}
        onchange={(e) => {
          onDigitalShuntingChange(parseBool((e.target as HTMLSelectElement).value));
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
