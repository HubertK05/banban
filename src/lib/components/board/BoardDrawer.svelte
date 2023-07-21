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
</script>

<Drawer position="right">
    {#if $drawerStore.id === DrawerTab.Activity}
        <ActivityDrawer />
    {:else if $drawerStore.id === DrawerTab.Settings}
        <SettingsDrawer />
    {/if}
</Drawer>
