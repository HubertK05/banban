<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import TagBadge from "../../board/TagBadge.svelte";
    import DebugLabel from "../../debug/DebugLabel.svelte";
    import type { Tag } from "../../../interfaces";
    import {
        activitiesRune,
        categoriesRune,
        categoryTagsRune,
        changeCategoryTagColor,
        changeOtherTagColor,
        columnsRune,
        idOtherTags,
        idTags,
        otherTagsRune,
        showToast,
    } from "../../../shared.svelte";
    import { getToastStore } from "@skeletonlabs/skeleton";

    interface Props {
        tagId: number;
        tag: Tag;
        categoryId: number | undefined;
    }

    let { tagId, tag, categoryId }: Props = $props();

    const toastStore = getToastStore();

    let inputTagName: string = $state("");
    let inputTagColor: string = $state(tag.color);

    async function handleRenameTag() {
        if (inputTagName.trim() === "") {
            showToast(toastStore, "⚠️ Tag name cannot be blank");
            return;
        }
        await renameTag();
    }

    async function removeTag() {
        await invoke("delete_tag", { categoryTagId: tagId });
        Object.entries(columnsRune).forEach(([colId, column]) => {
            column.activities.map((activityId) => {
                const runeActivity = activitiesRune[activityId];
                runeActivity.tags = runeActivity.tags.filter((id) => id !== tagId);
                activitiesRune[activityId] = runeActivity;
            });
            columnsRune[+colId] = column;
        });
        if (categoryId !== undefined && categoryId !== null) {
            const runeCategory = categoriesRune[categoryId];
            runeCategory.tags = runeCategory.tags.filter((id) => id !== tagId);
            categoriesRune[categoryId] = runeCategory;
            delete categoryTagsRune[tagId];
            idTags.update();
        } else {
            delete otherTagsRune[tagId];
            idOtherTags.update();
        }
    }

    async function renameTag() {
        await invoke("update_tag_name", {
            data: { categoryTagId: tagId, tagName: inputTagName },
        });
        if (categoryId) {
            const runeTag = categoryTagsRune[tagId];
            runeTag.name = inputTagName;
            categoryTagsRune[tagId] = runeTag;
            idTags.update();
        } else {
            const runeTag = otherTagsRune[tagId];
            runeTag.name = inputTagName;
            otherTagsRune[tagId] = runeTag;
            idOtherTags.update();
        }

        inputTagName = "";
    }

    async function changeColor() {
        if (categoryId) {
            changeCategoryTagColor(inputTagColor, tagId);
            idTags.update();
        } else {
            changeOtherTagColor(inputTagColor, tagId);
            idOtherTags.update();
        }
    }
</script>

<div class="flex flex-col p-2">
    <DebugLabel text={"ID: " + tagId} />
    <DebugLabel text={"ORD: " + tag.ord} />
    <TagBadge name={tag.name} color={tag.color} />
    <div class="flex flex-row mt-2 place-content-between align-center bg-gray-300 p-1 rounded-md">
        <div class="flex w-20 align-center justify-center">
            <input class="input self-center" type="color" bind:value={inputTagColor} onchange={changeColor} />
        </div>

        <div class="flex max-h-fit align-center justify-center self-center">
            <button class="btn btn-sm variant-filled self-center m-1" onclick={async () => { handleRenameTag() }}>Rename</button>
            <input
                class="input w-24 indent-2 self-center p-1 m-1"
                bind:value={inputTagName}
                placeholder="tag name"
                onkeypress={async (e) => {
                    if (e.key === "Enter") {
                        await handleRenameTag();
                    }
                }}
            />
        </div>

        <div class="w-20 self-center">
            <button class="btn btn-sm variant-filled self-center" onclick={removeTag}>Delete</button>
        </div>
    </div>
</div>
