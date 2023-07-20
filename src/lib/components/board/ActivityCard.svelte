<script lang="ts">
    import type { Activity } from "../../interfaces/main";

    export let id: number;
    export let activity: Activity;
</script>

<div
    class="relative flex flex-col items-start p-4 mt-3 bg-white rounded-lg cursor-pointer bg-opacity-90 group hover:bg-opacity-100"
    draggable="true"
>
    <button
        class="absolute top-0 right-0 flex items-center justify-center hidden w-5 h-5 mt-3 mr-2 text-gray-500 rounded hover:bg-gray-200 hover:text-gray-700 group-hover:flex"
    >
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
    <span class="flex items-center h-6 px-3 text-lg font-semibold rounded-full"
        >{activity.name}</span
    >
    <h4 class="mt-3 text-sm font-medium">
        {activity.body ?? ""}
    </h4>
    <div
        class="flex items-center w-full mt-3 text-xs font-medium text-gray-400"
    />
</div>
