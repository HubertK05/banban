<script lang="ts">
    import { invoke } from "@tauri-apps/api";
    import { activities, selectedActivity } from "../../../stores";
    import {
        modalStore,
        toastStore,
        type ToastSettings,
    } from "@skeletonlabs/skeleton";

    let displayBody = $selectedActivity.body;
    let isEditMode = false;

    let inputBody: string;

    function openEdit() {
        inputBody = displayBody;
        isEditMode = true;
    }

    async function save() {
        const trimmedBody = inputBody.trim();
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
        if (trimmedBody.length === 0) {
            const toast: ToastSettings = {
                message: "⚠️ Activity body cannot be blank",
                hoverable: true,
                autohide: true,
                hideDismiss: true,
                timeout: 2000,
                classes: "variant-ghost-warning",
            };
            toastStore.trigger(toast);
            return;
        }
        displayBody = trimmedBody;
        invoke("update_activity_content", {
            data: {
                id: $selectedActivity.id,
                name: $selectedActivity.name,
                body: trimmedBody,
            },
        });
        $selectedActivity.body = displayBody;
        const activity = $activities.get($selectedActivity.id);
        activity.body = displayBody;
        $selectedActivity = $selectedActivity;
        $activities = $activities;
        isEditMode = false;
    }

    function cancel() {
        isEditMode = false;
    }
</script>

<div class="flex flex-row">
    {#if isEditMode}
        <textarea class="textarea p-1 m-1" bind:value={inputBody} />
        <button class="btn btn-sm variant-ghost-surface m-1" on:click={cancel}
            >Cancel</button
        >
        <button class="btn btn-sm variant-ghost-success m-1" on:click={save}
            >Save</button
        >
    {:else if inputBody === undefined}
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
