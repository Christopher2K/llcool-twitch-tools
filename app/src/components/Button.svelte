<script lang="ts" context="module">
  export type ButtonTheme = 'primary'
</script>

<script lang="ts">
  export let label: string
  export let theme: ButtonTheme = 'primary'
  export let href: string | undefined = undefined
  let className: string = ''
  export { className as class }
</script>

{#if href}
  <a {href} class="{className} {theme}">{label}</a>
{:else}
  <button on:click class="{className} {theme}">
    {label}
  </button>
{/if}

<style lang="scss">
  @import 'theme.scss';

  a,
  button {
    display: block;
    padding: $space_xs;

    border: 4px solid $primary_dark;
    border-radius: $radius_s;

    font-weight: 700;
    cursor: pointer;

    &:hover {
      opacity: 0.7;
    }
  }

  .primary {
    background-color: $white;
    color: $primary_dark;
  }
</style>
