import { writable, type Writable } from "svelte/store";
import type { DrawerTab, Editable } from "./interfaces/main";
import { invoke } from "@tauri-apps/api/core";
import { activitiesRune, categoriesRune, categoryTagsRune, columnsRune, otherTagsRune } from "./shared.svelte";

export const isDebug: Writable<boolean> = writable(false);
export const previousDrawerTab: Writable<DrawerTab | null> = writable(null)
export const currentEditable: Writable<Editable | null> = writable(null)
export const selectedActivity: Writable<((Actv | OtherActv) & { id: number }) | null> = writable(null)
export const columnDragDisabled: Writable<boolean> = writable(true);
export const hoverColumnId: Writable<null | number> = writable(null);

export const columns: Writable<Map<number, Col>> = writable(new Map())
export const otherActivities: Writable<Map<number, OtherActv>> = writable(new Map())

currentEditable.subscribe((editable) => {
    if (editable !== null) {
        console.info(`Current editable - ${editable.field} ID: ${editable.id}`)
    }
})

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

    const _columns = new Map();
    Object.entries(res.columns).forEach(([columnId, column]) => {
        _columns.set(Number(columnId), column)
    })
    const _otherActivities = new Map()
    Object.entries(res.otherActivities).forEach(([activityId, activity]) => {
        _otherActivities.set(Number(activityId), activity)
    })
    columns.set(_columns);
    otherActivities.set(_otherActivities);

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
    
    Object.entries(res.activities).forEach(([activityId, activity]) => {
        activitiesRune[+activityId] = activity
    })

    Object.entries(res.columns).forEach(([columnId, column]) => {
        columnsRune[+columnId] = {...column, ord: column.ordinal}
    })
}



