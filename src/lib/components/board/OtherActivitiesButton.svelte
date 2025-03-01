<script lang="ts">
    import Fa from "svelte-fa";
    import { DrawerTab } from "../../interfaces";
    import { appState, otherActivitiesRune } from "../../shared.svelte";
    import { getDrawerStore, type DrawerSettings } from "@skeletonlabs/skeleton";
    import { faBoxArchive } from "@fortawesome/free-solid-svg-icons";

    let isAvailable = $derived(Object.entries(otherActivitiesRune.inner).length !== 0);
    const drawerStore = getDrawerStore();

    function openDrawer() {
        if (!isAvailable) return;
        appState.previousDrawerTab = null;
        $drawerStore.id = DrawerTab.OtherActivities;
        let drawer: DrawerSettings = {
            id: DrawerTab.OtherActivities,
            width: "w-min",
            bgBackdrop: "none",
        };
        drawerStore.open(drawer);
    }
</script>

<button class="btn variant-ghost-tertiary" onclick={openDrawer} disabled={!isAvailable}
    ><span>Stash</span><Fa icon={faBoxArchive} /></button
>
