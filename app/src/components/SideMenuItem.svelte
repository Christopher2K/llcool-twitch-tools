<script lang="ts">
  import Typography from './Typography.svelte'

  export let label: string
  export let href: string
  export let alternate: boolean = false
  let className: string = ''
  export { className as class }
</script>

<a {href} class="px-1 py-2 {className}" class:alternate>
  <Typography variant="bold">
    {label}
  </Typography>
</a>

<style lang="scss">
  @import 'theme.scss';

  a {
    display: block;
    width: 100%;

    border-radius: $radius_s;
    border: 4px solid $primary;

    color: $black;

    &:hover {
      background-color: $primary;
    }
  }

  .alternate {
    background-color: $white;
    color: $primary_dark;

    &:hover {
      color: $white;
    }
  }
</style>
