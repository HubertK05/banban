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
    import { dndzone } from "svelte-dnd-action";
    import { RadioItem } from "@skeletonlabs/skeleton";

    export let columnId: number;
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
        const column = $columns.get(columnId);
        column.activities.set(activityId, { name, body, tags });
        $columns.set(columnId, column);
        $columns = $columns;
    }

    function handleNameClick() {
        $currentEditable = { id: columnId, field: ActiveField.ColumnName };
    }

    $: idActivities = Array.from(column.activities).map(([id, activity]) => {
        return { id, activity };
    });

    function handleConsider(
        e: CustomEvent<
            DndEvent<{
                id: number;
                activity: Activity;
            }>
        > & {
            target: any;
        }
    ) {
        console.info("consider");
        idActivities = e.detail.items;
        // idActivities = e.detail.items;
    }
    function handleFinalize(
        e: CustomEvent<
            DndEvent<{
                id: number;
                activity: Activity;
            }>
        > & {
            target: any;
        }
    ) {
        console.info("finalize");
        // idActivities = e.detail.items;
        idActivities = e.detail.items;
    }
</script>

<div class="flex flex-col flex-shrink-0 w-72" transition:fly>
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
    <div class="h-96">
        <section
            class="flex flex-col pb-2 overflow-auto min-h-full"
            use:dndzone={{
                items: idActivities,
                zoneTabIndex: -1,
            }}
            on:consider={handleConsider}
            on:finalize={handleFinalize}
        >
            {#each idActivities as { id, activity } (id)}
                <ActivityCard {activity} {id} {columnId} />
            {/each}
        </section>
    </div>
</div>
