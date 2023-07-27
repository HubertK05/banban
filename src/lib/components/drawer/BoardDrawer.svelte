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
    import { selectedActivity } from "../../stores";
  import OtherColumn from "./OtherColumn.svelte";
</script>

<Drawer position="right">
    {#if $drawerStore.id === DrawerTab.Activity}
        <BaseDrawer name={$selectedActivity.name}><ActivityDrawer /></BaseDrawer>
    {:else if $drawerStore.id === DrawerTab.Settings}
        <BaseDrawer name="Settings"><SettingsDrawer /></BaseDrawer>
    {:else if $drawerStore.id === DrawerTab.OtherActivities}
        <BaseDrawer name="Other activities"><OtherColumn /></BaseDrawer>
    {/if}
</Drawer>
