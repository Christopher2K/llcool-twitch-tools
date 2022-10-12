<script lang="ts">
  import Button from '@app/components/Button.svelte'
  import ConfirmationModal from '@app/components/ConfirmationModal.svelte'
  import CommandFormModal from '@app/components/CommandFormModal.svelte'

  import {
    type UserCommand,
    deleteUserCommand,
    createUserCommand,
    editUserCommand,
  } from '@app/api/command'

  import type { PageData } from './$types'
  import { invalidateAll } from '$app/navigation'
  import CommandRow from '@app/components/CommandRow.svelte'

  // Props
  export let data: PageData

  // State
  let deleteConfirmationModalOpen = false
  let commandFormModalOpen = false
  let selectedCommand: UserCommand | undefined = undefined

  // Reactive
  $: confirmationModalDeleteMessage = selectedCommand
    ? `Are you sure to delete the command ${selectedCommand.name} ?`
    : ''

  // Callback
  function openDeleteConfirmationModal(command: UserCommand) {
    deleteConfirmationModalOpen = true
    selectedCommand = command
  }

  function closeDeleteConfirmationModal() {
    deleteConfirmationModalOpen = false
    selectedCommand = undefined
  }

  function openCommandFormModal(command: UserCommand | undefined = undefined) {
    selectedCommand = command
    commandFormModalOpen = true
  }

  function closeCommandFormModal() {
    commandFormModalOpen = false
    selectedCommand = undefined
  }

  async function deleteCommand() {
    if (selectedCommand) {
      await deleteUserCommand(selectedCommand.id)
      await invalidateAll()
      closeDeleteConfirmationModal()
    }
  }

  async function onConfirmFormModal(
    event: CustomEvent<{ id?: string; command: Omit<UserCommand, 'id'> }>,
  ) {
    const { id, command } = event.detail
    if (id) {
      await editUserCommand(id, command)
    } else {
      await createUserCommand(command)
    }

    await invalidateAll()
    commandFormModalOpen = false
  }
</script>

<h1>Commands</h1>

<section>
  <header class="flex flex-col justify-start items-start mb-6">
    <Button label="Add a new command" on:click={() => openCommandFormModal()} />
  </header>

  <table class="w-full">
    {#each data.userCommands as command}
      <tr class="grid mb-5">
        <CommandRow position="first" label="Name" hideBottomBorder>
          {command.name}
        </CommandRow>

        <CommandRow label="Message" hideBottomBorder>
          {command.message}
        </CommandRow>

        <CommandRow label="Actions" position="last">
          <Button label="Edit" on:click={() => openCommandFormModal(command)} />
          <Button
            label="Delete"
            theme="danger"
            on:click={() => openDeleteConfirmationModal(command)}
          />
        </CommandRow>
     </tr>
    {/each}
  </table>
</section>

<ConfirmationModal
  open={deleteConfirmationModalOpen}
  confirmationButtonLabel="Delete"
  title={confirmationModalDeleteMessage}
  on:confirm={deleteCommand}
  on:close={closeDeleteConfirmationModal}
/>

<CommandFormModal
  open={commandFormModalOpen}
  command={selectedCommand}
  on:confirm={onConfirmFormModal}
  on:close={closeCommandFormModal}
/>
