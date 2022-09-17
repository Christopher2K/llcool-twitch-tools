<script lang="ts">
  import SideMenuItem from '@app/components/SideMenuItem.svelte'
  import DashboardLayout from '@app/components/DashboardLayout.svelte'
</script>

<DashboardLayout title="Twitch Tools">
  <svelte:fragment slot="links">
    <SideMenuItem href="/user" label="Dashboard" />
    <SideMenuItem href="/user/commands" label="Commands" />
  </svelte:fragment>

  <svelte:fragment slot="content">
    <slot />
  </svelte:fragment>
</DashboardLayout>
