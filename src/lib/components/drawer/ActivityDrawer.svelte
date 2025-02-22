<script lang="ts">
    import { ListBox, ListBoxItem } from "@skeletonlabs/skeleton";
    import TagBadge from "../board/TagBadge.svelte";
    import BackButton from "./BackButton.svelte";
    import { invoke } from "@tauri-apps/api/core";
    import DebugLabel from "../debug/DebugLabel.svelte";
    import SettingsButton from "../board/SettingsButton.svelte";
    import ActivityContent from "./activityContent/ActivityContent.svelte";
    import {
        activitiesRune,
        categoriesRune,
        categoryTagsRune,
        changeCategoryTagColor,
        changeOtherTagColor,
        otherTagsRune,
    } from "../../shared.svelte";
    import type { Activity } from "../../interfaces";

    interface Props {
        activityId: number;
    }

    let { activityId }: Props = $props();
    console.assert(activitiesRune[activityId] !== undefined, "Selected activity is undefined");
    let selectedActivity: Activity = $derived(activitiesRune[activityId]);

    let selectedCategoryId: number | null = $state(null);

    function sortedTags(categoryId: number) {
        return categoriesRune[categoryId].tags
            .map((tagId) => {
                return { id: tagId, tag: categoryTagsRune[tagId] };
            })
            .sort(({ id: _1, tag: tag1 }, { id: _2, tag: tag2 }) => {
                return tag1.ord - tag2.ord;
            });
    }

    function sortedNonCategoryTags() {
        return Object.entries(otherTagsRune).sort(([_1, tag1], [_2, tag2]) => {
            return tag1.ord - tag2.ord;
        });
    }

    async function changeTagColor(newColor: string, tagId: number) {
        if (selectedCategoryId === null) {
            changeOtherTagColor(newColor, tagId);
        } else {
            changeCategoryTagColor(newColor, tagId);
        }
    }

    async function setActivityTag(newTagId: number, tagName: string) {
        console.assert(selectedCategoryId !== null, "Selected category id is null (in setActivityTag)");
        if (selectedCategoryId === null) return;
        const categoryTags = categoriesRune[selectedCategoryId].tags;
        for (let currentTagId of selectedActivity.tags) {
            if (categoryTags.includes(currentTagId)) {
                for (let categoryTag of categoryTags) {
                    const index = selectedActivity.tags.indexOf(categoryTag);
                    if (index !== -1) {
                        console.debug("Swapping category tag");
                        await invoke("remove_tag_from_activity", {
                            data: {
                                id: activityId,
                                categoryId: selectedCategoryId,
                                tagName: categoryTagsRune[currentTagId].name,
                            },
                        });
                        await invoke("add_tag_to_activity", {
                            data: {
                                id: activityId,
                                categoryId: selectedCategoryId,
                                tagName,
                            },
                        });
                        activitiesRune[activityId].tags[index] = newTagId;
                        return;
                    }
                }
            }
        }
        console.debug("Adding a new category tag");
        await invoke("add_tag_to_activity", {
            data: {
                id: activityId,
                categoryId: selectedCategoryId,
                tagName,
            },
        });
        activitiesRune[activityId].tags.push(newTagId);
    }

    async function removeActivityTag(tagId: number) {
        const activityTags: number[] = selectedActivity.tags;
        for (let i = 0; i < activityTags.length; i++) {
            console.debug(tagId, activityTags[i]);
            if (activityTags[i] === tagId) {
                console.debug(`Removing tag ${tagId} from activity`);
                await invoke("remove_tag_from_activity", {
                    data: {
                        id: activityId,
                        categoryId: selectedCategoryId,
                        tagName: categoryTagsRune[activityTags[i]].name,
                    },
                });
                activitiesRune[activityId].tags.splice(i, 1);
                return;
            }
        }
        console.warn(`No tag with id ${tagId} found in the activity`);
    }

    async function addNonCategoryTag(newTagId: number, tagName: string) {
        for (let currentTagId of selectedActivity.tags) {
            if (newTagId === currentTagId) {
                console.debug("a target tag already exists in the activity");
                return;
            }
        }
        await invoke("add_tag_to_activity", {
            data: {
                id: activityId,
                categoryId: selectedCategoryId,
                tagName,
            },
        });
        activitiesRune[activityId].tags.push(newTagId);
    }

    async function removeNonCategoryTag(tagId: number) {
        for (let currentTagId of selectedActivity.tags) {
            if (tagId === currentTagId) {
                await invoke("remove_tag_from_activity", {
                    data: {
                        id: activityId,
                        categoryId: selectedCategoryId,
                        tagName: otherTagsRune[currentTagId].name,
                    },
                });
                activitiesRune[activityId].tags = activitiesRune[activityId].tags.filter((x) => x !== currentTagId);
                return;
            }
        }
        console.debug("a target tag does not exist in the current activity");
    }
