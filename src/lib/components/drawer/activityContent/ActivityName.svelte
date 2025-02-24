<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { getToastStore } from "@skeletonlabs/skeleton";
    import { tick } from "svelte";
    import { activitiesRune, showToast } from "../../../shared.svelte";

    interface Props {
        activityId: number;
    }

    const { activityId }: Props = $props();
    console.assert(activitiesRune[activityId] !== undefined, "Selected activity is undefined");
    const selectedActivity = $derived(activitiesRune[activityId]);
    let displayName = $derived(selectedActivity.name);

    const toastStore = getToastStore();

    let isEditMode = $state(false);
    let inputName: string = $state("");
    let inputNode: HTMLInputElement | undefined = $state();

    async function openEdit() {
        inputName = displayName;
        isEditMode = true;
        await tick();
        inputNode?.focus();
    }

    async function save() {
        const trimmedName = inputName.trim();
        if (displayName === trimmedName) {
            showToast(toastStore, "📝 Set the same activity name");
            isEditMode = false;
            return;
        }
        if (trimmedName.length === 0) {
            showToast(toastStore, "⚠️ Activity name cannot be blank");
            return;
        }

        await sync(trimmedName);
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

    async function sync(newName: string) {
        invoke("update_activity_content", {
            data: {
                id: activityId,
                name: newName,
                body: selectedActivity.body,
            },
        });
        activitiesRune[activityId].name = newName;
    }
</script>

<div class="flex flex-row">
    {#if isEditMode}
        <input
            type="text"
            class="input p-1 m-1"
            bind:value={inputName}
            onkeypress={handleKeyPress}
            bind:this={inputNode}
            placeholder="New activity name"
        />
        <button class="btn btn-sm variant-ghost-surface m-1" onclick={cancel}>Cancel</button>
        <button class="btn btn-sm variant-ghost-success m-1" onclick={save}>Save</button>
    {:else}
        <div class="flex-1 p-2">
            <b>{displayName}</b>
        </div>

        <button class="btn btn-sm variant-ghost-warning m-1" onclick={openEdit}>Edit</button>
    {/if}
</div>
