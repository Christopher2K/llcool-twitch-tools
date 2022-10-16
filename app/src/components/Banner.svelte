<script lang="ts" context="module">
  import { createEventDispatcher } from 'svelte'

  export type BannerTheme = 'warning' | 'danger' | 'info'
</script>

<script lang="ts">
  // Event emitter
  export const dispatch = createEventDispatcher()

  // Props
  export let theme: BannerTheme
  export let title: string | undefined = undefined
  export let closable: boolean = false
  let className: string = ''
  export { className as class }

  // Callback
  function onClose() {
    dispatch('close')
  }

  // Reactive
  $: isHeaderDislayed = Boolean(title) || closable
</script>

<article class="w-full rounded-md {className} {theme}">
  {#if isHeaderDislayed}
    <header class="flex flex-row justify-between items-center rounded-t-md py-2 px-3">
      <div>
        {#if title}
          <h3 class="font-bold text-xl m-0">{title}</h3>
        {/if}
      </div>

      {#if closable}
        <button type="button" on:click={onClose} class="underline">Close</button>
      {/if}
    </header>
  {/if}

  <div class="py-2 px-3">
    <slot />
  </div>
</article>

<style lang="postcss">
  .warning {
    @apply bg-amber-400;
  }
  .warning header {
    @apply bg-amber-600;
  }

  .info {
    @apply bg-cyan-400;
  }
  .info header {
    @apply bg-cyan-600;
  }

  .danger {
    @apply bg-red-400;
  }
  .danger header {
    @apply bg-red-600;
  }
</style>
