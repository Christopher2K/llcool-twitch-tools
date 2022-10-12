<script lang="ts">
  import { createEventDispatcher } from 'svelte'

  import RootModal from './RootModal.svelte'
  import Button from './Button.svelte'
  import ModalHeader from './ModalHeader.svelte'
  import ModalFooter from './ModalFooter.svelte'

  const dispatch = createEventDispatcher()

  // Props
  export let open = false
  export let title: string | undefined = 'Confirm'
  export let message: string | undefined = undefined
  export let confirmationButtonLabel: string = 'Confirm'

  // Callbacks
  function dispatchCloseEvent() {
    dispatch('close')
  }

  function dispatchConfirmEvent() {
    dispatch('confirm')
  }
</script>

<RootModal fullSize on:close {open}>
  <div>
    {#if title}
      <ModalHeader>
        <h3>{title}</h3>
      </ModalHeader>
    {/if}

    {#if message}
      <div class="mb-3">
        <p>{message}</p>
      </div>
    {/if}

    <ModalFooter>
      <Button label={confirmationButtonLabel} on:click={dispatchConfirmEvent} />
      <Button label="Close" theme="danger" on:click={dispatchCloseEvent} />
    </ModalFooter>
  </div>
</RootModal>
