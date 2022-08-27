<script lang="ts" context="module">
  export type Severity = 'danger' | 'warning'
</script>

<script lang="ts">
  import { createEventDispatcher } from 'svelte'

  export let title: string = ''
  export let message: string
  export let severity: Severity = 'danger'
  export let persistent: boolean = false

  const dispatch = createEventDispatcher()

  function emitCloseEvent() {
    dispatch('close')
  }
</script>

<article class="message is-{severity}">
  <div class="message-header">
    <p>{title}</p>
    {#if !persistent}
      <button class="delete" on:click={emitCloseEvent} />
    {/if}
  </div>
  <div class="message-body">
    {message}
  </div>
</article>
