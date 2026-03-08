<script lang="ts">
  import * as m from '$lib/paraglide/messages.js';
  import type { ImportPreviewResponse } from '$lib/bindings';
  import { Button } from '$lib/components';

  interface Props {
    preview: ImportPreviewResponse;
    onConfirm?: () => Promise<void>;
    onCancel?: () => void;
    loading?: boolean;
  }

  const {
    preview,
    onConfirm = async () => {},
    onCancel = () => {},
    loading = false
  }: Props = $props();
</script>

<div class="preview-actions">
  <Button variant="outline" onclick={onCancel} disabled={loading} type="button">
    {m.common_cancel()}
  </Button>
  <Button onclick={onConfirm} disabled={!preview.canImport || loading} type="button">
    {loading ? m.app_loading() : 'Confirm Import'}
  </Button>
</div>

<style>
  .preview-actions {
    display: flex;
    justify-content: flex-end;
    gap: 1rem;
    padding-top: 1rem;
    border-top: 1px solid hsl(var(--border));
  }
</style>
