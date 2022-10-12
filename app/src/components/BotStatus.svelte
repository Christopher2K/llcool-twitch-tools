<script lang="ts">
  // Props
  export let isConnected: boolean
  export let isConnectedToUserChat: boolean
  let className: string = ''

  // Computed
  $: statusListItems = [
    {
      label: isConnected ? 'Connected to Twitch socket' : 'Disconnected from Twitch',
      emoji: isConnected ? '🟢' : '🔴',
    },
    {
      label: isConnectedToUserChat ? 'Reading your chat' : 'Not in your chat channel',
      emoji: isConnectedToUserChat ? '🤖' : '💤',
    },
  ]

  export { className as class }
</script>

<ul class="mb-5 {className}">
  {#each statusListItems as item}
    <li class="flex flex-row justify-start items-center mb-2">
      <span class="inline-block icon-emoji mr-5">{item.emoji}</span>
      <span class="inline-block">{item.label}</span>
    </li>
  {/each}
</ul>
