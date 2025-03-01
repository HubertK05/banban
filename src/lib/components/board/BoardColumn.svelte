<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import ActivityCard from "./ActivityCard.svelte";
    import { TRIGGERS, dndzone } from "svelte-dnd-action";
    import DebugLabel from "../debug/DebugLabel.svelte";
    import { flip } from "svelte/animate";
    import { ActiveField, type Activity, type Column } from "../../interfaces";
    import { getModalStore, getToastStore, type ModalSettings } from "@skeletonlabs/skeleton";
    import {
        activitiesRune,
        appState,
        columnsRune,
        draggableActivities,
        draggableColumns,
        draggableOtherActivities,
        otherActivitiesRune,
        showToast,
    } from "../../shared.svelte";
    import { event } from "@tauri-apps/api";
    import { tick } from "svelte";
    import Fa from "svelte-fa";
    import { faCheck, faTrashAlt, faXmark } from "@fortawesome/free-solid-svg-icons";

    interface Props {
        columnId: number;
        column: Column;
    }

    let { columnId, column = $bindable() }: Props = $props();
    const flipDurationMs = 125;

    const modalStore = getModalStore();
    const toastStore = getToastStore();

    let columnNameNode: HTMLSpanElement | undefined = $state();

    draggableActivities.update(columnId);

    async function createActivity() {
        const name = "New activity";
        const body = "";
        const tags: number[] = [];
        const res: {
            id: number;
            name: string;
            body?: string;
            column_id?: number;
            ordinal: number;
        } = await invoke("create_activity", {
            data: { name, body, columnId },
        });
        const column = columnsRune[columnId];
        const columnActivities: Map<number, Activity> = column.activities.reduce((acc, id) => {
            acc.set(id, activitiesRune[id]);
            return acc;
        }, new Map());
        Array.from(columnActivities.entries()).forEach(([id, activity]) => {
            activity.ordinal += 1;
            activitiesRune[id] = activity;
        });
        console.log("before:", $state.snapshot(column.activities));
        column.activities.push(res.id);
        console.log("after:", $state.snapshot(column.activities));
        activitiesRune[res.id] = {
            name,
            body,
            tags,
            ordinal: res.ordinal,
        };
        columnsRune[columnId] = column;
        draggableActivities.update(columnId);
    }

    async function handleNameClick() {
        if (appState.currentEditable) return;
        appState.currentEditable = { id: columnId, field: ActiveField.ColumnName, oldName: column.name };
        await tick();

        if (columnNameNode) {
            // Focuses on the column name and moves cursor to the end
            columnNameNode?.focus();
            const range = document.createRange();
            const selection = window.getSelection();

            range.selectNodeContents(columnNameNode);
            range.collapse(false);

            selection?.removeAllRanges();
            selection?.addRange(range);
        }
    }

    async function removeColumn() {
        await invoke("delete_column", { id: columnId });
        const newColumns = Object.entries(columnsRune);
        const index = newColumns.findIndex(([colId, column]) => +colId === columnId);
        newColumns.forEach(([colId, column], idx) => {
            if (idx >= index) {
                column.ord -= 1;
                columnsRune[+colId] = column;
            }
        });
        console.log($state.snapshot(column.activities));
        let columnActivities: Array<[number, Activity]> = Array.from(column.activities).map((activityId) => [
            activityId,
            activitiesRune[activityId],
        ]);
        column.activities.forEach((activityId) => {
            delete activitiesRune[activityId];
        });
        column.activities = [];
        let sortedColumnActivities = columnActivities.sort(([activityId1, activity1], [activityId2, activity2]) => {
            return activity1.ordinal - activity2.ordinal;
        });
        sortedColumnActivities.forEach(([activityId, activity]) => {
            activity.ordinal = Object.entries(otherActivitiesRune.inner).length;
            otherActivitiesRune.inner[activityId] = activity;
        });

        delete columnsRune[columnId];
        draggableColumns.update();
        draggableOtherActivities.update();
    }

    async function handleRenameColumn() {
        if (column.name.trim() === "") {
            showToast(toastStore, "⚠️ Column name cannot be blank");
            return;
        }
        await renameColumn();
        appState.currentEditable = null;
    }

    async function renameColumn() {
        invoke("rename_column", {
            data: { id: columnId, newName: column.name },
        });
    }

    function handleConsider(
        e: DndEvent<{
            id: number;
            activity: Activity;
            colId: number;
        }>,
    ) {
        if (e.info.trigger === TRIGGERS.DRAGGED_ENTERED) {
            appState.hoverColumnId = columnId;
        }
        e.items.forEach(({ id, activity }, index) => {
            activity.ordinal = index;
        });
        draggableActivities.inner[columnId] = e.items;
    }

    async function handleFinalize(
        e: DndEvent<{
            id: number;
            activity: Activity;
            colId: number;
        }>,
    ) {
        appState.hoverColumnId = null;
        const activitiesIds: number[] = e.items.map((x) => x.id);
        e.items = e.items.map(({ id, activity, colId }, index) => {
            activity.ordinal = index;
            activitiesRune[id] = activity;
            return { id, activity, colId: columnId };
        });

        columnsRune[columnId] = {
            ...columnsRune[columnId],
            activities: activitiesIds,
        };

        const activityId = +e.info.id;
        const index = e.items.findIndex(({ id }) => id === activityId);
        if (index !== -1) {
            await invoke("update_activity_column", {
                data: { id: activityId, columnId, newOrd: index },
            });
        }
        draggableActivities.inner[columnId] = e.items;
        draggableColumns.update();
    }

    function showRemoveModal() {
        const body =
            column.activities.length > 0
                ? `${column.activities.length} ${column.activities.length === 1 ? "activity" : "activities"} will be moved to stash.`
                : "Are you sure?";

        const modal: ModalSettings = {
            type: "confirm",
            title: `Remove '${column.name}'`,
            body,

            response: (r: boolean) => {
                if (r) {
                    removeColumn();
                }
            },
        };
        modalStore.trigger(modal);
    }
