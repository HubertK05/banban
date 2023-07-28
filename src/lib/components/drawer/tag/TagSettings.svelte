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
  import DebugLabel from "../../debug/DebugLabel.svelte";

    export let tagId: number;
    export let tag: Tag;
    export let categoryId: number | undefined;

    let inputTagName: string = "";
    let inputTagOrdinal: string = "";
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

    async function changeColor() {
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
    <DebugLabel text={"ID: " + tagId} />
    <DebugLabel text={"ORD: " + tag.ordinal} />
    <TagBadge name={tag.name} color={tag.color} />
    <div class="mt-2">
        <button class="btn btn-sm variant-filled" on:click={removeTag}
            >Delete</button
        >
        <button class="btn btn-sm variant-filled" on:click={() => renameTag()}
            >Rename</button
        >
        <input bind:value={inputTagName} placeholder="tag name" />
        <button class="btn btn-sm variant-filled" on:click={changeColor}
            >Change color</button
        >
        <input type="color" bind:value={inputTagColor} />
    </div>
</div>
