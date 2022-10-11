<script lang="ts">
  import { browser } from '$app/environment'

  import NavMenuItem, { type NavMenuItemDef } from '@app/components/NavMenuItem.svelte'
  import { LOGOUT_URL } from '@app/constants'

  // Props
  export let menuItems: NavMenuItemDef[] = [];

  // State
  let menuContainer: HTMLDivElement
  let isMenuOpen = false

  // Callback
  function toggleMenu() {
    isMenuOpen = !isMenuOpen
  }

  function closeMenu() {
    isMenuOpen = false
  }

  function onClickOutside(event: MouseEvent) {
    const target = event.target as HTMLElement

    if (!menuContainer.contains(target)) {
      isMenuOpen = false
    }
  }

  // Side effects
  $: if (isMenuOpen && browser) {
    document.addEventListener('click', onClickOutside)
  }
  $: if (!isMenuOpen && browser) {
    document.removeEventListener('click', onClickOutside)
  }
</script>

<div class="w-full h-full">
  <nav
    class="flex flex-row justify-between items-center px-4 w-full bg-blue-300 h-16 relative"
    class:open={isMenuOpen}
  >
    <a class="font-bold text-xl" href="/">Twitch Tools</a>
    <button type="button" on:click|stopPropagation={toggleMenu} class="font-bold text-lg">
      Menu
    </button>
    <div
      bind:this={menuContainer}
      class:hidden={!isMenuOpen}
      role="menu"
      aria-expanded={isMenuOpen}
      class="absolute right-4 top-full bg-blue-300 mt-2 rounded-md"
    >
      <div class="md:hidden">
        {#each menuItems as item}
          <NavMenuItem on:navigate={closeMenu} href={item.href} label={item.label} />
        {/each}
      </div>

      <NavMenuItem on:navigate={closeMenu} href={LOGOUT_URL} label="Logout" />
    </div>
  </nav>

  <div class="h-full w-full flex flex-row justify-start items-start">
    <nav class="hidden md:block shrink-0 w-80 h-full bg-blue-300">
      {#each menuItems as item}
        <NavMenuItem on:navigate={closeMenu} href={item.href} label={item.label} />
      {/each}
    </nav>
    <div class="shrink flex-1">
      <main class="p-4 mx-auto max-w-screen-lg">
        <slot name="content" />
      </main>
    </div>
  </div>
</div>
