<script lang="ts">
  import * as m from '$lib/paraglide/messages';

  interface Props {
    control: string;
    controlOptions: { value: string; label: string }[];
  }

  let { control = $bindable(), controlOptions }: Props = $props();
</script>

<section>
  <h3 class="mb-3 text-[10px] font-semibold tracking-widest text-zinc-500 uppercase">
    {m.specs_drawer_section_control()}
  </h3>
  <div>
    <label class="mb-1 block text-xs font-medium text-zinc-400" for="create-control">
      {m.model_rolling_stock_field_control()}
    </label>
    <select
      id="create-control"
      bind:value={control}
      class="w-full rounded-md border border-zinc-700 bg-zinc-900 px-3 py-1.5 text-sm text-zinc-100 outline-none focus:border-[#E2994F] focus:ring-1 focus:ring-[#E2994F]/30"
    >
      {#each controlOptions as opt (opt.value)}
        <option value={opt.value}>{opt.label}</option>
      {/each}
    </select>
  </div>
</section>
