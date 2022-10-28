<script lang="ts">
  import { createEventDispatcher } from 'svelte'

  import type { PlainCommand } from '@app/formType'
  import Fieldset from '../Fieldset.svelte'

  // Static
  const dispatch = createEventDispatcher()

  // Props
  export let id: string | undefined = undefined

  // State
  let formCommand: PlainCommand = {
    _type: 'Plain',
    name: '',
    message: '',
  }

  function onFormSubmit() {
    dispatch('submit', formCommand)
  }
</script>

<form
  {id}
  on:submit|preventDefault={onFormSubmit}
  class="flex flex-col justify-start items-start w-full gap-y-5"
>
  <Fieldset>
    <label for="name" class="font-semibold text-lg">Command name</label>
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
</form>
