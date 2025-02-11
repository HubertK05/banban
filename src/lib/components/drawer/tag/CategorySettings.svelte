<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { ListBox, ListBoxItem, type ModalComponent } from "@skeletonlabs/skeleton";
    import TagSettings from "./TagSettings.svelte";
    import DebugLabel from "../../debug/DebugLabel.svelte";
    import { dndzone, TRIGGERS } from "svelte-dnd-action";
    import TagBadge from "../../board/TagBadge.svelte";
    import { categoriesRune, categoryTagsRune, idOtherTags, idTags, otherTagsRune } from "../../../shared.svelte";
    import type { Tag } from "../../../interfaces";

    const flipDurationMs = 125;

    idTags.update();
    idOtherTags.update();

    async function createTag(tagName: string, categoryId?: number) {
        const res: {
            id: number;
            tagName: string;
            categoryId?: number;
            ordinal: number;
            color: string;
        } = await invoke("create_tag", { data: { tagName, categoryId } });

        if (categoryId !== undefined && categoryId !== null) {
            categoryTagsRune[res.id] = {
                name: res.tagName,
                ord: res.ordinal,
                color: res.color,
                categoryId,
            };

            const runeCategory = categoriesRune[categoryId];
            console.assert(runeCategory !== undefined, "Category's tag ids is undefined");
            if (runeCategory === undefined) return;
            runeCategory.tags.push(res.id);
            categoriesRune[categoryId] = runeCategory;
            console.log($state.snapshot(categoriesRune));

            idTags.update();
        } else {
            otherTagsRune[res.id] = {
                name: res.tagName,
                ord: res.ordinal,
                color: res.color,
            };

            idOtherTags.update();
        }
    }

    // TODO: remove repetitions in code, also leave one of categoryId and categoryIdx
    function handleConsider(
        e: DndEvent<{
            id: number;
            tag: Tag & { categoryId: number };
        }>,
        categoryId: number,
        categoryIdx: number,
    ) {
        e.items.forEach(({ id, tag }, index) => {
            tag.ord = index;
        });

        idTags.inner[categoryIdx] = e.items;
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

        const newOrdinal = e.items.findIndex((x) => x.id === selectedTagId);
        await invoke("update_tag_ordinal", {
            data: { categoryTagId: selectedTagId, newOrd: newOrdinal },
        });

        const reorderedTags = e.items.map((x, idx) => {
            return { id: x.id, tag: { ...x.tag, ord: idx } };
        });
        idTags.inner[categoryIdx] = reorderedTags;
        reorderedTags.forEach((tag) => {
            categoryTagsRune[tag.id] = tag.tag;
        });
        categoriesRune[categoryId] = { ...categoriesRune[categoryId], tags: reorderedTags.map((x) => x.id) };
    }

    function otherTagsConsider(e: DndEvent<{ id: number; tag: Tag }>) {
        e.items.forEach(({ id, tag }, index) => {
            tag.ord = index;
        });
        idOtherTags.inner = e.items;
    }

    async function otherTagsFinalize(e: DndEvent<{ id: number; tag: Tag }>) {
        const selectedTagId: number = Number(e.info.id);
        const selectedTag = e.items.find((x) => x.id === selectedTagId);
        console.assert(selectedTag !== undefined, "Selected tag on finalize not found");
        if (selectedTag === undefined) return;

        const newOrdinal = e.items.findIndex((x) => x.id === selectedTagId);
        await invoke("update_tag_ordinal", {
            data: { categoryTagId: selectedTagId, newOrd: newOrdinal },
        });

        e.items.forEach((tag, idx) => {
            otherTagsRune[tag.id].ord = idx;
        });
        idOtherTags.update();
    }

    let createTagForm = $state(false);
    let createCategoryId: number | undefined = $state();
    let createTagName: string = $state("");
    let createTagNode: HTMLInputElement | undefined = $state();
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
                items: idTags.inner[categoryIdx],
                flipDurationMs,
                type: `tags ${categoryId}`,
                dropTargetStyle: {
                    "box-shadow": `0px 0px 0px 4px rgba(164, 190, 224, 0.2)`,
                    "border-radius": "0.25rem",
                },
            }}
            onconsider={(e) => handleConsider(e.detail, +categoryId, categoryIdx)}
            onfinalize={async (e) => handleFinalize(e.detail, +categoryId, categoryIdx)}
        >
            {#each idTags.inner[categoryIdx] as { id, tag } (id)}
                <TagSettings {tag} tagId={id} categoryId={+categoryId} />
            {/each}
        </section>
    {:else}
        <button
            class="btn variant-ghost-tertiary"
            onclick={() => {
                createCategoryId = +categoryId;
                createTagNode?.focus();
            }}>Add new tag</button
        >
    {/if}
{:else}
    <p>There are no categories</p>
    <button class="btn varinat-fil">Create new</button>
{/each}

<hr class="m-2" />
<p>Other</p>
{#if Object.entries(otherTagsRune).length !== 0}
    <section
        class="flex flex-col pb-2 overflow-auto min-h-full bg-transaprent"
        use:dndzone={{
            items: idOtherTags.inner,
            flipDurationMs,
            type: `otherTags`,
            dropTargetStyle: {
                "box-shadow": `0px 0px 0px 4px rgba(164, 190, 224, 0.2)`,
                "border-radius": "0.25rem",
            },
        }}
        onconsider={(e) => otherTagsConsider(e.detail)}
        onfinalize={async (e) => otherTagsFinalize(e.detail)}
    >
        {#each idOtherTags.inner as { id, tag } (id)}
            <TagSettings {tag} tagId={id} categoryId={undefined} />
        {/each}
    </section>
{:else}
    <button
        class="btn variant-ghost-tertiary"
        onclick={() => {
            createCategoryId = undefined;
            createTagNode?.focus();
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
                <ListBoxItem bind:group={createCategoryId} value={+id} name="categoryId"
                    >{category.name} - {category.tags.length}</ListBoxItem
                >
            {/each}
            <ListBoxItem bind:group={createCategoryId} value={null} name="categoryId"
                >Other - {Object.entries(otherTagsRune).length}</ListBoxItem
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
        disabled={createTagName.length === 0 || createCategoryId === undefined}>Create</button
    >
</div>
