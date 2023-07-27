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
    } from "../../stores";
    import DebugLabel from "../debug/DebugLabel.svelte";
    import TagBadge from "./TagBadge.svelte";
    import {
        modalStore,
        type ModalSettings,
        drawerStore,
        type DrawerSettings,
    } from "@skeletonlabs/skeleton";

    export let id: number;
    export let columnId: number;
    export let activity: Actv;

    async function removeActivity() {
        await invoke("delete_activity", { id });

        const column = $columns.get(columnId);
        const index = column.activities.findIndex((aId) => aId === id);
        column.activities.splice(index, 1);
        $columns.set(columnId, column);
        $activities.delete(id);
        $activities = $activities;
        $columns = $columns;
    }

    function handleNameClick() {
        $currentEditable = { id, field: ActiveField.ActivityName };
    }

    function handleBodyClick() {
        $currentEditable = { id, field: ActiveField.ActivityBody };
    }

    function handleEnterKey(e: KeyboardEvent) {
        if (e.key === "Enter") {
            $currentEditable = null;
        }
    }

    function createBody() {
        const activity = $activities.get(id);
        activity.body = "new body";
        $activities.set(id, activity);
        $activities = $activities;
    }

    async function updateActivity() {
        console.debug(
            "Called updateActivity function and this wasn't expected, call 911!"
        );
        invoke("update_activity_content", {
            data: { id, name: activity.name, body: activity.body },
        });
        $activities.set(id, activity);
        $activities = $activities;
    }

    //WARNING! update on every keystroke, should use `updateActivity` in the future

    $: {
        invoke("update_activity_content", {
            data: { id, name: activity.name, body: activity.body },
        });
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
        $selectedActivity = { ...activity, id, columnId };
        const drawer: DrawerSettings = {
            id: DrawerTab.Activity,
            width: "w-1/2",
        };
        drawerStore.open(drawer);
    }
</script>

<div
    class="relative flex flex-col items-start p-4 mt-3 bg-white rounded-lg cursor-pointer bg-opacity-90 group hover:bg-opacity-100"
    draggable="true"
>
    <DebugLabel text={"ord: " + activity.ordinal} />
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

    <!-- svelte-ignore a11y-no-static-element-interactions -->
    {#if $currentEditable !== null && $currentEditable.id === id && $currentEditable.field === ActiveField.ActivityName}
        <span
            contenteditable="true"
            on:change={updateActivity}
            class="flex items-center h-6 px-3 text-lg font-semibold rounded-full outline-none variant-ghost-tertiary"
            bind:innerText={activity.name}
        />
    {:else}
        <!-- svelte-ignore a11y-click-events-have-key-events -->
        <span
            contenteditable="false"
            on:click={handleNameClick}
            class="flex items-center h-6 px-3 text-lg font-semibold rounded-full outline-none variant-soft-tertiary"
        >
            {activity.name}</span
        >
    {/if}

    <DebugLabel text={`ID ${id}`} />

    <div class="mb-3">
        {#if activity.body}
            {#if $currentEditable !== null && $currentEditable.id === id && $currentEditable.field === ActiveField.ActivityBody}
                <!-- svelte-ignore a11y-missing-content -->
                <h4
                    contenteditable="true"
                    on:change={updateActivity}
                    class="mt-3 text-sm font-medium outline-none"
                    bind:innerText={activity.body}
                />
            {:else}
                <!-- svelte-ignore a11y-click-events-have-key-events -->
                <h4
                    contenteditable="false"
                    on:click={handleBodyClick}
                    class="mt-3 text-sm font-medium"
                >
                    {activity.body ?? ""}
                </h4>
            {/if}
        {:else}
            <button
                on:click={createBody}
                class="bg-warning-hover-token rounded-md mt-3"
                ><svg
                    xmlns="http://www.w3.org/2000/svg"
                    height="1em"
                    viewBox="0 0 512 512"
                    ><path
                        d="M362.7 19.3L314.3 67.7 444.3 197.7l48.4-48.4c25-25 25-65.5 0-90.5L453.3 19.3c-25-25-65.5-25-90.5 0zm-71 71L58.6 323.5c-10.4 10.4-18 23.3-22.2 37.4L1 481.2C-1.5 489.7 .8 498.8 7 505s15.3 8.5 23.7 6.1l120.3-35.4c14.1-4.2 27-11.8 37.4-22.2L421.7 220.3 291.7 90.3z"
                    /></svg
                ></button
            >
        {/if}
    </div>
    <div class="flex flex-row">
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
