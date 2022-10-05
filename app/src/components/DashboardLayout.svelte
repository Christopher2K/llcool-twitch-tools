<script lang="ts">
  import Typography from '@app/components/Typography.svelte'
  import SideMenuItem from '@app/components/SideMenuItem.svelte'
  import Button from '@app/components/Button.svelte'
  import { LOGOUT_URL } from '@app/constants';

  // Props
  export let title: string

  // State
  let isMenuOpen = false

  // Callback
  function closeMenu() {
    isMenuOpen = false
  }

  function openMenu() {
    isMenuOpen = true
  }
</script>

<div class="root">
  <nav class="p-3" class:open={isMenuOpen}>
    <div class="responsive-navbar">
      <Button class="mb-2" label="Close" on:click={closeMenu} />
    </div>
    <Typography tag="h2">{title}</Typography>
    <div class="links my-3">
      <slot name="links" />
    </div>

    <SideMenuItem alternate href={LOGOUT_URL} label="Logout" />
  </nav>
  <main class="p-3">
    <div class="responsive-navbar">
      <Button class="mb-2" label="Menu" on:click={openMenu} />
    </div>
    <slot name="content" />
  </main>
</div>

<style lang="scss">
  @import 'theme.scss';
  @import 'responsive.scss';

  .root {
    width: 100%;
    height: 100vh;

    display: grid;
    grid-template-columns: 15rem minmax(0, 60rem);
    grid-gap: $space_s;

    @include mobileStyle {
      grid-template-columns: auto;
    }
  }

  nav {
    width: 100%;
    background-color: $primary_dark;
    min-height: 100%;

    @include mobileStyle {
      display: none;
      position: fixed;
      top: 0;
      left: 0;
      z-index: 99;

      width: 100%;
      height: 100%;

      &.open {
        display: block;
      }
    }
  }

  .links {
    width: 100%;
    display: grid;
    grid-template-columns: auto;
    row-gap: $space_s;
  }

  .responsive-navbar {
    @include desktopStyle {
      display: none;
    }
  }
</style>
