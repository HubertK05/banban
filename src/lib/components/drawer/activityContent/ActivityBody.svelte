<script lang="ts">
    import SvelteMarkdown from "@humanspeak/svelte-markdown";
    import { invoke } from "@tauri-apps/api/core";
    import {
        type ToastSettings,
        type ModalSettings,
        TabGroup,
        Tab,
        getToastStore,
        getModalStore,
    } from "@skeletonlabs/skeleton";
    import { fly } from "svelte/transition";
    import { activitiesRune, showToast } from "../../../shared.svelte";

    const modalStore = getModalStore();
    const toastStore = getToastStore();

    interface Props {
        activityId: number;
    }

    let { activityId }: Props = $props();
    console.assert(activitiesRune[activityId] !== undefined, "Selected activity is undefined");
    const selectedActivity = $derived(activitiesRune[activityId]);
    let displayBody = $derived(selectedActivity.body ?? "");

    let isEditMode = $state(false);
    let inputBody: string = $state("");

    function openEdit() {
        inputBody = displayBody;
        isEditMode = true;
    }

    async function save() {
        const trimmedBody = inputBody.trim();
        if (trimmedBody.length === 0) {
            await sync();
            activitiesRune[activityId].body = "";
            isEditMode = false;
            return;
        }

        if (displayBody === trimmedBody) {
            showToast(toastStore, "📝 Set the same activity body");
            isEditMode = false;
            return;
        }

        await sync(trimmedBody);
        isEditMode = false;
    }

    async function sync(newBody?: string) {
        await invoke("update_activity_content", {
            data: {
                id: activityId,
                name: selectedActivity.name,
                body: newBody,
            },
        });
        activitiesRune[activityId].body = newBody ?? "";
    }

    function clear() {
        const modal: ModalSettings = {
            type: "confirm",
            title: "Clear acivity body",
            body: "Are you sure?",
            response(r: boolean) {
                if (r) {
                    inputBody = "";
                }
            },
        };
        modalStore.trigger(modal);
    }

    function cancel() {
        isEditMode = false;
    }

    let tabSet = $state(0);
</script>

{#if isEditMode}
    <TabGroup>
        <Tab bind:group={tabSet} name="edit" value={0}>Edit</Tab>
        <Tab bind:group={tabSet} name="preview" value={1}>Preview</Tab>
    </TabGroup>
    <div class="flex">
        {#if tabSet === 0}
            <textarea rows="10" class="textarea m-2" bind:value={inputBody} placeholder="New activity body"></textarea>
        {:else if tabSet === 1}
            {#if inputBody.length > 0}
                <div class="flex-1 p-2 variant-outline rounded-md">
                    <div class="prose">
                        <SvelteMarkdown source={inputBody} />
                    </div>
                </div>
            {:else}
                <span>No content to preview</span>
            {/if}
        {/if}
    </div>
{/if}
<div class="flex flex-row">
    {#if isEditMode}
        <div class="flex flex-row">
            <button class="btn btn-sm variant-ghost-success m-1" onclick={save}>Save</button>
            <button class="btn btn-sm variant-ghost-surface m-1" onclick={cancel}>Cancel</button>
            {#if inputBody.length > 0}
                <button class="btn btn-sm variant-ghost-error m-1" transition:fly onclick={clear}>Clear</button>
            {/if}
        </div>
    {:else if displayBody.length === 0}
        <button class="btn btn-sm variant-ghost-warning m-1" onclick={openEdit}>Create body</button>
    {:else}
        <div class="flex-1 p-2 variant-outline rounded-md">
            <div class="prose"><SvelteMarkdown source={displayBody} /></div>
        </div>
        <button class="btn btn-sm variant-ghost-warning m-1" onclick={openEdit}>Edit</button>
    {/if}
</div>
