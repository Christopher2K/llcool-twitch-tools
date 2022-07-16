<div>
  <slot />
</div>

<style lang="scss">
  :global {
    @import 'reset.scss';
    @import 'global.scss';
  }

  div {
    height: 100vh;
    width: 100vw;
  }
</style>
