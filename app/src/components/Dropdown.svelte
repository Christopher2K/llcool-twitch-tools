<script lang="ts">
  import { createEventDispatcher } from 'svelte'
  import { browser } from '$app/environment'

  import { ChevronDown, ChevronUp } from 'lucide-svelte'
  import DropdownItem from './DropdownItem.svelte'

  const dispatch = createEventDispatcher()

  // Props
  export let open: boolean
  export let items: string[]
  export let selectedItem: string | undefined = undefined
  let className = ''
  export { className as class }

  // State
  let rootElement: HTMLDivElement

  // Computed
  $: dropdownTitle = selectedItem ?? 'Select an option'

  // Callbacks
  function toggleDropdown() {
    dispatch('toggle')
  }

  function closeOnClickOutside(event: MouseEvent) {
    if (event.target == null) return
    const target = event.target as HTMLElement

    const isRootElement = event.target === rootElement
    const isContainedByRootElement = rootElement.contains(target)

    if (isRootElement || isContainedByRootElement) return

    dispatch('close')
  }

  function onItemClick(item: string) {
    dispatch('itemClick', item)
  }

  $: if (open && browser) {
    document.addEventListener('click', closeOnClickOutside)
  } else if (browser) {
    document.removeEventListener('click', closeOnClickOutside)
  }
</script>

<div bind:this={rootElement} class="relative max-w-fit {className}">
  <button
    class="flex flex-row justify-start items-center text-left border border-solid rounded-md px-4 py-2 font-bold"
    on:click={toggleDropdown}
    type="menu"
  >
    <span class="mr-2">{dropdownTitle}</span>
    <span class:hidden={!open} class:visible={open}>
      <ChevronUp />
    </span>
    <span class:hidden={open}>
      <ChevronDown />
    </span>
  </button>

  <div
    class="items absolute top-full mt-3 flex flex-col justify-start items-start max-w-fit border border-solid rounded-md bg-white"
    class:open
  >
    {#each items as item}
      <DropdownItem label={item} on:click={() => onItemClick(item)} />
    {/each}
    <slot />
  </div>
</div>

<style lang="postcss">
  .items {
    visibility: hidden;
  }

  .items.open {
    visibility: visible;
  }
</style>
