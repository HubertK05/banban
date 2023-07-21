import { writable, type Writable } from "svelte/store";
import type { Activity, Categories, Columns, DrawerTab, Editable } from "./interfaces/main";
import { baseCategories, mockColumns } from "./mock";
import { invoke } from "@tauri-apps/api";

const isMock = true;
export const isDebug: Writable<boolean> = writable(false);
export const selectedActivity: Writable<(Activity & { id: number }) | null> = writable(null)
export const categories: Writable<Categories | null> = writable(isMock ? baseCategories : null)
export const previousDrawerTab: Writable<DrawerTab | null> = writable(null)
export const columns: Writable<Columns> = isMock ? writable(mockColumns) : writable(await invoke("get_all_columns"))
export const currentEditable: Writable<Editable | null> = writable(null)
currentEditable.subscribe((editable) => {
    if (editable !== null) {
        console.info(`Current editable - ${editable.field} ID: ${editable.id}`)
    }
})