<slot />

<style lang="scss">
  @import 'theme.scss';

  :global {
    @import 'reset.scss';
    @import 'global.scss';
  }

  :global body {
    min-width: 100%;
    min-height: 100vh;
    background-color: $primary;
  }
</style>
