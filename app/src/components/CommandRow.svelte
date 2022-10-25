<script lang="ts">
  export let position: 'first' | 'last' | undefined = undefined
  export let label: string
  export let hideBottomBorder: boolean = false
</script>

<td
  class="grid grid-cols-[8rem_auto] border border-black rounded-t-md"
  class:rounded-t-md={position === 'first'}
  class:rounded-b-md={position === 'last'}
  class:border-b-transparent={hideBottomBorder}
>
  <span class="font-bold underline border-r p-2 border-black">{label}</span>
  <span class="p-2 gap-2 inline-flex flex-col md:flex-row break-all"><slot /></span>
</td>
