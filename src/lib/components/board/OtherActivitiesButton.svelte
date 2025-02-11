<script lang="ts">
    import { DrawerTab } from "../../interfaces";
    import { appState, otherActivitiesRune } from "../../shared.svelte";
    import { getDrawerStore, type DrawerSettings } from "@skeletonlabs/skeleton";

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
    ><span>Stash</span><svg xmlns="http://www.w3.org/2000/svg" height="1em" viewBox="0 0 512 512"
        ><!--! Font Awesome Free 6.4.0 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license (Commercial License) Copyright 2023 Fonticons, Inc. --><path
            d="M32 32H480c17.7 0 32 14.3 32 32V96c0 17.7-14.3 32-32 32H32C14.3 128 0 113.7 0 96V64C0 46.3 14.3 32 32 32zm0 128H480V416c0 35.3-28.7 64-64 64H96c-35.3 0-64-28.7-64-64V160zm128 80c0 8.8 7.2 16 16 16H336c8.8 0 16-7.2 16-16s-7.2-16-16-16H176c-8.8 0-16 7.2-16 16z"
        /></svg
    ></button
>
