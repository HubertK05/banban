<script lang="ts">
    import {
        Drawer,
        drawerStore,
        type AutocompleteOption,
    } from "@skeletonlabs/skeleton";
    import TagBadge from "./TagBadge.svelte";
    import { DrawerTab, type Tag } from "../../interfaces/main";
    import ActivityDrawer from "../drawer/ActivityDrawer.svelte";
    import SettingsDrawer from "../drawer/SettingsDrawer.svelte";
    import BaseDrawer from "./BaseDrawer.svelte";
    import { selectedActivity } from "../../stores";
</script>

<Drawer position="right">
    {#if $drawerStore.id === DrawerTab.Activity}
        <BaseDrawer name={$selectedActivity.name}><ActivityDrawer /></BaseDrawer
        >
    {:else if $drawerStore.id === DrawerTab.Settings}
        <BaseDrawer name="Settings"><SettingsDrawer /></BaseDrawer>
    {/if}
</Drawer>
