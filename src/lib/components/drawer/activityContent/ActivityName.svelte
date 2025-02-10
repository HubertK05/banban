<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { selectedActivity } from "../../../stores";
  import {
    modalStore,
    toastStore,
    type ToastSettings,
  } from "@skeletonlabs/skeleton";
  import { tick } from "svelte";

  let displayName = $state($selectedActivity.name);
  let isEditMode = $state(false);

  let inputName: string = $state("");
  let inputNode: HTMLInputElement = $state();

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
        id: $selectedActivity.id,
        name: newName,
        body: $selectedActivity.body,
      },
    });
    displayName = newName;
    $selectedActivity.name = newName;
    $selectedActivity = $selectedActivity;

    const runeActivity = activitiesRune[$selectedActivity.id];
    runeActivity.name = $selectedActivity.name;
    activitiesRune[$selectedActivity.id] = runeActivity;
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
    <button class="btn btn-sm variant-ghost-surface m-1" onclick={cancel}
      >Cancel</button
    >
    <button class="btn btn-sm variant-ghost-success m-1" onclick={save}
      >Save</button
    >
  {:else}
    <div class="flex-1 p-2">
      <b>{displayName}</b>
    </div>

    <button class="btn btn-sm variant-ghost-warning m-1" onclick={openEdit}
      >Edit</button
    >
  {/if}
</div>
