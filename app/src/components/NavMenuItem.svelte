<script context="module" lang="ts">
  export type NavMenuItemDef = {
    label: string
    href: string
  }
</script>

<script lang="ts">
  import { createEventDispatcher } from 'svelte'

  import { page } from '$app/stores'

  // Constants
  const dispatch = createEventDispatcher<{ navigate: void }>()

  // Props
  export let label: string
  export let href: string
  let className: string = ''
  export { className as class }

  // Computed
  $: active = $page.url.pathname === href

  // Callback
  function onNavigate() {
    dispatch('navigate')
  }
</script>

<a
  {href}
  class="block px-4 py-2 hover:bg-blue-400 {className}"
  class:active
  on:click={onNavigate}
>
  {label}
</a>

<style lang="postcss">
  a:first-of-type {
    @apply rounded-t-md;
  }

  a:last-of-type {
    @apply rounded-b-md;
  }
</style>
