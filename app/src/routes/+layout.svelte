<script lang="ts">
  import type { LayoutData } from './$types'
  import { LOGIN_URL, LOGOUT_URL } from '@app/api'

  // Props
  export let data: LayoutData

  // Variables
  const { user, isBot } = data
  const logged = user != null
  const loggedUserName = logged ? `${isBot ? '[BOT]' : ''} ${user.username}` : undefined
</script>

<nav class="navbar is-light" aria-label="main navigation">
  <div class="navbar-brand" />
  <div class="navbar-menu">
    <div class="navbar-start">
      <a class="navbar-item" href="/">Home</a>
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
