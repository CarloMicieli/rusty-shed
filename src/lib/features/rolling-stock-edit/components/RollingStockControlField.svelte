<script lang="ts">
  import * as m from '$lib/paraglide/messages';
  import { FormSelect } from '$lib/components/drawer';

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
  <FormSelect
    id="create-control"
    label={m.model_rolling_stock_field_control()}
    options={controlOptions}
    bind:value={control}
    placeholder={m.rolling_stock_select_control()}
  />
</section>
