<script lang="ts">
    import Column from "./Column.svelte";
    import { columns, currentEditable, isDebug } from "../../stores";
    import { invoke } from "@tauri-apps/api/tauri";
    import { shortcut } from "../../actions/shortcut";
    import DebugLabel from "../debug/DebugLabel.svelte";
    import { dndzone, setDebugMode } from "svelte-dnd-action";
    setDebugMode(true);
    const boardName = "Kanban";
    async function createColumn({
        currentTarget,
    }: MouseEvent & {
        currentTarget: EventTarget & HTMLButtonElement;
    }) {
        const name = "New column";
        //const id = await invoke("create_column", { name })
        const id = new Date().getMilliseconds();
        $columns.set(id, { name, activities: new Map() });
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
    });

    function handleDndConsiderColumns(
        e: CustomEvent<
            DndEvent<{
                id: number;
                col: Column;
            }>
        > & {
            target: any;
        }
    ) {
        e.detail.items;
    }
    function handleDndFinalizeColumns(
        e: CustomEvent<
            DndEvent<{
                id: number;
                col: Column;
            }>
        > & {
            target: any;
        }
    ) {}
</script>

<div
    class="flex flex-col w-screen h-screen overflow-auto text-gray-700 bg-gradient-to-tr from-blue-200 via-indigo-200 to-pink-200"
>
    <div class="px-10 mt-6">
        <h1 class="text-2xl font-bold">{boardName}</h1>
    </div>
    <div class="flex flex-grow px-10 mt-4 space-x-6 overflow-auto">
        {#each $columns as [id, column] (id)}
            <DebugLabel text={`ID ${id}`} />
            <Column {column} columnId={id} />
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
    </div>
</div>
