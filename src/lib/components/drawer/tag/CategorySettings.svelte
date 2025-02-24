<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { getModalStore, ListBox, ListBoxItem, type ModalComponent, type ModalSettings } from "@skeletonlabs/skeleton";
    import TagSettings from "./TagSettings.svelte";
    import { dndzone } from "svelte-dnd-action";
    import {
        activitiesRune,
        categoriesRune,
        categoryTagsRune,
        idOtherTags,
        idTags,
        otherTagsRune,
    } from "../../../shared.svelte";
    import type { Tag } from "../../../interfaces";
    import { flip } from "svelte/animate";

    const flipDurationMs = 125;
    let editableCategory: { id: null } | { id: number; name: string } = $state({ id: null });
    const modalStore = getModalStore();

    idTags.update();
    idOtherTags.update();

    async function deleteCategory(id: number) {
        await invoke("delete_category", { id });
        let tagsToFilterOut = categoriesRune[id].tags;
        delete categoriesRune[id];

        tagsToFilterOut.forEach((tagId) => {
            delete categoryTagsRune[+tagId];
        });

        Object.entries(activitiesRune).forEach(([activityId, activityContent]) => {
            activityContent.tags = activityContent.tags.filter((tag) => !tagsToFilterOut.includes(tag));
            activitiesRune[+activityId] = activityContent;
        });
    }

    async function renameCategory(id: number) {
        console.assert(editableCategory.id !== null, "There is no editable category when trying to rename it");
        if (editableCategory.id === null) return;
        await invoke("update_category_name", { data: { id, name: editableCategory.name } });
        const runeCategory = categoriesRune[id];
        runeCategory.name = editableCategory.name;
        categoriesRune[id] = runeCategory;
        editableCategory = { id: null };
    }

    async function handleCreateTagClick(tagName: string, categoryId: number | null | undefined) {
        if (categoryId === undefined) return;
        createTagForm = false;
        await createTag(tagName, categoryId);
        createTagName = "";
        createCategoryId = undefined;
    }

    function showRemoveModal(categoryId: number) {
        const modal: ModalSettings = {
            type: "confirm",
            title: `Remove '${categoriesRune[categoryId].name}'`,
            body: "Are you sure?",

            response: async (r: boolean) => {
                if (r) {
                    await deleteCategory(categoryId);
                }
            },
        };
        modalStore.trigger(modal);
    }


    async function createTag(tagName: string, categoryId: number | null) {
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

    function handleConsider(
        e: DndEvent<{
            id: number;
            tag: Tag & { categoryId: number };
        }>,
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
    let createCategoryId: number | null | undefined = $state();
    let createTagName: string = $state("");
    let createTagNode: HTMLInputElement | undefined = $state();
</script>

<h2 class="h2">Tag options</h2>

{#each Object.entries(categoriesRune).sort(([_1, catA], [_2, catB]) => {
    return catA.ord - catB.ord;
}) as [categoryId, category], categoryIdx}
    <div class="flex flex-row mt-2 place-content-between align-center p-1 rounded-md">
        {#if editableCategory.id === +categoryId}
            <input
                class="input self-center"
                bind:value={editableCategory.name}
                onkeypress={async (e) => {
                    if (e.key === "Enter") {
                        await renameCategory(+categoryId);
                    }
                }}
            />
            <div class="flex flex-row">
                <!-- svelte-ignore a11y_consider_explicit_label -->
                <button
                    class="flex items-center justify-center w-10 h-10 ml-auto rounded hover:bg-error-hover-token"
                    onclick={async () => {
                        await renameCategory(+categoryId);
                    }}
                >
                    <svg xmlns="http://www.w3.org/2000/svg" height="1em" viewBox="0 0 448 512">
                        <path
                            d="M438.6 105.4c12.5 12.5 12.5 32.8 0 45.3l-256 256c-12.5 12.5-32.8 12.5-45.3 0l-128-128c-12.5-12.5-12.5-32.8 0-45.3s32.8-12.5 45.3 0L160 338.7 393.4 105.4c12.5-12.5 32.8-12.5 45.3 0z"
                        />
                    </svg>
                </button>
                <!-- svelte-ignore a11y_consider_explicit_label -->
                <button
                    class="flex items-center justify-center w-10 h-10 ml-auto rounded hover:bg-error-hover-token"
                    onclick={() => {
                        editableCategory = { id: null };
                    }}
                >
                    <svg xmlns="http://www.w3.org/2000/svg" height="1em" viewBox="0 0 384 512">
                        <path
                            d="M342.6 150.6c12.5-12.5 12.5-32.8 0-45.3s-32.8-12.5-45.3 0L192 210.7 86.6 105.4c-12.5-12.5-32.8-12.5-45.3 0s-12.5 32.8 0 45.3L146.7 256 41.4 361.4c-12.5 12.5-12.5 32.8 0 45.3s32.8 12.5 45.3 0L192 301.3 297.4 406.6c12.5 12.5 32.8 12.5 45.3 0s12.5-32.8 0-45.3L237.3 256 342.6 150.6z"
                        />
                    </svg>
                </button>
            </div>
        {:else}
            <span class="self-center">{category.name}</span>
            <div class="flex flex-row">
                <!-- svelte-ignore a11y_consider_explicit_label -->
                <button
                    class="flex items-center justify-center w-10 h-10 ml-auto rounded hover:bg-error-hover-token"
                    onclick={() => {
                        editableCategory = { id: +categoryId, name: category.name };
                    }}
                >
                    <svg xmlns="http://www.w3.org/2000/svg" height="1em" viewBox="0 0 512 512">
                        <path
                            d="M410.3 231l11.3-11.3-33.9-33.9-62.1-62.1L291.7 89.8l-11.3 11.3-22.6 22.6L58.6 322.9c-10.4 10.4-18 23.3-22.2 37.4L1 480.7c-2.5 8.4-.2 17.5 6.1 23.7s15.3 8.5 23.7 6.1l120.3-35.4c14.1-4.2 27-11.8 37.4-22.2L387.7 253.7 410.3 231zM160 399.4l-9.1 22.7c-4 3.1-8.5 5.4-13.3 6.9L59.4 452l23-78.1c1.4-4.9 3.8-9.4 6.9-13.3l22.7-9.1 0 32c0 8.8 7.2 16 16 16l32 0zM362.7 18.7L348.3 33.2 325.7 55.8 314.3 67.1l33.9 33.9 62.1 62.1 33.9 33.9 11.3-11.3 22.6-22.6 14.5-14.5c25-25 25-65.5 0-90.5L453.3 18.7c-25-25-65.5-25-90.5 0zm-47.4 168l-144 144c-6.2 6.2-16.4 6.2-22.6 0s-6.2-16.4 0-22.6l144-144c6.2-6.2 16.4-6.2 22.6 0s6.2 16.4 0 22.6z"
                        />
                    </svg>
                </button>
                <!-- svelte-ignore a11y_consider_explicit_label -->
                <button
                    class="flex items-center justify-center w-10 h-10 ml-auto rounded hover:bg-error-hover-token"
                    onclick={() => {
                        showRemoveModal(+categoryId);
                    }}
                >
                    <svg xmlns="http://www.w3.org/2000/svg" height="1em" viewBox="0 0 448 512">
                        <path
                            d="M135.2 17.7C140.6 6.8 151.7 0 163.8 0H284.2c12.1 0 23.2 6.8 28.6 17.7L320 32h96c17.7 0 32 14.3 32 32s-14.3 32-32 32H32C14.3 96 0 81.7 0 64S14.3 32 32 32h96l7.2-14.3zM32 128H416V448c0 35.3-28.7 64-64 64H96c-35.3 0-64-28.7-64-64V128zm96 64c-8.8 0-16 7.2-16 16V432c0 8.8 7.2 16 16 16s16-7.2 16-16V208c0-8.8-7.2-16-16-16zm96 0c-8.8 0-16 7.2-16 16V432c0 8.8 7.2 16 16 16s16-7.2 16-16V208c0-8.8-7.2-16-16-16zm96 0c-8.8 0-16 7.2-16 16V432c0 8.8 7.2 16 16 16s16-7.2 16-16V208c0-8.8-7.2-16-16-16z"
                        /></svg
                    >
                </button>
            </div>
        {/if}
    </div>
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
            onconsider={(e) => handleConsider(e.detail, categoryIdx)}
            onfinalize={async (e) => handleFinalize(e.detail, +categoryId, categoryIdx)}
        >
            {#each idTags.inner[categoryIdx] as { id, tag } (id)}
                <div animate:flip={{ duration: flipDurationMs }}>
                    <TagSettings {tag} tagId={id} categoryId={+categoryId} />
                </div>
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
            <div animate:flip={{ duration: flipDurationMs }}>
                <TagSettings {tag} tagId={id} categoryId={undefined} />
            </div>
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
            onkeypress={async (e) => {
                // TODO: inform user to select category if undefined
                if (e.key === "Enter") {
                    await handleCreateTagClick(createTagName, createCategoryId);
                }
            }}
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
        onclick={async () => await handleCreateTagClick(createTagName, createCategoryId)}
        class="btn variant-filled-primary"
        disabled={createTagName.length === 0 || createCategoryId === undefined}>Create</button
    >
</div>
