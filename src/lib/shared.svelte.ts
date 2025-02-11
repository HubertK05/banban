import { invoke } from "@tauri-apps/api/core";
import type { Activity, AppState, Category, Column, Tag } from "./interfaces/main";

export const appState: AppState = $state({
    isDebug: false,
    previousDrawerTab: null,
    currentEditable: null,
    selectedActivity: null,
    columnDragDisabled: true,
    hoverColumnId: null,
});

export const categoriesRune: Record<number, Category> = $state({});
export const categoryTagsRune: Record<number, Tag & { categoryId: number }> = $state({});
export const otherTagsRune: Record<number, Tag> = $state({});
export const activitiesRune: Record<number, Activity> = $state({});
export const columnsRune: Record<number, Column> = $state({});
export const otherActivitiesRune: { inner: Record<number, Activity> } = $state({inner: {}});

class IdTags {
    // we need both reactivity and functioning drag and drop at once.
    // $derived() rune on idTags confuses the drag and drop library.
    // $state() without explicit update is non-reactive.
    inner: { id: number, tag: Tag & { categoryId: number } }[][] = $state([])

    update = () => {
        this.inner = Object.entries(categoriesRune).map(([categoryId, category]) => {
            return category.tags.map(tagId => {
                const tag = categoryTagsRune[tagId];
                console.assert(tag !== undefined, "Category tag not found");
                return {id: tagId, tag};
            }).sort((idTagA, idTagB) => {
                return idTagA.tag.ord - idTagB.tag.ord;
            });
        })
    }
}

class IdOtherTags {
    inner: { id: number, tag: Tag }[] = $state([])

    update = () => {
        this.inner = Object.entries(otherTagsRune)
            .map(([id, tag]) => { return {id: +id, tag}; })
            .sort((idTagA, idTagB) => {
                return idTagA.tag.ord - idTagB.tag.ord;
            });
    }
}

export class DraggableColumns {
    inner: { id: number, column: Column }[] = $state([])

    update = () => {
        this.inner = Object.entries(columnsRune)
            .map(([id, column]) => { return { id: +id, column }; })
            .sort((colA, colB) => {
                return colA.column.ord - colB.column.ord;
            });
    }
}

export const idTags = new IdTags();
export const idOtherTags = new IdOtherTags();
export const draggableColumns = new DraggableColumns();

export async function changeCategoryTagColor(
    newColor: string,
    tagId: number,
) {
    const tag = categoryTagsRune[tagId];
    await invoke("update_tag_color", {
        data: { categoryTagId: tagId, color: newColor.slice(1) },
    });
    tag.color = newColor;
    categoryTagsRune[tagId] = tag;
}

export async function changeOtherTagColor(
    newColor: string,
    tagId: number,
) {
    const tag = otherTagsRune[tagId];
    await invoke("update_tag_color", {
        data: { categoryTagId: tagId, color: newColor.slice(1) },
    });
    tag.color = newColor;
    otherTagsRune[tagId] = tag;
}
