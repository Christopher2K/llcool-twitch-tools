<script lang="ts">
  import { portal } from 'svelte-portal'
  import { createEventDispatcher } from 'svelte'

  const dispatch = createEventDispatcher()

  // Props
  export let open = false
  export let fullSize = false

  // Callback
  function onDialogClick() {
    dispatch('close')
  }
</script>

<div class="root" class:open on:click|self={onDialogClick} use:portal={'body'}>
  <div class="content p-3" class:fullSize>
    <slot />
  </div>
</div>

<style lang="scss">
  @import 'theme';
  @import 'responsive';

  .root {
    display: none;
    position: fixed;
    top: 0;
    left: 0;

    flex-direction: row;
    justify-content: center;
    align-items: center;

    width: 100%;
    height: 100%;

    background-color: rgba(0, 0, 0, 0.3);
    backdrop-filter: blur(5px);
  }

  .root.open {
    display: inline-flex;
    z-index: 999;
  }

  .content {
    flex: 1;
    flex-shrink: 0;

    max-width: $modal_max_width;
    background-color: $white;
    border-radius: $radius_s;

    @include mobileStyle {
      width: 100%;
      height: 100%;
      max-width: 100%;

      border-radius: 0;
    }
  }

  .content.fullSize {
    width: 100%;

  }
</style>
