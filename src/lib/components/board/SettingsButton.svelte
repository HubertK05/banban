<script lang="ts">
    import Fa from "svelte-fa";
    import { DrawerTab } from "../../interfaces";
    import { appState } from "../../shared.svelte";
    import { getDrawerStore } from "@skeletonlabs/skeleton";
    import { faCog } from "@fortawesome/free-solid-svg-icons";

    const drawerStore = getDrawerStore();

    function openSettingsDrawer() {
        appState.previousDrawerTab = $drawerStore.id as DrawerTab;
        $drawerStore.id = DrawerTab.Settings;
    }
</script>

<button class="btn hover:bg-secondary-hover-token" onclick={openSettingsDrawer}>
    <Fa icon={faCog} />
</button>
