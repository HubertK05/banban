<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";
    import { categories, columns, tags } from "../../../stores";
    import type { Tag } from "../../../interfaces/main";

    export let tagId: number;
    export let tag: Tag;
    export let categoryId: number;

    async function removeTag() {
        // await invoke("delete_tag", { categoryTagId: tagId });
        $tags.delete(tagId);
        $columns.forEach((column, columnId) => {
            column.activities.forEach((activity, activityId) => {
                activity.tags = activity.tags.filter((id) => id !== tagId);
                column.activities.set(activityId, activity);
            });
            $columns.set(columnId, column);
        });
        const category = $categories.get(categoryId);
        category.tags = category.tags.filter((id) => id !== tagId);
        $categories.set(categoryId, category);
        $categories = $categories;
        $columns = $columns;
    }

    async function renameTag(newTagName: string) {
        // await invoke("update_tag_name", {
        //     data: { categoryTagId: tagId, tagName: newTagName },
        // });
        const tag = $tags.get(tagId);
        $tags.set(tagId, { ...tag, name: newTagName });
        $tags = $tags;
    }

    async function changeTagCategory(newCategoryId?: number) {
        // await invoke("attach_tag_to_category", {
        //     data: { categoryTagId: tagId, categoryId: newCategoryId },
        // });
        if (newCategoryId === undefined) {
            // detatch category
            return;
        }
        const category = $categories.get(categoryId);
        category.tags = category.tags.filter((id) => id !== tagId);
        $categories.set(categoryId, category);
        const updatedCategory = $categories.get(newCategoryId);
        updatedCategory.tags.push(tagId);
        $categories.set(newCategoryId, updatedCategory);
        $categories = $categories;
    }

    async function changeTagOrder(ord: number) {
        // await invoke("update_tag_ordinal", {
        //     data: { categoryTagId: tagId, newOrd: ord },
        // });
        const tag = $tags.get(tagId);
        $tags.set(tagId, { ...tag, ord });
        $tags = $tags;
        $categories = $categories;
    }
</script>

<div class="flex flex-col p-2">
    <p>{tag.name}</p>
    <div>
        <button class="btn btn-sm variant-filled" on:click={removeTag}
            >Delete</button
        >
        <button
            class="btn btn-sm variant-filled"
            on:click={() => renameTag("blah")}>Rename</button
        >
        <button
            class="btn btn-sm variant-filled"
            on:click={() => changeTagCategory(2)}>Change category</button
        >
        <button
            class="btn btn-sm variant-filled"
            on:click={() => changeTagOrder(1)}>Change order</button
        >
        <button class="btn btn-sm variant-filled">Change color</button>
    </div>
</div>
