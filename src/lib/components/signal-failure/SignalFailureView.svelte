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

<div class="flex min-h-screen items-center justify-center bg-[#050505] p-8">
  <div
    class="flex w-full max-w-xl flex-col items-center gap-8 rounded-[8px] border border-[#1F1F1F] bg-[#0F0F0F] p-10"
  >
    <!-- Railway Signal Icon -->
    <svg
      width="64"
      height="80"
      viewBox="0 0 64 80"
      xmlns="http://www.w3.org/2000/svg"
      aria-hidden="true"
    >
      <!-- Upright post -->
      <line x1="32" y1="72" x2="32" y2="12" stroke="#808080" stroke-width="2" fill="none" />
      <!-- Base -->
      <line x1="20" y1="72" x2="44" y2="72" stroke="#808080" stroke-width="2" fill="none" />
      <!-- Horizontal stop arm -->
      <line x1="32" y1="28" x2="54" y2="28" stroke="#808080" stroke-width="2" fill="none" />
      <!-- Lamp circle at tip of arm -->
      <circle cx="54" cy="28" r="8" stroke="#808080" stroke-width="2" fill="none" />
      <!-- Signal head box at top of post -->
      <rect
        x="22"
        y="12"
        width="20"
        height="24"
        rx="3"
        stroke="#808080"
        stroke-width="2"
        fill="none"
      />
      <!-- Lens circle in signal head -->
      <circle cx="32" cy="24" r="6" stroke="#808080" stroke-width="2" fill="none" />
    </svg>

    <!-- Headline -->
    <div class="flex flex-col items-center gap-3 text-center">
      <h1 class="text-2xl font-semibold tracking-wide text-[#D48A42]">
        {m.signal_failure_headline()}
      </h1>
      <p class="max-w-sm text-sm leading-relaxed text-[#808080]">
        {m.signal_failure_subtext()}
      </p>
    </div>

    <!-- Metadata Footer: three-column grid -->
    <div class="grid w-full grid-cols-3 gap-4 border-t border-[#1F1F1F] pt-6">
      <!-- ERROR CODE -->
      <div class="flex flex-col items-center gap-1 text-center">
        <span class="text-xs tracking-widest text-[#808080] uppercase">
          {m.signal_failure_label_error_code()}
        </span>
        <span class="font-mono text-sm break-all text-white">{errorId}</span>
      </div>

      <!-- MODULE -->
      <div class="flex flex-col items-center gap-1 text-center">
        <span class="text-xs tracking-widest text-[#808080] uppercase">
          {m.signal_failure_label_module()}
        </span>
        <span class="text-sm text-white">{moduleLabel}</span>
      </div>

      <!-- STATUS -->
      <div class="flex flex-col items-center gap-1 text-center">
        <span class="text-xs tracking-widest text-[#808080] uppercase">
          {m.signal_failure_label_status()}
        </span>
        <span class="text-sm text-white">{m.signal_failure_status_value()}</span>
      </div>
    </div>

    <!-- Action Buttons -->
    <div class="flex w-full flex-col gap-3 sm:flex-row">
      <button
        onclick={handleReset}
        class="flex-1 rounded-[6px] bg-[#D48A42] px-6 py-3 text-sm font-medium text-black transition-colors hover:bg-[#D48A42]/90"
      >
        {m.signal_failure_action_reset()}
      </button>
      <button
        onclick={handleReport}
        class="flex-1 rounded-[6px] border border-[#1F1F1F] px-6 py-3 text-sm font-medium text-[#808080] transition-colors hover:text-white"
      >
        {m.signal_failure_action_report()}
      </button>
    </div>
  </div>
</div>
