<script lang="ts">
  let { total } = $props<{ total: number }>();

  // Example "Memory" calculation (can be hardcoded or dynamic)
  const memoryUsage = 45;

  // NOTE: Terminal-style labels (Depot_Memory, Total_Units, Active_DCC, System_Online)
  // are intentional design choices styled as a retro depot monitoring dashboard.
  // These should NOT be localized as they are part of the visual/UX design.
</script>

<footer class="mt-12 flex items-center justify-between border-t border-border pt-6">
  <div class="flex items-center gap-6">
    <div class="flex flex-col gap-0.5">
      <span class="text-[10px] font-bold tracking-[0.2em] text-muted-foreground uppercase"
        >Depot_Memory</span
      >
      <div class="flex items-center gap-2">
        <div class="h-1.5 w-32 overflow-hidden rounded-sm border border-border bg-background/50">
          <div
            class="h-full bg-primary transition-all duration-1000"
            style="width: {memoryUsage}%;"
          ></div>
        </div>
        <span class="font-mono text-[10px] text-foreground">{memoryUsage}%</span>
      </div>
    </div>

    <div class="h-8 w-px bg-border"></div>

    <div class="flex gap-4">
      <div class="flex flex-col">
        <span class="text-[10px] font-bold tracking-[0.2em] text-muted-foreground uppercase"
          >Total_Units</span
        >
        <span class="font-mono text-sm font-bold text-foreground">{total}</span>
      </div>
      <div class="flex flex-col">
        <span class="text-[10px] font-bold tracking-[0.2em] text-muted-foreground uppercase"
          >Active_DCC</span
        >
        <span class="font-mono text-sm font-bold text-primary">— READY —</span>
      </div>
    </div>
  </div>

  <div class="flex items-center gap-2 text-muted-foreground">
    <div class="h-1.5 w-1.5 animate-pulse rounded-full bg-primary"></div>
    <span class="text-[10px] font-bold tracking-[0.2em] uppercase">System_Online</span>
  </div>
</footer>
