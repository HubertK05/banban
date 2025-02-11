import { invoke } from "@tauri-apps/api/core";
import { activitiesRune, categoriesRune, categoryTagsRune, columnsRune, draggableColumns, otherActivitiesRune, otherTagsRune } from "./shared.svelte";

export interface Col {
    name: string,
    ordinal: number
    activities: Array<number>
}

export interface OtherActv {
    name: string,
    body?: string,
    ordinal: number,
    tags: Array<number>
}

export interface Actv {
    name: string,
    body?: string,
    ordinal: number,
    tags: Array<number>,
    columnId: number
}

export interface Tag {
    name: string,
    color: string,
    ordinal: number
}

export interface Category {
    name: string,
    ordinal: number,
    tags: Array<number>
}


export interface LoadData {
    columns: Map<number, Col>
    activities: Map<number, Actv>
    otherActivities: Map<number, OtherActv>
    categories: Map<number, Category>,
    categoryTags: Map<number, Tag & { categoryId: number }>
    otherTags: Map<number, Tag>
}

export interface RawLoadData {
    columns: Record<number, Col>
    activities: Record<number, Actv>
    otherActivities: Record<number, OtherActv>
    categories: Record<number, Category>,
    categoryTags: Record<number, Tag>
    otherTags: Record<number, Tag>
}

export async function fetchAll() {
    const res = await invoke("fetch_all") as RawLoadData;
    const categoryIds: Record<number, number> = {};

    Object.entries(res.categories).forEach(([categoryId, category]) => {
        console.log(categoryId);
        categoriesRune[+categoryId] = {...category, ord: category.ordinal}
        category.tags.forEach(tagId => {
            categoryIds[tagId] = +categoryId
        })
    })

    Object.entries(res.categoryTags).forEach(([tagId, tag]) => {
        categoryTagsRune[+tagId] = {...tag, ord: tag.ordinal, categoryId: categoryIds[+tagId]}
    })

    Object.entries(res.otherTags).forEach(([tagId, tag]) => {
        otherTagsRune[+tagId] = {...tag, ord: tag.ordinal}
    })

    Object.entries(res.columns).forEach(([columnId, column]) => {
        columnsRune[+columnId] = {...column, ord: column.ordinal}
    })
    draggableColumns.update();
    
    Object.entries(res.activities).forEach(([activityId, activity]) => {
        activitiesRune[+activityId] = activity
    })

    Object.entries(res.otherActivities).forEach(([activityId, activity]) => {
        otherActivitiesRune.inner[+activityId] = activity
    });
}



