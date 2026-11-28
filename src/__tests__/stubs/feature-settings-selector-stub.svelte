<script lang="ts">
  interface Props {
    value: string;
    onchange: (value: string) => void;
    disabled?: boolean;
  }

  let { value, onchange, disabled = false }: Props = $props();

  function handleInput(event: Event) {
    const target = event.currentTarget as HTMLInputElement;
    onchange(target.value);
  }
</script>

<input aria-label={value} {value} {disabled} oninput={handleInput} />
