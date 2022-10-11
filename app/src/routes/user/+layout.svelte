<script lang="ts">
  import DashboardLayout from '@app/components/DashboardLayout.svelte'

  const menuItems = [
    { label: 'Dashboard', href: '/user' },
    { label: 'Commands', href: '/user/commands' },
  ]
</script>

<DashboardLayout {menuItems}>
  <svelte:fragment slot="content">
    <slot />
  </svelte:fragment>
</DashboardLayout>
