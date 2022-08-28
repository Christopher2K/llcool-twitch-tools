<script lang="ts">
  import SideMenuItem from '@app/components/SideMenuItem.svelte'
  import DashboardLayout from '@app/components/DashboardLayout.svelte'
</script>

<DashboardLayout title="Bot dashboard">
  <svelte:fragment slot="links">
    <SideMenuItem href="/bot" label="Status" />
    <SideMenuItem href="/bot/behaviour" label="Global behaviour" />
  </svelte:fragment>

  <svelte:fragment slot="content">
    <slot />
  </svelte:fragment>
</DashboardLayout>
