<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import {
        getModalStore,
        getToastStore,
        ListBox,
        ListBoxItem,
        type ModalComponent,
        type ModalSettings,
    } from "@skeletonlabs/skeleton";
    import TagSettings from "./TagSettings.svelte";
    import { dndzone } from "svelte-dnd-action";
    import {
        activitiesRune,
        categoriesRune,
        categoryTagsRune,
        idOtherTags,
        idTags,
        otherTagsRune,
        showToast,
    } from "../../../shared.svelte";
    import type { Tag } from "../../../interfaces";
    import { flip } from "svelte/animate";
    import Fa from "svelte-fa";
    import {
        faCheck,
        faEdit,
        faPen,
        faPencilAlt,
        faPenClip,
        faTrashAlt,
        faXmark,
    } from "@fortawesome/free-solid-svg-icons";

    const flipDurationMs = 125;
    let editableCategory: { id: null } | { id: number; name: string } = $state({ id: null });
    const modalStore = getModalStore();
    const toastStore = getToastStore();

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

    async function handleRenameCategory(id: number) {
        console.assert(editableCategory.id !== null, "There is no editable category when trying to rename it");
        if (editableCategory.id === null) return;

        if (editableCategory.name.trim() === "") {
            showToast(toastStore, "⚠️ Category name cannot be blank");
            return;
        }

        await renameCategory(id, editableCategory.name);
    }

    async function renameCategory(id: number, name: string) {
        await invoke("update_category_name", { data: { id, name } });
        const runeCategory = categoriesRune[id];
        runeCategory.name = name;
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
                class="input h-10 p-2 self-center"
                bind:value={editableCategory.name}
                onkeypress={async (e) => {
                    if (e.key === "Enter") {
                        await handleRenameCategory(+categoryId);
                    }
                }}
            />
            <div class="flex flex-row">
                <button
                    aria-label="Confirm renaming category"
                    class="flex items-center justify-center w-10 h-10 ml-auto rounded hover:bg-success-hover-token"
                    onclick={async () => {
                        await handleRenameCategory(+categoryId);
                    }}
                >
                    <Fa icon={faCheck} />
                </button>
                <button
                    aria-label="Cancel renaming category"
                    class="flex items-center justify-center w-10 h-10 ml-auto rounded hover:bg-error-hover-token"
                    onclick={() => {
                        editableCategory = { id: null };
                    }}
                >
                    <Fa icon={faXmark} />
                </button>
            </div>
        {:else}
            <span class="self-center">{category.name}</span>
            <div class="flex flex-row">
                <button
                    aria-label="Rename category {category.name}"
                    class="flex items-center justify-center w-10 h-10 ml-auto rounded hover:bg-warning-hover-token"
                    onclick={() => {
                        editableCategory = { id: +categoryId, name: category.name };
                    }}
                >
                    <Fa icon={faPencilAlt} />
                </button>
                <button
                    aria-label="Remove category {category.name}"
                    class="flex items-center justify-center w-10 h-10 ml-auto rounded hover:bg-error-hover-token"
                    onclick={() => {
                        showRemoveModal(+categoryId);
                    }}
                >
                    <Fa icon={faTrashAlt} />
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
