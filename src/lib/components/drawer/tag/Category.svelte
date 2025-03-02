<script lang="ts">
    import { faCheck, faPencilAlt, faPlus, faTrashAlt, faXmark } from "@fortawesome/free-solid-svg-icons";
    import { getModalStore, getToastStore, type ModalSettings } from "@skeletonlabs/skeleton";
    import Fa from "svelte-fa";
    import {
        activitiesRune,
        categoriesRune,
        categoryTagsRune,
        idOtherTags,
        idTags,
        otherTagsRune,
        showToast,
    } from "../../../shared.svelte";
    import { invoke } from "@tauri-apps/api/core";
    import { dndzone } from "svelte-dnd-action";
    import { flip } from "svelte/animate";
    import TagSettings from "./TagSettings.svelte";
    import type { Tag } from "../../../interfaces";

    interface Props {
        categoryData?: {
            id: number;
            idx: number;
            name: string;
            tags: number[];
        };
    }

    const { categoryData }: Props = $props();
    const id = categoryData?.id;
    const name = categoryData?.name;

    const flipDurationMs = 125;
    let newCategoryName: string | null = $state(null);
    let modalStore = getModalStore();
    let toastStore = getToastStore();

    let createTagForm = $state(false);
    let createCategoryId: number | null | undefined = $state();
    let createTagName: string = $state("");
    let createTagNode: HTMLInputElement | undefined = $state();

    async function handleRenameCategory() {
        if (id !== undefined) {
            console.assert(newCategoryName !== null, "New category name cannot be null");
            if (!newCategoryName) {
                showToast(toastStore, "⚠️ Category name cannot be blank");
                return;
            }

            await renameCategory(id, newCategoryName);
        }
    }

    async function renameCategory(id: number, name: string) {
        await invoke("update_category_name", { data: { id, name } });
        const runeCategory = categoriesRune[id];
        runeCategory.name = name;
        categoriesRune[id] = runeCategory;
        newCategoryName = null;
    }

    function showRemoveModal() {
        const modal: ModalSettings = {
            type: "confirm",
            title: `Remove '${name}'`,
            body: "Are you sure?",

            response: async (r: boolean) => {
                if (r) {
                    await deleteCategory();
                }
            },
        };
        modalStore.trigger(modal);
    }

    async function deleteCategory() {
        await invoke("delete_category", { id });
        if (id !== undefined) {
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

    async function handleCreateTagClick(tagName: string) {
        createTagForm = false;
        await createTag(tagName);
        createTagName = "";
        createCategoryId = undefined;
    }

    async function createTag(tagName: string) {
        const res: {
            id: number;
            tagName: string;
            categoryId?: number;
            ordinal: number;
            color: string;
        } = await invoke("create_tag", { data: { tagName, categoryId: id } });

        if (id !== undefined) {
            categoryTagsRune[res.id] = {
                name: res.tagName,
                ord: res.ordinal,
                color: res.color,
                categoryId: id,
            };

            const runeCategory = categoriesRune[id];
            console.assert(runeCategory !== undefined, "Category's tag ids is undefined");
            if (runeCategory === undefined) return;
            runeCategory.tags.push(res.id);
            categoriesRune[id] = runeCategory;

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
</script>

{#if categoryData !== undefined}
    <div class="flex flex-row mt-2 place-content-between align-center p-1 rounded-md">
        {#if newCategoryName !== null}
            <input
                class="input h-10 p-2 self-center"
                bind:value={newCategoryName}
                onkeypress={async (e) => {
                    if (e.key === "Enter") {
                        await handleRenameCategory();
                    }
                }}
            />
            <div class="flex flex-row">
                <button
                    aria-label="Confirm renaming category"
                    class="flex items-center justify-center w-10 h-10 ml-auto rounded hover:bg-success-hover-token"
                    onclick={async () => {
                        await handleRenameCategory();
                    }}
                >
                    <Fa icon={faCheck} />
                </button>
                <button
                    aria-label="Cancel renaming category"
                    class="flex items-center justify-center w-10 h-10 ml-auto rounded hover:bg-error-hover-token"
                    onclick={() => {
                        newCategoryName = null;
                    }}
                >
                    <Fa icon={faXmark} />
                </button>
            </div>
        {:else}
            <span class="self-center">{name}</span>
            <div class="flex flex-row">
                <button
                    aria-label="Rename category {name}"
                    class="flex items-center justify-center w-10 h-10 ml-auto rounded hover:bg-warning-hover-token"
                    onclick={() => {
                        newCategoryName = categoryData.name;
                    }}
                >
                    <Fa icon={faPencilAlt} />
                </button>
                <button
                    aria-label="Remove category {name}"
                    class="flex items-center justify-center w-10 h-10 ml-auto rounded hover:bg-error-hover-token"
                    onclick={() => {
                        showRemoveModal();
                    }}
                >
                    <Fa icon={faTrashAlt} />
                </button>
            </div>
        {/if}
    </div>
    {#if categoryData.tags.length !== 0}
        <section
            class="flex flex-col pb-2 overflow-auto min-h-full bg-transaprent"
            use:dndzone={{
                items: idTags.inner[categoryData.idx],
                flipDurationMs,
                type: `tags ${id}`,
                dropTargetStyle: {
                    "box-shadow": `0px 0px 0px 4px rgba(164, 190, 224, 0.2)`,
                    "border-radius": "0.25rem",
                },
            }}
            onconsider={(e) => handleConsider(e.detail, categoryData.idx)}
            onfinalize={async (e) => handleFinalize(e.detail, categoryData.id, categoryData.idx)}
        >
            {#each idTags.inner[categoryData.idx] as { id: tagId, tag } (tagId)}
                <div animate:flip={{ duration: flipDurationMs }}>
                    <TagSettings {tag} {tagId} categoryId={id} />
                </div>
            {/each}
        </section>
    {:else}
        <p>No tags</p>
    {/if}
{:else}
    <p class="flex items-center h-10">Other</p>
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
        <p>No tags</p>
    {/if}
{/if}
<div class="flex flex-row w-72">
    <input
        type="text"
        class="input p-2"
        bind:this={createTagNode}
        bind:value={createTagName}
        placeholder="New tag name"
        onkeypress={async (e) => {
            if (e.key === "Enter") {
                await handleCreateTagClick(createTagName);
            }
        }}
    />
    <button
        onclick={async () => {
            if (createTagName === "") {
                createTagNode?.focus();
            } else {
                await handleCreateTagClick(createTagName);
            }
        }}
        class="flex items-center justify-center w-10 h-10 ml-auto rounded hover:bg-success-hover-token"
        ><Fa icon={faPlus} /></button
    >
</div>
