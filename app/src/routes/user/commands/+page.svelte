<script lang="ts">
  import Button from '@app/components/Button.svelte'
  import Typography from '@app/components/Typography.svelte'
  import ConfirmationModal from '@app/components/ConfirmationModal.svelte'
  import CommandFormModal from '@app/components/CommandFormModal.svelte'
  import type { Command } from '@app/models'

  const fakeCommands: Command[] = [
    {
      id: '1',
      name: 'vsc',
      message: 'Fuck visual studio code',
    },
    {
      id: '2',
      name: 'intellij',
      message: 'Eat up your ram cuz the JVM is broken by design',
    },
    {
      id: '3',
      name: 'vim',
      message: 'Use nvim old ass',
    },
  ]

  // State
  let deleteConfirmationModalOpen = false
  let commandFormModalOpen = false
  let selectedCommand: Command | undefined = undefined

  // Reactive
  $: confirmationModalDeleteMessage = selectedCommand
    ? `Are you sure to delete the command ${selectedCommand.name} ?`
    : ''

  // Callback
  function openDeleteConfirmationModal(command: Command) {
    deleteConfirmationModalOpen = true
    selectedCommand = command
  }

  function closeDeleteConfirmationModal() {
    deleteConfirmationModalOpen = false
    selectedCommand = undefined
  }

  function openCommandFormModal(command: Command | undefined = undefined) {
    selectedCommand = command
    commandFormModalOpen = true
  }

  function closeCommandFormModal() {
    commandFormModalOpen = false
    selectedCommand = undefined
  }

  function deleteCommand() {
    console.log('Deleting command ', selectedCommand)
    closeDeleteConfirmationModal()
  }

  function onConfirmFormModal(
    event: CustomEvent<{ id?: string; command: Omit<Command, 'id'> }>,
  ) {
    console.log(event.detail)
  }
</script>

<Typography tag="h1" class="mb-3">Commands</Typography>

<section>
  <header class="mb-3">
    <Typography tag="h2">Your commands</Typography>
    <Button label="Add new command" on:click={() => openCommandFormModal()} />
  </header>

  <table>
    <tr>
      <th>Name</th>
      <th>Message</th>
      <th>Actions</th>
    </tr>

    {#each fakeCommands as command}
      <tr>
        <td>
          <span class="label">Name</span>
          <span class="value">{command.name}</span>
        </td>
        <td>
          <span class="label">Message</span>
          <span class="value">{command.message}</span>
        </td>
        <td class="actions">
          <span class="label">Actions</span>
          <span>
            <Button label="Edit" on:click={() => openCommandFormModal(command)} />
            <Button
              label="Delete"
              on:click={() => openDeleteConfirmationModal(command)}
            />
          </span>
        </td>
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

<style lang="scss">
  @import 'theme';
  @import 'responsive';

  header {
    display: inline-grid;
    gap: 1rem;

    @include desktopStyle {
      grid-template-columns: auto auto;
    }

    @include mobileStyle {
      grid-template-rows: auto auto;
    }
  }

  table {
    width: 100%;
  }

  tr {
    display: grid;

    @include desktopStyle {
      grid-template-columns: 5rem auto 10rem;
    }

    @include mobileStyle {
      grid-template-rows: auto auto auto;
    }

    &:first-of-type {
      @include mobileStyle {
        display: none;
      }
    }
  }

  th,
  td {
    justify-self: start;
    padding: $space_xxs $space_xs;
  }

  td {
    @include mobileStyle {
      display: grid;
      grid-template-columns: 6rem auto;
    }

    .label {
      display: none;

      @include mobileStyle {
        display: block;
        font-weight: 700;
      }
    }
  }

  .actions {
    @include desktopStyle {
      display: flex;
      flex-direction: row;
      justify-content: flex-start;
      align-items: flex-start;
      flex-wrap: wrap;

      :global button {
        margin-right: $space_xs;
      }
    }
  }
</style>
