<script lang="ts">
  let dialogElement: HTMLDialogElement

  // Exported functions
  export function getElement() {
    return dialogElement
  }
</script>

<dialog class="p-3" bind:this={dialogElement}><slot /></dialog>

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
