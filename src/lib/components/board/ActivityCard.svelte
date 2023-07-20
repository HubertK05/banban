<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";
    import type { Activity } from "../../interfaces/main";
    import { columns, currentEditableActivity } from "../../stores";
    import { fly } from "svelte/transition";

    export let id: number;
    export let columnId: number;
    export let activity: Activity;

    async function removeActivity() {
        //await invoke("remove_activity", { id });
        const column = $columns.get(columnId);
        column.activities.delete(id);
        $columns.set(columnId, column);
        $columns = $columns;
    }

    function handleNameDbClick() {
        $currentEditableActivity = id;
    }

    function handleEnterKey(e: KeyboardEvent) {
        if (e.key === "Enter") {
            $currentEditableActivity = null;
        }
    }

    function updateActivity() {
        console.log("AAA");
        const column = $columns.get(columnId);
        column.activities.set(id, {
            name: activity.name,
            body: activity.body,
        });
        $columns.set(columnId, column);
        $columns = $columns;
    }
    async function updateName() {
        //await invoke("update_activity", { name });
    }
</script>

<div
    transition:fly
    class="relative flex flex-col items-start p-4 mt-3 bg-white rounded-lg cursor-pointer bg-opacity-90 group hover:bg-opacity-100"
    draggable="true"
>
    <button
        on:click={removeActivity}
        class="absolute top-0 right-0 items-center justify-center hidden w-5 h-5 mt-3 mr-2 text-gray-500 rounded hover:bg-gray-200 hover:text-gray-700 group-hover:flex"
    >
        <svg
            xmlns="http://www.w3.org/2000/svg"
            height="1em"
            viewBox="0 0 448 512"
        >
            <style>
                svg {
                    fill: #db4343;
                }
            </style><path
                d="M135.2 17.7C140.6 6.8 151.7 0 163.8 0H284.2c12.1 0 23.2 6.8 28.6 17.7L320 32h96c17.7 0 32 14.3 32 32s-14.3 32-32 32H32C14.3 96 0 81.7 0 64S14.3 32 32 32h96l7.2-14.3zM32 128H416V448c0 35.3-28.7 64-64 64H96c-35.3 0-64-28.7-64-64V128zm96 64c-8.8 0-16 7.2-16 16V432c0 8.8 7.2 16 16 16s16-7.2 16-16V208c0-8.8-7.2-16-16-16zm96 0c-8.8 0-16 7.2-16 16V432c0 8.8 7.2 16 16 16s16-7.2 16-16V208c0-8.8-7.2-16-16-16zm96 0c-8.8 0-16 7.2-16 16V432c0 8.8 7.2 16 16 16s16-7.2 16-16V208c0-8.8-7.2-16-16-16z"
            /></svg
        >
    </button>

    <!-- svelte-ignore a11y-no-static-element-interactions -->
    {#if $currentEditableActivity === id}
        <span
            contenteditable="true"
            on:change={updateActivity}
            class="flex items-center h-6 px-3 text-lg font-semibold rounded-full outline-none variant-ghost-tertiary"
            bind:innerHTML={activity.name}
        />
    {:else}
        <!-- svelte-ignore a11y-click-events-have-key-events -->
        <span
            contenteditable="false"
            on:click={handleNameDbClick}
            class="flex items-center h-6 px-3 text-lg font-semibold rounded-full outline-none variant-soft-tertiary"
        >
            {activity.name}</span
        >
    {/if}

    <h4 class="mt-3 text-sm font-medium">
        {activity.body ?? ""}
    </h4>
    <div
        class="flex items-center w-full mt-3 text-xs font-medium text-gray-400"
    />
</div>
<svelte:document on:keydown={handleEnterKey} />
