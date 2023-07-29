<script lang="ts">
    import { ListBox, ListBoxItem, drawerStore } from "@skeletonlabs/skeleton";
    import {
        categories,
        columns,
        otherTags,
        previousDrawerTab,
        selectedActivity,
        tags,
        type Tag,
        activities,
    } from "../../stores";
    import TagBadge from "../board/TagBadge.svelte";
    import BackButton from "./BackButton.svelte";
    import { fly, slide } from "svelte/transition";
    import { invoke } from "@tauri-apps/api";
    import DebugLabel from "../debug/DebugLabel.svelte";
    import SettingsButton from "../board/SettingsButton.svelte";
    import ActivityContent from "./activityContent/ActivityContent.svelte";

    let selectedCategoryId: number | null;

    let inputActivityName: string = "";
    let inputActivityBody: string = "";

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
        if (selectedCategoryId == null) {
            $otherTags.set(tagId, { ...tag, color: newColor });
            $otherTags = $otherTags;
        } else {
            $tags.set(tagId, {
                ...tag,
                color: newColor,
                categoryId: selectedCategoryId,
            });
            $tags = $tags;
        }
    }

    async function setActivityTag(newTagId: number, tag: Tag) {
        const categoryTags = $categories.get(selectedCategoryId).tags;
        for (let currentTagId of $selectedActivity.tags) {
            if (categoryTags.includes(currentTagId)) {
                for (let categoryTag of categoryTags) {
                    const index = $selectedActivity.tags.indexOf(categoryTag);
                    if (index !== -1) {
                        console.debug("Swapping category tag");
                        await invoke("remove_tag_from_activity", {
                            data: {
                                id: $selectedActivity.id,
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

    async function removeActivityTag(tagId: number) {
        const activityTags: number[] = $selectedActivity.tags;
        for (let i = 0; i < activityTags.length; i++) {
            console.debug(tagId, activityTags[i]);
            if (activityTags[i] === tagId) {
                console.debug(`Removing tag ${tagId} from activity`);
                await invoke("remove_tag_from_activity", {
                    data: {
                        id: $selectedActivity.id,
                        categoryId: selectedCategoryId,
                        tagName: $tags.get(activityTags[i]).name,
                    },
                });
                $selectedActivity.tags.splice(i, 1);
                $selectedActivity = $selectedActivity;
                $columns = $columns;
                return;
            }
        }
        console.warn(`No tag with id ${tagId} found in the activity`);
    }

    async function addNonCategoryTag(newTagId: number, tag: Tag) {
        const tags = $otherTags;
        for (let currentTagId of $selectedActivity.tags) {
            if (newTagId === currentTagId) {
                console.debug("a target tag already exists in the activity");
                return;
            }
        }
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

    async function removeNonCategoryTag(tagId: number, tag: Tag) {
        const tags = $otherTags;
        for (let currentTagId of $selectedActivity.tags) {
            if (tagId === currentTagId) {
                await invoke("remove_tag_from_activity", {
                    data: {
                        id: $selectedActivity.id,
                        categoryId: selectedCategoryId,
                        tagName: tags.get(currentTagId).name,
                    },
                });
                let a = $selectedActivity.tags;
                a.splice(
                    $selectedActivity.tags.findIndex((x) => x === currentTagId),
                    1
                );
                $selectedActivity.tags = a;
                $columns = $columns;
                return;
            }
        }
        console.debug("a target tag does not exist in the current activity");
    }
</script>

<BackButton />
<SettingsButton />
<ActivityContent />
<h2 class="h2">Categories</h2>
<ListBox>
    {#each Array.from($categories.entries()).sort(([a, catA], [b, catB]) => {
        return catA.ordinal - catB.ordinal;
    }) as [categoryId, category]}
        <DebugLabel text={"ID: " + categoryId} />
        <ListBoxItem
            bind:group={selectedCategoryId}
            name={category.name}
            value={categoryId}>{category.name}</ListBoxItem
        >
    {/each}
    <ListBoxItem bind:group={selectedCategoryId} name="Other" value={null}
        >Other</ListBoxItem
    >
</ListBox>
{#if selectedCategoryId}
    <div class="flex flex-col">
        {#each $categories.get(selectedCategoryId).tags as tagId (tagId)}
            {@const tag = $tags.get(tagId)}
            <div
                class="flex flex-row space-x-6 place-content-between m-2 bg-gray-400"
            >
                <div class="w-24 flex align-center justify-center">
                    <input
                        class="input"
                        type="color"
                        value={tag.color}
                        on:change={(e) => changeTagColor(e, tag, tagId)}
                    />
                </div>

                <div class="max-h-fit flex align-center justify-center">
                    <TagBadge name={tag.name} color={tag.color} />
                </div>

                <div class="w-24 flex align-center justify-center">
                    {#if $selectedActivity.tags.find((id) => id === tagId)}
                        <button
                            class="btn btn-sm variant-ghost-secondary"
                            on:click={() => removeActivityTag(tagId)}>Remove</button
                        >
                    {:else}
                        <button
                            class="btn btn-sm variant-ghost-primary"
                            on:click={() => setActivityTag(tagId, tag)}
                            >Choose</button
                        >
                    {/if}
                </div>
            </div>
        {:else}
            <div class="self-center mt-2">Category is empty</div>
        {/each}
    </div>
{/if}
{#if selectedCategoryId === null}
    <div class="flex flex-col">
        {#each $otherTags as [tagId, storeTag] (tagId)}
            {@const tag = $otherTags.get(tagId)}
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
                {#if $selectedActivity.tags.find((id) => id === tagId)}
                    <button
                        class={`btn btn-sm variant-ghost-secondary`}
                        on:click={() => removeNonCategoryTag(tagId, tag)}
                        >Remove</button
                    >
                {:else}
                    <button
                        class={`btn btn-sm variant-ghost-primary`}
                        on:click={() => addNonCategoryTag(tagId, tag)}
                        >Choose</button
                    >
                {/if}
            </div>
        {:else}
            <div class="self-center mt-2">Category is empty</div>
        {/each}
    </div>
{/if}
