<script lang="ts" context="module">
  import Typography from './Typography.svelte'
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

<article class="{className} {theme}">
  {#if isHeaderDislayed}
    <header class="py-1 px-3">
      <div>
        {#if title}
          <Typography tag="h3">{title}</Typography>
        {/if}
      </div>

      {#if closable}
        <button type="button" on:click={onClose}>Close</button>
      {/if}
    </header>
  {/if}

  <div class="py-1 px-3">
    <slot />
  </div>
</article>

<style lang="scss">
  @import 'theme';

  article {
    width: 100%;
    border-radius: $radius_s;
  }

  header {
    display: flex;
    flex-direction: row;
    justify-content: space-between;
    align-items: center;

    border-top-left-radius: $radius_s;
    border-top-right-radius: $radius_s;

    button {
      font-size: 0.8rem;
      border: none;
      border-bottom: 2px solid $black;
      padding: 0;
      background-color: transparent;
      cursor: pointer;
    }
  }

  .warning {
    background-color: $warning;
    header {
      background-color: $warning_dark;
    }
  }

  .info {
    background-color: $info;
    header {
      background-color: $info_dark;
    }
  }

  .danger {
    background-color: $danger;
    header {
      background-color: $danger_dark;
    }
  }
</style>
