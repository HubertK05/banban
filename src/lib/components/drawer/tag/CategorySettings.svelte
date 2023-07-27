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
  import { dndzone } from "svelte-dnd-action";

    const flipDurationMs = 125;

    // $: idTags = Array.from($tags).map(([tagId, tag]) => {
    //     return {
    //         id: tagId,
    //         tag
    //     }
    // }).sort((a, b) => { return a.tag.ordinal - b.tag.ordinal });

    $: idTags = Array.from($tags).map(([tagId, tag]) => {
        return [tag.categoryId, {
            id: tagId,
            tag
        }]
    }).reduce((acc, entry) => {
        if (!acc.has(entry[0])) {
            acc.set(entry[0], [entry[1]]);
        } else {
            let arr = acc.get(entry[0]);
            arr.push(entry[1]);
            arr.sort((a, b) => {
                return a.tag.ordinal - b.tag.ordinal;
            })
            acc.set(entry[0], arr);
        }
        return acc;
    }, new Map());

    async function createTag(tagName: string, categoryId?: number) {
        const res: {
            id: number;
            tagName: string;
            categoryId?: number;
            ordinal: number;
            color: string;
        } = await invoke("create_tag", { data: { tagName, categoryId } });
        console.debug(res);

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
                tag: Tag & { categoryId: number },
                ordinal: number
            }>
        > & {
            target: any;
        }
    ) {
        const selectedTagId: number = Number(e.detail.info.id);
        const categoryId = e.detail.items.find(x => x.id = selectedTagId).tag.categoryId;
        
        // console.log("before change");
        // idTags.get(categoryId).forEach(item => {
        //     console.log(item);
        // })
        // let arr = 
        e.detail.items.forEach(({ id, tag }, index) => {
            tag.ordinal = index;
        });
        idTags.set(categoryId, e.detail.items);
        // idTags = idTags;
        // console.log("after change");
        // idTags.get(categoryId).forEach(item => {
        //     console.log(item);
        // })
        console.log("event detail items after");
        idTags.get(1).forEach(item => {
            console.log(item);
        })
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
        const selectedTag = e.detail.items.find(x => x.id = selectedTagId);
        const newOrdinal = e.detail.items.findIndex(x => x.id = selectedTagId);
        const oldOrdinal = selectedTag.tag.ordinal;
        await invoke("update_tag_ordinal", {
            data: { categoryTagId: selectedTagId, newOrd: newOrdinal },
        });

        if (selectedTag.tag.categoryId) {
            $tags.set(selectedTagId, selectedTag.tag);
            const orderedTags: Array<number> = $categories.get(selectedTag.tag.categoryId).tags;
            // orderedTags.splice(selectedTag.tag.ordinal, 1);
            // orderedTags.splice(newOrd, 0, tagId);
            orderedTags.forEach((x, idx) => {
                $tags.get(x).ordinal = idx;
            });

            const currCategory = $categories.get(selectedTag.tag.categoryId);
            $categories.set(selectedTag.tag.categoryId, { ...currCategory, tags: orderedTags });
            $tags = $tags;
            $categories = $categories;
        } else {
            // console.debug("previous state: ", $nonCategoryTags);
            // let a: Array<[number, Tag]> = Array.from($otherTags).sort(
            //     ([a, tagA], [b, tagB]) => {
            //         return tagA.ordinal - tagB.ordinal;
            //     }
            // );
            // console.debug("array from previous state: ", a);
            // a.forEach((x) => {
            //     console.debug(x);
            // });
            // let modifiedTag = a[oldOrdinal];
            // a.splice(tag.ordinal, 1);
            // a.splice(newOrd, 0, modifiedTag);
            // console.debug("array after splicing: ");
            // a.forEach((x) => {
            //     console.debug(x);
            // });
            Array.from($otherTags).forEach(([currTagId, currTag], idx) => {
                $otherTags.get(currTagId).ordinal = idx;
            });
            $otherTags.set(selectedTagId, { ...selectedTag.tag, ordinal: newOrdinal });
            $otherTags = $otherTags;
            // console.debug("array after changing indices: ", a);
            // console.debug("new state: ", $nonCategoryTags);
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
        <section use:dndzone={{
            items: idTags.get(categoryId),
            flipDurationMs,
            type: `tags`,
            // dropTargetStyle: {
            //     "box-shadow": `0px 0px 0px 4px rgba(164, 190, 224, 0.2)`,
            //     "border-radius": "0.25rem",
            // },
        }}
        on:consider={handleConsider}
        on:finalize={handleFinalize}>
            {#each idTags.get(categoryId) as {id: tagId, tag} (tagId)}
                <!-- {@const tag = $tags.get(id)} -->
                <DebugLabel text={"ID: " + tagId} />
                <DebugLabel text={"ORD: " + tag.ordinal} />
                <TagSettings {tag} {tagId} {categoryId} />
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
{#each Array.from($otherTags).sort((a, b) => {
    return a[1].ordinal - b[1].ordinal;
}) as tagData}
    <DebugLabel text={"ID: " + tagData[0]} />
    <DebugLabel text={"ORD: " + tagData[1].ordinal} />
    <TagSettings tag={tagData[1]} tagId={tagData[0]} categoryId={null} />
{:else}
    <button
        class="btn variant-ghost-tertiary"
        on:click={() => {
            createCategoryId = null;
            createTagNode.focus();
        }}>Add new tag</button
    >
{/each}
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
