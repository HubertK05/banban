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

    let selectedCategoryId: number = Array.from($categories).at(0)[0];

    function changeTagColor(
        e: Event & {
            currentTarget: EventTarget & HTMLInputElement;
        },
        tag: Tag,
        tagId: number
    ) {
        console.log(e.currentTarget.value, tagId);
        const color = e.currentTarget.value;
        $tags.set(tagId, { ...tag, color });
        $tags = $tags;
        $columns = $columns;
    }

    function openSettingsDrawer() {
        $previousDrawerTab = $drawerStore.id as DrawerTab;
        $drawerStore.id = DrawerTab.Settings;
    }

    async function changeActivityTag(newTagId: number, tag: Tag) {
        await invoke("add_tag_to_activity", {
            data: {
                id: newTagId,
                category_id: selectedCategoryId,
                tag_name: tag.name,
            },
        });
        const categoryTags = $categories.get(selectedCategoryId).tags;
        for (let currentTagId of $selectedActivity.tags) {
            if (categoryTags.includes(currentTagId)) {
                for (let categoryTag of categoryTags) {
                    console.log($selectedActivity.tags, categoryTag);
                    const index = $selectedActivity.tags.indexOf(categoryTag);
                    if (index !== -1) {
                        $selectedActivity.tags[index] = newTagId;
                        $columns = $columns;
                        return;
                    }
                }
            }
        }
        $selectedActivity.tags.push(newTagId);
        $columns = $columns;
    }
</script>

<BackButton />
<button class="btn" on:click={openSettingsDrawer}
    ><svg xmlns="http://www.w3.org/2000/svg" height="1em" viewBox="0 0 512 512"
        ><!--! Font Awesome Free 6.4.0 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license (Commercial License) Copyright 2023 Fonticons, Inc. --><path
            d="M495.9 166.6c3.2 8.7 .5 18.4-6.4 24.6l-43.3 39.4c1.1 8.3 1.7 16.8 1.7 25.4s-.6 17.1-1.7 25.4l43.3 39.4c6.9 6.2 9.6 15.9 6.4 24.6c-4.4 11.9-9.7 23.3-15.8 34.3l-4.7 8.1c-6.6 11-14 21.4-22.1 31.2c-5.9 7.2-15.7 9.6-24.5 6.8l-55.7-17.7c-13.4 10.3-28.2 18.9-44 25.4l-12.5 57.1c-2 9.1-9 16.3-18.2 17.8c-13.8 2.3-28 3.5-42.5 3.5s-28.7-1.2-42.5-3.5c-9.2-1.5-16.2-8.7-18.2-17.8l-12.5-57.1c-15.8-6.5-30.6-15.1-44-25.4L83.1 425.9c-8.8 2.8-18.6 .3-24.5-6.8c-8.1-9.8-15.5-20.2-22.1-31.2l-4.7-8.1c-6.1-11-11.4-22.4-15.8-34.3c-3.2-8.7-.5-18.4 6.4-24.6l43.3-39.4C64.6 273.1 64 264.6 64 256s.6-17.1 1.7-25.4L22.4 191.2c-6.9-6.2-9.6-15.9-6.4-24.6c4.4-11.9 9.7-23.3 15.8-34.3l4.7-8.1c6.6-11 14-21.4 22.1-31.2c5.9-7.2 15.7-9.6 24.5-6.8l55.7 17.7c13.4-10.3 28.2-18.9 44-25.4l12.5-57.1c2-9.1 9-16.3 18.2-17.8C227.3 1.2 241.5 0 256 0s28.7 1.2 42.5 3.5c9.2 1.5 16.2 8.7 18.2 17.8l12.5 57.1c15.8 6.5 30.6 15.1 44 25.4l55.7-17.7c8.8-2.8 18.6-.3 24.5 6.8c8.1 9.8 15.5 20.2 22.1 31.2l4.7 8.1c6.1 11 11.4 22.4 15.8 34.3zM256 336a80 80 0 1 0 0-160 80 80 0 1 0 0 160z"
        /></svg
    ></button
>

<h2 class="h2">Categories</h2>
<ListBox>
    {#each $categories as [categoryId, category]}
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
