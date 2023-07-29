<script lang="ts">
    import { invoke } from "@tauri-apps/api";
    import { activities, selectedActivity } from "../../../stores";
    import {
        modalStore,
        toastStore,
        type ToastSettings,
        type ModalSettings,
    } from "@skeletonlabs/skeleton";
    import { fly } from "svelte/transition";

    let displayBody = $selectedActivity.body ?? "";
    let isEditMode = false;

    let inputBody: string = "";

    function openEdit() {
        inputBody = displayBody;
        isEditMode = true;
    }

    async function save() {
        const trimmedBody = inputBody.trim();
        if (trimmedBody.length === 0) {
            await sync();
            displayBody = "";
            isEditMode = false;
            return;
        }

        if (displayBody === trimmedBody) {
            const toast: ToastSettings = {
                message: "📝 Set the same activity body",
                hoverable: true,
                autohide: true,
                hideDismiss: true,
                timeout: 2000,
                classes: "variant-ghost-warning",
            };
            toastStore.trigger(toast);
            isEditMode = false;
            return;
        }

        await sync(trimmedBody);
        isEditMode = false;
    }

    async function sync(newBody?: string) {
        await invoke("update_activity_content", {
            data: {
                id: $selectedActivity.id,
                name: $selectedActivity.name,
                body: newBody,
            },
        });
        displayBody = newBody ?? "";
        $selectedActivity.body = newBody;
        const activity = $activities.get($selectedActivity.id);
        activity.body = $selectedActivity.body;
        $selectedActivity = $selectedActivity;
        $activities = $activities;
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
</script>

<div class="flex flex-row">
    {#if isEditMode}
        <textarea
            class="textarea p-1 m-1"
            bind:value={inputBody}
            placeholder="New activity body"
        />
        <button
            class="btn bundefinedtn-sm variant-ghost-surface m-1"
            on:click={cancel}>Cancel</button
        >
        {#if inputBody.length > 0}
            <button
                class="btn btn-sm variant-ghost-error m-1"
                transition:fly
                on:click={clear}>Clear</button
            >
        {/if}

        <button class="btn btn-sm variant-ghost-success m-1" on:click={save}
            >Save</button
        >
    {:else if displayBody.length === 0}
        <button class="btn btn-sm variant-ghost-warning m-1" on:click={openEdit}
            >Create body</button
        >
    {:else}
        <div class="flex-1 p-2">
            <p>{displayBody}</p>
        </div>
        <button class="btn btn-sm variant-ghost-warning m-1" on:click={openEdit}
            >Edit</button
        >
    {/if}
</div>
