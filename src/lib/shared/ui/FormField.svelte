<script lang="ts">
  import type { Snippet } from 'svelte';
  import { resolveLabel } from '../../../utils/resolveLabel';
  import type { ConstantItem } from '$lib/features/catalogue/constants';

  interface Props {
    label: ConstantItem;
    error?: string;
    required?: boolean;
    fieldId?: string;
    children: Snippet;
  }

  const { label, error, required = false, fieldId, children }: Props = $props();
</script>

<label class="label" for={fieldId}>
  <span class="text-surface-300 text-sm font-bold tracking-wider uppercase">
    {resolveLabel(label)}{required ? ' *' : ''}
  </span>
  {@render children()}
  {#if error}
    <span class="text-error-500 text-sm">{error}</span>
  {/if}
</label>
