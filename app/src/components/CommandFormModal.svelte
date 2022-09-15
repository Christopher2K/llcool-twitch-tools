<script lang="ts">
  import { createEventDispatcher } from 'svelte'
  import { portal } from 'svelte-portal'

  import RootModal from './RootModal.svelte'

  const dispatch = createEventDispatcher()

  export let open = false

  // Binding
  let rootModalComponent: RootModal

  // Reactive
  // Callbacks
  function dispatchCloseEvent() {
    dispatch('close')
  }

  function dispatchConfirmEvent() {
    dispatch('confirm')
  }
</script>

<div use:portal={'body'}>
  <RootModal bind:this={rootModalComponent} />
</div>
