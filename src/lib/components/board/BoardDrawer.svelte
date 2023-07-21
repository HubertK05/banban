<script lang="ts">
    import {
        Autocomplete,
        Drawer,
        InputChip,
        ListBox,
        ListBoxItem,
        drawerStore,
        type AutocompleteOption,
    } from "@skeletonlabs/skeleton";
    import { categories, selectedActivity } from "../../stores";
    import TagBadge from "./TagBadge.svelte";
    import type { Tag } from "../../interfaces/main";

    let optionCategory: number;

    function changeTagColor(
        e: Event & {
            currentTarget: EventTarget & HTMLInputElement;
        },
        tag: Tag
    ) {
        console.log(e.currentTarget.value, tag.id);
        const color = e.currentTarget.value;
        const category = $categories.get(optionCategory);
        const inCategoryIndex = category.tags.findIndex((t) => t.id === tag.id);
        category.tags[inCategoryIndex].color = color;
        $categories.set(optionCategory, category);
        $categories = $categories;
    }

    function openSettingsDrawer() {
        $drawerStore.id = "settings";
    }
</script>

<Drawer position="right">
    {#if $drawerStore.id === "activity"}
        <h1 class="h1 variant-soft-surface p-2">
            {$selectedActivity.name}
        </h1>

        <button class="btn" on:click={openSettingsDrawer}
            ><svg
                xmlns="http://www.w3.org/2000/svg"
                height="1em"
                viewBox="0 0 512 512"
                ><!--! Font Awesome Free 6.4.0 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license (Commercial License) Copyright 2023 Fonticons, Inc. --><path
                    d="M495.9 166.6c3.2 8.7 .5 18.4-6.4 24.6l-43.3 39.4c1.1 8.3 1.7 16.8 1.7 25.4s-.6 17.1-1.7 25.4l43.3 39.4c6.9 6.2 9.6 15.9 6.4 24.6c-4.4 11.9-9.7 23.3-15.8 34.3l-4.7 8.1c-6.6 11-14 21.4-22.1 31.2c-5.9 7.2-15.7 9.6-24.5 6.8l-55.7-17.7c-13.4 10.3-28.2 18.9-44 25.4l-12.5 57.1c-2 9.1-9 16.3-18.2 17.8c-13.8 2.3-28 3.5-42.5 3.5s-28.7-1.2-42.5-3.5c-9.2-1.5-16.2-8.7-18.2-17.8l-12.5-57.1c-15.8-6.5-30.6-15.1-44-25.4L83.1 425.9c-8.8 2.8-18.6 .3-24.5-6.8c-8.1-9.8-15.5-20.2-22.1-31.2l-4.7-8.1c-6.1-11-11.4-22.4-15.8-34.3c-3.2-8.7-.5-18.4 6.4-24.6l43.3-39.4C64.6 273.1 64 264.6 64 256s.6-17.1 1.7-25.4L22.4 191.2c-6.9-6.2-9.6-15.9-6.4-24.6c4.4-11.9 9.7-23.3 15.8-34.3l4.7-8.1c6.6-11 14-21.4 22.1-31.2c5.9-7.2 15.7-9.6 24.5-6.8l55.7 17.7c13.4-10.3 28.2-18.9 44-25.4l12.5-57.1c2-9.1 9-16.3 18.2-17.8C227.3 1.2 241.5 0 256 0s28.7 1.2 42.5 3.5c9.2 1.5 16.2 8.7 18.2 17.8l12.5 57.1c15.8 6.5 30.6 15.1 44 25.4l55.7-17.7c8.8-2.8 18.6-.3 24.5 6.8c8.1 9.8 15.5 20.2 22.1 31.2l4.7 8.1c6.1 11 11.4 22.4 15.8 34.3zM256 336a80 80 0 1 0 0-160 80 80 0 1 0 0 160z"
                /></svg
            ></button
        >

        <h2 class="h2">Categories</h2>
        <ListBox>
            {#each $categories as [categoryId, category]}
                <ListBoxItem
                    bind:group={optionCategory}
                    name={category.name}
                    value={categoryId}>{category.name}</ListBoxItem
                >
            {/each}
        </ListBox>
        <div
            class="card w-full max-w-sm max-h-48 p-4 overflow-y-auto"
            tabindex="-1"
        />
        {#if optionCategory}
            <div class="flex flex-col">
                {#each $categories.get(optionCategory).tags as tag (tag.id)}
                    <div class="flex flex-row">
                        <input
                            class="input"
                            type="color"
                            bind:value={tag.color}
                            on:change={(e) => changeTagColor(e, tag)}
                        />
                        <TagBadge name={tag.name} color={tag.color} />
                    </div>
                {/each}
            </div>
        {/if}
    {:else if $drawerStore.id === "settings"}
        <h1 class="h1 variant-soft-surface p-2">Settings</h1>
        <button class="btn" on:click={() => ($drawerStore.id = "activity")}
            ><svg
                xmlns="http://www.w3.org/2000/svg"
                height="1em"
                viewBox="0 0 448 512"
                ><!--! Font Awesome Free 6.4.0 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license (Commercial License) Copyright 2023 Fonticons, Inc. --><path
                    d="M9.4 233.4c-12.5 12.5-12.5 32.8 0 45.3l160 160c12.5 12.5 32.8 12.5 45.3 0s12.5-32.8 0-45.3L109.2 288 416 288c17.7 0 32-14.3 32-32s-14.3-32-32-32l-306.7 0L214.6 118.6c12.5-12.5 12.5-32.8 0-45.3s-32.8-12.5-45.3 0l-160 160z"
                /></svg
            ></button
        >
    {/if}
</Drawer>
