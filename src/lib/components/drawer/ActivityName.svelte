<script lang="ts">
    import { invoke } from "@tauri-apps/api";
    import { activities, selectedActivity } from "../../stores";
    import {
        modalStore,
        toastStore,
        type ToastSettings,
    } from "@skeletonlabs/skeleton";

    let displayName = $selectedActivity.name;
    let isEditMode = false;

    let inputName: string;

    function openEdit() {
        inputName = displayName;
        isEditMode = true;
    }

    async function save() {
        const trimmed = inputName.trim();
        if (trimmed.length === 0) {
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
        displayName = trimmed;
        invoke("update_activity_content", {
            data: {
                id: $selectedActivity.id,
                name: trimmed,
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

<h2 class="h2">Content</h2>
<div class="flex flex-col place-content-center">
    <div class="flex flex-row">
        {#if isEditMode}
            <input
                type="text"
                class="input p-1 m-1"
                bind:value={inputName}
                on:keypress={handleKeyPress}
            />
            <button
                class="btn btn-sm variant-ghost-surface m-1"
                on:click={cancel}>Cancel</button
            >
            <button class="btn btn-sm variant-ghost-success m-1" on:click={save}
                >Save</button
            >
        {:else}
            <div class="flex-1 p-2">
                <div>Name: {displayName}</div>
            </div>

            <button
                class="btn btn-sm variant-ghost-warning m-1"
                on:click={openEdit}>Edit</button
            >
        {/if}
    </div>
</div>