</script>

<div class="flex flex-col flex-shrink-0 w-72">
    <DebugLabel text={`ID ${columnId}`} />
    <DebugLabel text={`ORD ${column.ord}`} />
    <div class="flex items-center flex-shrink-0 h-10 px-2">
        {#if appState.currentEditable !== null && appState.currentEditable.id === columnId && appState.currentEditable.field === ActiveField.ColumnName}
            <span
                role="textbox"
                tabindex="0"
                contenteditable="true"
                class="block text-sm font-semibold"
                bind:innerText={column.name}
                bind:this={columnNameNode}
                onkeypress={async (e) => {
                    if (e.key === "Enter") {
                        await handleRenameColumn();
                        e.preventDefault();
                    }
                }}
            ></span>
            <button
                aria-label="Confirm renaming column"
                class="flex items-center justify-center w-10 h-10 ml-auto rounded hover:bg-success-hover-token"
                onclick={async () => {
                    await handleRenameColumn();
                }}
            >
                <Fa icon={faCheck} />
            </button>
            <button
                aria-label="Cancel renaming column"
                class="flex items-center justify-center w-10 h-10 ml-auto rounded hover:bg-error-hover-token"
                onclick={() => {
                    if (appState.currentEditable) column.name = appState.currentEditable.oldName;
                    appState.currentEditable = null;
                }}
            >
                <Fa icon={faXmark} />
            </button>
        {:else}
            <button contenteditable="false" onclick={handleNameClick} class="block text-sm font-semibold"
                >{column.name}</button
            >
            <span
                class="flex items-center justify-center w-5 h-5 ml-2 text-sm font-semibold text-indigo-500 bg-white rounded bg-opacity-30"
                >{column.activities.length}</span
            >
            <button
                aria-label="Delete column"
                onclick={showRemoveModal}
                class="flex items-center justify-center w-6 h-6 ml-auto rounded hover:bg-error-hover-token"
            >
                <Fa icon={faTrashAlt} />
            </button>
            <button
                aria-label="Create activity"
                onclick={createActivity}
                class="flex items-center justify-center w-6 h-6 ml-auto text-indigo-500 rounded hover:bg-indigo-500 hover:text-indigo-100"
            >
                <svg class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                    <path
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        stroke-width="2"
                        d="M12 6v6m0 0v6m0-6h6m-6 0H6"
                    />
                </svg>
            </button>
        {/if}
    </div>
    <div class="h-[70vh]">
        <section
            class="flex flex-col pb-2 overflow-auto max-h-full min-h-full {appState.hoverColumnId === columnId
                ? 'shadow-2xl rounded-md'
                : ''}"
            use:dndzone={{
                items: draggableActivities.inner[columnId],
                flipDurationMs,
                type: "activities",
                dropTargetStyle: {},
            }}
            onconsider={(e) => handleConsider(e.detail)}
            onfinalize={(e) => handleFinalize(e.detail)}
        >
            {#each draggableActivities.inner[columnId] as { id, activity, colId } (id)}
                <div animate:flip={{ duration: flipDurationMs }}>
                    <ActivityCard activity={{ ...activity }} {id} columnId={colId} />
                </div>
            {/each}
        </section>
    </div>
</div>
