<script lang="ts">
  import * as m from '$lib/paraglide/messages.js';
  import { toaster } from '$lib/toaster';

  interface Props {
    errorId: string;
    moduleLabel: string;
    onReset?: () => void;
  }

  const { errorId, moduleLabel, onReset }: Props = $props();

  async function handleReset() {
    if (onReset) {
      onReset();
    } else {
      window.location.href = '/';
    }
  }

  async function handleReport() {
    try {
      await navigator.clipboard.writeText(errorId);
      toaster.success(m.signal_failure_report_copied());
    } catch {
      toaster.warning(m.signal_failure_report_copy_failed());
    }
  }
</script>

<div class="flex min-h-screen items-center justify-center bg-layout-surface p-8">
  <div
    class="flex w-full max-w-xl flex-col items-center gap-8 rounded-[8px] border border-layout-border bg-layout-surface p-10"
  >
    <!-- Railway Signal Icon -->
    <svg
      width="64"
      height="80"
      viewBox="0 0 64 80"
      xmlns="http://www.w3.org/2000/svg"
      aria-hidden="true"
      class="text-muted-foreground"
    >
      <!-- Upright post -->
      <line x1="32" y1="72" x2="32" y2="12" stroke="currentColor" stroke-width="2" fill="none" />
      <!-- Base -->
      <line x1="20" y1="72" x2="44" y2="72" stroke="currentColor" stroke-width="2" fill="none" />
      <!-- Horizontal stop arm -->
      <line x1="32" y1="28" x2="54" y2="28" stroke="currentColor" stroke-width="2" fill="none" />
      <!-- Lamp circle at tip of arm -->
      <circle cx="54" cy="28" r="8" stroke="currentColor" stroke-width="2" fill="none" />
      <!-- Signal head box at top of post -->
      <rect
        x="22"
        y="12"
        width="20"
        height="24"
        rx="3"
        stroke="currentColor"
        stroke-width="2"
        fill="none"
      />
      <!-- Lens circle in signal head -->
      <circle cx="32" cy="24" r="6" stroke="currentColor" stroke-width="2" fill="none" />
    </svg>

    <!-- Headline -->
    <div class="flex flex-col items-center gap-3 text-center">
      <h1 class="text-2xl font-semibold tracking-wide text-primary">
        {m.signal_failure_headline()}
      </h1>
      <p class="max-w-sm text-sm leading-relaxed text-muted-foreground">
        {m.signal_failure_subtext()}
      </p>
    </div>

    <!-- Metadata Footer: three-column grid -->
    <div class="grid w-full grid-cols-3 gap-4 border-t border-layout-border pt-6">
      <!-- ERROR CODE -->
      <div class="flex flex-col items-center gap-1 text-center">
        <span class="text-xs tracking-widest text-muted-foreground uppercase">
          {m.signal_failure_label_error_code()}
        </span>
        <span class="font-mono text-sm break-all text-white">{errorId}</span>
      </div>

      <!-- MODULE -->
      <div class="flex flex-col items-center gap-1 text-center">
        <span class="text-xs tracking-widest text-muted-foreground uppercase">
          {m.signal_failure_label_module()}
        </span>
        <span class="text-sm text-white">{moduleLabel}</span>
      </div>

      <!-- STATUS -->
      <div class="flex flex-col items-center gap-1 text-center">
        <span class="text-xs tracking-widest text-muted-foreground uppercase">
          {m.signal_failure_label_status()}
        </span>
        <span class="text-sm text-white">{m.signal_failure_status_value()}</span>
      </div>
    </div>

    <!-- Action Buttons -->
    <div class="flex w-full flex-col gap-3 sm:flex-row">
      <button
        onclick={handleReset}
        class="flex-1 rounded-[6px] bg-primary px-6 py-3 text-sm font-medium text-primary-foreground transition-colors hover:bg-primary/90"
      >
        {m.signal_failure_action_reset()}
      </button>
      <button
        onclick={handleReport}
        class="flex-1 rounded-[6px] border border-layout-border px-6 py-3 text-sm font-medium text-muted-foreground transition-colors hover:text-white"
      >
        {m.signal_failure_action_report()}
      </button>
    </div>
  </div>
</div>
