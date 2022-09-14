<script lang="ts">
  import { createEventDispatcher } from 'svelte'
  import { portal } from 'svelte-portal'

  import RootModal from './RootModal.svelte'
  import Button from './Button.svelte'
  import Typography from './Typography.svelte'

  const dispatch = createEventDispatcher()

  // Props
  export let open = false

  // Binding
  let rootModalComponent: RootModal

  // Reactive
  $: {
    if (rootModalComponent) {
      if (open) {
        rootModalComponent.getElement().showModal()
      } else {
        rootModalComponent.getElement().close()
      }
    }
  }

  // Callbacks
  function dispatchCloseEvent() {
    dispatch('close')
  }
</script>

<div use:portal={'body'}>
  <RootModal bind:this={rootModalComponent}>
    <Typography>Hello world</Typography>
    <Button label="Close" on:click={dispatchCloseEvent} />
  </RootModal>
</div>

<style lang="scss">
</style>
