<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import {
        activities,
        categories,
        columns,
        otherTags,
        tags,
    } from "../../../stores";
    import TagBadge from "../../board/TagBadge.svelte";
    import DebugLabel from "../../debug/DebugLabel.svelte";
  import type { Tag } from "../../../interfaces/main";
  import { categoriesRune, categoryTagsRune, changeCategoryTagColor, idTags } from "../../../shared.svelte";

    interface Props {
        tagId: number;
        tag: Tag;
        categoryId: number | undefined;
    }

    let { tagId, tag, categoryId }: Props = $props();

    let inputTagName: string = $state("");
    let inputTagColor: string = $state(tag.color);

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

            const runeCategory = categoriesRune[categoryId];
            runeCategory.tags = runeCategory.tags.filter(id => id !== tagId);
            categoriesRune[categoryId] = runeCategory;
            delete categoryTagsRune[tagId];
            idTags.update();
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
        if (categoryId) {
            const tag = $tags.get(tagId);
            $tags.set(tagId, { ...tag, name: inputTagName });
            $tags = $tags;

            const runeTag = categoryTagsRune[tagId]
            runeTag.name = inputTagName;
            categoryTagsRune[tagId] = runeTag;
            idTags.update()
        } else {
            const tag = $otherTags.get(tagId);
            $otherTags.set(tagId, { ...tag, name: inputTagName });
            $otherTags = $otherTags;
        }

        inputTagName = "";
    }

    async function changeColor() {
        if (categoryId) {
            changeCategoryTagColor(inputTagColor, tagId);
            idTags.update();
        } else {
            // changeOtherTagColor();
            const tag = $otherTags.get(tagId);
            $otherTags.set(tagId, { ...tag, color: inputTagColor });
            $otherTags = $otherTags;
        }
    }    
</script>

<div class="flex flex-col p-2">
    <DebugLabel text={"ID: " + tagId} />
    <DebugLabel text={"ORD: " + tag.ord} />
    <TagBadge name={tag.name} color={tag.color} />
    <div
        class="flex flex-row mt-2 place-content-between align-center bg-gray-300 p-1 rounded-md"
    >
        <div class="flex w-20 align-center justify-center">
            <input
                class="input self-center"
                type="color"
                bind:value={inputTagColor}
                onchange={changeColor}
            />
        </div>

        <div class="flex max-h-fit align-center justify-center self-center">
            <button
                class="btn btn-sm variant-filled self-center m-1"
                onclick={renameTag}>Rename</button
            >
            <input
                class="input w-24 indent-2 self-center p-1 m-1"
                bind:value={inputTagName}
                placeholder="tag name"
            />
        </div>

        <div class="w-20 self-center">
            <button
                class="btn btn-sm variant-filled self-center"
                onclick={removeTag}>Delete</button
            >
        </div>
    </div>
</div>
