import { writable, type Writable } from "svelte/store";
import type { Activities, Activity, Categories, Columns, DrawerTab, Editable, NonCategoryTags, Tags } from "./interfaces/main";
import { invoke } from "@tauri-apps/api";


const { _tags, _categories, _otherTags } = await getAllTags()
const _columns = await getAllActivities()

export const isDebug: Writable<boolean> = writable(false);
export const previousDrawerTab: Writable<DrawerTab | null> = writable(null)
export const currentEditable: Writable<Editable | null> = writable(null)
export const selectedActivity: Writable<(Activity & { id: number, columnId: number }) | null> = writable(null)

export const columns: Writable<Columns> = writable(_columns)
export const categories: Writable<Categories | null> = writable(_categories)
export const tags: Writable<Tags> = writable(_tags);
export const nonCategoryTags: Writable<Tags> = writable(_otherTags)

currentEditable.subscribe((editable) => {
    if (editable !== null) {
        console.info(`Current editable - ${editable.field} ID: ${editable.id}`)
    }
})

async function getAllTags() {

    const res: {
        categoryTags: Record<number, { name?: string, ordinal?: number, tags: { tag: string, ordinal: number, id: number, color: string }[] }>,
        otherTags: { name?: string, ordinal?: number, tags: { tag: string, ordinal: number, id: number, color: string }[] }
    } = await invoke("select_all_categories");

    const _categories: Categories = new Map();
    const _tags: Tags = new Map();
    const _otherTags: NonCategoryTags = new Map();
    Object.entries(res.categoryTags).forEach(([categoryId, category]) => {
        const tagIds = category.tags.map(t => t.id)
        category.tags.forEach((tag) => {
            _tags.set(tag.id, { name: tag.tag, ord: tag.ordinal, color: `#${tag.color}` })
        })
        _categories.set(Number(categoryId), { name: category.name, ord: category.ordinal, tags: tagIds })
    })
    res.otherTags.tags.forEach(tag => {
        _otherTags.set(tag.id, { name: tag.tag, ord: tag.ordinal, color: `#${tag.color}` });
    });

    console.debug("tags", _tags)
    console.debug("non category tags", _otherTags)
    console.debug("categories", _categories)
    return { _categories, _tags, _otherTags };
}
async function getAllActivities() {
    const res = await invoke("query_all_activities") as {
        columns: Record<number,
            {
                name?: string,
                columnOrdinal?: number,
                activities: Record<number,
                    {
                        title: string,
                        body?: string,
                        categoryTags: Record<number, {
                            categoryName: string,
                            categoryOrdinal: number,
                            tagId: number
                            tagName: string,
                            tagOrdinal: number
                        }>,
                        otherTags: Record<number, string>,
                        activityOrdinal: number
                    }>
            }>
    };
    const _columns: Columns = new Map();

    Object.entries(res.columns).forEach(([columnId, column]) => {
        const activities: Activities = new Map();
        Object.entries(column.activities).forEach(([activityId, activity]) => {
            const tagIds = Object.entries(activity.categoryTags).map(([tagId, tag]) => tag.tagId)
            Object.entries(activity.otherTags).forEach(([tagId, tag]) => { console.log("tag id:", tagId); tagIds.push(Number(tagId)) });
            activities.set(Number(activityId), { name: activity.title, tags: tagIds, ord: activity.activityOrdinal, body: activity.body })
        })
        _columns.set(Number(columnId), { activities, name: column.name, ord: column.columnOrdinal })
    })

    console.debug("columns", _columns)
    return _columns
}


interface Col {
    name: string,
    ordinal: number
}

interface OtherActv {
    name: string,
    body?: string,
    ordinal: number,
    tags: Array<number>
}

interface Actv {
    name: string,
    body?: string,
    ordinal: number,
    tags: Array<number>,
    columnId: number
}

interface Tag {
    name: string,
    color: string,
    categoryId: number,
    ordinal: number
}

interface OtherTag {
    name: string,
    color: string,
    ordinal: number
}

interface Category {
    name: string,
    ordinal: number,
    tags: Array<number>
}


interface LoadData {
    columns: Map<number, Col>
    activities: Map<number, Actv>
    otherActivities: Map<number, OtherActv>
    categories: Map<number, Category>,
    categoryTags: Map<number, Tag>
    otherTags: Map<number, OtherTag>
}

interface RawLoadData {
    columns: Record<number, Col>
    activities: Record<number, Actv>
    otherActivities: Record<number, OtherActv>
    categories: Record<number, Category>,
    categoryTags: Record<number, Tag>
    otherTags: Record<number, OtherTag>
}
fetchAll()
async function fetchAll() {
    let res = await invoke("fetch_all") as RawLoadData;
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

}