</script>

<BackButton />
<SettingsButton />
<ActivityContent {activityId} />
<h2 class="h2">Categories</h2>
<ListBox>
    {#each Object.entries(categoriesRune).sort(([a, catA], [b, catB]) => {
        return catA.ord - catB.ord;
    }) as [categoryId, category]}
        <DebugLabel text={"ID: " + categoryId} />
        <ListBoxItem bind:group={selectedCategoryId} name={category.name} value={+categoryId}
            >{category.name}</ListBoxItem
        >
    {/each}
    <ListBoxItem bind:group={selectedCategoryId} name="Other" value={null}>Other</ListBoxItem>
</ListBox>
{#if selectedCategoryId}
    <div class="flex flex-col">
        {#each sortedTags(selectedCategoryId) as { id: tagId, tag } (tagId)}
            <div class="flex flex-row space-x-6 place-content-between m-2 bg-gray-300 p-1 rounded">
                <div class="w-20 flex align-center justify-center self-center">
                    <input
                        class="input"
                        type="color"
                        value={tag.color}
                        onchange={(e) => changeTagColor(e.currentTarget.value, tagId)}
                    />
                </div>

                <div class="max-h-fit flex align-center justify-center self-center">
                    <TagBadge name={tag.name} color={tag.color} />
                </div>

                <div class="w-20 flex align-center justify-center self-center">
                    {#if selectedActivity.tags.find((id) => id === tagId)}
                        <button
                            class="btn btn-sm variant-ghost-secondary self-center"
                            onclick={() => removeActivityTag(tagId)}>Remove</button
                        >
                    {:else}
                        <button
                            class="btn btn-sm variant-ghost-primary self-center"
                            onclick={() => setActivityTag(tagId, tag.name)}>Choose</button
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
        {#each sortedNonCategoryTags() as [tagId, tag] (tagId)}
            <div class="flex flex-row space-x-6 items-center place-content-between m-2 bg-gray-300 p-1 rounded">
                <div class="w-20 flex align-center justify-center self-center">
                    <input
                        class="input"
                        type="color"
                        value={tag.color}
                        onchange={(e) => changeTagColor(e.currentTarget.value, +tagId)}
                    />
                </div>

                <div class="max-h-fit flex align-center justify-center self-center">
                    <TagBadge name={tag.name} color={tag.color} />
                </div>

                <div class="w-20 flex align-center justify-center self-center">
                    {#if selectedActivity.tags.find((id) => id === +tagId)}
                        <button
                            class="btn btn-sm variant-ghost-secondary self-center"
                            onclick={() => removeNonCategoryTag(+tagId)}>Remove</button
                        >
                    {:else}
                        <button
                            class="btn btn-sm variant-ghost-primary self-center"
                            onclick={() => addNonCategoryTag(+tagId, tag.name)}>Choose</button
                        >
                    {/if}
                </div>
            </div>
        {:else}
            <div class="self-center mt-2">Category is empty</div>
        {/each}
    </div>
{/if}
