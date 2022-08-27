<script lang="ts" context="module">
  export type LoaderSize = 's' | 'm' | 'l'
</script>

<script lang="ts">
  export let size: LoaderSize = 'm'
</script>

<span class="bulma-loader-mixin {size}" />

<style lang="scss">
  @import 'bulma/sass/utilities/mixins.sass';

  span {
    @include loader;
  }

  .s {
    $size: 1rem;
    width: $size;
    height: $size;
  }

  .m {
    $size: 1.6rem;
    width: $size;
    height: $size;
  }

  .l {
    $size: 2.4rem;
    width: $size;
    height: $size;
  }
</style>

