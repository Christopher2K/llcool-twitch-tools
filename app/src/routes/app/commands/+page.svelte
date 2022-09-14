<script lang="ts">
  import Button from '@app/components/Button.svelte'
  import Typography from '@app/components/Typography.svelte'
  import ConfirmationModal from '@app/components/ConfirmationModal.svelte'

  type Command = {
    name: string
    message: string
  }

  const fakeCommands: Command[] = [
    {
      name: 'vsc',
      message: 'Fuck visual studio code',
    },
    {
      name: 'intellij',
      message: 'Eat up your ram cuz the JVM is broken by design',
    },
    {
      name: 'vim',
      message: 'Use nvim old ass',
    },
  ]

  // State
  let deleteConfirmationModalOpen = false

  // Callback
  function openDeleteConfirmationModal() {
    deleteConfirmationModalOpen = true
  }

  function closeDeleteConfirmationModal() {
    deleteConfirmationModalOpen = false
  }
</script>

<Typography tag="h1" class="mb-3">Commands</Typography>

<section>
  <header class="mb-3">
    <Typography tag="h2">Your commands</Typography>
    <Button label="Add new command" />
  </header>

  <table>
    <tr>
      <th>Name</th>
      <th>Message</th>
      <th>Actions</th>
    </tr>

    {#each fakeCommands as command}
      <tr>
        <td>{command.name}</td>
        <td>{command.message}</td>
        <td class="actions">
          <Button label="Edit" />
          <Button label="Delete" on:click={openDeleteConfirmationModal} />
        </td>
      </tr>
    {/each}
  </table>
</section>

<ConfirmationModal
  open={deleteConfirmationModalOpen}
  on:close={closeDeleteConfirmationModal}
/>

<style lang="scss">
  @import 'theme';

  header {
    display: inline-grid;
    grid-template-columns: auto auto;
    column-gap: 1rem;
  }

  table {
    width: 100%;
  }

  tr {
    display: grid;
    grid-template-columns: 10rem auto 10rem;
  }

  th,
  td {
    justify-self: start;
    padding: $space_xxs $space_xs;
  }

  .actions {
    display: flex;
    flex-direction: row;
    justify-content: flex-start;
    align-items: flex-start;
    flex-wrap: wrap;

    :global button {
      margin-right: $space_xs;
    }
  }
</style>
