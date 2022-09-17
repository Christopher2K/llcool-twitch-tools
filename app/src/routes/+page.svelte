<script lang="ts">
  import { LOGIN_URL, LOGOUT_URL } from '@app/api'
  import Button from '@app/components/Button.svelte'
  import Typography from '@app/components/Typography.svelte'
  import Banner from '@app/components/Banner.svelte'

  import type { PageData } from './$types'

  // Props
  export let data: PageData
  const { user, isBot } = data

  // Computed
  $: logged = user != null
  $: title = logged ? `Heyo ${user.username}, howdy?` : 'LL Cool Twitch Tools'
</script>

<main>
  <div>
    <Typography tag="h1" align="center" class="mb-3">{title}</Typography>

    {#if isBot}
      <Banner class="mb-3" theme="info" title="Notice">
        <Typography align="center">
          You're using your bot account. This is super useful if you want to administrate
          your bot behavior or maybe fix an issue with its connection to Twitch.
        </Typography>
      </Banner>
    {/if}

    <Typography align="center" class="mb-5"
      >Bot, overlays, and fun extensions for your Twitch channel</Typography
    >

    <div class="buttons">
      {#if !user}
        <Button href={LOGIN_URL} label="Login with Twitch" />
      {:else}
        {#if isBot}
          <Button href="/bot" label="Go to bot dashboard" />
        {:else}
          <Button href="/user" label="Go to dashboard" />
        {/if}
        <Button href={LOGOUT_URL} label="Logout" />
      {/if}
    </div>
  </div>
</main>

<style lang="scss">
  @import 'theme.scss';

  main {
    width: 100%;
    height: 100vh;

    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;

    padding: $space_xl;
  }

  div {
    max-width: 720px;

    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
  }

  .buttons {
    width: 100%;

    display: flex;
    flex-direction: row;
    justify-content: center;
    align-items: center;

    :global * {
      margin: $space_m;
    }
  }
</style>
