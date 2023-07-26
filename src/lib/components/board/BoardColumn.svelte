<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";
    import {
        ActiveField,
        type Activity,
        type Column,
    } from "../../interfaces/main";
    import ActivityCard from "./ActivityCard.svelte";
    import { columns, currentEditable } from "../../stores";
    import { dndzone } from "svelte-dnd-action";
    import DebugLabel from "../debug/DebugLabel.svelte";
    import { flip } from "svelte/animate";

    export let columnId: number;
    export let column: Column;
    const flipDurationMs = 125;

    $: {
        // WARNING! Updates every key stroke
        invoke("rename_column", {
            data: { id: columnId, newName: column.name },
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
            data: { name, body, columnId },
        });
        const column = $columns.get(columnId);
        Array.from(column.activities.entries()).forEach(([id, activity]) => {
            activity.ord += 1;
            column.activities.set(id, activity);
        });
        column.activities.set(res.id, { name, body, tags, ord: res.ordinal });
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

    $: draggableActivities = Array.from(column.activities)
        .map(([id, activity]) => {
            return { activity, id, colId: columnId };
        })
        .sort((a, b) => {
            return a.activity.ord - b.activity.ord;
        });

    function handleConsider(
        e: CustomEvent<
            DndEvent<{
                id: number;
                activity: Activity;
                colId: number;
            }>
        > & {
            target: any;
        }
    ) {
        const activityId = Number(e.detail.info.id);
        e.detail.items.forEach(({ id, activity }, index) => {
            activity.ord = index;
        });
        draggableActivities = e.detail.items;
    }

    async function handleFinalize(
        e: CustomEvent<
            DndEvent<{
                id: number;
                activity: Activity;
                colId: number;
            }>
        > & {
            target: any;
        }
    ) {
        e.detail.info.id;
        const activities = new Map();
        e.detail.items.forEach(({ id, activity, colId }, index) => {
            activity.ord = index;
            activities.set(id, activity);
        });
        column.activities = activities;

        const activityId = Number(e.detail.info.id);
        const index = e.detail.items.findIndex(({ id }) => id === activityId);
        if (index !== -1) {
            await invoke("update_activity_column", {
                data: { id: activityId, columnId, newOrd: index },
            });
        }
    }
</script>

<div class="flex flex-col flex-shrink-0 w-72">
    <DebugLabel text={`ID ${columnId}`} />
    <DebugLabel text={`ORD ${column.ord}`} />
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
            >{draggableActivities.length}</span
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
    <div class="h-96">
        <section
            class="flex flex-col pb-2 overflow-auto min-h-full"
            use:dndzone={{
                items: draggableActivities,
                flipDurationMs,
                type: "activities",
                dropTargetStyle: {
                    "box-shadow": "0px 0px 0px 4px rgba(164, 190, 224, 0.2)",
                    "border-radius": "0.25rem",
                },
            }}
            on:consider={handleConsider}
            on:finalize={handleFinalize}
        >
            {#each Array.from(draggableActivities) as { id, activity } (id)}
                <div animate:flip={{ duration: flipDurationMs }}>
                    <ActivityCard {activity} {id} {columnId} />
                </div>
            {/each}
        </section>
    </div>
</div>
