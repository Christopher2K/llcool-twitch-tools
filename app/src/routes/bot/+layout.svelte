<script lang="ts">
  import DashboardLayout from '@app/components/DashboardLayout.svelte'

  const menuItems = [
    { label: 'Status', href: '/bot' },
    { label: 'Global behaviour', href: '/bot/behaviour' },
  ]
</script>

<DashboardLayout {menuItems}>
  <svelte:fragment slot="content">
    <slot />
  </svelte:fragment>
</DashboardLayout>
