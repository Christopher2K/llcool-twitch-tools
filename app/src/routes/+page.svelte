<script lang="ts">
  import Button from '@app/components/Button.svelte'
  import Banner from '@app/components/Banner.svelte'
  import { LOGIN_URL, LOGOUT_URL } from '@app/constants'

  import type { PageData } from './$types'

  // Props
  export let data: PageData
  const { user, isBot } = data

  // Computed
  $: logged = user != null
  $: title = logged ? `Heyo ${user.username}, howdy?` : 'LL Cool Twitch Tools'
</script>

<main
  class="p-6 mx-auto w-full max-w-screen-lg flex flex-col justify-center items-center min-h-full"
>
  <h1 class="text-center font-black text-4xl mb-16">{title}</h1>

  {#if isBot}
    <Banner class="mb-3" theme="info" title="Notice">
      <p>
        You're using your bot account. This is super useful if you want to administrate
        your bot behavior or maybe fix an issue with its connection to Twitch.
      </p>
    </Banner>
  {/if}

  <p class="text-center text-xl mb-16">
    Bot, overlays, and fun extensions for your Twitch channel
  </p>

  <div class="flex flex-col gap-5">
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
</main>
