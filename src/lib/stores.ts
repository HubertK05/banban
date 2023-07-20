import { writable, type Writable } from "svelte/store";
import type { Activity, Columns } from "./interfaces/main";
import { mockColumns } from "./mock";

export const columns: Writable<Columns> = writable(mockColumns)