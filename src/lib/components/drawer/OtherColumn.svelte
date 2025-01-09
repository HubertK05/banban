<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import {
        ActiveField,
        type Activity,
        type Column,
    } from "../../interfaces/main";
    import ActivityCard from "../board/ActivityCard.svelte";
    import {
        activities,
        columns,
        currentEditable,
        otherActivities,
    } from "../../stores";
    import { dndzone } from "svelte-dnd-action";
    import DebugLabel from "../debug/DebugLabel.svelte";
    import { flip } from "svelte/animate";
    import { drawerStore } from "@skeletonlabs/skeleton";

    const flipDurationMs = 100;

    $: draggableActivities = Array.from($otherActivities)
        .map(([id, activity]) => {
            return { activity, id, colId: null };
        })
        .sort((a, b) => {
            return a.activity.ordinal - b.activity.ordinal;
        });

    $: {
        if ($otherActivities.size === 0) {
            drawerStore.close();
        }
    }

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
            activity.ordinal = index;
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
            activity.ordinal = index;
            activities.set(id, activity);
        });
        $otherActivities = activities;

        const activityId = Number(e.detail.info.id);
        const index = e.detail.items.findIndex(({ id }) => id === activityId);
        if (index !== -1) {
            await invoke("update_activity_column", {
                data: { id: activityId, columnId: null, newOrd: index },
            });
        }
    }
</script>

<div class="flex flex-col flex-shrink-0 w-72">
    <div class="flex items-center flex-shrink-0 h-10 px-2">
        <span
            class="flex items-center justify-center w-5 h-5 ml-2 text-sm font-semibold text-indigo-500 bg-white rounded bg-opacity-30"
            >{draggableActivities.length}</span
        >
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
                    <ActivityCard
                        activity={{
                            name: activity.name,
                            ordinal: activity.ordinal,
                            tags: activity.tags,
                            body: activity.body,
                            columnId: null,
                        }}
                        {id}
                    />
                </div>
            {/each}
        </section>
    </div>
</div>
