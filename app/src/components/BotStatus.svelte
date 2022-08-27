<script lang="ts">
  export let isConnected: boolean
  export let isConnectedToUserChat: boolean

  const connexionLabel = isConnected
    ? 'Connected to Twitch socket'
    : 'Disconnected from Twitch'
  const chatConnexionLabel = isConnectedToUserChat
    ? 'Reading your chat'
    : 'Not in your chat channel'
</script>

<ul class="mb-5">
  <li>
    <span class="mr-2">{isConnected ? '🟢' : '🔴'}</span>
    <span>{connexionLabel}</span>
  </li>
  <li>
    <span class="mr-2">{isConnectedToUserChat ? '🤖' : '💤'}</span>
    <span>{chatConnexionLabel}</span>
  </li>
</ul>

<style lang="scss">
  li {
    display: flex;
    flex-direction: row;
    justify-content: flex-start;
    align-items: center;
  }

  li:first-child {
  }

  span {
    display: block;
  }
</style>
