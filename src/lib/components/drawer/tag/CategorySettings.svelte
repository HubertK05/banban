<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";
    import { categories, columns, tags } from "../../../stores";
    import { ListBox, ListBoxItem } from "@skeletonlabs/skeleton";
    import TagSettings from "./TagSettings.svelte";

    async function createTag(tagName: string, categoryId?: number) {
        // const res: {
        //     id: number;
        //     tagName: string;
        //     categoryId?: number;
        //     ordinal: number;
        // } = await invoke("create_tag", { data: { tagName, categoryId } });
        const res: {
            id: number;
            tagName: string;
            categoryId?: number;
            ordinal: number;
        } = { id: 123, tagName: "asdasd", categoryId: 1, ordinal: 6 };
        $tags.set(res.id, { name: res.tagName, ord: res.ordinal });
        $tags = $tags;
        if (res.categoryId !== undefined && categoryId !== undefined) {
            const category = $categories.get(categoryId);
            category.tags.push(res.id);
            $categories = $categories;
        }
    }
</script>

<h2 class="h2">Tag options</h2>

{#each $categories as [categoryId, category]}
    <p>{category.name}</p>
    {#each category.tags.sort((a, b) => {
        return $tags.get(a).ord - $tags.get(b).ord;
    }) as tagId}
        {@const tag = $tags.get(tagId)}
        <TagSettings {tag} {tagId} {categoryId} />
    {:else}
        <button
            class="btn variant-ghost-tertiary"
            on:click={() => createTag("asdasd", categoryId)}
            >Add new {category.name} tag</button
        >
    {/each}
{:else}
    <p>There are no categories</p>
    <button class="btn varinat-fil">Create new</button>
{/each}
