<script lang="ts" context="module">
  export type ButtonTheme = 'primary' | 'danger'
</script>

<script lang="ts">
  export let label: string
  export let isLoading: boolean = false
  export let theme: ButtonTheme = 'primary'
  export let href: string | undefined = undefined

  let className: string = ''
  export { className as class }
</script>

{#if href}
  <a {href} class="{className} {theme}">{label}</a>
{:else}
  <button disabled={isLoading} on:click class="{className} {theme}">
    {isLoading ? 'Loading...' : label}
  </button>
{/if}

<style lang="scss">
  @import 'theme.scss';

  a,
  button {
    display: inline-block;
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

  .danger {
    border-color: $danger_dark;
    background-color: $danger_dark;
    color: $white;
  }
</style>
