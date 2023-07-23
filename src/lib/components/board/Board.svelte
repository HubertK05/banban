<script lang="ts">
    import Column from "./Column.svelte";
    import { columns, currentEditable, isDebug } from "../../stores";
    import { invoke } from "@tauri-apps/api/tauri";
    import { shortcut } from "../../actions/shortcut";
    import DebugLabel from "../debug/DebugLabel.svelte";

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
</script>

<div
    class="flex flex-col w-screen h-screen overflow-auto text-gray-700 bg-gradient-to-tr from-blue-200 via-indigo-200 to-pink-200"
>
    <div class="px-10 mt-6">
        <h1 class="text-2xl font-bold">{boardName}</h1>
    </div>
    <div class="flex flex-grow px-10 mt-4 space-x-6 overflow-auto">
        {#each Array.from($columns.entries()).sort(([aId,colA], [bId, colB]) => {
            return $columns.get(aId).ord - $columns.get(bId).ord;
        }) as [id, column] (id)}
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
