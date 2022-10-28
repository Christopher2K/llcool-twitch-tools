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
  const forms: GlobalCommandType[] = ['Plain', 'Pattern']
  const formIdByFormType: Record<GlobalCommandType, string> = {
    Plain: 'plainFormId',
    Pattern: 'patternFormId',
  }

  // Props
  export let open = false
  export let initialGlobalCommand: GlobalCommand | undefined = undefined

  // State
  let commandType: GlobalCommandType | undefined = undefined
  let formDropdownOpen = false

  // Dynamic
  $: activeFormId = commandType ? formIdByFormType[commandType] : undefined

  // Callback
  function toggleFormDropdown() {
    formDropdownOpen = !formDropdownOpen
  }

  function closeFormDropdown() {
    formDropdownOpen = false
  }

  function onFormSelect(event: CustomEvent) {
    commandType = event.detail
    formDropdownOpen = false
  }

  function onCloseModal() {
    dispatch('close')
  }

  function onGlobalCommandForm(event: CustomEvent) {
    console.log('Form sent', event)
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
    {#if commandType === 'Pattern'}
      <PatternForm on:submit={onGlobalCommandForm} id={formIdByFormType['Pattern']} />
    {:else if commandType === 'Plain'}
      <PlainForm on:submit={onGlobalCommandForm} id={formIdByFormType['Plain']} />
    {/if}
  </div>

  <ModalFooter>
    {#if commandType}
      <Button type="submit" form={activeFormId} label="Create" />
    {/if}
    <Button label="Close" theme="danger" on:click={onCloseModal} />
  </ModalFooter>
</RootModal>
