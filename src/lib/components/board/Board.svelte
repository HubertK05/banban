<script lang="ts">
    import Column from "./Column.svelte";
    import { columns } from "../../stores";

    const boardName = "Kanban";
</script>

<div
    class="flex flex-col w-screen h-screen overflow-auto text-gray-700 bg-gradient-to-tr from-blue-200 via-indigo-200 to-pink-200"
>
    <div class="px-10 mt-6">
        <h1 class="text-2xl font-bold">{boardName}</h1>
    </div>
    <div class="flex flex-grow px-10 mt-4 space-x-6 overflow-auto">
        {#each $columns as [id, column]}
            <Column {column} {id} />
        {/each}

        <div class="flex-shrink-0 w-6" />
    </div>
</div>
