<script lang="ts">
  import { Wrench } from 'lucide-svelte';
  import * as m from '$lib/paraglide/messages.js';

  let { onCreate } = $props<{ onCreate: () => void }>();
</script>

<div class="space-y-4 card p-8 text-center">
  <div class="flex justify-center">
    <span class="variant-filled-surface badge-icon">
      <Wrench size={32} />
    </span>
  </div>
  <div class="space-y-2">
    <h3 class="h3">{m.maintenance_empty_title()}</h3>
    <p class="text-surface-500">{m.maintenance_empty_message()}</p>
  </div>
  <button type="button" class="variant-filled-primary btn" onclick={onCreate}>
    {m.maintenance_empty_action()}
  </button>
</div>
