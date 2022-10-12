<script lang="ts">
  export let tag: string = 'footer'
</script>

<svelte:element this={tag} class="flex flex-row justify-end items-start gap-x-2 flex-wrap w-full">
  <slot />
</svelte:element>
