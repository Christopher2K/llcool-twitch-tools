<script lang="ts" context="module">
  import Typography from './Typography.svelte'

  export type BannerTheme = 'warning' | 'error' | 'info'
</script>

<script lang="ts">
  export let theme: BannerTheme
  export let title: string | undefined = undefined
  let className: string = ""
  export { className as class }

  $: isHeaderDislayed = Boolean(title)
</script>

<article class="{className} {theme}">
  {#if isHeaderDislayed}
    <header class="py-1 px-3">
      {#if title}
        <Typography tag="h3">{title}</Typography>
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
    border-top-left-radius: $radius_s;
    border-top-right-radius: $radius_s;
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
</style>
