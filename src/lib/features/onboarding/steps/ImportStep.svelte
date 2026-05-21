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
    <h2 class="font-bebas text-3xl tracking-[0.18em] text-foreground uppercase sm:text-4xl">
      {m.onboarding_step_3_title()}
    </h2>
    <p class="mt-2 font-mono text-xs tracking-[0.16em] text-muted-foreground uppercase">
      {m.onboarding_step_3_hint()}
    </p>
  </div>

  <div class="grid gap-4">
    <Button
      variant="outline"
      class="group relative h-auto w-full justify-start rounded-sm border-2 border-border/80 bg-card/80 px-4 py-4 text-left shadow-[inset_0_1px_0_hsl(var(--foreground)/0.08)] transition-all duration-150 hover:border-primary/70 hover:bg-card hover:shadow-[0_0_0_2px_hsl(var(--primary)/0.2)] focus-visible:border-primary focus-visible:ring-primary/60 disabled:cursor-not-allowed disabled:opacity-70 sm:px-5"
      onclick={onLocalImport}
      disabled={isBusy}
      aria-describedby="import-local-caption"
    >
      <span class="flex w-full items-center justify-between gap-4">
        <span class="space-y-1">
          <span class="block text-sm font-semibold tracking-wide text-foreground sm:text-base">
            {m.onboarding_import_local()}
          </span>
          <span
            id="import-local-caption"
            class="block font-mono text-[11px] tracking-[0.12em] text-muted-foreground uppercase"
          >
            {m.onboarding_import_local_caption()}
          </span>
        </span>
        <span
          class="font-mono text-[11px] tracking-[0.14em] text-primary uppercase transition-colors duration-150 group-hover:text-primary"
          aria-hidden="true"
        >
          {m.onboarding_import_local_status()}
        </span>
      </span>
    </Button>

    <Button
      variant="outline"
      class="group relative h-auto w-full justify-start rounded-sm border-2 border-border/80 bg-card/80 px-4 py-4 text-left shadow-[inset_0_1px_0_hsl(var(--foreground)/0.08)] transition-all duration-150 hover:border-primary/70 hover:bg-card hover:shadow-[0_0_0_2px_hsl(var(--primary)/0.2)] focus-visible:border-primary focus-visible:ring-primary/60 disabled:cursor-not-allowed disabled:opacity-70 sm:px-5"
      onclick={onGoogleDriveImport}
      disabled={isBusy}
      aria-describedby="import-drive-caption"
    >
      <span class="flex w-full items-center justify-between gap-4">
        <span class="space-y-1">
          <span class="block text-sm font-semibold tracking-wide text-foreground sm:text-base">
            {m.onboarding_import_drive()}
          </span>
          <span
            id="import-drive-caption"
            class="block font-mono text-[11px] tracking-[0.12em] text-muted-foreground uppercase"
          >
            {m.onboarding_import_drive_caption()}
          </span>
        </span>
        <span
          class="font-mono text-[11px] tracking-[0.14em] text-primary uppercase transition-colors duration-150 group-hover:text-primary"
          aria-hidden="true"
        >
          {m.onboarding_import_drive_status()}
        </span>
      </span>
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

  <Button
    variant="ghost"
    class="group h-auto w-full justify-start rounded-sm border-2 border-dashed border-border bg-background/55 px-4 py-3 text-left transition-all duration-150 hover:border-primary/80 hover:bg-background focus-visible:border-primary focus-visible:ring-primary/60 disabled:cursor-not-allowed disabled:opacity-70 sm:px-5"
    onclick={onSkipAndStartFresh}
    disabled={isBusy}
    aria-describedby="skip-start-fresh-helper"
  >
    <span class="space-y-1">
      <span class="block text-sm font-semibold tracking-wide text-foreground sm:text-base">
        {m.onboarding_skip_start_fresh()}
      </span>
      <span
        id="skip-start-fresh-helper"
        class="block font-mono text-[11px] tracking-[0.1em] text-muted-foreground"
      >
        {m.onboarding_skip_start_fresh_helper()}
      </span>
    </span>
  </Button>
</section>
