<script lang="ts">
  import { createEventDispatcher } from 'svelte'

  import type { GlobalCommandType, GlobalCommand } from '@app/api'
  import Dropdown from './Dropdown.svelte'
  import RootModal from './RootModal.svelte'
  import ModalFooter from './ModalFooter.svelte'
  import ModalHeader from './ModalHeader.svelte'
  import PatternForm from './globalCommandForm/PatternForm.svelte'
  import PlainForm from './globalCommandForm/PlainForm.svelte'
  import Button from './Button.svelte'

  // Static
  const dispatch = createEventDispatcher()
  const forms: GlobalCommandType[] = ['plain', 'pattern']
  const formIdByFormType: Record<GlobalCommandType, string> = {
    plain: 'plainFormId',
    pattern: 'patternFormId',
  }

  // Props
  export let open = false
  export let initialGlobalCommand: GlobalCommand | undefined = undefined
  export let loading = false

  // State
  let controlledCommandType: GlobalCommandType | undefined =
    initialGlobalCommand?.commandDefinition._type
  let formDropdownOpen = false

  // Dynamic
  $: activeFormId = commandType ? formIdByFormType[commandType] : undefined
  $: submitButtonLabel = initialGlobalCommand ? 'Save' : 'Create'
  $: commandType = initialGlobalCommand?.commandDefinition._type ?? controlledCommandType 

  // Callback
  function toggleFormDropdown() {
    formDropdownOpen = !formDropdownOpen
  }

  function closeFormDropdown() {
    formDropdownOpen = false
  }

  function onFormSelect(event: CustomEvent<GlobalCommandType>) {
    commandType = event.detail
    formDropdownOpen = false
  }

  function onCloseModal() {
    dispatch('close')
  }

  function onGlobalCommandForm(event: CustomEvent<GlobalCommand['commandDefinition']>) {
    dispatch('confirm', event.detail)
  }
</script>

<RootModal {open} fullSize on:close>
  <ModalHeader>
    <h3>New global command</h3>
  </ModalHeader>

  {#if !initialGlobalCommand}
    <Dropdown
      class="mb-5"
      on:close={closeFormDropdown}
      on:toggle={toggleFormDropdown}
      on:itemClick={onFormSelect}
      open={formDropdownOpen}
      items={forms}
      selectedItem={commandType}
    />
  {/if}

  <div class="mb-5">
    {#if open}
      {#if commandType === 'pattern'}
        <PatternForm
          initialForm={initialGlobalCommand?.commandDefinition._type === 'pattern'
            ? initialGlobalCommand.commandDefinition
            : undefined}
          on:submit={onGlobalCommandForm}
          id={formIdByFormType['pattern']}
        />
      {:else if commandType === 'plain'}
        <PlainForm
          initialForm={initialGlobalCommand?.commandDefinition._type === 'plain'
            ? initialGlobalCommand.commandDefinition
            : undefined}
          on:submit={onGlobalCommandForm}
          id={formIdByFormType['plain']}
        />
      {/if}
    {/if}
  </div>

  <ModalFooter>
    {#if commandType}
      <Button
        isLoading={loading}
        type="submit"
        form={activeFormId}
        label={submitButtonLabel}
      />
    {/if}
    <Button isLoading={loading} label="Close" theme="danger" on:click={onCloseModal} />
  </ModalFooter>
</RootModal>
