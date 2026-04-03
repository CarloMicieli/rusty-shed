<script lang="ts">
  import DrawerInput from './DrawerInput.svelte';

  interface Props {
    label: string;
    id?: string;
    type?: string;
    min?: string | number;
    max?: string | number;
    placeholder?: string;
    value?: string | number | null;
    disabled?: boolean;
    error?: string;
    required?: boolean;
    'aria-invalid'?: boolean | 'true' | 'false';
    'aria-describedby'?: string;
    oninput?: (e: Event & { currentTarget: HTMLInputElement }) => void;
  }

  let {
    label,
    id,
    value = $bindable<string | number | null>(''),
    error,
    required,
    ...rest
  }: Props = $props();
</script>

<div class="space-y-1">
  <label for={id} class="text-[10px] font-bold text-muted-foreground uppercase"
    >{label}{required ? ' *' : ''}</label
  >
  <DrawerInput bind:value {id} {...rest} />
  {#if error}
    <p class="text-xs text-destructive">{error}</p>
  {/if}
</div>
