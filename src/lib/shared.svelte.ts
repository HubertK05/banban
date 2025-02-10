import { invoke } from "@tauri-apps/api/core";
import type { Activity, Category, Column, Tag } from "./interfaces/main";

export const categoriesRune: Record<number, Category> = $state({});
export const categoryTagsRune: Record<number, Tag & { categoryId: number }> = $state({});
export const otherTagsRune: Record<number, Tag> = $state({});
export const activitiesRune: Record<number, Activity> = $state({});
export const columnsRune: Record<number, Column> = $state({});

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

export class DraggableActivities {
    inner: { id: number, colId: number, activity: Activity }[] = $state([])

    update = (column: Column, colId: number) => {
        this.inner = Array.from(column.activities)
            .map(id => {
                const activity = activitiesRune[id];
                return { activity, id, colId };
            })
            .sort((a, b) => {
                return a.activity.ordinal - b.activity.ordinal;
            });
    }
}

export const idTags = new IdTags();
export const idOtherTags = new IdOtherTags();

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
