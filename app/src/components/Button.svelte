<script lang="ts" context="module">
  export type ButtonTheme = 'primary' | 'danger'
</script>

<script lang="ts">
  export let label: string
  export let isLoading: boolean = false
  export let theme: ButtonTheme = 'primary'
  export let href: string | undefined = undefined
  export let type: HTMLButtonElement['type'] = 'button'
  export let tailwindClassNames = 'border-4 rounded-lg px-5 py-3 text-2xl text-center'

  let className: string = ''
  export { className as class }
</script>

{#if href}
  <a {href} class="{tailwindClassNames} {className} {theme}">{label}</a>
{:else}
  <button
    {type}
    disabled={isLoading}
    on:click
    class="{tailwindClassNames} {className} {theme}"
  >
    {isLoading ? 'Loading...' : label}
  </button>
{/if}

<style lang="postcss">
  .danger {
    @apply bg-red-400 hover:bg-red-700 border-red-900 text-white;
  }

  .primary {
    @apply bg-blue-400 hover:bg-blue-900 border-white text-white;
  }
</style>
