<script lang="ts">
    import BoardColumn from "./BoardColumn.svelte";
    import { columns, currentEditable, isDebug } from "../../stores";
    import { invoke } from "@tauri-apps/api/tauri";
    import { dndzone, setDebugMode } from "svelte-dnd-action";
    import type { Column } from "../../interfaces/main";
    import { flip } from "svelte/animate";
    import DebugButton from "../debug/DebugButton.svelte";
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
        $columns.set(res.id, { name, activities: new Map(), ord: res.ordinal });
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
            return a.col.ord - b.col.ord;
        });

    function handleConsider(
        e: CustomEvent<
            DndEvent<{
                id: number;
                col: Column;
            }>
        > & {
            target: any;
        }
    ) {
        e.detail.items.forEach(({ id, col }, index) => {
            col.ord = index;
        });
        draggableColumns = e.detail.items;
    }
    async function handleFinalize(
        e: CustomEvent<
            DndEvent<{
                id: number;
                col: Column;
            }>
        > & {
            target: any;
        }
    ) {
        e.detail.items.forEach(({ id, col }, index) => {
            const c = $columns.get(id);
            c.ord = index;
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
</script>

<div
    class="flex flex-col w-screen h-screen overflow-auto text-gray-700 bg-gradient-to-tr from-blue-200 via-indigo-200 to-pink-200"
>
    <div class="px-10 mt-6">
        <h1 class="text-2xl font-bold">{boardName}</h1>
    </div>
    <section
        class="flex flex-grow px-10 mt-4 space-x-6 overflow-auto"
        use:dndzone={{
            items: draggableColumns,
            flipDurationMs,
            type: "columns",
        }}
        on:consider={handleConsider}
        on:finalize={handleFinalize}
    >
        {#each Array.from(draggableColumns).sort((a, b) => {
            return a.col.ord - b.col.ord;
        }) as { id, col } (id)}
            <div animate:flip={{ duration: flipDurationMs }}>
                <BoardColumn column={col} columnId={id} />
            </div>
        {/each}

        <button
            on:click={createColumn}
            class="btn variant-ghost-tertiary max-h-96">+</button
        >
        <DebugButton />
        <div class="flex-shrink-0 w-6" />
    </section>
</div>
