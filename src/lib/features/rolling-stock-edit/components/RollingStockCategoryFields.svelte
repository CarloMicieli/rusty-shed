<script lang="ts">
  import * as m from '$lib/paraglide/messages';
  import { FormSelect } from '$lib/components/drawer';

  interface Props {
    railwayCompanyId: string;
    railwayCompanyName: string;
    category: string;
    companyOptions: { id: string; label: string }[];
    categoryOptions: { value: string; label: string }[];
    onCompanySelect?: (id: string, name: string) => void;
  }

  let {
    railwayCompanyId = $bindable(),
    railwayCompanyName = $bindable(),
    category = $bindable(),
    companyOptions,
    categoryOptions,
    onCompanySelect
  }: Props = $props();

  // Map company options to the { value, label } shape FormSelect expects
  const mappedCompanyOptions = $derived(
    companyOptions.map((c) => ({ value: c.id, label: c.label }))
  );

  // Sync railwayCompanyName and fire callback whenever the selected company changes
  $effect(() => {
    if (!railwayCompanyId) return;
    railwayCompanyName =
      companyOptions.find((c) => c.id === railwayCompanyId)?.label ?? railwayCompanyId;
    onCompanySelect?.(railwayCompanyId, railwayCompanyName);
  });

  // Filter out the blank sentinel option from categoryOptions — FormSelect uses its placeholder instead
  const filteredCategoryOptions = $derived(categoryOptions.filter((o) => o.value !== ''));
</script>

<section>
  <h3 class="mb-3 text-[10px] font-semibold tracking-widest text-zinc-500 uppercase">
    {m.rolling_stock_create_drawer_title()}
  </h3>
  <div class="space-y-4">
    <!-- Railway Company -->
    <FormSelect
      id="create-company"
      label={m.model_rolling_stock_field_company()}
      options={mappedCompanyOptions}
      bind:value={railwayCompanyId}
      isSearchable={true}
      placeholder={m.rolling_stock_select_company()}
      disabled={companyOptions.length === 0}
      required
    />

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
