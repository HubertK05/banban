<script lang="ts">
    import BoardColumn from "./BoardColumn.svelte";
    import {
        columns,
        currentEditable,
        isDebug,
        previousDrawerTab,
        selectedActivity,
        type Col,
        activities,
    } from "../../stores";
    import { invoke } from "@tauri-apps/api/tauri";
    import { dndzone, setDebugMode } from "svelte-dnd-action";
    import { DrawerTab, type Column } from "../../interfaces/main";
    import { flip } from "svelte/animate";
    import DebugButton from "../debug/DebugButton.svelte";
    import OtherActivitiesButton from "./OtherActivitiesButton.svelte";
    import { drawerStore, type DrawerSettings } from "@skeletonlabs/skeleton";
    setDebugMode(false);
    const boardName = "Kanban";
    const flipDurationMs = 300;
    async function createColumn({
        currentTarget,
    }: MouseEvent & {
        currentTarget: EventTarget & HTMLButtonElement;
    }) {
        const name = "New column";
        const res: { id: number; name: string; ordinal: number } = await invoke(
            "create_column",
            { name }
        );
        $columns.set(res.id, { name, activities: [], ordinal: $columns.size });
        $columns = $columns;
        setTimeout(() => {
            currentTarget.scrollIntoView({
                behavior: "smooth",
                block: "nearest",
                inline: "center",
            });
        }, 100);
    }

    $: draggableColumns = Array.from($columns.entries())
        .map(([id, col]) => {
            return {
                id,
                col,
            };
        })
        .sort((a, b) => {
            return a.col.ordinal - b.col.ordinal;
        });

    function handleConsider(
        e: CustomEvent<
            DndEvent<{
                id: number;
                col: Col;
            }>
        > & {
            target: any;
        }
    ) {
        e.detail.items.forEach(({ id, col }, index) => {
            col.ordinal = index;
        });
        draggableColumns = e.detail.items;
    }
    async function handleFinalize(
        e: CustomEvent<
            DndEvent<{
                id: number;
                col: Col;
            }>
        > & {
            target: any;
        }
    ) {
        e.detail.items.forEach(({ id, col }, index) => {
            const c = $columns.get(id);
            c.ordinal = index;
            $columns.set(id, c);
        });
        $columns = $columns;

        const draggedColumnId = Number(e.detail.info.id);
        const index = e.detail.items.findIndex(
            ({ id }) => id === draggedColumnId
        );
        await invoke("update_column_ordinal", {
            data: {
                columnId: draggedColumnId,
                newOrd: index,
            },
        });
    }

    function showDrawer() {
        $previousDrawerTab = null;
        let drawer: DrawerSettings = {
            id: DrawerTab.OtherActivities,
            width: "w-min",
            bgBackdrop: "none",
            // bgDrawer: 'none'
        };
        drawerStore.open(drawer);
    }
</script>

<!-- {@debug $activities}
{@debug $columns} -->
<div
    class="flex flex-col w-screen h-screen overflow-auto text-gray-700 bg-gradient-to-tr from-blue-200 via-indigo-200 to-pink-200"
>
    <div class="px-10 mt-6">
        <h1 class="text-2xl font-bold">{boardName}</h1>
        <button on:click={showDrawer}>
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
    </div>
    <div class="flex">
        <section
            class="flex flex-row px-10 mt-4 space-x-6 w-max"
            use:dndzone={{
                items: draggableColumns,
                flipDurationMs,
                type: "columns",
                dropTargetStyle: {},
            }}
            on:consider={handleConsider}
            on:finalize={handleFinalize}
        >
            {#each Array.from(draggableColumns).sort((a, b) => {
                return a.col.ordinal - b.col.ordinal;
            }) as { id, col } (id)}
                <div animate:flip={{ duration: flipDurationMs }}>
                    <BoardColumn column={col} columnId={id} />
                </div>
            {/each}
        </section>
        <div class="flex flex-row space-x-6">
            <button
                on:click={createColumn}
                class="btn variant-ghost-tertiary h-96">+</button
            >
            <DebugButton />
            <div class="flex-shrink-0 w-6" />
        </div>
    </div>
</div>
