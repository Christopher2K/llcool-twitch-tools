<script lang="ts">
  import Typography from '@app/components/Typography.svelte'
  import Banner from '@app/components/Banner.svelte'
  import Button from '@app/components/Button.svelte'
  import { connectBotToTwitch } from '@app/api'
  import { invalidate } from '$app/navigation'

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
      await invalidate()
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

<Typography tag="h1">Status</Typography>

{#if error}
  <Banner title="Error!" theme="danger" class="mb-2">
    <Typography>{error}</Typography>
  </Banner>
{/if}

{#if !botInfo.connected}
  <Banner title="The bot is disconnected!" theme="warning" class="mb-2">
    <Typography class="mb-2"
      >The bot cannot listen to user chat. Try to reconnect manually.</Typography
    >
    <Button
      label="Reconnect"
      on:click={manuallyConnectToSocket}
      isLoading={connectingToSocket}
    />
  </Banner>
{/if}

<Typography>
  <span class="icon-emoji mr-2">{botInfo.connected ? '🟢' : '🔴'}</span>
  <span>{connexionLabel}</span>
</Typography>

<style lang="scss">
</style>
