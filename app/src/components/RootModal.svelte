<script lang="ts">
  import { portal } from 'svelte-portal'
  import { createEventDispatcher } from 'svelte'

  const dispatch = createEventDispatcher()

  // Props
  export let open = false
  export let fullSize = false

  // Callback
  function onDialogClick() {
    dispatch('close')
  }
</script>

<div
  class="fixed top-0 left-0 z-50 flex-row justify-center items-center w-full h-full md:bg-gray-900/30 md:backdrop-blur-sm"
  class:hidden={!open}
  class:inline-flex={open}
  on:click|self={onDialogClick}
  use:portal={'body'}
>
  <div
    class="flex-1 flex-shrink-0 p-3 bg-white w-full h-full md:max-w-2xl md:rounded-md md:h-auto"
    class:w-full={fullSize}
  >
    <slot />
  </div>
</div>
