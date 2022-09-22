<script lang="ts">
  import { createEventDispatcher } from 'svelte'

  const dispatch = createEventDispatcher()

  // Props
  export let open = false

  // State
  let dialogElement: HTMLDialogElement

  // Reactive
  $: {
    if (dialogElement) {
      if (open) {
        dialogElement.showModal()
      } else {
        dialogElement.close()
      }
    }
  }

  // Callback
  function onDialogClick() {
    dispatch('close')
  }
</script>

<dialog on:click|self={onDialogClick} class="p-3" bind:this={dialogElement}>
  <slot />
</dialog>

<style lang="scss">
  @import 'theme';

  dialog {
    border: $border_m $white;
    outline: none;
    border-radius: $radius_s;
  }

  dialog::backdrop {
    background-color: rgba(0, 0, 0, 0.3);
    backdrop-filter: blur(5px);
  }
</style>
