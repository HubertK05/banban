<script lang="ts">
    import { Drawer, getDrawerStore, type AutocompleteOption } from "@skeletonlabs/skeleton";
    import TagBadge from "../board/TagBadge.svelte";
    import { DrawerTab, type Tag } from "../../interfaces";
    import ActivityDrawer from "./ActivityDrawer.svelte";
    import SettingsDrawer from "./settings/SettingsDrawer.svelte";
    import BaseDrawer from "./BaseDrawer.svelte";
    import OtherColumn from "./OtherColumn.svelte";
    import { activitiesRune, appState } from "../../shared.svelte";
    import { app } from "@tauri-apps/api";

    const drawerStore = getDrawerStore();
</script>

<Drawer position="right">
    {#if $drawerStore.id === DrawerTab.Activity}
        {#if appState.selectedActivity && activitiesRune[appState.selectedActivity]}
            <BaseDrawer name={activitiesRune[appState.selectedActivity].name}>
                <ActivityDrawer activityId={appState.selectedActivity} />
            </BaseDrawer>
        {:else}
            {console.log(appState.selectedActivity)}
            <BaseDrawer name={"Error"}>
                <p>No selected activity found</p>
            </BaseDrawer>
        {/if}
    {:else if $drawerStore.id === DrawerTab.Settings}
        <BaseDrawer name="Settings"><SettingsDrawer /></BaseDrawer>
    {:else if $drawerStore.id === DrawerTab.OtherActivities}
        <BaseDrawer name="Stash"><OtherColumn /></BaseDrawer>
    {/if}
</Drawer>
