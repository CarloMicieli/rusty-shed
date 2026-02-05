<script lang="ts">
  import type { DashboardDepotEntry } from '$lib/bindings';
  import * as m from '$lib/paraglide/messages.js';
  import StatusBadge from '$lib/components/StatusBadge.svelte';
  import { Badge } from '$lib/components';

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
        <th>Status</th>
        <th>{m.depot_description()}</th>
      </tr>
    </thead>
    <tbody>
      {#each data as row (row.id)}
        <tr>
          <td class="font-bold">{row.manufacturer.name ?? '—'}</td>
          <td class="text-primary-400 font-mono">{row.product_code ?? '—'}</td>
          <td>
            <Badge variant="outline">
              {row.category ?? '—'}
            </Badge>
          </td>
          <td>
            <Badge variant="secondary" class="font-bold">
              {row.scale ?? '—'}
            </Badge>
          </td>
          <td>
            <Badge variant="default" class="font-bold tracking-wider">
              {row.railway_company ?? '—'}
            </Badge>
          </td>
          <td>
            <StatusBadge status="in-service" />
          </td>
          <td class="text-surface-300">{row.description ?? '—'}</td>
        </tr>
      {/each}
    </tbody>
  </table>
</div>
