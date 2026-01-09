<script lang="ts">
  import type { DashboardDepotEntry } from '$lib/bindings';
  import * as m from '$lib/paraglide/messages.js';

  let { data } = $props<{ data: DashboardDepotEntry[] }>();
</script>

<div class="table-container">
  <table class="table-hover table">
    <thead>
      <tr>
        <th>{m.depot_manufacturer()}</th>
        <th>{m.depot_product_code()}</th>
        <th>{m.depot_category()}</th>
        <th>{m.depot_scale()}</th>
        <th>{m.depot_company()}</th>
        <th>{m.depot_description()}</th>
      </tr>
    </thead>
    <tbody>
      {#each data as row (row.id)}
        <tr>
          <td class="font-bold">{row.manufacturer.name ?? '—'}</td>
          <td class="font-mono text-primary-400">{row.product_code ?? '—'}</td>
          <td>
            <span class="variant-soft-surface badge">
              {row.category ?? '—'}
            </span>
          </td>
          <td>
            <span class="variant-soft-secondary badge font-bold">
              {row.scale ?? '—'}
            </span>
          </td>
          <td>
            <span class="variant-filled-surface badge font-bold tracking-wider">
              {row.railway_company.name ?? '—'}
            </span>
          </td>
          <td class="text-surface-300">{row.description ?? '—'}</td>
        </tr>
      {/each}
    </tbody>
  </table>
</div>
