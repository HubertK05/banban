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
    import { faCheck, faEraser, faEye, faPen, faPenToSquare, faXmark } from "@fortawesome/free-solid-svg-icons";
    import Fa from "svelte-fa";

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
        <Tab bind:group={tabSet} name="edit" value={0}><Fa icon={faPen}/></Tab>
        <Tab bind:group={tabSet} name="preview" value={1}><Fa icon={faEye}/></Tab>
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
        <div class="flex flex-row mb-6">
            <button class="btn btn-lg hover:bg-success-hover-token m-1" onclick={save}><Fa icon={faCheck}/></button>
            <button class="btn btn-lg hover:bg-error-hover-token m-1" onclick={cancel}><Fa icon={faXmark}/></button>
            {#if inputBody.length > 0}
                <button class="btn btn-lg hover:bg-warning-hover-token m-1" transition:fly onclick={clear}><Fa icon={faEraser}/></button>
            {/if}
            <hr />
        </div>
    {:else if displayBody.length === 0}
        <button class="btn btn-sm variant-filled m-1" onclick={openEdit}>Create body <Fa icon={faPenToSquare} class="ml-2"/></button>
    {:else}
        <div class="flex-1 self-center p-2 variant-outline rounded-md">
            <div class="prose"><SvelteMarkdown source={displayBody} /></div>
        </div>
        <button class="btn btn-sm variant-filled m-1 min-h-10 min-w-28" onclick={openEdit}>Edit body<Fa icon={faPen} class="ml-2"/></button>
    {/if}
</div>
