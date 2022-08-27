<script lang="ts">
  import BotStatus from '@app/components/BotStatus.svelte'
  import Message from '@app/components/Message.svelte'
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

<div class="card">
  <div class="card-content">
    {#if credentialsWarning}
      <Message
        title="The bot is disconnected!"
        message={credentialsWarning}
        severity="warning"  
        persistent
      />
    {/if}
    {#if error}
      <Message on:close={clearError} title="Error!" message={error} />
    {/if}
    <h1 class="title">Bot status</h1>
    <BotStatus isConnected={botInfo.connected} isConnectedToUserChat={isBotInChat} />
    <button
      type="button"
      class="button is-primary"
      class:is-loading={updatingBotPresence}
      on:click={onBotCtaPress}>{botCtaLabel}</button
    >
  </div>
</div>

<style lang="scss">
  @import 'theme.scss';
</style>
