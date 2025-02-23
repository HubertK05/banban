<script lang="ts">
    import BackButton from "../BackButton.svelte";
    import { invoke } from "@tauri-apps/api/core";
    import CategorySettings from "../tag/CategorySettings.svelte";
    import { categoriesRune } from "../../../shared.svelte";

    let categoryName: string = $state("");

    async function handleCreateCategoryClick() {
        if (categoryName === "") return;
        createCategory();
        categoryName = "";
    }

    async function createCategory() {
        const res: {
            id: number;
            name: string;
            ordinal: number;
        } = await invoke("create_category", { name: categoryName });

        categoriesRune[res.id] = {
            name: res.name,
            tags: [],
            ord: res.ordinal,
        };
    }
</script>

<BackButton />
<div class="flex flex-col">
    <CategorySettings />
    <input
        class="input p-2"
        bind:value={categoryName}
        onkeypress={async (e) => {
            if (e.key === "Enter") {
                await handleCreateCategoryClick();
            }
        }}
    />
    <button
        onclick={async () => {
            await handleCreateCategoryClick();
        }}
        class="btn variant-ghost-primary">Add new category</button
    >
</div>
