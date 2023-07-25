<script lang="ts">
    import BoardColumn from "./BoardColumn.svelte";
    import { columns, currentEditable, isDebug } from "../../stores";
    import { invoke } from "@tauri-apps/api/tauri";
    import { shortcut } from "../../actions/shortcut";
    import DebugLabel from "../debug/DebugLabel.svelte";
    import { dndzone, setDebugMode } from "svelte-dnd-action";
    import type { Column } from "../../interfaces/main";
  import { onMount } from "svelte";
    setDebugMode(false);
    const boardName = "Kanban";

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

    $: idColumns = Array.from($columns).map(([id, col]) => {
        return {
            id,
            col,
        };
    }).sort((a, b) => {return a.col.ord - b.col.ord});

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
        idColumns = e.detail.items;
    }
    function handleFinalize(
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
            items: idColumns,
            type: "columns",
            zoneTabIndex: -1,
        }}
        on:consider={handleConsider}
        on:finalize={handleFinalize}
    >
        {#each Array.from(idColumns).sort((a, b) => {
            return a.col.ord - b.col.ord;
        }) as { id, col } (id)}
            <BoardColumn column={col} columnId={id} />
        {/each}

        <button
            on:click={createColumn}
            class="btn variant-ghost-tertiary max-h-96">+</button
        >
        <button
            on:click={() => {
                $isDebug = !$isDebug;
            }}
            use:shortcut={{ control: true, key: "d" }}
            class="btn variant-ghost-warning max-h-10"
            >Debug <br /><kbd class="kbd">⌘ + D</kbd></button
        >
        <div class="flex-shrink-0 w-6" />
    </section>
</div>
