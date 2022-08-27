<script lang="ts">
  import type { LayoutData } from './$types'
  import { LOGIN_URL, LOGOUT_URL } from '@app/api'

  // Props
  export let data: LayoutData

  // Binding
  let menu: HTMLDivElement

  // State
  let menuIsOpen = false

  // Variables
  const { user, isBot } = data
  const logged = user != null
  const loggedUserName = logged ? `${isBot ? '[BOT]' : ''} ${user.username}` : undefined

  function closeMenuOnNavigate(event: Event) {
    menuIsOpen = false
    event.target?.removeEventListener('click', closeMenuOnNavigate)
  }

  function toggleMenu() {
    if (!menuIsOpen) {
      menu.querySelectorAll('a').forEach(el => {
        el.addEventListener('click', closeMenuOnNavigate)
      })
    }
    menuIsOpen = !menuIsOpen
  }
</script>

<nav class="navbar is-light" aria-label="main navigation">
  <div class="navbar-brand">
    <button
      type="button"
      class="navbar-burger"
      class:is-active={menuIsOpen}
      on:click={toggleMenu}
    >
      <span />
      <span />
      <span />
    </button>
  </div>
  <div bind:this={menu} class="navbar-menu" class:is-active={menuIsOpen}>
    <div class="navbar-start">
      <a class="navbar-item" href="/">Home</a>
      {#if user && isBot}
        <a class="navbar-item" href="/bot">Bot settings</a>
      {/if}

      {#if user && !isBot}
        <a class="navbar-item" href="/app">Dashboard</a>
      {/if}
    </div>
    <div class="navbar-end">
      {#if loggedUserName}
        <div class="navbar-item">
          <p>{loggedUserName}</p>
        </div>
      {/if}
      <div class="navbar-item">
        {#if user}
          <a class="button is-primary" href={LOGOUT_URL}>Logout</a>
        {:else}
          <a class="button is-primary" href={LOGIN_URL}>Log in with Twitch</a>
        {/if}
      </div>
    </div>
  </div>
</nav>

<slot />

<style lang="scss">
  @import 'bulma/css/bulma.css';

  :global {
    @import 'global.scss';
  }
</style>
