<script lang="ts">
    import { run } from 'svelte/legacy';

    import { invoke } from "@tauri-apps/api/core";
    import {
        categories,
        columns,
        otherTags,
        tags,
    } from "../../../stores";
    import {
        ListBox,
        ListBoxItem,
        type ModalComponent,
    } from "@skeletonlabs/skeleton";
    import TagSettings from "./TagSettings.svelte";
    import DebugLabel from "../../debug/DebugLabel.svelte";
    import { dndzone, TRIGGERS } from "svelte-dnd-action";
    import TagBadge from "../../board/TagBadge.svelte";
  import { categoriesRune, categoryTagsRune } from '../../../shared.svelte';
  import type { Tag } from '../../../interfaces/main';

    const flipDurationMs = 125;

    let idTags: { id: number, tag: Tag & { categoryId: number } }[][] = $state([]);
    updateIdTags();
    
    // we need both reactivity and functioning drag and drop at once.
    // $derived() rune on idTags confuses the drag and drop library.
    // $state() without explicit update is non-reactive.
    function updateIdTags() {
        idTags = Object.entries(categoriesRune).map(([categoryId, category]) => {
            return category.tags.map(tagId => {
                const tag = categoryTagsRune[tagId];
                console.assert(tag !== undefined, "Category tag not found");
                return {id: tagId, tag: {...tag!, categoryId: +categoryId}};
            }).sort((idTagA, idTagB) => {
                return idTagA.tag.ord - idTagB.tag.ord;
            });
        })
    }

    let idOtherTags;
    run(() => {
        idOtherTags = Array.from($otherTags)
            .map(([tagId, tag]) => {
                return {
                    id: tagId,
                    tag,
                };
            })
            .sort((a, b) => {
                return a.tag.ordinal - b.tag.ordinal;
            });
    });

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

            categoryTagsRune[res.id] = {
                name: res.tagName,
                ord: res.ordinal,
                color: res.color,
            };

            const runeCategory = categoriesRune[categoryId]
            console.assert(runeCategory !== undefined, "Category's tag ids is undefined")
            if (runeCategory === undefined) return;
            runeCategory.tags.push(res.id)
            categoriesRune[categoryId] = runeCategory;
            console.log($state.snapshot(categoriesRune));

            // TODO: creating new tags sometimes results in non-reactive update of idTags.
            updateIdTags();
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
        e: DndEvent<{
            id: number;
            tag: Tag & { categoryId: number };
        }>,
        categoryId: number,
        categoryIdx: number,
    ) {
        if (categoryId) {
            e.items.forEach(({ id, tag }, index) => {
                tag.ord = index;
            });

            idTags[categoryIdx] = e.items
        } else {
            e.items.forEach(({ id, tag }, index) => {
                tag.ord = index;
            });
            idOtherTags = e.items;
        }
    }

    async function handleFinalize(
        e: DndEvent<{
            id: number;
            tag: Tag & { categoryId: number };
        }>,
        categoryId: number,
        categoryIdx: number,
    ) {
        const selectedTagId: number = Number(e.info.id);
        const selectedTag = e.items.find((x) => x.id === selectedTagId);
        console.assert(selectedTag !== undefined, "Selected tag on finalize not found");
        if (selectedTag === undefined) return;

        const newOrdinal = e.items.findIndex(
            (x) => x.id === selectedTagId
        );
        await invoke("update_tag_ordinal", {
            data: { categoryTagId: selectedTagId, newOrd: newOrdinal },
        });

        if (categoryId) {
            e.items.forEach((x, idx) => {
                $tags.get(x.id).ordinal = idx;
            });
            const currCategory = $categories.get(categoryId);
            console.assert(currCategory !== undefined, "Current category on finalize not found");
            if (currCategory === undefined) return;

            $tags.set(selectedTagId, {...selectedTag.tag, ordinal: selectedTag.tag.ord, color: selectedTag.tag.color!});
            $tags = $tags;
            $categories.set(categoryId, {
                ...currCategory,
                tags: idTags[categoryIdx].map(x => x.id),
            });
            $categories = $categories;

            const reorderedTags = e.items.map((x, idx) => {
                return {id: x.id, tag: {...x.tag, ord: idx}};
            });
            idTags[categoryIdx] = reorderedTags;
            reorderedTags.forEach(tag => {
                categoryTagsRune[tag.id] = tag.tag;
            });
            categoriesRune[categoryId] = {...currCategory, ord: currCategory.ordinal, tags: reorderedTags.map(x => x.id) }
        } else {
            idOtherTags.forEach((item, idx) => {
                $otherTags.get(item.id).ordinal = idx;
            });
            $otherTags.set(selectedTagId, {
                ...selectedTag.tag,
                ordinal: newOrdinal,
            });
            $otherTags = $otherTags;
        }
    }

    let createTagForm = $state(false);
    let createCategoryId: number | undefined = $state();
    let createTagName: string = $state("");
    let createTagNode: HTMLInputElement = $state();
