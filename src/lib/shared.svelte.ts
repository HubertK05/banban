import { invoke } from "@tauri-apps/api/core";
import type { Activity, AppState, Category, Column, Tag } from "./interfaces";

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
export const otherActivitiesRune: { inner: Record<number, Activity> } = $state({ inner: {} });

class IdTags {
    // we need both reactivity and functioning drag and drop at once.
    // $derived() rune on idTags confuses the drag and drop library.
    // $state() without explicit update is non-reactive.
    inner: { id: number; tag: Tag & { categoryId: number } }[][] = $state([]);

    update = () => {
        this.inner = Object.entries(categoriesRune).map(([categoryId, category]) => {
            return category.tags
                .map((tagId) => {
                    const tag = categoryTagsRune[tagId];
                    console.assert(tag !== undefined, "Category tag not found");
                    return { id: tagId, tag };
                })
                .sort((idTagA, idTagB) => {
                    return idTagA.tag.ord - idTagB.tag.ord;
                });
        });
    };
}

class IdOtherTags {
    inner: { id: number; tag: Tag }[] = $state([]);

    update = () => {
        this.inner = Object.entries(otherTagsRune)
            .map(([id, tag]) => {
                return { id: +id, tag };
            })
            .sort((idTagA, idTagB) => {
                return idTagA.tag.ord - idTagB.tag.ord;
            });
    };
}

export class DraggableColumns {
    inner: { id: number; column: Column }[] = $state([]);

    update = () => {
        this.inner = Object.entries(columnsRune)
            .map(([id, column]) => {
                return { id: +id, column };
            })
            .sort((colA, colB) => {
                return colA.column.ord - colB.column.ord;
            });
    };
}

export class DraggableOtherActivities {
    inner: { id: number, activity: Activity }[] = $state([]);

    update = () => {
        this.inner = Object.entries(otherActivitiesRune.inner).map(([activityId, activity]) => {
            return { id: +activityId, activity };
        });
    }
}

export const idTags = new IdTags();
export const idOtherTags = new IdOtherTags();
export const draggableColumns = new DraggableColumns();
export const draggableOtherActivities = new DraggableOtherActivities();

export async function changeCategoryTagColor(newColor: string, tagId: number) {
    const tag = categoryTagsRune[tagId];
    await invoke("update_tag_color", {
        data: { categoryTagId: tagId, color: newColor.slice(1) },
    });
    tag.color = newColor;
    categoryTagsRune[tagId] = tag;
}

export async function changeOtherTagColor(newColor: string, tagId: number) {
    const tag = otherTagsRune[tagId];
    await invoke("update_tag_color", {
        data: { categoryTagId: tagId, color: newColor.slice(1) },
    });
    tag.color = newColor;
    otherTagsRune[tagId] = tag;
}

interface BackendCol {
    name: string;
    ordinal: number;
    activities: Array<number>;
}

interface BackendOtherActv {
    name: string;
    body?: string;
    ordinal: number;
    tags: Array<number>;
}

interface BackendActv {
    name: string;
    body?: string;
    ordinal: number;
    tags: Array<number>;
    columnId: number;
}

interface BackendTag {
    name: string;
    color: string;
    ordinal: number;
}

interface BackendCategory {
    name: string;
    ordinal: number;
    tags: Array<number>;
}

interface RawLoadData {
    columns: Record<number, BackendCol>;
    activities: Record<number, BackendActv>;
    otherActivities: Record<number, BackendOtherActv>;
    categories: Record<number, BackendCategory>;
    categoryTags: Record<number, BackendTag>;
    otherTags: Record<number, BackendTag>;
}

export async function fetchAll() {
    const res = (await invoke("fetch_all")) as RawLoadData;
    const categoryIds: Record<number, number> = {};

    Object.entries(res.categories).forEach(([categoryId, category]) => {
        console.log(categoryId);
        categoriesRune[+categoryId] = { ...category, ord: category.ordinal };
        category.tags.forEach((tagId) => {
            categoryIds[tagId] = +categoryId;
        });
    });

    Object.entries(res.categoryTags).forEach(([tagId, tag]) => {
        categoryTagsRune[+tagId] = { ...tag, ord: tag.ordinal, categoryId: categoryIds[+tagId] };
    });

    Object.entries(res.otherTags).forEach(([tagId, tag]) => {
        otherTagsRune[+tagId] = { ...tag, ord: tag.ordinal };
    });

    Object.entries(res.columns).forEach(([columnId, column]) => {
        columnsRune[+columnId] = { ...column, ord: column.ordinal };
    });
    draggableColumns.update();

    Object.entries(res.activities).forEach(([activityId, activity]) => {
        activitiesRune[+activityId] = activity;
    });

    Object.entries(res.otherActivities).forEach(([activityId, activity]) => {
        otherActivitiesRune.inner[+activityId] = activity;
    });
}
