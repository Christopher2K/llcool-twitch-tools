<script lang="ts">
  import { createEventDispatcher } from 'svelte'

  import type { PatternCommandDefinition } from '@app/api/globalCommand'
  import Fieldset from '../Fieldset.svelte'

  // Static
  const dispatch = createEventDispatcher()

  // Props
  export let id: string | undefined = undefined
  export let initialForm: PatternCommandDefinition | undefined = undefined

  // State
  let formCommand: PatternCommandDefinition = initialForm ?? {
    _type: 'pattern',
    pattern: '',
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
  {#if formCommand._type === 'pattern'}
    <Fieldset>
      <label for="name" class="font-semibold text-lg">Pattern to look for</label>
      <input
        name="pattern"
        type="text"
        bind:value={formCommand.pattern}
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
  {/if}
</form>
