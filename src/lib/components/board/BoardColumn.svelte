<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";
    import ActivityCard from "./ActivityCard.svelte";
    import {
        columns,
        currentEditable,
        type Col,
        activities,
        otherActivities,
        type Actv,
        hoverColumnId,
        columnDragDisabled,
    } from "../../stores";
    import { TRIGGERS, dndzone } from "svelte-dnd-action";
    import DebugLabel from "../debug/DebugLabel.svelte";
    import { flip } from "svelte/animate";
    import { ActiveField } from "../../interfaces/main";
    import { modalStore, type ModalSettings } from "@skeletonlabs/skeleton";

    export let columnId: number;
    export let column: Col;
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
        const columnActivities: Map<number, Actv> = column.activities.reduce(
            (acc, id) => {
                acc.set(id, $activities.get(id));
                return acc;
            },
            new Map()
        );
        Array.from(columnActivities.entries()).forEach(([id, activity]) => {
            activity.ordinal += 1;
            $activities.set(id, activity);
        });
        column.activities.push(res.id);
        $activities.set(res.id, {
            name,
            body,
            tags,
            ordinal: res.ordinal,
            columnId: columnId,
        });
        $columns.set(columnId, column);
        $activities = $activities;
        $columns = $columns;
    }

    function handleNameClick() {
        $currentEditable = { id: columnId, field: ActiveField.ColumnName };
    }

    async function removeColumn() {
        await invoke("delete_column", { id: columnId });
        const newColumns = Array.from($columns.entries());
        const index = newColumns.findIndex(
            ([colId, column]) => colId === columnId
        );
        newColumns.forEach(([colId, column], idx) => {
            if (idx >= index) {
                column.ordinal -= 1;
                $columns.set(colId, column);
            }
        });
        let columnActivities: Array<[number, Actv]> = Array.from(
            column.activities
        ).map((activityId) => [activityId, $activities.get(activityId)]);
        column.activities.forEach((activityId) => {
            $activities.delete(activityId);
        });
        column.activities = [];
        let sortedColumnActivities = columnActivities.sort(
            ([activityId1, activity1], [activityId2, activity2]) => {
                return activity1.ordinal - activity2.ordinal;
            }
        );
        sortedColumnActivities.forEach(([activityId, activity]) => {
            // let activity = $activities.get(activityId);
            activity.ordinal = $otherActivities.size;
            $otherActivities.set(activityId, activity);
        });

        $columns.delete(columnId);
        $activities = $activities;

        $columns = $columns;
        $otherActivities = $otherActivities;
    }

    $: draggableActivities = Array.from(column.activities)
        .map((id) => {
            const activity = $activities.get(id);
            return { activity, id, colId: columnId };
        })
        .sort((a, b) => {
            return a.activity.ordinal - b.activity.ordinal;
        });

    function handleConsider(
        e: CustomEvent<
            DndEvent<{
                id: number;
                activity: Actv;
                colId: number;
            }>
        > & {
            target: any;
        }
    ) {
        const trigger = e.detail.info.trigger;
        console.log(columnId, "trigger", trigger);
        if (trigger === TRIGGERS.DRAGGED_ENTERED) {
            $hoverColumnId = columnId;
        } else if (trigger === TRIGGERS.DRAGGED_LEFT) {
            //$hoverColumnId = null;
        }
        const activityId = Number(e.detail.info.id);
        e.detail.items.forEach(({ id, activity }, index) => {
            activity.ordinal = index;
        });
        draggableActivities = e.detail.items;
    }

    async function handleFinalize(
        e: CustomEvent<
            DndEvent<{
                id: number;
                activity: Actv;
                colId: number;
            }>
        > & {
            target: any;
        }
    ) {
        $hoverColumnId = null;
        const activitiesIds = [];
        e.detail.items.forEach(({ id, activity, colId }, index) => {
            activity.ordinal = index;
            $activities.set(id, activity);
            activitiesIds.push(id);
        });
        $columns.set(columnId, {
            ...$columns.get(columnId),
            activities: activitiesIds,
        });
        $activities = $activities;
        $columns = $columns;
        const activityId = Number(e.detail.info.id);
        const index = e.detail.items.findIndex(({ id }) => id === activityId);
        if (index !== -1) {
            await invoke("update_activity_column", {
                data: { id: activityId, columnId, newOrd: index },
            });
        }
    }

    function showRemoveModal() {
        const body = `Are you sure? ${
            column.activities.length > 0
                ? `Activities (${column.activities.length}) will be moved to stash.`
                : "Are you sure?"
        })`;
        const modal: ModalSettings = {
            type: "confirm",
            title: `Remove '${column.name}'`,
            body,

            response: (r: boolean) => {
                if (r) {
                    removeColumn();
                }
            },
        };
        modalStore.trigger(modal);
    }

    function startDrag() {
        $columnDragDisabled = false;
    }
</script>

<!-- svelte-ignore a11y-no-static-element-interactions -->
<div
    class="flex flex-col flex-shrink-0 w-72 {$columnDragDisabled
        ? 'cursor-grab'
        : ''}"
    on:mousedown={startDrag}
    on:touchstart={startDrag}
>
    <DebugLabel text={`ID ${columnId}`} />
    <DebugLabel text={`ORD ${column.ordinal}`} />
    <div class="flex items-center flex-shrink-0 h-10 px-2">
        {#if $currentEditable !== null && $currentEditable.id === columnId && $currentEditable.field === ActiveField.ColumnName}
            <span
                contenteditable="true"
                class="block text-sm font-semibold cursor-default"
                bind:innerText={column.name}
            />
        {:else}
            <!-- svelte-ignore a11y-click-events-have-key-events -->
            <!-- svelte-ignore a11y-no-static-element-interactions -->
            <span
                contenteditable="false"
                on:click={handleNameClick}
                class="block text-sm font-semibold cursor-default"
                >{column.name}</span
            >
        {/if}

        <span
            class="flex items-center justify-center w-5 h-5 ml-2 text-sm font-semibold text-indigo-500 bg-white rounded bg-opacity-30 cursor-default"
            >{draggableActivities.length}</span
        >
        <button
            on:click={showRemoveModal}
            class="flex items-center justify-center w-6 h-6 ml-auto rounded hover:bg-error-hover-token"
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
    <div class="h-[70vh]">
        <!-- svelte-ignore missing-declaration -->
        <section
            class="flex flex-col pb-2 overflow-auto max-h-full min-h-full {$hoverColumnId ===
            columnId
                ? 'shadow-2xl rounded-md'
                : ''}"
            use:dndzone={{
                items: draggableActivities,
                flipDurationMs,
                type: "activities",
                dropTargetStyle: {},
            }}
            on:consider={handleConsider}
            on:finalize={handleFinalize}
        >
            {#each draggableActivities as { id, activity } (id)}
                <div animate:flip={{ duration: flipDurationMs }}>
                    <ActivityCard {activity} {id} />
                </div>
            {/each}
        </section>
    </div>
</div>
