<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";
    import { categories, columns, otherTags, tags, type Tag } from "../../../stores";
    import {
        ListBox,
        ListBoxItem,
        type ModalComponent,
    } from "@skeletonlabs/skeleton";
    import TagSettings from "./TagSettings.svelte";
    import DebugLabel from "../../debug/DebugLabel.svelte";
  import { dndzone, TRIGGERS } from "svelte-dnd-action";
  import TagBadge from "../../board/TagBadge.svelte";

    const flipDurationMs = 125;

    $: idTags = Array.from($tags)
        .map(([tagId, tag]) => {
            return {
                id: tagId,
                tag
            };
        })    
        .sort((a, b) => { return a.tag.ordinal - b.tag.ordinal });

    $: idOtherTags = Array.from($otherTags).map(([tagId, tag]) => {
        return {
            id: tagId,
            tag
        }
    }).sort((a, b) => {
        return a.tag.ordinal - b.tag.ordinal;
    })

    async function createTag(tagName: string, categoryId?: number) {
        const res: {
            id: number;
            tagName: string;
            categoryId?: number;
            ordinal: number;
            color: string;
        } = await invoke("create_tag", { data: { tagName, categoryId } });

        if (categoryId !== undefined && categoryId !== null) {
            $tags.set(res.id, {
                name: res.tagName,
                ordinal: res.ordinal,
                color: res.color,
                categoryId,
            });
            const category = $categories.get(categoryId);
            category.tags.push(res.id);
            $categories.set(categoryId, category);
            $tags = $tags;
            $categories = $categories;
        } else {
            $otherTags.set(res.id, {
                name: res.tagName,
                ordinal: res.ordinal,
                color: res.color,
            });
            $otherTags = $otherTags;
        }
    }

    function handleConsider(
        e: CustomEvent<
            DndEvent<{
                id: number,
                tag: Tag & { categoryId: number }
            }>
        > & {
            target: any;
        }
    ) {
        const selectedTagId: number = Number(e.detail.info.id);
        const categoryId = $tags.get(selectedTagId)?.categoryId;

        if (categoryId) {
            e.detail.items.forEach(({ id, tag }, index) => {
                tag.ordinal = index;
            });

            idTags = idTags.filter(x => x.tag.categoryId !== categoryId);

            e.detail.items.forEach(item => {
                idTags.push(item);
            });
        } else {
            e.detail.items.forEach(({id, tag}, index) => {
                tag.ordinal = index;
            });
            idOtherTags = e.detail.items;
        }
    }

    async function handleFinalize(
        e: CustomEvent<
            DndEvent<{
                id: number,
                tag: Tag & { categoryId: number }
            }>
        > & {
            target: any;
        }
    ) {
        const selectedTagId: number = Number(e.detail.info.id);
        const selectedTag = e.detail.items.find(x => x.id === selectedTagId);
        const newOrdinal = e.detail.items.findIndex(x => x.id === selectedTagId);
        const oldOrdinal = selectedTag.tag.ordinal;
        await invoke("update_tag_ordinal", {
            data: { categoryTagId: selectedTagId, newOrd: newOrdinal },
        });

        if (selectedTag.tag.categoryId) {
            e.detail.items.forEach((x, idx) => {
                $tags.get(x.id).ordinal = idx;
            });
            const currCategory = $categories.get(selectedTag.tag.categoryId);
            
            $tags.set(selectedTagId, selectedTag.tag);
            $tags = $tags;
            $categories.set(selectedTag.tag.categoryId, { ...currCategory, tags: idTags.filter(x => x.tag.categoryId === selectedTag.tag.categoryId).map(x => { return x.id; }) });
            $categories = $categories;
        } else {
            idOtherTags.forEach((item, idx) => {
                $otherTags.get(item.id).ordinal = idx;
            });
            $otherTags.set(selectedTagId, { ...selectedTag.tag, ordinal: newOrdinal });
            $otherTags = $otherTags;
        }
    }

    let createTagForm = false;
    let createCategoryId: number | undefined;
    let createTagName: string = "";
    let createTagNode: HTMLInputElement;
</script>

<h2 class="h2">Tag options</h2>

{#each Array.from($categories).sort(([shit1, catA], [shit2, catB]) => {
    return catA.ordinal - catB.ordinal;
}) as [categoryId, category]}
    <p>{category.name}</p>
    {#if category.tags.length !== 0}
        <section class="flex flex-col pb-2 overflow-auto min-h-full bg-transaprent" use:dndzone={{
            items: idTags.filter(x => x.tag.categoryId === categoryId),
            flipDurationMs,
            type: `tags ${categoryId}`,
            dropTargetStyle: {
                "box-shadow": `0px 0px 0px 4px rgba(164, 190, 224, 0.2)`,
                "border-radius": "0.25rem",
            },
        }}
        on:consider={handleConsider}
        on:finalize={handleFinalize}>
            {#each idTags.filter(x => x.tag.categoryId === categoryId) as {id, tag} (id)}
                <TagSettings {tag} tagId={id} {categoryId} />
            {/each}
        </section>
    {:else}
        <button
            class="btn variant-ghost-tertiary"
            on:click={() => {
                createCategoryId = categoryId;
                createTagNode.focus();
            }}>Add new tag</button
        >
    {/if}
{:else}
    <p>There are no categories</p>
    <button class="btn varinat-fil">Create new</button>
{/each}

<hr />
<p>Other</p>
{#if $otherTags.size !== 0}
    <section class="flex flex-col pb-2 overflow-auto min-h-full bg-transaprent" use:dndzone={{
        items: idOtherTags,
        flipDurationMs,
        type: `otherTags`,
        dropTargetStyle: {
            "box-shadow": `0px 0px 0px 4px rgba(164, 190, 224, 0.2)`,
            "border-radius": "0.25rem",
        },
    }}
    on:consider={handleConsider}
    on:finalize={handleFinalize}>
        {#each idOtherTags.sort((a, b) => {
            return a.tag.ordinal - b.tag.ordinal;
        }) as {id, tag} (id)}            
            <TagSettings {tag} tagId={id} categoryId={null} />
        {/each}
    </section>
{:else}
    <button
        class="btn variant-ghost-tertiary"
        on:click={() => {
            createCategoryId = null;
            createTagNode.focus();
        }}>Add new tag</button
    >
{/if}
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
            <ListBoxItem
                bind:group={createCategoryId}
                value={null}
                name="categoryId">other - {$otherTags.size}</ListBoxItem
            >
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
