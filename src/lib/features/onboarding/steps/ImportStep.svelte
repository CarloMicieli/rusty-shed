<script lang="ts">
  import { Button } from '$lib/components/ui/button';
  import * as m from '$lib/paraglide/messages.js';

  interface Props {
    isBusy: boolean;
    errorMessage: string | null;
    onLocalImport: () => Promise<void>;
    onGoogleDriveImport: () => Promise<void>;
    onSkipAndStartFresh: () => Promise<void>;
  }

  let { isBusy, errorMessage, onLocalImport, onGoogleDriveImport, onSkipAndStartFresh }: Props =
    $props();
</script>

<section class="space-y-6">
  <div>
    <h2 class="text-xl font-semibold">{m.onboarding_step_3_title()}</h2>
    <p class="text-sm text-muted-foreground">{m.onboarding_step_3_hint()}</p>
  </div>

  <div class="grid gap-3">
    <Button variant="outline" class="justify-start" onclick={onLocalImport} disabled={isBusy}>
      {m.onboarding_import_local()}
    </Button>
    <Button variant="outline" class="justify-start" onclick={onGoogleDriveImport} disabled={isBusy}>
      {m.onboarding_import_drive()}
    </Button>
  </div>

  {#if errorMessage}
    <div
      class="rounded-md border border-destructive/40 bg-destructive/10 px-3 py-2 text-sm"
      role="alert"
    >
      {errorMessage}
    </div>
  {/if}

  <Button variant="ghost" onclick={onSkipAndStartFresh} disabled={isBusy}>
    {m.onboarding_skip_start_fresh()}
  </Button>
</section>
