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
      class="flex h-10 w-full rounded-[8px] border border-[#1F1F1F] bg-[#0F0F0F] px-3 py-2 text-sm text-[#E0E0E0] focus:border-[#D48A42]/60 focus:ring-2 focus:ring-[#D48A42]/20 focus:outline-none disabled:cursor-not-allowed disabled:opacity-50"
    >
      <option value="" disabled>{m.rolling_stock_select_control()}</option>
      {#each controlOptions as opt (opt.value)}
        <option value={opt.value}>{opt.label}</option>
      {/each}
    </select>
  </div>
</section>
