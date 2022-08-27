<script lang="ts">
  import BotStatus from '@app/components/BotStatus.svelte'
  import { askBotToJoinChat, askBotToLeaveChat } from '@app/api'

  import type { PageData } from './$types'

  // Props
  export let data: PageData
  export const errors: unknown = null

  // State
  let updatingBotPresence = false
  let isBotInChat = data.botInfo.connectedToChat

  // Helpers
  const { botInfo } = data

  // Reactive statement
  $: botCtaLabel = isBotInChat ? 'Leave chat' : 'Join chat'
  $: onBotCtaPress = isBotInChat ? leaveChat : joinChat

  // Callback
  function joinChat() {
    updatingBotPresence = true
    askBotToJoinChat().then(() => {
      updatingBotPresence = false
      isBotInChat = true
    })
  }

  function leaveChat() {
    updatingBotPresence = true
    askBotToLeaveChat().then(() => {
      updatingBotPresence = false
      isBotInChat = false
    })
  }
</script>

<div class="card">
  <div class="card-content">
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
