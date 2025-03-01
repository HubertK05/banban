<script lang="ts">
    import BoardColumn from "./BoardColumn.svelte";
    import { invoke } from "@tauri-apps/api/core";
    import { dragHandle, dragHandleZone } from "svelte-dnd-action";
    import { type Column } from "../../interfaces";
    import { flip } from "svelte/animate";
    import DebugButton from "../debug/DebugButton.svelte";
    import OtherActivitiesButton from "./OtherActivitiesButton.svelte";
    import { columnsRune, draggableColumns } from "../../shared.svelte";
    import Fa from "svelte-fa";
    import { faPlus, faUpDownLeftRight } from "@fortawesome/free-solid-svg-icons";

    const boardName = "Kanban";
    const flipDurationMs = 300;

    async function createColumn({
        currentTarget,
    }: MouseEvent & {
        currentTarget: EventTarget & HTMLButtonElement;
    }) {
        const name = "New column";
        const res: { id: number; name: string; ordinal: number } = await invoke("create_column", { name });

        columnsRune[res.id] = { name, activities: [], ord: Object.entries(columnsRune).length };

        setTimeout(() => {
            currentTarget.scrollIntoView({
                behavior: "smooth",
                block: "nearest",
                inline: "center",
            });
        }, 100);

        draggableColumns.update();
    }

    function handleConsider(
        e: DndEvent<{
            id: number;
            columnId: number;
            column: Column;
        }>,
    ) {
        e.items.forEach(({ id, column }, index) => {
            column.ord = index;
        });
        draggableColumns.inner = e.items;
    }

    async function handleFinalize(
        e: DndEvent<{
            id: number;
            columnId: number;
            column: Column;
        }>,
    ) {
        e.items.forEach(({ id, column }, index) => {
            const c = columnsRune[id];
            c.ord = index;
            columnsRune[id] = c;
        });

        const draggedColumnId = +e.info.id;
        const index = e.items.findIndex(({ id }) => id === draggedColumnId);
        await invoke("update_column_ordinal", {
            data: {
                columnId: draggedColumnId,
                newOrd: index,
            },
        });
        draggableColumns.inner = e.items;
    }
</script>

<div
    class="flex flex-col w-screen h-screen overflow-auto text-gray-700 bg-gradient-to-tr from-blue-200 via-indigo-200 to-pink-200"
>
    <div class="px-10 mt-6">
        <h1 class="text-2xl font-bold">{boardName}</h1>
        <OtherActivitiesButton />
    </div>
    <div class="flex">
        <section
            class="flex flex-row px-10 mt-4 space-x-6 w-max"
            use:dragHandleZone={{
                items: draggableColumns.inner,
                flipDurationMs,
                type: "columns",
                dropTargetStyle: {},
            }}
            onconsider={(e) => handleConsider(e.detail)}
            onfinalize={(e) => handleFinalize(e.detail)}
        >
            {#each draggableColumns.inner as { id, columnId, column } (id)}
                <div animate:flip={{ duration: flipDurationMs }}>
                    <div use:dragHandle class="bg-gray-200 bg-opacity-30 rounded h-6 flex align-center">
                        <Fa icon={faUpDownLeftRight} class="block m-auto opacity-20" />
                    </div>
                    <BoardColumn {column} {columnId} />
                </div>
            {:else}
                <div class="flex flex-col">
                    <h5 class="h5">Click here to add a new column</h5>
                    <br />
                    <div class="opacity-5">
                        <svg
                            xmlns="http://www.w3.org/2000/svg"
                            xmlns:xlink="http://www.w3.org/1999/xlink"
                            version="1.1"
                            width="256"
                            height="256"
                            viewBox="0 0 256 256"
                            xml:space="preserve"
                        >
                            <defs />
                            <g
                                style="stroke: none; stroke-width: 0; stroke-dasharray: none; stroke-linecap: butt; stroke-linejoin: miter; stroke-miterlimit: 10; fill: none; fill-rule: nonzero; opacity: 1;"
                                transform="translate(1.4065934065934016 1.4065934065934016) scale(2.81 2.81)"
                            >
                                <path
                                    d="M 66.134 90 c 0.262 0 0.52 -0.104 0.712 -0.298 L 89.712 66.52 c 0.384 -0.39 0.384 -1.015 0 -1.404 L 66.846 41.933 c -0.285 -0.29 -0.718 -0.376 -1.091 -0.223 c -0.375 0.154 -0.621 0.52 -0.621 0.925 V 64.81 C 30.23 64.275 2 35.856 2 1 c 0 -0.552 -0.447 -1 -1 -1 S 0 0.448 0 1 c 0 36.292 29.668 65.817 66.134 65.817 c 0.552 0 1 -0.447 1 -1 V 45.073 l 20.461 20.745 L 67.134 86.563 V 76.523 c 0 -0.553 -0.448 -1 -1 -1 s -1 0.447 -1 1 V 89 c 0 0.405 0.245 0.771 0.621 0.925 C 65.877 89.976 66.006 90 66.134 90 z"
                                    style="stroke: none; stroke-width: 1; stroke-dasharray: none; stroke-linecap: butt; stroke-linejoin: miter; stroke-miterlimit: 10; fill: rgb(0,0,0); fill-rule: nonzero; opacity: 1;"
                                    transform=" matrix(1 0 0 1 0 0) "
                                    stroke-linecap="round"
                                />
                            </g>
                        </svg>
                    </div>
                </div>
            {/each}
        </section>
        <div class="flex flex-row space-x-6">
            <button onclick={createColumn} class="btn variant-ghost-tertiary h-96"><Fa icon={faPlus} /></button>
            {#if import.meta.env.DEV}
                <DebugButton />
            {/if}
            <div class="flex-shrink-0 w-6"></div>
        </div>
    </div>
</div>
