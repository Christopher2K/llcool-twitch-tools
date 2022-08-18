<script lang="ts">
  import type { LayoutData } from './$types'
  import { LOGIN_URL, LOGOUT_URL } from '@app/api'

  export let data: LayoutData
  const { user } = data
</script>

<nav class="navbar is-light" aria-label="main navigation">
  <div class="navbar-brand" />
  <div class="navbar-menu">
    <div class="navbar-start">
      <a class="navbar-item" href="/">Home</a>
    </div>
    <div class="navbar-end">
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
  :global {
    @import 'global.scss';
  }
</style>
