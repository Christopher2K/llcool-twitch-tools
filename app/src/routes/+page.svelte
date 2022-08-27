<script lang="ts">
  import BotStatus from '@app/components/BotStatus.svelte'
  import Loader from '@app/components/Loader.svelte'

  import type { PageData } from './$types'

  export let data: PageData
  export const errors: unknown = null

  const { botInfo } = data
</script>

<div class="card">
  <div class="card-content">
    <h1 class="title">Bot status</h1>
    <BotStatus
      isConnected={botInfo.connected}
      isConnectedToUserChat={botInfo.connectedToChat}
    />
    <button type="button" class="button is-primary">Join chat</button>
  </div>
</div>

<style lang="scss">
  @import 'theme.scss';
</style>
