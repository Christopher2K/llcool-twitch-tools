<script lang="ts">
  let dialogElement: HTMLDialogElement

  // Exported functions
  export function getElement() {
    return dialogElement
  }
</script>

<dialog bind:this={dialogElement}><slot /></dialog>
