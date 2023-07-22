import { writable, type Writable } from "svelte/store";
import type { Activity, Categories, Columns, DrawerTab, Editable, Tag } from "./interfaces/main";
import { baseCategories, mockColumns, mockTags } from "./mock";
import { invoke } from "@tauri-apps/api";

const isMock = true;
export const isDebug: Writable<boolean> = writable(false);
export const selectedActivity: Writable<(Activity & { id: number, columnId: number }) | null> = writable(null)
export const tags: Writable<Map<number, Tag>> = writable(mockTags);
export const categories: Writable<Categories | null> = writable(isMock ? baseCategories : null)
export const previousDrawerTab: Writable<DrawerTab | null> = writable(null)
export const columns: Writable<Columns> = writable(mockColumns)
export const currentEditable: Writable<Editable | null> = writable(null)
currentEditable.subscribe((editable) => {
    if (editable !== null) {
        console.info(`Current editable - ${editable.field} ID: ${editable.id}`)
    }
})

async function queryAllActivities() {
    const res: Record<number | null, {
        name?: string, activities: Record<number, {
            title: string,
            body?: string,
            category_tags: Record<number, {
                category_name: string,
                tag_name: string,
            }>,
            other_tags: Set<string>,
        }>
    }> = await invoke("query_all_activities");
    JSON.stringify(res);
    console.log(res)
}

// setTimeout(async () => {
//     await queryAllActivities()
// }, 3000)