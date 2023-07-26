<script lang="ts">
    import { ListBox, ListBoxItem, drawerStore } from "@skeletonlabs/skeleton";
    import {
        categories,
        columns,
        previousDrawerTab,
        selectedActivity,
        tags,
    } from "../../stores";
    import { DrawerTab, type Tag } from "../../interfaces/main";
    import TagBadge from "../board/TagBadge.svelte";
    import BackButton from "./BackButton.svelte";
    import { fly, slide } from "svelte/transition";
    import { invoke } from "@tauri-apps/api";
    import DebugLabel from "../debug/DebugLabel.svelte";
    import SettingsButton from "../board/SettingsButton.svelte";

    let selectedCategoryId: number;

    async function changeTagColor(
        e: Event & {
            currentTarget: EventTarget & HTMLInputElement;
        },
        tag: Tag,
        tagId: number
    ) {
        const newColor = e.currentTarget.value;
        await invoke("update_tag_color", {
            data: { categoryTagId: tagId, color: newColor.slice(1) },
        });

        $tags.set(tagId, { ...tag, color: newColor });
        $tags = $tags;
        $columns = $columns;
    }

    async function changeActivityTag(newTagId: number, tag: Tag) {
        const categoryTags = $categories.get(selectedCategoryId).tags;
        for (let currentTagId of $selectedActivity.tags) {
            if (categoryTags.includes(currentTagId)) {
                for (let categoryTag of categoryTags) {
                    console.log($selectedActivity.tags, categoryTag);
                    const index = $selectedActivity.tags.indexOf(categoryTag);
                    if (index !== -1) {
                        console.debug("Swapping category tag");
                        await invoke("remove_tag_from_activity", {
                            data: {
                                id: currentTagId,
                                categoryId: selectedCategoryId,
                                tagName: $tags.get(currentTagId).name,
                            },
                        });
                        await invoke("add_tag_to_activity", {
                            data: {
                                id: $selectedActivity.id,
                                categoryId: selectedCategoryId,
                                tagName: tag.name,
                            },
                        });
                        $selectedActivity.tags[index] = newTagId;
                        $columns = $columns;
                        return;
                    }
                }
            }
        }
        console.debug("Adding a new category tag");
        await invoke("add_tag_to_activity", {
            data: {
                id: $selectedActivity.id,
                categoryId: selectedCategoryId,
                tagName: tag.name,
            },
        });
        $selectedActivity.tags.push(newTagId);
        $selectedActivity = $selectedActivity;
        $columns = $columns;
    }
</script>

<BackButton />
<SettingsButton />
<h2 class="h2">Categories</h2>
<ListBox>
    {#each Array.from($categories.entries()).sort(([a, catA], [b, catB]) => {
        return catA.ord - catB.ord;
    }) as [categoryId, category]}
        <DebugLabel text={"ID: " + categoryId} />
        <ListBoxItem
            bind:group={selectedCategoryId}
            name={category.name}
            value={categoryId}>{category.name}</ListBoxItem
        >
    {/each}
</ListBox>
{#if selectedCategoryId}
    <div class="flex flex-col">
        {#each $categories.get(selectedCategoryId).tags as tagId (tagId)}
            {@const tag = $tags.get(tagId)}
            <div
                class="flex flex-row space-x-6 items-center place-content-between m-2"
            >
                <input
                    class="input"
                    type="color"
                    value={tag.color}
                    on:change={(e) => changeTagColor(e, tag, tagId)}
                />

                <TagBadge name={tag.name} color={tag.color} />
                <button
                    class={`btn btn-sm ${
                        $selectedActivity.tags.find((id) => id === tagId)
                            ? "variant-ghost-secondary"
                            : "variant-ghost-primary"
                    }`}
                    on:click={() => changeActivityTag(tagId, tag)}
                    >Choose</button
                >
            </div>
        {:else}
            <div class="self-center mt-2">Category is empty</div>
        {/each}
    </div>
{/if}
