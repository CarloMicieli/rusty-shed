<script lang="ts">
  import type { Snippet } from 'svelte';

  let {
    theme,
    class: className,
    style,
    loadingIcon,
    successIcon,
    errorIcon,
    infoIcon,
    warningIcon,
    ...restProps
  }: {
    theme?: string;
    class?: string;
    style?: string;
    loadingIcon?: Snippet;
    successIcon?: Snippet;
    errorIcon?: Snippet;
    infoIcon?: Snippet;
    warningIcon?: Snippet;
  } & Record<string, unknown> = $props();
</script>

<div
  data-testid="sonner-root"
  data-theme={theme}
  data-class={className}
  data-style={style}
  data-rest={JSON.stringify(restProps)}
>
  <div data-testid="sonner-loading">{@render loadingIcon?.()}</div>
  <div data-testid="sonner-success">{@render successIcon?.()}</div>
  <div data-testid="sonner-error">{@render errorIcon?.()}</div>
  <div data-testid="sonner-info">{@render infoIcon?.()}</div>
  <div data-testid="sonner-warning">{@render warningIcon?.()}</div>
</div>
