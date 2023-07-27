<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";
    import {
        activities,
        categories,
        columns,
        otherTags,
        tags,
        type Tag,
    } from "../../../stores";
    import TagBadge from "../../board/TagBadge.svelte";

    export let tagId: number;
    export let tag: Tag;
    export let categoryId: number | undefined;

    let inputTagName: string = "";
    let inputTagOrdinal: string = "";
    let inputCategoryName: string = "";
    let inputTagColor: string = tag.color;

    async function removeTag() {
        await invoke("delete_tag", { categoryTagId: tagId });
        $columns.forEach((column, columnId) => {
            column.activities.map((activityId) => {
                const activity = $activities.get(activityId);
                activity.tags = activity.tags.filter((id) => id !== tagId);
                $activities.set(activityId, activity);
            });
            $columns.set(columnId, column);
        });
        if (categoryId !== undefined && categoryId !== null) {
            $tags.delete(tagId);
            const category = $categories.get(categoryId);
            category.tags = category.tags.filter((id) => id !== tagId);
            $categories.set(categoryId, category);
            $tags = $tags;
            $categories = $categories;
            $columns = $columns;
        } else {
            $otherTags.delete(tagId);
            $otherTags = $otherTags;
            $columns = $columns;
        }
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
        // console.debug(tagId);

        if (categoryId) {
            const tag = $tags.get(tagId);
            $tags.set(tagId, { ...tag, ordinal: newOrd });
            const orderedTags: Array<number> = $categories.get(categoryId).tags;
            orderedTags.splice(tag.ordinal, 1);
            orderedTags.splice(newOrd, 0, tagId);
            orderedTags.forEach((x, idx) => {
                $tags.get(x).ordinal = idx;
            });

            $categories.get(categoryId).tags = orderedTags;
            $tags = $tags;
            $categories = $categories;
        } else {
            const tag = $otherTags.get(tagId);
            // console.debug("previous state: ", $nonCategoryTags);
            let a: Array<[number, Tag]> = Array.from($otherTags).sort(
                ([a, tagA], [b, tagB]) => {
                    return tagA.ordinal - tagB.ordinal;
                }
            );
            // console.debug("array from previous state: ", a);
            a.forEach((x) => {
                console.debug(x);
            });
            let modifiedTag = a[tag.ordinal];
            a.splice(tag.ordinal, 1);
            a.splice(newOrd, 0, modifiedTag);
            // console.debug("array after splicing: ");
            a.forEach((x) => {
                console.debug(x);
            });
            a.forEach(([currTagId, currTag], idx) => {
                $otherTags.get(currTagId).ordinal = idx;
            });
            $otherTags.set(tagId, { ...tag, ordinal: newOrd });
            $otherTags = $otherTags;
            // console.debug("array after changing indices: ", a);
            // console.debug("new state: ", $nonCategoryTags);
        }

        inputTagOrdinal = "";
    }

    async function changeColor() {
        console.debug(tagId);
        console.debug(inputTagColor.slice(1));
        await invoke("update_tag_color", {
            data: { categoryTagId: tagId, color: inputTagColor.slice(1) },
        });
        if (categoryId) {
            const tag = $tags.get(tagId);
            $tags.set(tagId, { ...tag, color: inputTagColor });
            $tags = $tags;
        } else {
            const tag = $otherTags.get(tagId);
            $otherTags.set(tagId, { ...tag, color: inputTagColor });
            $otherTags = $otherTags;
        }
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
