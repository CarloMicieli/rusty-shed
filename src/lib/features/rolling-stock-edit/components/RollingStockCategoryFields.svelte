<script lang="ts">
  import * as m from '$lib/paraglide/messages';
  import { FormSelect } from '$lib/components/drawer';

  interface Props {
    category: string;
    categoryOptions: { value: string; label: string }[];
  }

  let { category = $bindable(), categoryOptions }: Props = $props();

  // Filter out the blank sentinel option from categoryOptions — FormSelect uses its placeholder instead
  const filteredCategoryOptions = $derived(categoryOptions.filter((o) => o.value !== ''));
</script>

<section>
  <h3 class="mb-3 text-[10px] font-semibold tracking-widest text-zinc-500 uppercase">
    {m.rolling_stock_create_drawer_title()}
  </h3>
  <div class="space-y-4">
    <!-- Category -->
    <FormSelect
      id="create-category"
      label={m.rolling_stock_field_category()}
      options={filteredCategoryOptions}
      bind:value={category}
      placeholder={m.rolling_stock_select_category()}
      required
    />
  </div>
</section>
