<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";
    import { categories, columns, tags } from "../../../stores";
    import {
        ListBox,
        ListBoxItem,
        type ModalComponent,
    } from "@skeletonlabs/skeleton";
    import TagSettings from "./TagSettings.svelte";
    import DebugLabel from "../../debug/DebugLabel.svelte";

    async function createTag(tagName: string, categoryId?: number) {
        const res: {
            id: number;
            tagName: string;
            categoryId?: number;
            ordinal: number;
        } = await invoke("create_tag", { data: { tagName, categoryId } });
        console.debug(res);
        $tags.set(res.id, { name: res.tagName, ord: res.ordinal });
        $tags = $tags;
        if (categoryId !== undefined) {
            const category = $categories.get(categoryId);
            category.tags.push(res.id);
            $categories = $categories;
        }
    }

    let createTagForm = false;
    let createCategoryId: number | undefined;
    let createTagName: string = "";
    let createTagNode: HTMLInputElement;
</script>

<h2 class="h2">Tag options</h2>

{#each Array.from($categories).sort(([shit1, catA], [shit2, catB]) => {
    return catA.ord - catB.ord;
}) as [categoryId, category]}
    <p>{category.name}</p>
    {#each category.tags.sort((a, b) => {
        return $tags.get(a).ord - $tags.get(b).ord;
    }) as tagId}
        {@const tag = $tags.get(tagId)}
        <DebugLabel text={"ID: " + tagId} />
        <DebugLabel text={"ORD: " + tag.ord} />
        <TagSettings {tag} {tagId} {categoryId} />
    {:else}
        <button
            class="btn variant-ghost-tertiary"
            on:click={() => {
                createCategoryId = categoryId;
                createTagNode.focus();
            }}>Add new tag</button
        >
    {/each}
{:else}
    <p>There are no categories</p>
    <button class="btn varinat-fil">Create new</button>
{/each}
<hr />
<div class="flex flex-row items-center p-5">
    <div>
        <input
            type="text"
            class="input p-2"
            bind:this={createTagNode}
            bind:value={createTagName}
            placeholder="New tag name"
        />
        <ListBox>
            {#each Array.from($categories.entries()) as [id, category]}
                <ListBoxItem
                    bind:group={createCategoryId}
                    value={id}
                    name="categoryId"
                    >{category.name} - {category.tags.length}</ListBoxItem
                >
            {/each}
        </ListBox>
    </div>
    <button
        on:click={async () => {
            createTagForm = false;
            await createTag(createTagName, createCategoryId);
            createTagName = "";
            createCategoryId = undefined;
        }}
        class="btn variant-filled-primary"
        disabled={createTagName.length === 0 || createCategoryId === undefined}
        >Create</button
    >
</div>
