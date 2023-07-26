<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";
    import { categories, columns, tags } from "../../../stores";
    import type { Category, Tag } from "../../../interfaces/main";
    import TagBadge from "../../board/TagBadge.svelte";

    export let tagId: number;
    export let tag: Tag;
    export let categoryId: number;

    let inputTagName: string = "";
    let inputTagOrdinal: string = "";
    let inputCategoryName: string = "";
    let inputTagColor: string = tag.color;

    async function removeTag() {
        await invoke("delete_tag", { categoryTagId: tagId });
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

    async function renameTag() {
        await invoke("update_tag_name", {
            data: { categoryTagId: tagId, tagName: inputTagName },
        });
        const tag = $tags.get(tagId);
        $tags.set(tagId, { ...tag, name: inputTagName });
        $tags = $tags;
        inputTagName = "";
    }

    async function changeTagCategoryByName() {
        const category = [...$categories].find(
            (x) => x[1].name === inputCategoryName
        );
        if (category === undefined) {
            console.debug(
                `No category with name of ${inputCategoryName} found`
            );
            return;
        }
        changeTagCategory(category[0]);
        inputCategoryName = "";
    }

    async function changeTagCategory(newCategoryId?: number) {
        if (newCategoryId === undefined) {
            // TODO: detatch category
            return;
        }
        const category = $categories.get(categoryId);
        if (!category) {
            console.debug(`No category with id of ${newCategoryId} found`);
            return;
        }
        await invoke("attach_tag_to_category", {
            data: { categoryTagId: tagId, categoryId: newCategoryId },
        });
        category.tags = category.tags.filter((id) => id !== tagId);
        $categories.set(categoryId, category);
        const updatedCategory = $categories.get(newCategoryId);
        updatedCategory.tags.push(tagId);
        $categories.set(newCategoryId, updatedCategory);
        $categories = $categories;
    }

    async function changeTagOrder() {
        let newOrd = Number(inputTagOrdinal);
        if (isNaN(newOrd)) {
            console.debug("new tag ordinal is not a number");
            return;
        }
        await invoke("update_tag_ordinal", {
            data: { categoryTagId: tagId, newOrd },
        });
        const tag = $tags.get(tagId);
        $tags.set(tagId, { ...tag, ord: newOrd });
        let a: Array<number> = $categories.get(categoryId).tags;
        a.splice(tag.ord, 1);
        a.splice(newOrd, 0, tagId);
        a.forEach((x, idx) => {
            $tags.get(x).ord = idx;
        });
        $categories.get(categoryId).tags = a;
        $tags = $tags;
        $categories = $categories;
        inputTagOrdinal = "";
    }

    async function changeColor() {
        await invoke("update_tag_color", {
            data: { categoryTagId: tagId, color: inputTagColor.slice(1) },
        });
        const tag = $tags.get(tagId);
        $tags.set(tagId, { ...tag, color: inputTagColor });
        $tags = $tags;
    }
</script>

<div class="flex flex-col p-2">
    <TagBadge name={tag.name} color={tag.color} />
    <div class="mt-2">
        <button class="btn btn-sm variant-filled" on:click={removeTag}
            >Delete</button
        >
        <button class="btn btn-sm variant-filled" on:click={() => renameTag()}
            >Rename</button
        >
        <input bind:value={inputTagName} placeholder="tag name" />
        <button
            class="btn btn-sm variant-filled"
            on:click={() => changeTagCategoryByName()}>Change category</button
        >
        <input bind:value={inputCategoryName} placeholder="category name" />
        <button
            class="btn btn-sm variant-filled"
            on:click={() => changeTagOrder()}>Change order</button
        >
        <input bind:value={inputTagOrdinal} placeholder="tag ordinal" />
        <button class="btn btn-sm variant-filled" on:click={changeColor}
            >Change color</button
        >
        <input type="color" bind:value={inputTagColor} />
    </div>
</div>
