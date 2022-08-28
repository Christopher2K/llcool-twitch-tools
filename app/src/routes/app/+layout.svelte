<script lang="ts">
  import SideMenuItem from '@app/components/SideMenuItem.svelte'
  import DashboardLayout from '@app/components/DashboardLayout.svelte'
</script>

<DashboardLayout title="Dashboard">
  <svelte:fragment slot="links">
    <SideMenuItem href="/app" label="Dashboard" />
    <SideMenuItem href="/app/commands" label="Commands" />
  </svelte:fragment>

  <svelte:fragment slot="content">
    <slot />
  </svelte:fragment>
</DashboardLayout>
