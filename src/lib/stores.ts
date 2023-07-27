import { writable, type Writable } from "svelte/store";
import type { DrawerTab, Editable } from "./interfaces/main";
import { invoke } from "@tauri-apps/api";


const data = await fetchAll()
export const isDebug: Writable<boolean> = writable(false);
export const previousDrawerTab: Writable<DrawerTab | null> = writable(null)
export const currentEditable: Writable<Editable | null> = writable(null)
export const selectedActivity: Writable<((Actv | OtherActv) & { id: number }) | null> = writable(null)

export const columns: Writable<Map<number, Col>> = writable(data.columns)
export const activities: Writable<Map<number, Actv>> = writable(data.activities);
export const otherActivities: Writable<Map<number, OtherActv>> = writable(data.otherActivities)
export const categories: Writable<Map<number, Category>> = writable(data.categories)
export const tags: Writable<Map<number, Tag & { categoryId: number }>> = writable(data.categoryTags);
export const otherTags: Writable<Map<number, Tag>> = writable(data.otherTags)

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

async function fetchAll() {
    const res = await invoke("fetch_all") as RawLoadData;

    const columns = new Map();
    Object.entries(res.columns).forEach(([columnId, column]) => {
        columns.set(Number(columnId), column)
    })
    const activities = new Map()
    Object.entries(res.activities).forEach(([activityId, activity]) => {
        activities.set(Number(activityId), activity)
    })
    const otherActivities = new Map()
    Object.entries(res.otherActivities).forEach(([activityId, activity]) => {
        otherActivities.set(Number(activityId), activity)
    })
    const categories = new Map()
    Object.entries(res.categories).forEach(([categoryId, category]) => {
        categories.set(Number(categoryId), category)
    })
    const categoryTags = new Map()
    Object.entries(res.categoryTags).forEach(([tagId, tag]) => {
        categoryTags.set(Number(tagId), tag)
    })
    const otherTags = new Map()
    Object.entries(res.otherTags).forEach(([tagId, tag]) => {
        otherTags.set(Number(tagId), tag)
    })
    const data: LoadData = { columns, activities, otherActivities, categories, categoryTags, otherTags }
    console.debug(data)
    return data
}