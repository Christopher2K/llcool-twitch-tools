<script lang="ts">
  import { createEventDispatcher } from 'svelte'
  import { portal } from 'svelte-portal'

  import type { Command } from '@app/models'

  import Typography from './Typography.svelte'
  import RootModal from './RootModal.svelte'
  import Button from './Button.svelte'
  import ModalHeader from './ModalHeader.svelte'
  import ModalFooter from './ModalFooter.svelte'

  const dispatch = createEventDispatcher()
  let formElement: HTMLFormElement

  // Props
  export let open = false
  export let command: Command | undefined = undefined

  // State
  export let formCommand: Omit<Command, 'id'> = {
    name: '',
    message: '',
  }

  // Reactive
  $: title = command ? `Edit command !${command.name}` : 'Create command'
  $: confirmButtonLabel = command ? 'Save' : 'Create'

  // Reactive blocks
  $: if (open && command) {
    const { id: _, ...data } = command
    setEditCommand(data)
  }

  // Callbacks
  function setEditCommand(data: Omit<Command, 'id'>) {
    formCommand = data
  }

  function dispatchConfirmEvent() {
    dispatch('confirm', {
      id: command?.id,
      command: formCommand,
    })
  }

  function onClose() {
    dispatch('close')
    formElement.reset()
  }
</script>

<div use:portal={'body'}>
  <RootModal {open} on:close={onClose}>
    <div class="root">
      <ModalHeader>
        <Typography tag="h3">{title}</Typography>
      </ModalHeader>
      <form on:submit|preventDefault={dispatchConfirmEvent} bind:this={formElement}>
        <fieldset>
          <label for="name">Name</label>
          <input name="name" type="text" bind:value={formCommand.name} />
        </fieldset>
        <fieldset>
          <label for="message">Message</label>
          <textarea name="message" bind:value={formCommand.message} />
        </fieldset>
        <ModalFooter tag="fieldset">
          <Button label={confirmButtonLabel} type="submit" />
          <Button label="Close" type="button" theme="danger" on:click={onClose} />
        </ModalFooter>
      </form>
    </div>
  </RootModal>
</div>

<style lang="scss">
  @import 'theme.scss';

  .root {
    max-width: 600px;
    width: 100%;
  }

  form {
    display: flex;
    flex-direction: column;
    justify-content: flex-start;
    align-items: flex-start;
    gap: $space_m;

    width: 100%;
  }

  fieldset {
    display: flex;
    flex-direction: column;
    justify-content: flex-start;
    align-items: flex-start;
    width: 100%;

    gap: $space_xxs;
  }

  label {
    font-weight: 600;
    font-size: 1.1rem;
  }

  input,
  textarea {
    width: 100%;

    border: 2px solid;
    border-radius: $radius_s;

    font-size: 1rem;
    font-weight: 500;

    padding: $space_xs;
  }

  input {
    height: 2rem;
  }
</style>
