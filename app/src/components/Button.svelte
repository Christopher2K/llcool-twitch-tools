<script lang="ts" context="module">
  export type ButtonTheme = 'primary' | 'danger'
</script>

<script lang="ts">
  export let label: string
  export let isLoading: boolean = false
  export let theme: ButtonTheme = 'primary'
  export let href: string | undefined = undefined
  export let type: HTMLButtonElement['type'] = 'button'
  export let tailwindClassNames = 'rounded-lg px-4 py-2 text-lg text-center'
  export let form: string | undefined = undefined

  let className: string = ''
  export { className as class }
</script>

{#if href}
  <a {href} class="{tailwindClassNames} {className} {theme}">{label}</a>
{:else}
  <button
    {type}
    {form}
    disabled={isLoading}
    on:click
    class="{tailwindClassNames} {className} {theme}"
  >
    {isLoading ? 'Loading...' : label}
  </button>
{/if}

<style lang="postcss">
  .danger {
    @apply bg-red-400 hover:bg-red-700 text-white;
  }

  .primary {
    @apply bg-blue-400 hover:bg-blue-900 text-white;
  }
</style>
