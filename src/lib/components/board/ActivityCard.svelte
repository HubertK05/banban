<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";
    import { ActiveField, DrawerTab } from "../../interfaces/main";
    import {
        activities,
        columns,
        currentEditable,
        isDebug,
        otherTags,
        previousDrawerTab,
        selectedActivity,
        tags,
        type Actv,
        type Col,
        otherActivities,
    } from "../../stores";
    import DebugLabel from "../debug/DebugLabel.svelte";
    import TagBadge from "./TagBadge.svelte";
    import {
        modalStore,
        type ModalSettings,
        drawerStore,
        type DrawerSettings,
    } from "@skeletonlabs/skeleton";
    import SvelteMarkdown from "svelte-markdown";

    export let id: number;
    export let activity: Actv;

    async function removeActivity() {
        await invoke("delete_activity", { id });

        if (activity.columnId) {
            const column = $columns.get(activity.columnId);
            const index = column.activities.findIndex((aId) => aId === id);
            column.activities.splice(index, 1);
            $columns.set(activity.columnId, column);
            $activities.delete(id);
            $activities = $activities;
            $columns = $columns;
        } else {
            $otherActivities.delete(id);
            $activities.delete(id);
            $activities = $activities;
            $otherActivities = $otherActivities;
        }
    }

    function handleEnterKey(e: KeyboardEvent) {
        if (e.key === "Enter") {
            $currentEditable = null;
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
        $previousDrawerTab = null;
        $selectedActivity = { ...activity, id, columnId: activity.columnId };
        const drawer: DrawerSettings = {
            id: DrawerTab.Activity,
            width: "w-2/3",
        };
        drawerStore.open(drawer);
    }

    $: bodyPreview = () => {
        const elements = activity.body.split("\n");
        const out = elements.slice(0, 5).join("\n");
        if (elements.length > 5) {
            return out.concat("\n\n...");
        }
        return out;
    };
</script>

<div
    class="relative flex flex-col items-start p-4 mt-3 bg-white rounded-lg bg-opacity-90 group hover:bg-opacity-100"
    draggable="true"
>
    <DebugLabel text={"ord: " + activity.ordinal} />
    {#if activity.columnId}
        <button
            on:click={showDrawer}
            class="absolute top-0 right-5 items-center justify-center hidden w-5 h-5 mt-3 mr-2 text-gray-500 rounded hover:bg-gray-200 hover:text-gray-700 group-hover:flex"
        >
            <svg
                class="w-4 h-4 fill-current"
                xmlns="http://www.w3.org/2000/svg"
                viewBox="0 0 20 20"
                fill="currentColor"
            >
                <path
                    d="M10 6a2 2 0 110-4 2 2 0 010 4zM10 12a2 2 0 110-4 2 2 0 010 4zM10 18a2 2 0 110-4 2 2 0 010 4z"
                />
            </svg>
        </button>
    {/if}
    <button
        on:click={showRemoveModal}
        class="absolute top-0 right-0 items-center justify-center hidden w-5 h-5 mt-3 mr-2 text-gray-500 rounded hover:bg-gray-200 hover:text-gray-700 group-hover:flex"
    >
        <svg
            xmlns="http://www.w3.org/2000/svg"
            height="1em"
            viewBox="0 0 448 512"
        >
            <path
                d="M135.2 17.7C140.6 6.8 151.7 0 163.8 0H284.2c12.1 0 23.2 6.8 28.6 17.7L320 32h96c17.7 0 32 14.3 32 32s-14.3 32-32 32H32C14.3 96 0 81.7 0 64S14.3 32 32 32h96l7.2-14.3zM32 128H416V448c0 35.3-28.7 64-64 64H96c-35.3 0-64-28.7-64-64V128zm96 64c-8.8 0-16 7.2-16 16V432c0 8.8 7.2 16 16 16s16-7.2 16-16V208c0-8.8-7.2-16-16-16zm96 0c-8.8 0-16 7.2-16 16V432c0 8.8 7.2 16 16 16s16-7.2 16-16V208c0-8.8-7.2-16-16-16zm96 0c-8.8 0-16 7.2-16 16V432c0 8.8 7.2 16 16 16s16-7.2 16-16V208c0-8.8-7.2-16-16-16z"
            /></svg
        >
    </button>

    <button
        on:click={showDrawer}
        class="flex items-center h-6 px-3 text-lg font-semibold rounded-full outline-none variant-soft-tertiary hover:underline"
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
            {@const tag = $tags.get(tagId)}
            {#if tag}
                <TagBadge name={tag.name} color={tag.color} />
            {:else}
                {@const nonCategoryTag = $otherTags.get(tagId)}
                <TagBadge
                    name={nonCategoryTag.name}
                    color={nonCategoryTag.color}
                />
            {/if}
        {/each}
    </div>
    <div
        class="flex items-center w-full mt-3 text-xs font-medium text-gray-400"
    />
</div>
<svelte:document on:keydown={handleEnterKey} />
