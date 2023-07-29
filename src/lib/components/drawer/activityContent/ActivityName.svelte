<script lang="ts">
    import { invoke } from "@tauri-apps/api";
    import { activities, selectedActivity } from "../../../stores";
    import {
        modalStore,
        toastStore,
        type ToastSettings,
    } from "@skeletonlabs/skeleton";
    import { tick } from "svelte";

    let displayName = $selectedActivity.name;
    let isEditMode = false;

    let inputName: string;
    let inputNode: HTMLInputElement;

    async function openEdit() {
        inputName = displayName;
        isEditMode = true;
        await tick();
        inputNode.focus();
    }

    async function save() {
        const trimmedName = inputName.trim();
        if (displayName === trimmedName) {
            const toast: ToastSettings = {
                message: "📝 Set the same activity name",
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
        if (trimmedName.length === 0) {
            const toast: ToastSettings = {
                message: "⚠️ Activity name cannot be blank",
                hoverable: true,
                autohide: true,
                hideDismiss: true,
                timeout: 2000,
                classes: "variant-ghost-warning",
            };
            toastStore.trigger(toast);
            return;
        }
        displayName = trimmedName;
        invoke("update_activity_content", {
            data: {
                id: $selectedActivity.id,
                name: trimmedName,
                body: $selectedActivity.body,
            },
        });
        $selectedActivity.name = displayName;
        const activity = $activities.get($selectedActivity.id);
        activity.name = displayName;
        $selectedActivity = $selectedActivity;
        $activities = $activities;
        isEditMode = false;
    }

    function cancel() {
        isEditMode = false;
    }

    async function handleKeyPress(e: KeyboardEvent) {
        if (e.key === "Enter") {
            await save();
        }
    }
</script>

<div class="flex flex-row">
    {#if isEditMode}
        <input
            type="text"
            class="input p-1 m-1"
            bind:value={inputName}
            on:keypress={handleKeyPress}
            bind:this={inputNode}
        />
        <button class="btn btn-sm variant-ghost-surface m-1" on:click={cancel}
            >Cancel</button
        >
        <button class="btn btn-sm variant-ghost-success m-1" on:click={save}
            >Save</button
        >
    {:else}
        <div class="flex-1 p-2">
            <div>Name: {displayName}</div>
        </div>

        <button class="btn btn-sm variant-ghost-warning m-1" on:click={openEdit}
            >Edit</button
        >
    {/if}
</div>
