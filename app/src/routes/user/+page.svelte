<script lang="ts">
  import BotStatus from '@app/components/BotStatus.svelte'
  import Banner from '@app/components/Banner.svelte'
  import Button from '@app/components/Button.svelte'

  import { askBotToJoinChat, askBotToLeaveChat } from '@app/api'

  import type { PageData } from './$types'

  // Props
  export let data: PageData
  export const errors: unknown = null

  // State
  let error: string | undefined = undefined
  let updatingBotPresence = false
  let isBotInChat = data.botInfo.connectedToChat

  // Helpers
  const { botInfo } = data

  // Reactive statement
  $: botCtaLabel = isBotInChat ? 'Leave chat' : 'Join chat'
  $: onBotCtaPress = isBotInChat ? leaveChat : joinChat
  let credentialsWarning: string | undefined = undefined
  $: {
    switch (botInfo.credentialsState) {
      case 'invalid':
        credentialsWarning =
          'Bot credentials cannot be renewed manually. Please, log in with the bot account, or contact the admin to do so'
        break
      case 'notFound':
        credentialsWarning =
          'Bot credentials were not created. Please log in with your bot account.'
    }
  }

  // Callback
  function clearError() {
    error = undefined
  }

  function joinChat() {
    clearError()
    updatingBotPresence = true
    askBotToJoinChat()
      .then(() => {
        updatingBotPresence = false
        isBotInChat = true
      })
      .catch(() => {
        updatingBotPresence = false
        error = 'The bot is unavailable!'
      })
  }

  function leaveChat() {
    clearError()
    updatingBotPresence = true
    askBotToLeaveChat()
      .then(() => {
        updatingBotPresence = false
        isBotInChat = false
      })
      .catch(() => {
        updatingBotPresence = false
        error = 'The bot is unavailable!'
      })
  }
</script>

<h1 class="text-5xl font-bold mb-10">Dashboard</h1>

<section>
  <p class="text-xl mb-10">
    In the section, you can get any available metric the bot can show
  </p>

  {#if credentialsWarning}
    <Banner title="The bot is disconnected!" theme="warning" class="mb-5">
      <p>{credentialsWarning}</p>
    </Banner>
  {/if}

  {#if error}
    <Banner title="Error!" on:close={clearError} theme="danger" closable class="mb-5">
      <p>{error}</p>
    </Banner>
  {/if}

  <h2 class="text-3xl font-bold mb-6">Bot global status</h2>
  <BotStatus isConnected={botInfo.connected} isConnectedToUserChat={isBotInChat} />

  <Button
    class="button is-primary"
    isLoading={updatingBotPresence}
    on:click={onBotCtaPress}
    label={botCtaLabel}
  />
</section>