</script>

<h2 class="h2">Tag options</h2>

{#each Object.entries(categoriesRune).sort(([_1, catA], [_2, catB]) => {
    return catA.ord - catB.ord;
}) as [categoryId, category], categoryIdx}
    <p>{category.name}</p>
    {#if category.tags.length !== 0}
        <section
            class="flex flex-col pb-2 overflow-auto min-h-full bg-transaprent"
            use:dndzone={{
                items: idTags[categoryIdx],
                flipDurationMs,
                type: `tags ${categoryId}`,
                dropTargetStyle: {
                    "box-shadow": `0px 0px 0px 4px rgba(164, 190, 224, 0.2)`,
                    "border-radius": "0.25rem",
                },
            }}
            onconsider={e => handleConsider(e.detail, +categoryId, categoryIdx)}
            onfinalize={async e => handleFinalize(e.detail, +categoryId, categoryIdx)}
        >
            {#each idTags[categoryIdx] as { id, tag } (id)}
                <TagSettings {tag} tagId={id} categoryId={+categoryId} />
            {/each}
        </section>
    {:else}
        <button
            class="btn variant-ghost-tertiary"
            onclick={() => {
                createCategoryId = +categoryId;
                createTagNode.focus();
            }}>Add new tag</button
        >
    {/if}
{:else}
    <p>There are no categories</p>
    <button class="btn varinat-fil">Create new</button>
{/each}

<hr class="m-2" />
<p>Other</p>
{#if $otherTags.size !== 0}
    <section
        class="flex flex-col pb-2 overflow-auto min-h-full bg-transaprent"
        use:dndzone={{
            items: idOtherTags,
            flipDurationMs,
            type: `otherTags`,
            dropTargetStyle: {
                "box-shadow": `0px 0px 0px 4px rgba(164, 190, 224, 0.2)`,
                "border-radius": "0.25rem",
            },
        }}
        onconsider={handleConsider}
        onfinalize={handleFinalize}
    >
        {#each idOtherTags.sort((a, b) => {
            return a.tag.ordinal - b.tag.ordinal;
        }) as { id, tag } (id)}
            <TagSettings {tag} tagId={id} categoryId={undefined} />
        {/each}
    </section>
{:else}
    <button
        class="btn variant-ghost-tertiary"
        onclick={() => {
            createCategoryId = undefined;
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
            {#each Object.entries(categoriesRune) as [id, category]}
                <ListBoxItem
                    bind:group={createCategoryId}
                    value={+id}
                    name="categoryId"
                    >{category.name} - {category.tags.length}</ListBoxItem
                >
            {/each}
            <ListBoxItem
                bind:group={createCategoryId}
                value={null}
                name="categoryId">Other - {$otherTags.size}</ListBoxItem
            >
        </ListBox>
    </div>
    <button
        onclick={async () => {
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
