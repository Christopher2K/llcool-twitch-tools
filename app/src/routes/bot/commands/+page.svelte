<script lang="ts">
  import type { PageData } from './$types'

  import type { GlobalCommand } from '@app/api'
  import Button from '@app/components/Button.svelte'
  import CommandRow from '@app/components/CommandRow.svelte'

  export let data: PageData

  function openGlobalCommandFormModal(command: GlobalCommand | undefined = undefined) {
    console.log('Implement form for global command', command)
  }

  function openDeleteConfirmationModal(command: GlobalCommand) {
    console.log('Implement openDeleteConfirmationModal', command)
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

        {#if command.commandDefinition._type === 'Plain'}
          <CommandRow label="Name" hideBottomBorder
            >{command.commandDefinition.name}</CommandRow
          >
          <CommandRow label="Message" hideBottomBorder
            >{command.commandDefinition.message}</CommandRow
          >
        {/if}

        {#if command.commandDefinition._type === 'Pattern'}
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
