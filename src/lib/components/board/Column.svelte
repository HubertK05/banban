<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";
    import {
        ActiveField,
        type Activity,
        type Column,
    } from "../../interfaces/main";
    import ActivityCard from "./ActivityCard.svelte";
    import { columns, currentEditable } from "../../stores";
    import { fly } from "svelte/transition";

    export let id: number;
    export let column: Column;
    async function createActivity() {
        const name = "New activity";
        const body = "";
        const tags = [];
        // const activityId: number = await invoke("create_activity", {
        //     name,
        //     body,
        // });
        const activityId = new Date().getMilliseconds();
        const column = $columns.get(id);
        column.activities.set(activityId, { name, body, tags });
        $columns.set(id, column);
        $columns = $columns;
    }

    function handleNameClick() {
        $currentEditable = { id, field: ActiveField.ColumnName };
    }
</script>

<div class="flex flex-col flex-shrink-0 w-72" transition:fly>
    <div class="flex items-center flex-shrink-0 h-10 px-2">
        {#if $currentEditable !== null && $currentEditable.id === id && $currentEditable.field === ActiveField.ColumnName}
            <span
                contenteditable="true"
                class="block text-sm font-semibold"
                bind:innerText={column.name}
            />
        {:else}
            <!-- svelte-ignore a11y-click-events-have-key-events -->
            <!-- svelte-ignore a11y-no-static-element-interactions -->
            <span
                contenteditable="false"
                on:click={handleNameClick}
                class="block text-sm font-semibold">{column.name}</span
            >
        {/if}

        <span
            class="flex items-center justify-center w-5 h-5 ml-2 text-sm font-semibold text-indigo-500 bg-white rounded bg-opacity-30"
            >{column.activities.size}</span
        >
        <button
            on:click={createActivity}
            class="flex items-center justify-center w-6 h-6 ml-auto text-indigo-500 rounded hover:bg-indigo-500 hover:text-indigo-100"
        >
            <svg
                class="w-5 h-5"
                fill="none"
                viewBox="0 0 24 24"
                stroke="currentColor"
            >
                <path
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    stroke-width="2"
                    d="M12 6v6m0 0v6m0-6h6m-6 0H6"
                />
            </svg>
        </button>
    </div>
    <div class="flex flex-col pb-2 overflow-auto">
        {#each [...column.activities].reverse() as [activityId, activity] (activityId)}
            <ActivityCard {activity} id={activityId} columnId={id} />
        {/each}
    </div>
</div>
