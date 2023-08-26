<script lang="ts">
  import type { PageData } from './$types'
  import { invalidate } from '$app/navigation'

  import {
    type GlobalCommand,
    updateGlobalCommand,
    deleteGlobalCommand,
    createGlobalCommand,
  } from '@app/api'
  import Button from '@app/components/Button.svelte'
  import CommandRow from '@app/components/CommandRow.svelte'
  import ConfirmationModal from '@app/components/ConfirmationModal.svelte'
  import GlobalCommandFormModal from '@app/components/GlobalCommandFormModal.svelte'

  // Props
  export let data: PageData

  // State
  let pendingDeletionId: string | undefined = undefined
  let pendingEditingCommand: GlobalCommand | undefined = undefined
  let formModalOpen = false
  let ongoingDeletion = false
  let ongoingCreation = false

  // Computed
  $: deleteModalOpen = pendingDeletionId !== undefined

  function openGlobalCommandFormModal(command: GlobalCommand | undefined = undefined) {
    formModalOpen = true
    pendingEditingCommand = command
  }

  function openDeleteConfirmationModal(command: GlobalCommand) {
    pendingDeletionId = command.id
  }

  function closeDeleteModal() {
    pendingDeletionId = undefined
  }

  function closeGlobalCommandFormModal() {
    formModalOpen = false
  }

  async function confirmCommandDelete() {
    if (pendingDeletionId === undefined) return

    try {
      ongoingDeletion = true

      await deleteGlobalCommand(pendingDeletionId)
      await invalidate('globalCommand:all')
    } catch (e) {
      console.error(e)
    } finally {
      pendingDeletionId = undefined
      ongoingDeletion = false
    }
  }

  async function onGlobalCommandFormConfirm(
    event: CustomEvent<GlobalCommand['commandDefinition']>,
  ) {
    try {
      ongoingCreation = true

      if (pendingEditingCommand) {
        await updateGlobalCommand(pendingEditingCommand.id, event.detail)
      } else {
        await createGlobalCommand(event.detail)
      }

      await invalidate('globalCommand:all')

    } catch (e) {
      console.error(e)
    } finally {
      pendingEditingCommand = undefined
      formModalOpen = false
      ongoingCreation = false
    }
  }
</script>

<h1>Global commands</h1>

<p class="mb-5">
  Manage application-wide commands that every streamer can choose to use!
</p>

<section>
  <header class="mb-6">
    <Button
      label="Create a new global command"
      on:click={() => openGlobalCommandFormModal()}
    />
  </header>

  <table class="w-full">
    {#each data.globalCommands as command}
      <tr class="grid mb-5">
        <CommandRow position="first" label="Type" hideBottomBorder>
          {command.commandDefinition._type}
        </CommandRow>

        {#if command.commandDefinition._type === 'plain'}
          <CommandRow label="Name" hideBottomBorder
            >{command.commandDefinition.name}</CommandRow
          >
          <CommandRow label="Message" hideBottomBorder
            >{command.commandDefinition.message}</CommandRow
          >
        {/if}

        {#if command.commandDefinition._type === 'pattern'}
          <CommandRow label="Pattern" hideBottomBorder
            >{command.commandDefinition.pattern}</CommandRow
          >
          <CommandRow label="Message" hideBottomBorder
            >{command.commandDefinition.message}</CommandRow
          >
        {/if}

        <CommandRow label="Actions" position="last">
          <Button label="Edit" on:click={() => openGlobalCommandFormModal(command)} />
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
  on:confirm={confirmCommandDelete}
  on:close={closeDeleteModal}
  open={deleteModalOpen}
  loading={ongoingDeletion}
  title="Delete global command"
  message="Are you sure to delete this global command? Every streamer will loose access to this command!"
/>

{#if formModalOpen}
  <GlobalCommandFormModal
    initialGlobalCommand={pendingEditingCommand}
    open={formModalOpen}
    on:confirm={onGlobalCommandFormConfirm}
    on:close={closeGlobalCommandFormModal}
    loading={ongoingCreation}
  />
{/if}
