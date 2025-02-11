<script lang="ts">
    import {
        Drawer,
        drawerStore,
        type AutocompleteOption,
    } from "@skeletonlabs/skeleton";
    import TagBadge from "../board/TagBadge.svelte";
    import { DrawerTab, type Tag } from "../../interfaces/main";
    import ActivityDrawer from "./ActivityDrawer.svelte";
    import SettingsDrawer from "./settings/SettingsDrawer.svelte";
    import BaseDrawer from "./BaseDrawer.svelte";
    import OtherColumn from "./OtherColumn.svelte";
  import { appState } from "../../shared.svelte";
</script>

<Drawer position="right">
    {#if $drawerStore.id === DrawerTab.Activity}
        <BaseDrawer name={appState.selectedActivity.name}><ActivityDrawer /></BaseDrawer
        >
    {:else if $drawerStore.id === DrawerTab.Settings}
        <BaseDrawer name="Settings"><SettingsDrawer /></BaseDrawer>
    {:else if $drawerStore.id === DrawerTab.OtherActivities}
        <BaseDrawer name="Stash"><OtherColumn /></BaseDrawer>
    {/if}
</Drawer>
