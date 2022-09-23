<script lang="ts">
  import { createEventDispatcher } from 'svelte'
  import { portal } from 'svelte-portal'

  import RootModal from './RootModal.svelte'
  import Button from './Button.svelte'
  import Typography from './Typography.svelte'
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

<div use:portal={'body'}>
  <RootModal on:close {open}>
    <div>
      {#if title}
        <ModalHeader>
          <Typography tag="h3">{title}</Typography>
        </ModalHeader>
      {/if}

      {#if message}
        <div class="mb-3">
          <Typography>{message}</Typography>
        </div>
      {/if}

      <ModalFooter>
        <Button label={confirmationButtonLabel} on:click={dispatchConfirmEvent} />
        <Button label="Close" theme="danger" on:click={dispatchCloseEvent} />
      </ModalFooter>
    </div>
  </RootModal>
</div>

