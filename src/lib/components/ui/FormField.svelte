<script lang="ts">
  import { resolveLabel } from '../../../utils/resolveLabel';
  import type { ConstantItem } from '$lib/features/catalogue/constants';

  export let label: ConstantItem;
  export let error: string | undefined = undefined;
  export let required = false;
  export let fieldId: string | undefined = undefined;
</script>

<label class="label" for={fieldId}>
  <span class="text-sm font-bold tracking-wider text-surface-300 uppercase">
    {resolveLabel(label)}{required ? ' *' : ''}
  </span>
  <slot />
  {#if error}
    <span class="text-sm text-error-500">{error}</span>
  {/if}
</label>
