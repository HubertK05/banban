<script lang="ts">
    import { drawerStore } from "@skeletonlabs/skeleton";
    import { categories, previousDrawerTab } from "../../stores";
    import BackButton from "./BackButton.svelte";
    import { invoke } from "@tauri-apps/api";
    import TagSettings from "./tag/TagSettings.svelte";
    import CategorySettings from "./tag/CategorySettings.svelte";

    let categoryName: string = "";
    async function createCategory() {
        const res: {
            id: number;
            name: string;
            ordinal: number;
        } = await invoke("create_category", { name: categoryName });
        $categories.set(res.id, { name: res.name, tags: [], ord: res.ordinal });
        categoryName = "";
        $categories = $categories;
    }
</script>

<BackButton />
<div class="flex flex-col">
    <CategorySettings />
    <input class="input p-2" bind:value={categoryName} />
    <button on:click={createCategory} class="btn variant-ghost-primary"
        >Add new category</button
    >
</div>
