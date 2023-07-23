import { writable, type Writable } from "svelte/store";
import type { Activities, Activity, Categories, Columns, DrawerTab, Editable, Tag, Tags } from "./interfaces/main";
import { baseCategories, mockColumns, mockTags } from "./mock";
import { invoke } from "@tauri-apps/api";


// const serverData1 = await getAllCategories()
// const serverColumns = await queryAllActivities()
const isMock = false;
export const isDebug: Writable<boolean> = writable(false);
export const selectedActivity: Writable<(Activity & { id: number, columnId: number }) | null> = writable(null)
export const tags: Writable<Map<number, Tag>> = writable(mockTags);
export const categories: Writable<Categories | null> = writable(baseCategories)
export const previousDrawerTab: Writable<DrawerTab | null> = writable(null)
export const columns: Writable<Columns> = writable(mockColumns)
export const currentEditable: Writable<Editable | null> = writable(null)
currentEditable.subscribe((editable) => {
    if (editable !== null) {
        console.info(`Current editable - ${editable.field} ID: ${editable.id}`)
    }
})

async function main() {
    const {_tags, _categories} = await getAllCategories()
    const _columns = await queryAllActivities()   
    console.debug("tags", _tags);
    console.debug("categories", _categories)
    console.debug("columns", _columns)
    tags.set(_tags)
    console.info("Tags set up")
    categories.set(_categories)
    console.info("Categories set up")
    columns.set(new Map())
    columns.set(_columns)
    console.info("Columns set up")

}

main()

async function getAllCategories() {

    const res: {
        categoryTags: Record<number, {name?: string, ordinal?: number, tags: {tag: string, ordinal: number, id: number}[]}>, 
        otherTags: {name?: string, ordinal?: number, tags: {tag: string, ordinal: number, id: number}[]}[]} = await invoke("select_all_categories");

    const _categories: Categories = new Map();
    const _tags: Tags = new Map();
    Object.entries(res.categoryTags).forEach(([categoryId, category])=>{
        const tagIds = category.tags.map(t=>t.id)
        category.tags.forEach((tag)=>{
            _tags.set(tag.id, {name: tag.tag, ord: tag.ordinal})
        })
        _categories.set(Number(categoryId), {name: category.name, ord: category.ordinal, tags: tagIds})
    })
    // res.otherTags.forEach(tag=>{
    //    // do something with other tags
    // })
    tags.set(_tags)
    categories.set(_categories)
    console.debug("tags",_tags)
    console.debug("categories", _categories)
    return {  _categories, _tags };
}
async function queryAllActivities() {
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
                otherTags: string[],
                activityOrdinal: number}>
        }>
    };
    const _columns: Columns = new Map();

    Object.entries(res.columns).forEach(([columnId,column])=>{
        const activities: Activities = new Map();
        Object.entries(column.activities).forEach(([activityId, activity])=>{
            const tagIds = Object.entries(activity.categoryTags).map(([tagId,tag])=>tag.tagId)
            activities.set(Number(activityId), {name: activity.title, tags: tagIds, ord: activity.activityOrdinal, body: activity.body})
        })
        _columns.set(Number(columnId), {activities, name: column.name, ord: column.columnOrdinal})
    })
    console.debug("columns", _columns)
    return _columns
}

