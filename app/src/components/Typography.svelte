<script context="module" lang="ts">
  export type Tag = 'h1' | 'h2' | 'h3' | 'p'
  export type Alignment = 'left' | 'right' | 'center'
</script>

<script lang="ts">
  export let tag: Tag = 'p'
  export let align: Alignment = 'left'
  export let className: string = ''

  export { className as class }
</script>

<svelte:element this={tag} class="{className} type" style:text-align={align}>
  <slot />
</svelte:element>

<style lang="scss">
  @import 'theme.scss';

  :global {
    h1.type {
      font-size: 2.5rem;
      font-weight: 800;
    }

    p.type {
      font-size: 1rem;
    }
  }
</style>
