<script lang="ts">
  import { LOGIN_URL, LOGOUT_URL } from '@app/api'
  import { userIsLogged, authIsLoading } from '@app/stores'

  export let data: unknown = null
</script>

<nav class="navbar is-light" aria-label="main navigation">
  <div class="navbar-brand" />
  <div class="navbar-menu">
    <div class="navbar-start">
      <a class="navbar-item" href="/">Home</a>
    </div>
    <div class="navbar-end">
      <div class="navbar-item">
        {#if $authIsLoading}
          Loading...
        {:else if $userIsLogged}
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
  :global {
    @import 'global.scss';
  }
</style>
