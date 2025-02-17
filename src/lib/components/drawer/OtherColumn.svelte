<script lang="ts">
    import { run } from "svelte/legacy";

    import { invoke } from "@tauri-apps/api/core";
    import { ActiveField, type Activity } from "../../interfaces";
    import ActivityCard from "../board/ActivityCard.svelte";
    import { dndzone } from "svelte-dnd-action";
    import DebugLabel from "../debug/DebugLabel.svelte";
    import { flip } from "svelte/animate";
    import { getDrawerStore } from "@skeletonlabs/skeleton";
    import { draggableOtherActivities, idOtherTags, otherActivitiesRune } from "../../shared.svelte";

    draggableOtherActivities.update();

    const flipDurationMs = 100;

    const drawerStore = getDrawerStore();
    $effect.pre(() => {
        if (Object.entries(otherActivitiesRune.inner).length === 0) {
            drawerStore.close();
        }
    });

    function handleConsider(
        e: CustomEvent<
            DndEvent<{
                id: number;
                activity: Activity;
            }>
        > & {
            target: any;
        },
    ) {
        e.detail.items.forEach(({ id, activity }, index) => {
            activity.ordinal = index;
        });
        draggableOtherActivities.inner = e.detail.items;
    }

    async function handleFinalize(
        e: CustomEvent<
            DndEvent<{
                id: number;
                activity: Activity;
            }>
        > & {
            target: any;
        },
    ) {
        e.detail.items.forEach(({ id, activity }, index) => {
            activity.ordinal = index;
        });

        const activityId = Number(e.detail.info.id);
        const index = e.detail.items.findIndex(({ id }) => id === activityId);
        if (index !== -1) {
            await invoke("update_activity_column", {
                data: { id: activityId, columnId: null, newOrd: index },
            });
        }
        const activityRecord: Record<number, Activity> = {};
        e.detail.items.map(({ id, activity }) => {
            activityRecord[id] = activity;
        });
        otherActivitiesRune.inner = activityRecord;
        draggableOtherActivities.inner = e.detail.items;
    }
</script>

<div class="flex flex-col flex-shrink-0 w-72">
    <div class="flex items-center flex-shrink-0 h-10 px-2">
        <span
            class="flex items-center justify-center w-5 h-5 ml-2 text-sm font-semibold text-indigo-500 bg-white rounded bg-opacity-30"
            >{draggableOtherActivities.inner.length}</span
        >
    </div>
    <div class="h-96">
        <section
            class="flex flex-col pb-2 overflow-auto min-h-full"
            use:dndzone={{
                items: draggableOtherActivities.inner,
                flipDurationMs,
                type: "activities",
                dropTargetStyle: {
                    "box-shadow": "0px 0px 0px 4px rgba(164, 190, 224, 0.2)",
                    "border-radius": "0.25rem",
                },
            }}
            onconsider={handleConsider}
            onfinalize={handleFinalize}
        >
            {#each draggableOtherActivities.inner as { id, activity } (id)}
                <div animate:flip={{ duration: flipDurationMs }}>
                    <ActivityCard
                        activity={{
                            name: activity.name,
                            ordinal: activity.ordinal,
                            tags: activity.tags,
                            body: activity.body,
                        }}
                        {id}
                    />
                </div>
            {/each}
        </section>
    </div>
</div>
