<script lang="ts">
  import { createEventDispatcher } from 'svelte'
  import { portal } from 'svelte-portal'

  import type { Command } from '@app/formType'

  import RootModal from './RootModal.svelte'
  import Button from './Button.svelte'
  import ModalHeader from './ModalHeader.svelte'
  import ModalFooter from './ModalFooter.svelte'
  import Fieldset from './Fieldset.svelte'

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
    <div class="w-full">
      <ModalHeader>
        <h3>{title}</h3>
      </ModalHeader>
      <form
        on:submit|preventDefault={dispatchConfirmEvent}
        bind:this={formElement}
        class="flex flex-col justify-start items-start w-full gap-y-5"
      >
        <Fieldset>
          <label for="name" class="font-semibold text-lg">Name</label>
          <input
            name="name"
            type="text"
            bind:value={formCommand.name}
            class="w-full border-2 border-black rounded-md font-semibold p-2"
          />
        </Fieldset>
        <Fieldset>
          <label for="message" class="font-semibold text-lg">Message</label>
          <textarea
            name="message"
            bind:value={formCommand.message}
            class="w-full border-2 border-black rounded-md font-semibold p-2"
          />
        </Fieldset>
        <ModalFooter tag="fieldset">
          <Button label={confirmButtonLabel} type="submit" />
          <Button label="Close" type="button" theme="danger" on:click={onClose} />
        </ModalFooter>
      </form>
    </div>
  </RootModal>
</div>
