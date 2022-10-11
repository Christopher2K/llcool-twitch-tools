<script lang="ts">
  import Banner from '@app/components/Banner.svelte'
  import Button from '@app/components/Button.svelte'
  import { connectBotToTwitch } from '@app/api'
  import { invalidateAll } from '$app/navigation'

  import type { PageData } from './$types'

  // Props
  export let data: PageData

  // State
  let connectingToSocket = false
  let error: string | undefined = undefined

  // Reactive properties
  $: botInfo = data.botInfo
  $: connexionLabel = botInfo.connected
    ? 'Connected to Twitch socket'
    : 'Disconnected from Twitch socket'

  async function manuallyConnectToSocket() {
    clearError()
    connectingToSocket = true

    try {
      await connectBotToTwitch()
      await invalidateAll()
    } catch (e) {
      console.error(e)
      error = 'Cannot connect to Twitch socket'
      // TODO: component error handling
    } finally {
      connectingToSocket = false
    }
  }

  function clearError() {
    error = undefined
  }
</script>

<h1 class="text-5xl font-bold mb-10">Status</h1>

{#if error}
  <Banner title="Error!" theme="danger" class="mb-5">
    <p>{error}</p>
  </Banner>
{/if}

{#if !botInfo.connected}
  <Banner title="The bot is disconnected!" theme="warning" class="mb-5">
    <p class="mb-5">The bot cannot listen to user chat. Try to reconnect manually.</p>
    <Button
      label="Reconnect"
      on:click={manuallyConnectToSocket}
      isLoading={connectingToSocket}
    />
  </Banner>
{/if}

<p class="flex flex-row justify-start items-center">
  <span class="icon-emoji mr-5">{botInfo.connected ? '🟢' : '🔴'}</span>
  <span>{connexionLabel}</span>
</p>
