<script lang="ts">
  import { apiUrl } from '@app/env'
  const loginToTwitchUrl = `${apiUrl}/auth/login`
</script>

<nav class="navbar is-light" aria-label="main navigation">
  <div class="navbar-brand" />
  <div class="navbar-menu">
    <div class="navbar-start">
      <a class="navbar-item" href="/">Home</a>
    </div>
    <div class="navbar-end">
      <div class="navbar-item">
        <a class="button is-primary" href={loginToTwitchUrl}>
          Log in with Twitch
        </a>
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
