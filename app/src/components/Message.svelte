<script lang="ts">
  import { createEventDispatcher } from 'svelte'

  export let message: string

  const dispatch = createEventDispatcher()

  function emitCloseEvent() {
    dispatch('close')
  }
</script>

<article class="message is-danger">
  <div class="message-header">
    <button class="delete" on:click={emitCloseEvent} />
  </div>
  <div class="message-body">
    {message}
  </div>
</article>
