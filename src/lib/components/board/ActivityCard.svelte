<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { DrawerTab, type Activity } from "../../interfaces";
    import DebugLabel from "../debug/DebugLabel.svelte";
    import TagBadge from "./TagBadge.svelte";
    import { type ModalSettings, type DrawerSettings, getDrawerStore, getModalStore } from "@skeletonlabs/skeleton";
    import SvelteMarkdown from "@humanspeak/svelte-markdown";
    import {
        activitiesRune,
        appState,
        categoryTagsRune,
        columnsRune,
        draggableActivities,
        draggableColumns,
        draggableOtherActivities,
        otherActivitiesRune,
        otherTagsRune,
    } from "../../shared.svelte";
    import Fa from "svelte-fa";
    import { faEllipsisV, faTrashAlt } from "@fortawesome/free-solid-svg-icons";

    interface Props {
        id: number;
        activity: Activity;
        columnId?: number;
    }

    let { id, activity, columnId }: Props = $props();

    const drawerStore = getDrawerStore();
    const modalStore = getModalStore();

    async function removeActivity() {
        await invoke("delete_activity", { id });

        if (columnId) {
            const runeColumn = columnsRune[columnId];
            runeColumn.activities = runeColumn.activities.filter((aId) => aId !== id);
            columnsRune[columnId] = runeColumn;
            delete activitiesRune[id];
            draggableColumns.update();
            draggableActivities.update(columnId);
        } else {
            delete otherActivitiesRune.inner[id];
            delete activitiesRune[id];
            draggableOtherActivities.update();
        }
    }

    function showRemoveModal() {
        const modal: ModalSettings = {
            type: "confirm",
            title: `Remove '${activity.name}'`,
            body: "Are you sure?",

            response: (r: boolean) => {
                if (r) {
                    removeActivity();
                }
            },
        };
        modalStore.trigger(modal);
    }

    function showDrawer() {
        if (columnId === undefined) return;
        appState.previousDrawerTab = null;
        appState.selectedActivity = id;
        const drawer: DrawerSettings = {
            id: DrawerTab.Activity,
            width: "w-2/3",
        };
        drawerStore.open(drawer);
    }

    let bodyPreview = $derived(() => {
        if (!activity.body) return "";
        const elements = activity.body.split("\n");
        const out = elements.slice(0, 5).join("\n");
        if (elements.length > 5) {
            return out.concat("\n\n...");
        }
        return out;
    });
</script>

<div
    class="relative flex flex-col items-start p-4 mt-3 bg-white rounded-lg bg-opacity-90 group hover:bg-opacity-100"
    draggable="true"
>
    <DebugLabel text={"ord: " + activity.ordinal} />
    {#if columnId}
        <button
            aria-label="Open activity settings"
            onclick={showDrawer}
            class="absolute top-0 right-5 items-center justify-center hidden w-5 h-5 mt-3 mr-2 text-gray-500 rounded hover:bg-gray-200 hover:text-gray-700 group-hover:flex"
        >
            <Fa icon={faEllipsisV} />
        </button>
    {/if}
    <button
        aria-label="Delete activity"
        onclick={showRemoveModal}
        class="absolute top-0 right-0 items-center justify-center hidden w-5 h-5 mt-3 mr-2 text-gray-500 rounded hover:bg-gray-200 hover:text-gray-700 group-hover:flex"
    >
        <Fa icon={faTrashAlt} />
    </button>

    <button
        onclick={showDrawer}
        class="flex items-center min-h-6 px-3 text-lg font-semibold rounded-lg outline-none variant-soft-tertiary hover:underline"
    >
        {activity.name}
    </button>

    <DebugLabel text={`ID ${id}`} />

    <div class="mb-3">
        {#if activity.body}
            <div class="prose prose-sm mt-3 text-sm">
                <SvelteMarkdown source={bodyPreview()} />
            </div>
        {/if}
    </div>
    <div class="flex flex-row flex-wrap">
        {#each activity.tags as tagId}
            <DebugLabel text={"ID: " + tagId} />
            {@const tag = categoryTagsRune[tagId]}
            {#if tag}
                <TagBadge name={tag.name} color={tag.color} />
            {:else}
                {@const nonCategoryTag = otherTagsRune[tagId]}
                <TagBadge name={nonCategoryTag.name} color={nonCategoryTag.color} />
            {/if}
        {/each}
    </div>
    <div class="flex items-center w-full mt-3 text-xs font-medium text-gray-400"></div>
</div>
