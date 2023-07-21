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

    export let columnId: number;
    export let column: Column;

    $: {
        // WARNING! Updates every key stroke
        invoke("rename_column", {
            data: { id: columnId, new_name: column.name },
        });
    }
    async function createActivity() {
        const name = "New activity";
        const body = "";
        const tags = [];
        const res: {
            id: number;
            name: string;
            body?: string;
            column_id?: number;
            ordinal: number;
        } = await invoke("create_activity", {
            data: { name, body, column_id: columnId },
        });
        const column = $columns.get(columnId);
        column.activities.set(res.id, { name, body, tags });
        $columns.set(columnId, column);
        $columns = $columns;
    }

    function handleNameClick() {
        $currentEditable = { id: columnId, field: ActiveField.ColumnName };
    }

    async function removeColumn() {
        await invoke("delete_column", { id: columnId });
        $columns.delete(columnId);
        $columns = $columns;
    }
</script>

<div class="flex flex-col flex-shrink-0 w-72" transition:fly>
    <button class="btn btn-sm variant-ghost-error" on:click={removeColumn}
        >Remove column</button
    >
    <div class="flex items-center flex-shrink-0 h-10 px-2">
        {#if $currentEditable !== null && $currentEditable.id === columnId && $currentEditable.field === ActiveField.ColumnName}
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
            <ActivityCard {activity} id={activityId} {columnId} />
        {/each}
    </div>
</div>
