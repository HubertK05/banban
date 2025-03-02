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

<button class="flex items-center justify-center w-10 h-10 rounded-lg hover:bg-secondary-hover-token" onclick={openSettingsDrawer}>
    <Fa icon={faCog} />
</button>
