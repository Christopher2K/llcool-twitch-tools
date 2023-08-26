<script lang="ts">
  export let label: string
  let className: string = "" 
  export { className as class }
</script>

<button
  type="button"
  on:click
  class="px-3 py-2 hover:bg-blue-500 hover:text-white self-stretch text-left first:rounded-t-md last:rounded-b-md {className}"
  >{label}</button
>
