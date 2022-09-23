<script lang="ts">
  export let tag: string = 'footer'
</script>

<svelte:element this={tag} class="footer">
  <slot />
</svelte:element>

<style lang="scss">
  @import 'theme';

  .footer {
    display: flex;
    flex-direction: row;
    justify-content: flex-end;
    align-items: flex-start;
    flex-wrap: wrap;

    column-gap: $space_xxs;

    width: 100%;
  }
</style>
