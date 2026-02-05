<script lang="ts">
  import { Wrench } from 'lucide-svelte';
  import * as m from '$lib/paraglide/messages.js';
  import { Badge, Button } from '$lib/components';

  let { onCreate } = $props<{ onCreate: () => void }>();
</script>

<div class="card space-y-4 p-8 text-center">
  <div class="flex justify-center">
    <Badge variant="default" class="h-16 w-16 rounded-full">
      <Wrench size={32} />
    </Badge>
  </div>
  <div class="space-y-2">
    <h3 class="h3">{m.maintenance_empty_title()}</h3>
    <p class="text-surface-500">{m.maintenance_empty_message()}</p>
  </div>
  <Button type="button" variant="default" onclick={onCreate}>
    {m.maintenance_empty_action()}
  </Button>
</div>
