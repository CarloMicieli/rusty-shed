<script lang="ts">
  import { setContext } from 'svelte';
  import DashboardPage from '../../routes/dashboard/+page.svelte';

  interface Props {
    openAcquisitionDrawer?: () => void;
    openWishlistDrawer?: () => void;
    openLogMaintenanceDrawer?: () => void;
  }

  let {
    openAcquisitionDrawer = () => {},
    openWishlistDrawer = () => {},
    openLogMaintenanceDrawer = () => {}
  }: Props = $props();

  setContext('openAcquisitionDrawer', () => openAcquisitionDrawer());
  setContext('openWishlistDrawer', () => openWishlistDrawer());
  setContext('openLogMaintenanceDrawer', () => openLogMaintenanceDrawer());
</script>

<DashboardPage />
