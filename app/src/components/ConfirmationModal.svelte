<script lang="ts">
  import { createEventDispatcher } from 'svelte'
  import { portal } from 'svelte-portal'

  import RootModal from './RootModal.svelte'
  import Button from './Button.svelte'
  import Typography from './Typography.svelte'

  const dispatch = createEventDispatcher()

  // Props
  export let open = false
  export let title: string | undefined = 'Confirm'
  export let message: string | undefined = undefined
  export let confirmationButtonLabel: string = 'Confirm'

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

  function dispatchConfirmEvent() {
    dispatch('confirm')
  }
</script>

<div use:portal={'body'}>
  <RootModal bind:this={rootModalComponent}>
    {#if title}
      <header class="mb-3">
        <Typography tag="h3">{title}</Typography>
      </header>
    {/if}

    {#if message}
      <div class="mb-3">
        <Typography>{message}</Typography>
      </div>
    {/if}

    <footer>
      <Button label={confirmationButtonLabel} on:click={dispatchConfirmEvent} />
      <Button label="Close" theme="danger" on:click={dispatchCloseEvent} />
    </footer>
  </RootModal>
</div>

<style lang="scss">
  @import 'theme';

  footer {
    display: flex;
    flex-direction: row;
    justify-content: flex-end;
    align-items: flex-start;
    flex-wrap: wrap;

    column-gap: $space_xxs;
  }
</style>
